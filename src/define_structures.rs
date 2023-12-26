use std::collections::HashMap;

#[derive(Debug)]
pub struct Parameters {
    pub value_current: f64,
    pub value_new: f64,
    pub value_best: f64,
    pub lb: f64,
    pub ub: f64,
    pub sigma: f64,
}

pub enum FunctionHolder {
    FittingFct(fn(&HashMap<&str, Parameters>) -> f64),
}

impl FunctionHolder {

    pub fn core_function(&self, parameters: &HashMap<&str, Parameters>) -> f64 {

        match self {
            FunctionHolder::FittingFct(f) => f(parameters),
        }

    }

}
