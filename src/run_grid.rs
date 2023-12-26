mod define_structures;
mod functions;
mod grid_search;
mod plotting;

use std::collections::HashMap;
use functions::rosenbrock;
use define_structures::{Parameters, FunctionHolder};
use grid_search::grid_search;
use plotting::plot_heatmap;

fn main() {

    let mut parameters: HashMap<&str, Parameters> = vec![
        ("x", Parameters {value_current: 0.0, value_new: 0.0, value_best: 0.0, lb: -2.0, ub: 2.0, sigma: 0.0}),
        ("y", Parameters {value_current: 0.0, value_new: 0.0, value_best: 0.0, lb: -1.0, ub: 3.0, sigma: 0.0}),
    ]
        .into_iter()
        .collect();

    let function_holder = FunctionHolder::FittingFct(rosenbrock);

    let minimizing = grid_search(&mut parameters, &function_holder);

    plot_heatmap(minimizing.grid_results, "Rosenbrock", "plots/", "grid_search.svg", minimizing.range_x.to_vec(), minimizing.range_y.to_vec());
    plot_heatmap(minimizing.grid_results_log, "Rosenbrock (log scaled)", "plots/", "grid_search_log.svg", minimizing.range_x.to_vec(), minimizing.range_y.to_vec());

}

