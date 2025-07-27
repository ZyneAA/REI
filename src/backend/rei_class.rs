use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::exec_signal::ExecSignal;

use super::interpreter::Interpreter;

use super::rei_callable::ReiCallable;

use super::rei_function::ReiFunction;

use super::rei_instance::ReiInstance;

use crate::crux::token::Object;

use crate::backend::stack_trace::ExecContext;

#[derive(Debug, Clone)]
pub struct ReiClass {
    pub name: String,
    pub superclass_refs: Vec<Rc<ReiClass>>,
    pub methods: HashMap<String, ReiFunction>,
    pub static_methods: HashMap<String, ReiFunction>,
}

impl ReiClass {
    pub fn new(
        name: String,
        superclass_refs: Vec<Rc<ReiClass>>,
        methods: HashMap<String, ReiFunction>,
        static_methods: HashMap<String, ReiFunction>,
    ) -> Self {
        ReiClass {
            name,
            superclass_refs,
            methods,
            static_methods,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<ReiFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        for superclass in &self.superclass_refs {
            if let Some(method) = superclass.find_method(name) {
                return Some(method);
            }
        }

        None
    }

    pub fn find_static_method(&self, name: &str) -> Option<ReiFunction> {
        if let Some(method) = self.static_methods.get(name) {
            return Some(method.clone());
        }

        for superclass in &self.superclass_refs {
            if let Some(method) = superclass.find_static_method(name) {
                return Some(method);
            }
        }

        None
    }
}

impl ReiCallable for ReiClass {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
        context: Rc<RefCell<ExecContext>>,
    ) -> Result<Object, ExecSignal> {
        let instance = ReiInstance::new(self.clone(), context.clone());

        let init = self.find_method("init");
        match init {
            Some(i) => {
                i.bind(instance.clone(), context.clone())?.call(
                    interpreter,
                    arguments,
                    context.clone(),
                )?;
            }
            None => {}
        }

        instance.call()
    }

    fn arity(&self) -> usize {
        let init = self.find_method("init");
        match init {
            Some(i) => i.arity(),
            None => 0,
        }
    }

    fn to_string(&self) -> String {
        format!("{}", self.name)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
