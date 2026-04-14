use math::Mod;

use crate::{enviroment::Enviroment, parser::callable::ExposedCallable};

pub mod math;

pub fn math_globals() -> Enviroment {
    let math_globals = Enviroment::default();

    let (name, ret_val) = Mod::definition();
    math_globals.define_var(&name, ret_val);

    math_globals
}
