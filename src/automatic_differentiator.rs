use crate::{number::Number, operation::Operation, shared_data_communication_channel};

use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use statrs::distribution::{Continuous, Normal};
use std::{collections::HashMap, sync::Mutex};

use sorted_vec::SortedVec;

static DATA_RACE: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

#[derive(Debug, Clone)]
pub struct Evaluation {
    pub result: f64,
    pub derivatives: Vec<Derivative>,
}

#[derive(Debug, Clone)]
pub struct Derivative {
    pub input: Number,
    pub derivative: f64,
}

#[derive(Debug, Clone)]
pub struct AutomaticDifferentiator {
    record: HashMap<i64, Operation>,
    node_list: SortedVec<i64>,
    parent_child_map: OrderedHashMap<i64, Vec<i64>>,
    child_parent_map: OrderedHashMap<i64, Vec<i64>>,
}

impl Default for AutomaticDifferentiator {
    fn default() -> Self {
        AutomaticDifferentiator::new()
    }
}

impl AutomaticDifferentiator {
    pub fn new() -> Self {
        AutomaticDifferentiator {
            record: HashMap::new(),
            node_list: SortedVec::new(),
            parent_child_map: OrderedHashMap::new(),
            child_parent_map: OrderedHashMap::new(),
        }
    }

    pub fn derivatives<F>(&mut self, func: F, arguments: &[Number]) -> Evaluation
    where
        F: Fn(&[Number]) -> Number,
    {
        let forward_evalutation = self.forward_evaluate(func, arguments);
        self.reverse_propagate_adjoints();

        let derivatives = arguments
            .iter()
            .filter_map(|arg| {
                self.record.get(&arg.id).and_then(|op| {
                    if let Operation::Value(_, _, _) = op {
                        Some((arg, op.get_adjoint()))
                    } else {
                        None
                    }
                })
            })
            .map(|der| Derivative {
                input: *der.0,
                derivative: der.1,
            })
            .collect();

        Evaluation {
            result: forward_evalutation.result,
            derivatives,
        }
    }

    fn forward_evaluate<F>(&mut self, func: F, arguments: &[Number]) -> Number
    where
        F: Fn(&[Number]) -> Number,
    {
        // Lock to avoid data races. Effectively serializes calls to forward_evaluate across all instances of AutomaticDifferentiator
        let _lock = DATA_RACE.lock().unwrap();

        // Run forward evaluate. This does not require much compute.
        let eval_res = func(arguments);

        // take local copy from which everything else is evaluated
        self.record = shared_data_communication_channel::global_record_clone();
        self.node_list = shared_data_communication_channel::global_node_list_clone();
        self.parent_child_map = shared_data_communication_channel::global_parent_child_map_clone();
        self.child_parent_map = shared_data_communication_channel::global_child_parent_map_clone();

        // clear communications channel after use, before releasing data race lock
        shared_data_communication_channel::global_clear();

        eval_res
    }

    fn reverse_propagate_adjoints(&mut self) {
        print!("Running reverse mode adjoint propagation");

        // Set adjoint of f() = y to 1.0. i.e. set value of the last entry to 1.0.
        if let Some(last_id) = self.node_list.last() {
            println!("Setting adjoint to 1.0 for id {}", last_id);
            if let Some(rec) = self.record.get_mut(last_id) {
                rec.set_adjoint(1.0);
            }
        }

        // Reverse through the rest of the nodes, except the last which has already been set to 1.0
        for node_map_entry in self.node_list.iter().rev().skip(1) {
            // Implement the adjoint equation
            let mut adjoint = 0.0;
            if let Some(node) = self.record.get(node_map_entry) {
                let node_id = node.get_id();

                println!("Calculating adjoint for node Vi with id {}", node_id);
                if let Some(parents) = self.child_parent_map.get(&node_id) {
                    for parent in parents {
                        if let Some(parent_operation) = self.record.get(parent) {
                            let parent_adj = parent_operation.get_adjoint();
                            let parent_id = parent_operation.get_id();
                            match parent_operation {
                                // lhs_ = parent_ * Dparent/Dlhs = parent_ * 1
                                // rhs_ = parent_ * Dparent/Drhs = parent_ * 1
                                Operation::Add(_, _, _, _, _) => {
                                    adjoint += parent_adj; // lhs, rhs are identical, so this is enough
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, parent_id
                                    );
                                }
                                Operation::Sub(_, lhs_id, rhs_id, _, _) => {
                                    // lhs_ = parent_ * Dparent/Dlhs = parent_
                                    if node_id == *lhs_id {
                                        adjoint += parent_adj;
                                    }
                                    // rhs_ = parent_ * Dparent/Drhs = -1 * parent_
                                    if node_id == *rhs_id {
                                        adjoint -= parent_adj;
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, parent_id
                                    );
                                }
                                Operation::Mul(_, lhs_id, rhs_id, _, _) => {
                                    // lhs_ = parent_ * Dparent/Dlhs = parent_ * rhs
                                    if node_id == *lhs_id {
                                        if let Some(rhs) = self.record.get(rhs_id) {
                                            adjoint += parent_adj * rhs.get_result();
                                        }
                                    }
                                    // rhs_ = parent_ * Dparent/Drhs = parent_ * lhs
                                    if node_id == *rhs_id {
                                        if let Some(lhs) = self.record.get(lhs_id) {
                                            adjoint += parent_adj * lhs.get_result();
                                        }
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, parent_id
                                    );
                                }
                                Operation::Div(_, num_id, den_id, _, _) => {
                                    // num_ = parent_ * Dparent/Dnum = parent_ * 1/den
                                    if node_id == *num_id {
                                        if let Some(den) = self.record.get(den_id) {
                                            adjoint += parent_adj / den.get_result();
                                        }
                                    }
                                    // den_ = parent_ * Dparent/Dden = parent_ * -1 * (num/den^2)
                                    if node_id == *den_id {
                                        if let Some(num) = self.record.get(num_id) {
                                            if let Some(den) = self.record.get(den_id) {
                                                let num = num.get_result();
                                                let den = den.get_result();
                                                adjoint -= parent_adj * num / (den * den);
                                            }
                                        }
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, parent_id
                                    );
                                }
                                Operation::Ln(_, arg_id, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * 1/arg
                                    if let Some(arg) = self.record.get(arg_id) {
                                        adjoint += parent_adj / arg.get_result();
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Sin(_, arg_id, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * cos(arg)
                                    if let Some(arg) = self.record.get(arg_id) {
                                        adjoint += parent_adj * arg.get_result().cos();
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Cos(_, arg_id, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * -sin(arg)
                                    if let Some(arg) = self.record.get(arg_id) {
                                        adjoint -= parent_adj * arg.get_result().sin();
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Exp(_, _, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * result (d(e^x)/dx = e^x)
                                    adjoint += parent_adj * parent_operation.get_result();
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, parent_id
                                    );
                                }
                                Operation::Pow(_, base_id, exp, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * exp * base ^ (exp - 1)
                                    if let Some(base) = self.record.get(base_id) {
                                        let exp = *exp;
                                        adjoint += parent_adj * exp * base.get_result().powf(exp - 1.0);
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Sqrt(_, arg_id, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * (1 / (2*sqrt(x)))
                                    if let Some(arg) = self.record.get(arg_id) {
                                        adjoint += parent_adj / (2.0 * arg.get_result().sqrt());
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Log(_, arg_id, base, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * (1/(arg*ln(base)))
                                    if let Some(arg) = self.record.get(arg_id) {
                                        adjoint += parent_adj / (arg.get_result() * base.ln());
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Cdf(_, arg_id, _, _) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * pdf(x)
                                    if let Some(arg) = self.record.get(arg_id) {
                                        let norm = Normal::new(0.0, 1.0).unwrap();
                                        adjoint += parent_adj * norm.pdf(arg.get_result());
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, parent_id
                                        );
                                    }
                                }
                                Operation::Value(_, _, _) => {
                                    adjoint += parent_adj;
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, parent_id
                                    );
                                }
                            };
                        }
                    }
                }
            }

            // update adjoint of record with node_id
            if let Some(node) = self.record.get_mut(node_map_entry) {
                node.add_adjoint(adjoint);
            }
        }
    }

    pub fn print_parent_map(&self) {
        for kv in self.parent_child_map.iter() {
            if let Some(rec) = self.record.get(kv.0) {
                println!("{0}", rec);
            }
        }
    }

    pub fn print_parent_map_id(&self) {
        for kv in self.parent_child_map.iter() {
            let children: Vec<String> = kv.1.iter().map(|x| x.to_string()).collect();
            println!("parent {0}. Children: {1}", kv.0, children.join(", "));
        }
    }

    pub fn print_child_map_id(&self) {
        println!("Printing child map id");

        for kv in self.child_parent_map.iter() {
            let children: Vec<String> = kv.1.iter().map(|x| x.to_string()).collect();
            println!("Child {0}. Parent: {1}", kv.0, children.join(", "));
        }
    }

    pub fn print_nodes_list(&self) {
        println!("Printing nodes list");
        for node in self.node_list.iter() {
            println!("{0}", node);
        }
    }

    pub fn print_record_collection(&self) {
        println!("Printing record collection");
        for kv in self.record.iter() {
            println!("{0}", kv.1);
        }
    }

    pub fn print_record_collection_value_operations(&self) {
        let value_operations = self
            .record
            .iter()
            .filter(|(_, op)| matches!(op, Operation::Value(_, _, _)));
        for (_, op) in value_operations {
            println!("{0}", op);
        }
    }

    pub fn print_graph(&self) {
        println!("digraph G {{");
        for kv in self.parent_child_map.iter() {
            for child in kv.1.iter() {
                let parent_record_option = self.record.get(kv.0);
                let child_record_option = self.record.get(child);
                if let Some(parent_record) = parent_record_option {
                    if let Some(child_record) = child_record_option {
                        println!(
                            "{} -> {};",
                            parent_record.get_graph_string(),
                            child_record.get_graph_string()
                        );
                    }
                }
            }
        }
        println!("}}");
    }
}

