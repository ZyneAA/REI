use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::backend::environment::Environment;
use crate::backend::exec_signal::runtime_error::{RuntimeError, RuntimeErrorType};
use crate::backend::exec_signal::ExecSignal;
use crate::backend::interpreter::Interpreter;
use crate::backend::rei_callable::ReiCallable;
use crate::backend::rei_instance::ReiInstance;
use crate::backend::stack_trace::ExecContext;

use crate::crux::token::Object;

#[derive(Clone, Debug)]
pub struct ReiClone;
impl ReiCallable for ReiClone {
    fn arity(&self) -> usize {
        1
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let value = arguments.get(0).ok_or_else(|| {
            let err_type = RuntimeErrorType::ErrorInNativeFn {
                msg: "Expected one argument".to_string(),
            };
            ExecSignal::RuntimeError(RuntimeError::new(err_type, context.clone()))
        })?;

        fn deep_clone(obj: &Object, context: Rc<RefCell<ExecContext>>) -> Object {
            match obj {
                Object::Number(n) => Object::Number(*n),
                Object::Str(s) => Object::Str(s.clone()),
                Object::Bool(b) => Object::Bool(*b),
                Object::Null => Object::Null,
                Object::Dummy => Object::Dummy,
                Object::Range(s, e) => Object::Range(*s, *e),

                Object::Vec(vec_ref) => {
                    let vec_borrow = vec_ref.borrow();
                    let cloned_vec: Vec<Object> = vec_borrow
                        .iter()
                        .map(|o| deep_clone(o, context.clone()))
                        .collect();
                    Object::Vec(Rc::new(RefCell::new(cloned_vec)))
                }

                Object::Instance(inst_ref) => {
                    let inst = inst_ref.borrow();
                    let mut cloned_fields = HashMap::new();

                    for (key, val) in inst.fields.borrow().iter() {
                        cloned_fields.insert(key.clone(), deep_clone(val, context.clone()));
                    }

                    let new_inst = ReiInstance {
                        class: inst.class.clone(),
                        fields: Rc::new(RefCell::new(cloned_fields)),
                        context,
                    };
                    Object::Instance(Rc::new(RefCell::new(new_inst)))
                }

                Object::Callable(c) => Object::Callable(c.clone()), // shallow copy
                Object::MBlock(p, s) => Object::MBlock(*p, *s),     // risky af
            }
        }

        Ok(deep_clone(value, context))
    }

    fn to_string(&self) -> String {
        String::from("<native_fn>clone")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn register(env: &mut Environment) -> Result<(), ExecSignal> {
    env.define("_Mi_clone".to_string(), Object::Callable(Rc::new(ReiClone)))?;

    Ok(())
}
