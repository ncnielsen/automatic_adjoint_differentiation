use crate::{automatic_differentiator::AutomaticDifferentiator, number::Number};

pub mod automatic_differentiator;
pub mod global_counter;
pub mod number;
pub mod operation;

fn f(args: Vec<Number>) -> Number {
    let y1 = args[2] * (args[4] * args[0] + args[1]);
    let y2 = y1.log();
    let y = (y1 + args[3] * y2) * (y1 + y2);
    y
}
fn main() {
    let automatic_differentiator = AutomaticDifferentiator::new();

    let arguments = vec![
        Number::new(1.0),
        Number::new(2.0),
        Number::new(3.0),
        Number::new(4.0),
        Number::new(5.0),
    ];

    let _forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

    println!("printing parent map after forward evaluation");
    automatic_differentiator::print_parent_map_id();

    let _backward_prop = automatic_differentiator.backward_propagate();

    println!("Printing parent map after back propagation");
    automatic_differentiator::print_parent_map_id();

    println!("Printing child map after back propagation");
    automatic_differentiator::print_child_map_id();

    println!("\n\nPrinting record colection values after back propagation");
    automatic_differentiator::print_record_collection();
}
