use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;
use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

fn f_call(args: &[Number]) -> Number {
    let s = args[0]; // Current stock price
    let k = args[1]; // Strike price
    let t = args[2]; // Time to maturity in years
    let r = args[3]; // Risk-free interest rate
    let sigma = args[4]; // Volatility

    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let norm = Normal::new(0.0, 1.0).unwrap();

    s * d1.cdf() - k * (-1.0 * r * t).exp() * d2.cdf()

    // Call
    //s * Number::new(norm.cdf(d1.result))
    //    - k * (-1.0 * r * t).exp() * Number::new(norm.cdf(d2.result))
}

#[test]
fn black_scholes_test() {
    let s = 100.0; // underlying stock price
    let k = 100.0; // strike
    let t = 1.0; // time to maturity
    let r = 0.05; // Risk free interest rate
    let sigma = 0.2; // Volatility

    let mut ad = AutomaticDifferentiator::new();

    let s = Number::new(s);
    let k = Number::new(k);
    let t = Number::new(t);
    let r = Number::new(r);
    let sigma = Number::new(sigma);

    let arguments = vec![s, k, t, r, sigma];

    let numerical_evaluation = ad.derivatives(f_call, &arguments);
    let call_price = numerical_evaluation.result;
    let call_delta = numerical_evaluation // dOptionPrice/dStockPrice
        .derivatives
        .iter()
        .filter(|d| d.input.id == s.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let call_vega = numerical_evaluation // dOptionPrice/dSigma (and sigma is volatility)
        .derivatives
        .iter()
        .filter(|d| d.input.id == sigma.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let call_theta = numerical_evaluation // dOptionPrice/dt
        .derivatives
        .iter()
        .filter(|d| d.input.id == t.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let call_rho = numerical_evaluation // dOptionPrice/dr
        .derivatives
        .iter()
        .filter(|d| d.input.id == r.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    let call_strike = numerical_evaluation // dOptionPrice/dStockPrice^2 (aka. stock price acceleration)
        .derivatives
        .iter()
        .filter(|d| d.input.id == k.id) // This is the strike, not twice the price derivative
        .map(|x| x.derivative)
        .next()
        .unwrap();

    println!(
        "s (price) has id {} and derivative (delta) {}",
        s.id, call_delta
    );
    println!(
        "t (time to maturity) has id {} and derivative (theta) {}",
        t.id, call_theta
    );
    println!(
        "r (interest rate) has id {} and derivative (rho) {}",
        r.id, call_rho
    );
    println!(
        "sigma (volatility) has id {} and derivative (vega) {}",
        sigma.id, call_vega
    );

    let epsilon = 1e-5;
    assert!((call_price - 10.45058).abs() < epsilon);
    assert!((call_delta - 0.63683).abs() < epsilon);
    assert!((call_rho - 53.23248).abs() < epsilon);
    // Gamma is the second derivative, and we only support first
    // derivative for now. So we will skip it until further notice
    // assert!((call_gamma - 0.01876).abs() < epsilon);
    assert!((call_vega - 37.52403).abs() < epsilon);
    assert!((call_theta - 6.41403).abs() < epsilon);

    // TODO:
    // Run a second derivative, this time with delta in place of price to obtain the second derivative of the price
    let s = Number::new(call_delta);
    let sigma = Number::new(call_vega);
    let k = Number::new(call_strike);
    let t = Number::new(call_theta);
    let r = Number::new(call_rho);

    let arguments = vec![s, k, t, r, sigma];

    let numerical_evaluation = ad.derivatives(f_call, &arguments);

    let call_delta_second = numerical_evaluation // dOptionPrice/dStockPrice
        .derivatives
        .iter()
        .filter(|d| d.input.id == s.id)
        .map(|x| x.derivative)
        .next()
        .unwrap();

    println!(
        "s (price) has id {} and has second derivative (gamma) {}",
        s.id, call_delta_second
    );
}
