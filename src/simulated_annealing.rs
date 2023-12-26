use rand_distr::{Distribution, Normal, Uniform};
use rand::thread_rng;
use std::collections::HashMap;

use crate::{Parameters, FunctionHolder};

struct State {
    error_old: f64,
    error_new: f64,
    error_best: f64,
    probability: f64,
}

impl State {
    fn new() -> Self {
        State {
            error_old: 99999999.0,
            error_new: 0.0,
            error_best: 99999999.0,
            probability: 1.0,
        }
    }
}

pub fn simulated_annealing(parameters: &mut HashMap<&str, Parameters>, function_holder: &FunctionHolder) -> f64 {

    // maximum number of iterations
    let n_max = 100_000_000;
    // initial temperature
    let init_temp = 0.01;

    // standard deviation to generate neighbours
    for (_, parameter) in parameters.iter_mut() {
        parameter.sigma = (parameter.ub - parameter.lb) * 0.1;
    }

    // random number generator
    let mut rng = thread_rng();
    // generate uniform distribution for acceptance
    let unif = Uniform::from(0.0..=1.0);
    // generate normal distribution to generate neighbours
    let norm = Normal::new(0.0, 1.0).unwrap();

    let mut state = State::new();

    // now generate a first candidate
    for (_, parameter) in &mut *parameters {
        parameter.value_current = (unif.sample(&mut rng) * (parameter.ub - parameter.lb)) + parameter.lb;
    }

    for k in 0..n_max {

        // here we define the linear decrease for that iterations
        // just to avoid unnecessary computations
        let linear_decrease = 1.0 - (k as f64 + 1.0) / n_max as f64;

        // initialize temperature, based on max number of iterations and initial temperature
        let temperature = init_temp * linear_decrease;

        // propose a neighbor value
        for (_, parameter) in parameters.iter_mut() {
            parameter.value_new = parameter.value_current + norm.sample(&mut rng) * parameter.sigma * linear_decrease;
        }

        // compute the probability of acceptance, based on current proposal
        // let prob = acceptance_probability(function_holder::FittingFct(parameters) f(&s_0), f(&s_new), temperature);
        state.error_new = function_holder.core_function(parameters);

        state.probability = acceptance_probability(state.error_old, state.error_new, temperature);

        // accept or reject the new value based on that probability
        if state.probability > unif.sample(&mut rng) {
            for (_, parameter) in parameters.iter_mut() {
                parameter.value_current = parameter.value_new;
            }
            state.error_old = state.error_new;
        }

        if state.error_old < state.error_best {
            for (_, parameter) in parameters.iter_mut() {
                parameter.value_best = parameter.value_current;
            }
            state.error_best = state.error_old;
        }

        if state.error_old < 0.0001 {
            // println!("Stopped after {k} iterations.");
            break;
        }

    }

    state.error_best

}

fn acceptance_probability(initial: f64, new: f64, temperature: f64) -> f64 {

    if new < initial {
        1.0
    } else {
        ((initial - new) / temperature).exp()
    }

}
