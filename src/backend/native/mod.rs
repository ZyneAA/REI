use std::cell::RefMut;

use crate::backend::environment::Environment;

pub mod time;


pub fn register_all_native_fns(mut env: RefMut<Environment>) -> Result<(), Box<dyn std::error::Error>> {

    time::clock::register(&mut env)?;
    Ok(())

}
