mod simulated_annealing;
mod functions;
mod define_structures;
mod plotting;

use simulated_annealing::*;
use std::collections::HashMap;
use functions::rosenbrock;
use define_structures::{Parameters, FunctionHolder};
use plotting::plot_histogram;

struct Total {
    error: Vec<f64>,
    parameters: Vec<Vec<f64>>,
}

impl Total {
    fn new(n_sim: usize, n_par: usize) -> Self {
        Total {
            error: vec![0.0; n_sim],
            parameters: vec![vec![0.0; n_sim]; n_par],
        }
    }
}

fn main() {

    let n_sim = 50_000;

    let function_holder: FunctionHolder;

    let mut parameters: HashMap<&str, Parameters> = vec![
        ("x", Parameters {value_current: 0.0, value_new: 0.0, value_best: 0.0, lb: -2.0, ub: 2.0, sigma: 0.0}),
        ("y", Parameters {value_current: 0.0, value_new: 0.0, value_best: 0.0, lb: -1.0, ub: 3.0, sigma: 0.0}),
    ]
        .into_iter()
        .collect();

    function_holder = FunctionHolder::FittingFct(rosenbrock);

    let mut total = Total::new(n_sim, parameters.len());

    for epoch in 0..n_sim {

        total.error[epoch] = simulated_annealing(&mut parameters, &function_holder);

        total.parameters[0][epoch] = parameters.get("x").unwrap().value_best;
        total.parameters[1][epoch] = parameters.get("y").unwrap().value_best;

    }

    plot_histogram(&total.error, "plots/error_distribution.svg", "f(x,y) distribution", "f(x,y)", vec![0.0, 0.001]);
    plot_histogram(&total.parameters[0], "plots/x_distribution.svg", "X parameter distribution", "X parameter", vec![-2.0, 2.0]);
    plot_histogram(&total.parameters[1], "plots/y_distribution.svg", "Y parameter distribution", "Y parameter", vec![-1.0, 3.0]);

}


