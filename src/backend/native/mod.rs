use std::cell::RefMut;

use crate::backend::environment::Environment;

pub mod chrono;
pub mod memory;
pub mod io;
pub mod math;

pub fn register_all_native_fns(mut env: RefMut<Environment>) -> Result<(), Box<dyn std::error::Error>> {

    chrono::clock::register(&mut *env)?;
    memory::mem::register(&mut *env)?;
    io::std_io::register(&mut *env)?;
    math::math::register(&mut *env)?;

    Ok(())

}
