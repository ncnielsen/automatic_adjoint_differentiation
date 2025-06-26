use crate::{automatic_differentiator::AutomaticDifferentiator, number::Number};

pub mod automatic_differentiator;
pub mod global_counter;
pub mod number;
pub mod operation;

fn f(args: Vec<Number>) -> Number {
    let x1 = args[0];
    let x2 = args[1];
    let frac = x1 / x2;

    frac
}

fn main() {
    let automatic_differentiator = AutomaticDifferentiator::new();

    let arguments = vec![Number::new(1.5), Number::new(0.5)];

    let _forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

    println!("forward eval {}", _forward_eval);
    println!("printing parent map after forward evaluation");
    automatic_differentiator::print_parent_map_id();

    automatic_differentiator.reverse_propagate_adjoints();

    // println!("Printing parent map after back propagation");
    // automatic_differentiator::print_parent_map_id();

    // println!("Printing child map after back propagation");
    // automatic_differentiator::print_child_map_id();

    // println!("\n\nPrinting record colection values after back propagation");
    // automatic_differentiator::print_record_collection();
}
