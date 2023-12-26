use std::collections::HashMap;
use super::Parameters;

// pub fn hyperbolic(parameters: &HashMap<&str, Parameters>) -> f64 {
//
//     let x = parameters.get("x").unwrap().value_new;
//     x.powf(2.0)
//
// }

pub fn rosenbrock(parameters: &HashMap<&str, Parameters>) -> f64 {

    let x = parameters.get("x").unwrap().value_new;
    let y = parameters.get("y").unwrap().value_new;

    let a = 1.0;
    let b = 100.0;

    (a - x).powf(2.0) + b * (y - x.powf(2.0)).powf(2.0)

}
