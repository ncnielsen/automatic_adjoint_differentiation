use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;
use aad::operation::Operation;

// tests verified with https://www.derivative-calculator.net/

#[test]
fn test_operators_add_mul_ln() {
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

    let forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

    automatic_differentiator.reverse_propagate_adjoints();
    let differentials = automatic_differentiator.get_differentials();
    assert_eq!(differentials.len(), 5);

    let adjoints: Vec<(i64, f64, f64)> = differentials
        .iter()
        .map(|op| match op {
            Operation::Value(id, res, adj) => (*id, *res, *adj),
            _ => (0, 0.0, 0.0),
        })
        .collect();

    let x1 = adjoints
        .iter()
        .filter(|x| x.0 == x1.id)
        .map(|x| x.2)
        .next()
        .unwrap();
    let x2 = adjoints
        .iter()
        .filter(|x| x.0 == x2.id)
        .map(|x| x.2)
        .next()
        .unwrap();
    let x3 = adjoints
        .iter()
        .filter(|x| x.0 == x3.id)
        .map(|x| x.2)
        .next()
        .unwrap();
    let x4 = adjoints
        .iter()
        .filter(|x| x.0 == x4.id)
        .map(|x| x.2)
        .next()
        .unwrap();
    let x5 = adjoints
        .iter()
        .filter(|x| x.0 == x5.id)
        .map(|x| x.2)
        .next()
        .unwrap();

    let epsilon = 1e-10;
    assert!(forward_eval.result - 797.75132345616487 < epsilon);
    assert!(x1 - 950.7364539019619 < epsilon);
    assert!(x2 - 190.14729078039238 < epsilon);
    assert!(x3 - 443.6770118209156 < epsilon);
    assert!(x4 - 73.20408806599326 < epsilon);
    assert!(x5 - 190.14729078039238 < epsilon);
}

#[test]
fn test_operators_sub_sin_div() {
    let mut automatic_differentiator = AutomaticDifferentiator::new();

    let x1 = Number::new(1.5);
    let x2 = Number::new(0.5);

    let arguments = vec![x1, x2];

    fn f(args: Vec<Number>) -> Number {
        let x1 = args[0];
        let x2 = args[1];
        let frac = x1 / x2;
        (frac.sin() + frac - x2.exp()) * (frac - x2.exp())
    }

    let forward_eval = automatic_differentiator.forward_evaluate(f, arguments);

    automatic_differentiator.reverse_propagate_adjoints();
    let differentials = automatic_differentiator.get_differentials();
    assert_eq!(differentials.len(), 2);

    let adjoints: Vec<(i64, f64, f64)> = differentials
        .iter()
        .map(|op| match op {
            Operation::Value(id, res, adj) => (*id, *res, *adj),
            _ => (0, 0.0, 0.0),
        })
        .collect();

    let x1 = adjoints
        .iter()
        .filter(|x| x.0 == x1.id)
        .map(|x| x.2)
        .next()
        .unwrap();
    let x2 = adjoints
        .iter()
        .filter(|x| x.0 == x2.id)
        .map(|x| x.2)
        .next()
        .unwrap();
    let epsilon = 1e-10;
    assert!(forward_eval.result - 2.017 < epsilon);
    assert!(x1 - 3.0118433276739069 < epsilon);
    assert!(x2 - (-13.723961509314076) < epsilon);
}
