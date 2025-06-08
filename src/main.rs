use crate::{
    automatic_differentiator::AutomaticDifferentiator,
    number::{Number, OperationRich},
};

pub mod automatic_differentiator;
pub mod number;

fn main() {
    let automatic_differentiator = AutomaticDifferentiator::new();

    let arguments = vec![
        Number::new(1.0),
        Number::new(2.0),
        Number::new(3.0),
        Number::new(4.0),
        Number::new(5.0),
    ];

    let forward_eval = automatic_differentiator.forward_evaluate(f, arguments);
    println!("Forward evaluated result: {:?}", forward_eval);
}

fn f(args: Vec<Number>) -> Number {
    let y1 = args[2] * (Number::new(5.0) * args[0] + args[1]);
    let y2 = y1.log();
    let y = (y1 + args[3] * y2) * (y1 + y2);
    y
}
