use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;

fn _g(args: Vec<Number>) -> Number {
    let x1 = args[0];
    let x2 = args[1];
    let frac = x1 / x2;
    (frac.sin() + frac - x2.exp()) * (frac - x2.exp())
}

fn main() {
    let _arguments_g = vec![Number::new(1.5), Number::new(0.5)];

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

    let arg_clone = arguments.clone();

    let _forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

    println!("forward eval {}", _forward_eval);
    //println!("printing parent map after forward evaluation");
    //automatic_differentiator.print_parent_map_id();

    automatic_differentiator.reverse_propagate_adjoints();

    println!("Differentials");
    automatic_differentiator.print_differentials(arg_clone);

    // println!("Printing child map after back propagation");
    //automatic_differentiator.print_child_map_id();

    // println!("\n\nPrinting record colection values after back propagation");
    //automatic_differentiator.print_record_collection();
}
