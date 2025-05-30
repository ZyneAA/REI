use std::cell::RefMut;

use crate::backend::environment::Environment;

pub mod chrono;
pub mod memory;

pub fn register_all_native_fns(mut env: RefMut<Environment>) -> Result<(), Box<dyn std::error::Error>> {

    chrono::clock::register(&mut *env)?;
    memory::mem::register(&mut *env)?;
    Ok(())

}
