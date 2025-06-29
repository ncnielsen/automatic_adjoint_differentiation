use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;

// Tests verified with https://www.derivative-calculator.net/
// an exellent resource for playing around with functions and their derivatives.

fn test_unary_operator<F>(func: F, arguments: &[Number], expected_result: f64, expected_dfdx: f64)
where
    F: Fn(&[Number]) -> Number,
{
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let evaluation = automatic_differentiator.derivatives(func, &arguments);

    let x = arguments[0];

    let dfdx = evaluation
        .derivatives
        .iter()
        .filter(|d| d.input.id == x.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let epsilon = 1e-5;
    assert_eq!(evaluation.derivatives.len(), 1);
    assert!(evaluation.result - expected_result < epsilon);
    assert!(dfdx - expected_dfdx < epsilon);
}

#[test]
fn test_cos_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.cos()
    }
    test_unary_operator(f, &arguments, -0.98999, -0.14112);
}
//0.963982
#[test]
fn test_sin_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.sin()
    }
    test_unary_operator(f, &arguments, 0.141120, -0.98999);
}

#[test]
fn test_sin_of_0_comma_96398171442035951() {
    // This failed in a combination test, so testing it separately
    let arg = Number::new(0.96398171442035951);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.sin()
    }
    test_unary_operator(f, &arguments, 0.821469, 0.570254);
}

#[test]
fn test_ln_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.ln()
    }
    test_unary_operator(f, &arguments, 1.098612, 0.333333);
}

#[test]
fn test_sqrt_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.sqrt()
    }
    test_unary_operator(f, &arguments, 1.732051, 0.288675);
}

#[test]
fn test_exp_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.exp()
    }
    test_unary_operator(f, &arguments, 20.08554, 20.08554);
}

#[test]
fn test_pow_fifth_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.pow(5.0)
    }
    test_unary_operator(f, &arguments, 243.0000, 405.0000);
}

#[test]
fn test_log_base_eight_of_3() {
    let arg = Number::new(3.0);
    let arguments = vec![arg];

    fn f(args: &[Number]) -> Number {
        let arg = args[0];
        arg.log(8.0)
    }
    test_unary_operator(f, &arguments, 0.528321, 0.160299);
}
