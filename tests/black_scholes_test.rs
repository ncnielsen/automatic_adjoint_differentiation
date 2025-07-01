use aad::automatic_differentiator::AutomaticDifferentiator;
use aad::number::Number;
use statrs::distribution::Continuous;
use statrs::distribution::{ContinuousCDF, Normal};
use std::f64::consts::E;

fn f_call(args: &[Number]) -> Number {
    let s = args[0]; // Current stock price
    let k = args[1]; // Strike price
    let t = args[2]; // Time to maturity in years
    let r = args[3]; // Risk-free interest rate
    let sigma = args[4]; // Volatility

    let d1 = ((s / k).ln() + (r + 0.5 * sigma.pow(2.0)) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let norm = Normal::new(0.0, 1.0).unwrap();

    // Call
    let e_powf = Number::new(E.powf(-r.result * t.result));
    s * norm.cdf(d1.result) - k * e_powf * norm.cdf(d2.result)
}

#[derive(Debug)]
pub struct OptionParameters {
    pub s: f64,     // Current stock price
    pub k: f64,     // Strike price
    pub t: f64,     // Time to maturity in years
    pub r: f64,     // Risk-free interest rate
    pub sigma: f64, // Volatility
}

pub fn d1(params: &OptionParameters) -> f64 {
    let OptionParameters { s, k, t, r, sigma } = *params;
    (f64::ln(s / k) + (r + 0.5 * sigma.powf(2.0)) * t) / (sigma * f64::sqrt(t))
}

pub fn d2(params: &OptionParameters) -> f64 {
    d1(params) - params.sigma * f64::sqrt(params.t)
}

pub fn black_scholes_price(params: &OptionParameters, is_call: bool) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    let d1_val = d1(params);
    let d2_val = d2(params);

    if is_call {
        params.s * norm.cdf(d1_val) - params.k * E.powf(-params.r * params.t) * norm.cdf(d2_val)
    } else {
        params.k * E.powf(-params.r * params.t) * norm.cdf(-d2_val) - params.s * norm.cdf(-d1_val)
    }
}

// Greeks
pub fn delta(params: &OptionParameters, is_call: bool) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    let d1_val = d1(params);

    if is_call {
        norm.cdf(d1_val)
    } else {
        norm.cdf(d1_val) - 1.0
    }
}

pub fn gamma(params: &OptionParameters) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    let d1_val = d1(params);

    norm.pdf(d1_val) / (params.s * params.sigma * f64::sqrt(params.t))
}

pub fn vega(params: &OptionParameters) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    let d1_val = d1(params);

    params.s * norm.pdf(d1_val) * f64::sqrt(params.t) / 100.0
}

pub fn theta(params: &OptionParameters, is_call: bool) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    let d1_val = d1(params);
    let d2_val = d2(params);
    let s = params.s;
    let k = params.k;
    let t = params.t;
    let r = params.r;
    let sigma = params.sigma;
    let pdf = norm.pdf(d1_val);
    let sqrt_t = f64::sqrt(t);

    let first = -(s * pdf * sigma) / (2.0 * sqrt_t);
    let second_call = -r * k * E.powf(-r * t) * norm.cdf(d2_val);
    let second_put = r * k * E.powf(-r * t) * norm.cdf(-d2_val);

    if is_call {
        (first + second_call) / 365.0
    } else {
        (first + second_put) / 365.0
    }
}

pub fn rho(params: &OptionParameters, is_call: bool) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    let d2_val = d2(params);
    let k = params.k;
    let t = params.t;
    let r = params.r;

    if is_call {
        k * t * E.powf(-r * t) * norm.cdf(d2_val) / 100.0
    } else {
        -k * t * E.powf(-r * t) * norm.cdf(-d2_val) / 100.0
    }
}

#[test]
fn black_scholes_test() {
    let params = OptionParameters {
        s: 100.0,   // price
        k: 100.0,   // strike
        t: 1.0,     // time to maturity
        r: 0.05,    // Risk free interest rate
        sigma: 0.2, // Volatility
    };

    let analytical_call_black_scholes_price = black_scholes_price(&params, true);
    let analytical_call_delta = delta(&params, true);
    let analytical_call_theta = theta(&params, true);
    let analytical_call_rho = rho(&params, true);

    let analytical_put_black_scholes_price = black_scholes_price(&params, false);
    let analytical_put_delta = delta(&params, false);
    let analytical_put_theta = theta(&params, false);
    let analytical_put_rho = rho(&params, false);

    let analytical_gamma = gamma(&params);
    let analytical_vega = vega(&params);

    println!("Analytical Call Option:");
    println!("Price: {:.4}", analytical_call_black_scholes_price);
    println!("Delta: {:.4}", analytical_call_delta);
    println!("Gamma: {:.4}", analytical_gamma);
    println!("Vega: {:.4}", analytical_vega);
    println!("Theta: {:.4}", analytical_call_theta);
    println!("Rho: {:.4}", analytical_call_rho);

    println!("Analytical Put Option:");
    println!("Price: {:.4}", analytical_put_black_scholes_price);
    println!("Delta: {:.4}", analytical_put_delta);
    println!("Gamma: {:.4}", analytical_gamma);
    println!("Vega: {:.4}", analytical_vega);
    println!("Theta: {:.4}", analytical_put_theta);
    println!("Rho: {:.4}", analytical_put_rho);

    let OptionParameters { s, k, t, r, sigma } = params;
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

    let call_gamma = numerical_evaluation // dOptionPrice/dStockPrice^2 (aka. stock price acceleration)
        .derivatives
        .iter()
        .filter(|d| d.input.id == k.id) // This is the strike, not twice the price derivative
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

    let epsilon = 1e-5;
    assert!((call_price - 10.45058).abs() < epsilon);
    assert!((call_delta - 0.63683).abs() < epsilon);
    assert!((call_gamma - 0.01876).abs() < epsilon);
    assert!((call_vega - 37.52403).abs() < epsilon);
    assert!((call_theta - 6.41403).abs() < epsilon);
    assert!((call_rho - 53.23248).abs() < epsilon);
}
