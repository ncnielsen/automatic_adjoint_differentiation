use std::f64::consts::PI;

use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;

// Tests verified with https://www.derivative-calculator.net/
// an exellent resource for playing around with functions and their derivatives.

#[test]
fn test_operators_add_mul_ln() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x1 = Number::new(1.0);
    let x2 = Number::new(2.0);
    let x3 = Number::new(3.0);
    let x4 = Number::new(4.0);
    let x5 = Number::new(5.0);

    let arguments = vec![x1, x2, x3, x4, x5];

    fn f(args: &[Number]) -> Number {
        let y1 = args[2] * (args[4] * args[0] + args[1]);
        let y2 = y1.ln();
        let y = (y1 + args[3] * y2) * (y1 + y2);
        y
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx1 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x1.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdx2 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x2.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdx3 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x3.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdx4 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x4.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdx5 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x5.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-10;
    assert!(evaluation.result - 797.75132345616487 < epsilon);
    assert!(dfdx1 - 950.7364539019619 < epsilon);
    assert!(dfdx2 - 190.14729078039238 < epsilon);
    assert!(dfdx3 - 443.6770118209156 < epsilon);
    assert!(dfdx4 - 73.20408806599326 < epsilon);
    assert!(dfdx5 - 190.14729078039238 < epsilon);
}

#[test]
fn test_operators_sub_sin_div() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x1 = Number::new(1.5);
    let x2 = Number::new(0.5);

    let arguments = vec![x1, x2];

    fn f(args: &[Number]) -> Number {
        let x1 = args[0];
        let x2 = args[1];
        let frac = x1 / x2;
        (frac.sin() + frac - x2.exp()) * (frac - x2.exp())
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx1 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x1.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdx2 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x2.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-10;

    assert!(evaluation.result - 2.017 < epsilon);
    assert!(dfdx1 - 3.0118433276739069 < epsilon);
    assert!(dfdx2 - (-13.723961509314076) < epsilon);
}

#[test]
fn test_operators_cos_exp() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x1 = Number::new(1.0);

    let arguments = vec![x1];

    fn f(args: &[Number]) -> Number {
        let x1 = args[0];

        (x1.exp() + PI / 2.0).cos()
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx1 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x1.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-10;

    assert!(evaluation.result - (-0.41078) < epsilon);
    assert!(dfdx1 - 2.478350 < epsilon);
}

#[test]
fn test_operators_cos_pow() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x1 = Number::new(-1.0);

    let arguments = vec![x1];

    fn f(args: &[Number]) -> Number {
        let x1 = args[0];

        (x1.pow(5.0) + PI / 2.0).cos()
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx1 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x1.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-10;
    assert!(evaluation.result - (0.841471) < epsilon);
    assert!(dfdx1 - (-2.70151) < epsilon);
}

#[test]
fn test_operators_sin_sqrt_exp() {
    // f = sin(sqrt(e^x + pi)/2)
    // x = 5
    // f(x) = -0.12746
    // f'(x) = 2.989310

    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x1 = Number::new(5.0);

    let arguments = vec![x1];

    fn f(args: &[Number]) -> Number {
        let x1 = args[0];

        (((x1.exp() + PI).sqrt()) / 2.0).sin()
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx1 = evaluation
        .derivatives
        .iter()
        .filter(|x| x.input.id == x1.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-10;

    assert!(evaluation.result - (-0.12745886733521275) < epsilon);
    assert!(dfdx1 - 2.989310 < epsilon);
}
