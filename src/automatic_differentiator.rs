use crate::{number::Number, operation::Operation, shared_data_communication_channel};

use once_cell::sync::Lazy;
use ordered_hash_map::OrderedHashMap;
use std::{collections::HashMap, sync::Mutex};

use sorted_vec::SortedVec;

static DATA_RACE: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

#[derive(Debug, Clone)]
pub struct AutomaticDifferentiator {
    record: HashMap<i64, Operation>,
    node_list: SortedVec<i64>,
    parent_child_map: OrderedHashMap<i64, Vec<i64>>,
    child_parent_map: OrderedHashMap<i64, Vec<i64>>,
}

impl AutomaticDifferentiator {
    pub fn new() -> Self {
        let ad = AutomaticDifferentiator {
            record: HashMap::new(),
            node_list: SortedVec::new(),
            parent_child_map: OrderedHashMap::new(),
            child_parent_map: OrderedHashMap::new(),
        };
        ad
    }

    pub fn forward_evaluate<F>(&mut self, func: F, arguments: Vec<Number>) -> Number
    where
        F: Fn(Vec<Number>) -> Number,
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

    pub fn reverse_propagate_adjoints(&mut self) {
        print!("Running reverse mode adjoint propagation");

        // Set adjoint of f() = y to 1.0. i.e. set value of the last entry to 1.0.
        if let Some(last_id) = self.node_list.last() {
            println!("Setting adjoint to 1.0 for id {}", last_id);

            if let Some(rec) = self.record.get_mut(last_id) {
                match rec {
                    Operation::Add(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Sub(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Mul(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Div(_, _, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Ln(_, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Sin(_, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Exp(_, _, _, adjoint) => *adjoint = 1.0,
                    Operation::Value(_, _, adjoint) => *adjoint = 1.0,
                }
            }
        }

        // Reverse through the rest of the nodes, except the last which has already been set to 1.0
        for node_map_entry in self.node_list.iter().rev().skip(1) {
            // Implement the adjoint equation
            let mut adjoint = 0.0;
            if let Some(node) = self.record.get(node_map_entry) {
                let node_id = match node {
                    Operation::Add(id, _lhs_id, _rhs_id, _res, _adj) => id,
                    Operation::Sub(id, _lhs_id, _rhs_id, _res, _adj) => id,
                    Operation::Mul(id, _lhs_id, _rhs_id, _res, _adj) => id,
                    Operation::Div(id, _num_id, _den_id, _res, _adj) => id,
                    Operation::Ln(id, _arg_id, _res, _adj) => id,
                    Operation::Sin(id, _arg_id, _res, _adj) => id,
                    Operation::Exp(id, _arg_id, _res, _adj) => id,
                    Operation::Value(id, _res, _adj) => id,
                };

                println!("Calculating adjoint for node Vi with id {}", node_id);
                if let Some(parents) = self.child_parent_map.get(&node_id) {
                    for parent in parents {
                        if let Some(parent_operation) = self.record.get(&parent) {
                            match parent_operation {
                                // lhs_ = parent_ * Dparent/Dlhs = parent_ * 1
                                // rhs_ = parent_ * Dparent/Drhs = parent_ * 1
                                Operation::Add(id, _lhs_id, _rhs_id, _res, adj) => {
                                    adjoint += adj; // lhs, rhs are identical, so this is enough
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                                Operation::Sub(id, lhs_id, rhs_id, _res, adj) => {
                                    // lhs_ = parent_ * Dparent/Dlhs = parent_
                                    if node_id == lhs_id {
                                        adjoint += adj;
                                    }

                                    // rhs_ = parent_ * Dparent/Drhs = -1 * parent_
                                    if node_id == rhs_id {
                                        adjoint += adj * -1.0;
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                                Operation::Mul(id, lhs_id, rhs_id, _res, adj) => {
                                    // lhs_ = parent_ * Dparent/Dlhs = parent_ * rhs
                                    if node_id == lhs_id {
                                        if let Some(rhs) = self.record.get(rhs_id) {
                                            let rhs = get_res_from_operation(&rhs);
                                            adjoint += adj * rhs;
                                        }
                                    }

                                    // rhs_ = parent_ * Dparent/Drhs = parent_ * lhs
                                    if node_id == rhs_id {
                                        if let Some(lhs) = self.record.get(lhs_id) {
                                            let lhs = get_res_from_operation(&lhs);
                                            adjoint += adj * lhs;
                                        }
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                                Operation::Div(id, num_id, den_id, _res, adj) => {
                                    // num_ = parent_ * Dparent/Dnum = parent_ * 1/den
                                    if node_id == num_id {
                                        if let Some(den) = self.record.get(den_id) {
                                            let den = get_res_from_operation(&den);
                                            adjoint += adj * 1.0 / den;
                                        }
                                    }

                                    // den_ = parent_ * Dparent/Dden = parent_ * -1 * (num/den^2)
                                    if node_id == den_id {
                                        if let Some(num) = self.record.get(num_id) {
                                            if let Some(den) = self.record.get(den_id) {
                                                let num = get_res_from_operation(&num);
                                                let den = get_res_from_operation(&den);
                                                adjoint += adj * -1.0 * (num / (den * den));
                                            }
                                        }
                                    }
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }

                                Operation::Ln(id, arg_id, _res, adj) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * 1/arg
                                    if let Some(arg) = self.record.get(arg_id) {
                                        let arg = get_res_from_operation(&arg);
                                        let arg_res = arg;
                                        adjoint += adj * (1.0 / arg_res);
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, id
                                        );
                                    }
                                }
                                Operation::Sin(id, arg_id, _res, adj) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * cos(arg)
                                    if let Some(arg) = self.record.get(arg_id) {
                                        let arg = get_res_from_operation(&arg);
                                        adjoint += adj * arg.cos();
                                        println!(
                                            "node with id {} has adjoint {}. ParentId: {}",
                                            node_id, adjoint, id
                                        );
                                    }
                                }
                                Operation::Exp(id, _arg_id, res, adj) => {
                                    // arg_ = parent_ * Dparent / Darg = parent_ * 1.0 * res
                                    adjoint += adj * res;
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                                Operation::Value(id, _res, adj) => {
                                    adjoint += adj;
                                    println!(
                                        "node with id {} has adjoint {}. ParentId: {}",
                                        node_id, adjoint, id
                                    );
                                }
                            };
                        }
                    }
                }
            }

            // update adjoint of record with node_id
            if let Some(node) = self.record.get_mut(node_map_entry) {
                match node {
                    Operation::Add(_id, _lhs_id, _rhs_id, _res, adj) => *adj += adjoint,
                    Operation::Sub(_id, _lhs_id, _rhs_id, _res, adj) => *adj += adjoint,
                    Operation::Mul(_id, _lhs_id, _rhs_id, _res, adj) => *adj += adjoint,
                    Operation::Div(_id, _num_id, _den_id, _res, adj) => *adj += adjoint,
                    Operation::Ln(_id, _arg_id, _res, adj) => *adj += adjoint,
                    Operation::Sin(_id, _arg_id, _res, adj) => *adj += adjoint,
                    Operation::Exp(_id, _arg_id, _res, adj) => *adj += adjoint,
                    Operation::Value(_id, _res, adj) => *adj += adjoint,
                };
            }
        }
    }

    pub fn get_differentials(&self) -> Vec<Operation> {
        self.record
            .values()
            .filter(|op| matches!(op, Operation::Value(_, _, _)))
            .cloned()
            .collect()
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
        for kv in self.child_parent_map.iter() {
            let children: Vec<String> = kv.1.iter().map(|x| x.to_string()).collect();
            println!("Child {0}. Parent: {1}", kv.0, children.join(", "));
        }
    }

    pub fn print_record_collection(&self) {
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

    pub fn print_differentials(&self, args: Vec<Number>) {
        let differentials = self.get_differentials();
        let adjoints: Vec<(i64, f64, f64)> = differentials
            .iter()
            .map(|op| match op {
                Operation::Value(id, res, adj) => (*id, *res, *adj),
                _ => (0, 0.0, 0.0),
            })
            .collect();

        for arg in args {
            let diff = adjoints
                .iter()
                .filter(|x| x.0 == arg.id)
                .map(|x| x.2)
                .next()
                .unwrap();
            println!(
                "Argument with id {} and Value {} has differential {}",
                arg.id, arg.result, diff
            );
        }
    }
}

fn get_res_from_operation(op: &Operation) -> f64 {
    match op {
        Operation::Add(_, _, _, res, _) => *res,
        Operation::Sub(_, _, _, res, _) => *res,
        Operation::Mul(_, _, _, res, _) => *res,
        Operation::Div(_, _, _, res, _) => *res,
        Operation::Ln(_, _, res, _) => *res,
        Operation::Sin(_, _, res, _) => *res,
        Operation::Exp(_, _, res, _) => *res,
        Operation::Value(_, res, _) => *res,
    }
}

#[cfg(test)]
mod automatic_differentiator_tests {
    use super::*;

    #[test]
    fn test_operators_add_mul_ln() {
        let mut automatic_differentiator = AutomaticDifferentiator::new();

        let x1 = Number::new(1.0);
        let x2 = Number::new(2.0);
        let x3 = Number::new(3.0);
        let x4 = Number::new(4.0);
        let x5 = Number::new(5.0);

        let arguments = vec![x1, x2, x3, x4, x5];

        fn f(args: Vec<Number>) -> Number {
            let y1 = args[2] * (args[4] * args[0] + args[1]);
            let y2 = y1.ln();
            let y = (y1 + args[3] * y2) * (y1 + y2);
            y
        }

        let forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

        automatic_differentiator.reverse_propagate_adjoints();
        let differentials = automatic_differentiator.get_differentials();
        assert_eq!(differentials.len(), 5);

        let adjoints: Vec<(i64, f64, f64)> = differentials
            .iter()
            .map(|op| match op {
                Operation::Value(id, res, adj) => (*id, *res, *adj),
                _ => (0, 0.0, 0.0),
            })
            .collect();

        let x1 = adjoints
            .iter()
            .filter(|x| x.0 == x1.id)
            .map(|x| x.2)
            .next()
            .unwrap();
        let x2 = adjoints
            .iter()
            .filter(|x| x.0 == x2.id)
            .map(|x| x.2)
            .next()
            .unwrap();
        let x3 = adjoints
            .iter()
            .filter(|x| x.0 == x3.id)
            .map(|x| x.2)
            .next()
            .unwrap();
        let x4 = adjoints
            .iter()
            .filter(|x| x.0 == x4.id)
            .map(|x| x.2)
            .next()
            .unwrap();
        let x5 = adjoints
            .iter()
            .filter(|x| x.0 == x5.id)
            .map(|x| x.2)
            .next()
            .unwrap();

        let epsilon = 1e-10;
        assert!(forward_eval.result - 797.75132345616487 < epsilon);
        assert!(x1 - 950.7364539019619 < epsilon);
        assert!(x2 - 190.14729078039238 < epsilon);
        assert!(x3 - 443.6770118209156 < epsilon);
        assert!(x4 - 73.20408806599326 < epsilon);
        assert!(x5 - 190.14729078039238 < epsilon);
    }

    #[test]
    fn test_operators_sub_sin_div() {
        let mut automatic_differentiator = AutomaticDifferentiator::new();

        let x1 = Number::new(1.5);
        let x2 = Number::new(0.5);

        let arguments = vec![x1, x2];

        fn f(args: Vec<Number>) -> Number {
            let x1 = args[0];
            let x2 = args[1];
            let frac = x1 / x2;
            (frac.sin() + frac - x2.exp()) * (frac - x2.exp())
        }

        let forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

        automatic_differentiator.reverse_propagate_adjoints();
        let differentials = automatic_differentiator.get_differentials();
        assert_eq!(differentials.len(), 2);

        let adjoints: Vec<(i64, f64, f64)> = differentials
            .iter()
            .map(|op| match op {
                Operation::Value(id, res, adj) => (*id, *res, *adj),
                _ => (0, 0.0, 0.0),
            })
            .collect();

        let x1 = adjoints
            .iter()
            .filter(|x| x.0 == x1.id)
            .map(|x| x.2)
            .next()
            .unwrap();
        let x2 = adjoints
            .iter()
            .filter(|x| x.0 == x2.id)
            .map(|x| x.2)
            .next()
            .unwrap();
        let epsilon = 1e-10;
        assert!(forward_eval.result - 2.017 < epsilon);
        assert!(x1 - 3.0118433276739069 < epsilon);
        assert!(x2 - (-13.723961509314076) < epsilon);
    }
}
