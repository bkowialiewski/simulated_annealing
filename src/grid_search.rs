use std::collections::HashMap;
use crate::{Parameters, FunctionHolder};

pub struct Minimizing {
    pub range_x: Vec<f64>,
    pub range_y: Vec<f64>,
    pub grid_results: Vec<Vec<f64>>,
    pub grid_results_log: Vec<Vec<f64>>,
}

impl Minimizing {
    fn new() -> Self {
        Minimizing {
            range_x: Vec::new(),
            range_y: Vec::new(),
            grid_results: Vec::new(),
            grid_results_log: Vec::new(),
        }
    }
}

pub fn grid_search(parameters: &mut HashMap<&str, Parameters>, function_holder: &FunctionHolder) -> Minimizing {

    let mut minimizing = Minimizing::new();

    for (key, parameter) in parameters.iter_mut() {
        if *key == "x" {
            minimizing.range_x = seq(parameter.lb, parameter.ub, (parameter.ub - parameter.lb) / 2_000.0);
        } else {
            minimizing.range_y = seq(parameter.lb, parameter.ub, (parameter.ub - parameter.lb) / 2_000.0);
        }
    }

    minimizing.grid_results = vec![vec![0.0; minimizing.range_x.len()]; minimizing.range_y.len()];
    minimizing.grid_results_log = vec![vec![0.0; minimizing.range_x.len()]; minimizing.range_y.len()];

    for (i, x) in minimizing.range_x.iter().enumerate() {
        for (j, y) in minimizing.range_y.iter().enumerate() {
            for (key, parameter) in parameters.iter_mut() {
                if *key == "x" {
                    parameter.value_new = *x;
                } else {
                    parameter.value_new = *y;
                }
            }

            minimizing.grid_results[j][i] = function_holder.core_function(parameters);
            minimizing.grid_results_log[j][i] = minimizing.grid_results[j][i].log(10.0);

        }
    }

    minimizing

}

pub fn seq(min: f64, max: f64, step: f64) -> Vec<f64> {

    let mut v = vec![0.0; ((max - min) / step + 1.0).round() as usize];

    for i in 0..v.len() {
        v[i] = min + step * i as f64;
    }

    v

}
