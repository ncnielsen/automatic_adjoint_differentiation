use crate::{automatic_differentiator::AutomaticDifferentiator, number::Number};

pub mod automatic_differentiator;
pub mod number;
pub mod operation;

fn f(args: Vec<Number>) -> Number {
    let y1 = args[2] * (Number::new(5.0) * args[0] + args[1]);
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

    println!("Result in forward order:");
    for _x in automatic_differentiator::get_record_collection() {
        //println!("{:?}", x);
    }

    let _backward_prop = automatic_differentiator.backward_propagate();

    println!("Result in reverse order after back propagation:");

    let reverse = automatic_differentiator::get_record_collection()
        .into_iter()
        .rev();
    for x in reverse {
        println!("{}", x);
    }
}
