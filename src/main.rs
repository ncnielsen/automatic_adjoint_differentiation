use std::f64::consts::PI;

use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;

fn main() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    /*
    let arguments = vec![Number::new(1.5), Number::new(0.5)];

    fn f(args: &Vec<Number>) -> Number {
        let x1 = args[0];
        let x2 = args[1];
        let frac = x1 / x2;
        (frac.sin() + frac - x2.exp()) * (frac - x2.exp())
    }
    */
    /*
        let x1 = Number::new(-1.0);

        let arguments = vec![x1];

        fn f(args: Vec<Number>) -> Number {
            let x1 = args[0];

            (x1.pow(5.0) + PI / 2.0).cos()
        }
    */

    let x1 = Number::new(5.0);

    let arguments = vec![x1];

    fn f(args: &[Number]) -> Number {
        //sin(sqrt(e^x+π))÷2)
        let x1 = args[0];

        (((x1.exp() + PI).sqrt()) / 2.0).sin()
    }

    /*
        let x1 = Number::new(1.0);
        let x2 = Number::new(2.0);
        let x3 = Number::new(3.0);
        let x4 = Number::new(4.0);
        let x5 = Number::new(5.0);

        let arguments = vec![x1, x2, x3, x4, x5];

        fn f(args: &Vec<Number>) -> Number {
            let y1 = args[2] * (args[4] * args[0] + args[1]);
            let y2 = y1.ln();
            let y = (y1 + args[3] * y2) * (y1 + y2);
            y
        }
    */

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    println!("Printing derivatives: ");
    for derivative in evaluation.derivatives {
        println!(
            "Input {} has derivative {}",
            derivative.input, derivative.derivative
        )
    }

    println!("Printing child map after back propagation");
    automatic_differentiator.print_child_map_id();

    println!("\n\nPrinting record colection values after back propagation");
    automatic_differentiator.print_record_collection();
}
