use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;

// Tests verified with https://www.derivative-calculator.net/
// an exellent resource for playing around with functions and their derivatives.

fn test_binary_operator<F>(
    func: F,
    arguments: &[Number],
    expected_result: f64,
    expected_dfdx: f64,
    expected_dfdy: f64,
) where
    F: Fn(&[Number]) -> Number,
{
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let evaluation = automatic_differentiator.derivatives(func, &arguments);

    let x = arguments[0];
    let y = arguments[1];

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
    assert_eq!(evaluation.derivatives.len(), 2);
    assert!(evaluation.result - expected_result < epsilon);
    assert!(dfdx - expected_dfdx < epsilon);
    assert!(dfdy - expected_dfdy < epsilon);
}

#[test]
fn test_add_3_5() {
    let x = Number::new(3.0);
    let y = Number::new(5.0);
    let arguments = vec![x, y];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        x + y
    }
    test_binary_operator(f, &arguments, 8.000000, 1.00000, 1.00000);
}

#[test]
fn test_sub_3_5() {
    let x = Number::new(3.0);
    let y = Number::new(5.0);
    let arguments = vec![x, y];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        x - y
    }
    test_binary_operator(f, &arguments, -2.000000, 1.00000, -1.00000);
}

#[test]
fn test_mul_3_5() {
    let x = Number::new(3.0);
    let y = Number::new(5.0);
    let arguments = vec![x, y];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        x - y
    }
    test_binary_operator(f, &arguments, 15.00000, 5.00000, 3.00000);
}

#[test]
fn test_div_3_5() {
    let x = Number::new(3.0);
    let y = Number::new(5.0);
    let arguments = vec![x, y];

    fn f(args: &[Number]) -> Number {
        let x = args[0];
        let y = args[1];
        x / y
    }
    test_binary_operator(f, &arguments, 0.600000, 0.200000, -0.12000);
}
