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
    assert!((evaluation.result - 797.75132345616487).abs() < epsilon);
    assert!((dfdx1 - 950.7364539019619).abs() < epsilon);
    assert!((dfdx2 - 190.14729078039238).abs() < epsilon);
    assert!((dfdx3 - 443.6770118209156).abs() < epsilon);
    assert!((dfdx4 - 73.20408806599326).abs() < epsilon);
    assert!((dfdx5 - 190.14729078039238).abs() < epsilon);
}

#[test]
fn test_operators_sub_sin_div_exp() {
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

    assert!((evaluation.result - 2.0166466694282015).abs() < epsilon);
    assert!((dfdx1 - 3.0118433276739069).abs() < epsilon);
    assert!((dfdx2 - (-13.723961509314076)).abs() < epsilon);
}

#[test]
fn test_operators_cos_pi_div_exp_cos() {
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

    assert!((evaluation.result - (-0.41078129050290929)).abs() < epsilon);
    assert!((dfdx1 - 2.478349732955234).abs() < epsilon);
}

#[test]
fn test_operators_cos_pow_pi_add_div() {
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
    assert!((evaluation.result - (0.8414709848078965)).abs() < epsilon);
    assert!((dfdx1 - (-2.7015115293406984)).abs() < epsilon);
}

#[test]
fn test_operators_sin_sqrt_exp_div_add() {
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

    assert!((evaluation.result - (-0.12745886733521275)).abs() < epsilon);
    assert!((dfdx1 - 2.9893099479208347).abs() < epsilon);
}

#[test]
fn test_operators_add_sub_mull_div_ln_sin_cos_exp_pow_sqrt_log() {
    // sin(ln(sqrt((((x+y)*(y-z))) / pi))) + log(8, (pow(exp(cos(w)),5.0))
    // x = 3
    // y = 3
    // w = 3
    // z = -0.6

    // f(x) = -1.55896
    // df/dz = -0.07920
    // df/dx = 0.047521

    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);
    let w = Number::new(3.0);
    let z = Number::new(-0.6);

    let arguments = vec![x, y, w, z];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        let w = args[2];
        let z = args[3];

        // sin(ln(sqrt(((x+y)*(y-z)) / pi ))) + log(8, (pow(exp(cos(w)),5.0))

        (((x + y) * (y - z)) / PI).sqrt().ln().sin() + (w.cos().exp().pow(5.0).log(8.0))
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == x.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdy = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == y.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdz = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == z.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdw = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == w.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-10;

    assert!((evaluation.result - (-1.5589601142820477)).abs() < epsilon);
    assert!((dfdz - (-0.079201897431872462)).abs() < epsilon);
    assert!((dfdx - 0.04752113845912348).abs() < epsilon);
    assert!((dfdw - (-0.33932189299696824)).abs() < epsilon);
    assert!((dfdy - 0.12672303589099593).abs() < epsilon);
}

#[test]
fn test_operators_add_sub_mul_div_pi_ln_sqrt() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);
    let z = Number::new(-0.6);

    let arguments = vec![x, y, z];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        let z = args[2];

        // ln(sqrt( ( (x+y)*(y-z) ) / pi ))
        (((x + y) * (y - z)) / PI).sqrt().ln()
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == x.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdy = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == y.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdz = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == z.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-5;

    assert!((evaluation.result - (0.96398171442035951)).abs() < epsilon);
    assert!((dfdz - (-0.1388888888888889)).abs() < epsilon);
    assert!((dfdy - 0.222222).abs() < epsilon);
    assert!((dfdx - 0.083333333333333342).abs() < epsilon);
}

#[test]
fn test_operators_ln_sqrt() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);
    let z = Number::new(-0.6);

    let arguments = vec![x, y, z];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        let z = args[2];

        // ln(sqrt(sqrt( x+y+z )))
        ((x + y + z) + (y * y)).sqrt().sqrt()
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

    let dfdx = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == x.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdy = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == y.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let dfdz = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == z.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-5;

    assert!((evaluation.result - (1.948007)).abs() < epsilon);
    assert!((dfdx - 0.033820).abs() < epsilon);
    assert!((dfdy - 0.236737).abs() < epsilon);
    assert!((dfdz - 0.033820).abs() < epsilon);
}
