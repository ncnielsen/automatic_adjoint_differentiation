use std::f64::consts::PI;

use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;

// This file contains test cases that once failed. The reason was duplicate entries in the nodes_list
// when a single value had multiple parents
#[test]
fn test_operators_add_sub_mul_sin() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);
    let z = Number::new(-0.6);

    let arguments = vec![x, y, z];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        let z = args[2];

        // sin((x+y)*(y-z))

        ((x + y) * (y - z)).sin()
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

    assert!((evaluation.result - (0.381250)).abs() < epsilon);
    assert!((dfdx - (-3.32810)).abs() < epsilon);
    assert!((dfdy - (-8.87493)).abs() < epsilon);
    assert!((dfdz - (5.546831)).abs() < epsilon);
}

#[test]
fn test_operators_add_div_pi() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);

    let arguments = vec![x, y];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];

        // sin( (x+y)/PI )

        ((x + y) / PI).sin()
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

    let epsilon = 1e-5;

    assert!((evaluation.result - (0.943067)).abs() < epsilon);
    assert!((dfdx - (-0.10587)).abs() < epsilon);
    assert!((dfdy - (-0.10587)).abs() < epsilon);
}

#[test]
fn test_operators_sub_div_pi() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let y = Number::new(3.0);
    let z = Number::new(-0.6);

    let arguments = vec![y, z];

    fn f(args: &[Number]) -> Number {
        let y = args[0];
        let z = args[1];

        // sin( (y-z)/PI )

        ((y - z) / PI).sin()
    }

    let evaluation = automatic_differentiator.derivatives(f, &arguments);

    assert_eq!(evaluation.derivatives.len(), arguments.len());

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

    assert!((evaluation.result - (0.911088)).abs() < epsilon);
    assert!((dfdy - (0.131211)).abs() < epsilon);
    assert!((dfdz - (-0.13121)).abs() < epsilon);
}

#[test]
fn test_operators_mul_div_pi() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);

    let arguments = vec![x, y];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];

        // sin( (x*y)/PI )

        ((x * y) / PI).sin()
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

    let epsilon = 1e-5;

    assert!((evaluation.result - (0.273282)).abs() < epsilon);
    assert!((dfdx - (-0.91858)).abs() < epsilon);
    assert!((dfdy - (-0.91858)).abs() < epsilon);
}

#[test]
fn test_operators_add_sub_mul_div_pi() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x = Number::new(3.0);
    let y = Number::new(3.0);
    let z = Number::new(-0.6);

    let arguments = vec![x, y, z];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        let z = args[2];

        // sin( ((x+y)*(y-z))/PI )

        (((x + y) * (y - z)) / PI).sin()
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

    assert!((evaluation.result - (0.558278)).abs() < epsilon);
    assert!((dfdx - (0.950714)).abs() < epsilon);
    assert!((dfdy - (2.535237)).abs() < epsilon);
    assert!((dfdz - (-1.58452)).abs() < epsilon);
}
