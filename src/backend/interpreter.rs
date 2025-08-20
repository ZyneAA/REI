use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::crux::token::{Object, Token, TokenType};

use crate::frontend::expr;
use crate::frontend::expr::ExprId;

use crate::backend::environment::{EnvRef, Environment};
use crate::backend::rei_callable::ReiCallable;
use crate::backend::rei_class::ReiClass;
use crate::backend::stack_trace::{CallFrame, ExecContext};
use crate::backend::stmt;

use super::exec_signal::control_flow::ControlFlow;
use super::exec_signal::runtime_error::{RuntimeError, RuntimeErrorType};
use super::exec_signal::ExecSignal;

use super::native;
use super::rei_function::ReiFunction;

pub struct Interpreter {
    pub environment: EnvRef,
    locals: HashMap<ExprId, usize>,
    exposed_value: Option<Object>,
    context: Rc<RefCell<ExecContext>>,
}

impl expr::Visitor<Result<Object, ExecSignal>> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Object) -> Result<Object, ExecSignal> {
        Ok(value.clone())
    }

    fn visit_grouping_expr(&mut self, expression: &expr::Expr) -> Result<Object, ExecSignal> {
        self.evaluate(expression)
    }

    fn visit_unary_expr(
        &mut self,
        operator: &Token,
        expression: &expr::Expr,
    ) -> Result<Object, ExecSignal> {
        let right = self.evaluate(expression)?;

        match operator.token_type {
            TokenType::Minus => {
                self.check_number_operand(operator.clone(), right.clone())?;
                match right {
                    Object::Number(v) => Ok(Object::Number(-v)),
                    _ => {
                        unreachable!("Both operands should be numbers due to prior checks")
                    }
                }
            }
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => {
                let err_type = RuntimeErrorType::InvalidOperator {
                    token: operator.clone(),
                };
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
        }
    }

    fn visit_variable_expr(&mut self, id: ExprId, name: &Token) -> Result<Object, ExecSignal> {
        self.look_up_variable(id, name)
    }

    fn visit_this_expr(&mut self, id: ExprId, keyword: &Token) -> Result<Object, ExecSignal> {
        self.look_up_variable(id, keyword)
    }

    fn visit_binary_expr(
        &mut self,
        left: &expr::Expr,
        operator: &Token,
        right: &expr::Expr,
    ) -> Result<Object, ExecSignal> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Plus => match (left, right) {
                (Object::Number(a), Object::Number(b)) => Ok(Object::Number(a + b)),
                (Object::Str(a), Object::Str(b)) => Ok(Object::Str(a + &b)),
                (Object::Str(a), b) => Ok(Object::Str(a + &b.to_string())),
                (a, Object::Str(b)) => Ok(Object::Str(a.to_string() + &b)),
                _ => {
                    let err_type = RuntimeErrorType::TypeMismatch {
                        token: operator.clone(),
                    };
                    let stack_trace = RuntimeError::new(err_type, self.context.clone());
                    Err(ExecSignal::RuntimeError(stack_trace))
                }
            },
            TokenType::Minus => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.binary_number_operation(left, right, operator.clone(), |a, b| a - b)
            }
            TokenType::Slash => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.binary_number_operation(left, right, operator.clone(), |a, b| a / b)
            }
            TokenType::Star => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.binary_number_operation(left, right, operator.clone(), |a, b| a * b)
            }
            TokenType::Greater => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a > b)
            }
            TokenType::GreaterEqual => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a >= b)
            }
            TokenType::Less => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a < b)
            }
            TokenType::LessEqual => {
                self.check_number_operands(operator.clone(), left.clone(), right.clone())?;
                self.compare_number_operation(left, right, operator.clone(), |a, b| a <= b)
            }
            TokenType::EqualEqual => Ok(Object::Bool(self.is_equal(left, right))),
            TokenType::BangEqual => Ok(Object::Bool(!self.is_equal(left, right))),
            _ => {
                let err_type = RuntimeErrorType::UnexpectedBinaryOperation {
                    token: operator.clone(),
                };
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
        }
    }

    fn visit_assign_expr(
        &mut self,
        id: ExprId,
        name: &Token,
        value: &expr::Expr,
    ) -> Result<Object, ExecSignal> {
        let value = self.evaluate(value)?;
        if let Some(distance) = self.locals.get(&id) {
            Environment::assign_at(&self.environment, distance.clone(), name, value.clone());
        } else {
            self.environment.borrow_mut().assign(name, value.clone())?;
        }

        Ok(value)
    }

    fn visit_logical_expr(
        &mut self,
        left: &expr::Expr,
        operator: &Token,
        right: &expr::Expr,
    ) -> Result<Object, ExecSignal> {
        let left = self.evaluate(left)?;

        if operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(right)
    }

    fn visit_range_expr(
        &mut self,
        start: &expr::Expr,
        end: &expr::Expr,
    ) -> Result<Object, ExecSignal> {
        let start = self.evaluate(start)?;
        let end = self.evaluate(end)?;

        match (&start, &end) {
            (Object::Number(s), Object::Number(e)) => {
                if s.fract() != 0.0 || e.fract() != 0.0 {
                    let err_type = RuntimeErrorType::InvalidRangeType { start, end };
                    let stack_trace = RuntimeError::new(err_type, self.context.clone());
                    return Err(ExecSignal::RuntimeError(stack_trace));
                }
                if e < s {
                    let err_type = RuntimeErrorType::InvalidRange;
                    let stack_trace = RuntimeError::new(err_type, self.context.clone());
                    Err(ExecSignal::RuntimeError(stack_trace))
                } else {
                    Ok(Object::Range(s.clone(), e.clone()))
                }
            }
            _ => {
                let err_type = RuntimeErrorType::InvalidRangeType {
                    start: start.clone(),
                    end: end.clone(),
                };
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
        }
    }

    fn visit_call_expr(
        &mut self,
        callee: &expr::Expr,
        paren: &Token,
        arguments: &Vec<expr::Expr>,
    ) -> Result<Object, ExecSignal> {
        let callee = self.evaluate(callee)?;
        let mut args = vec![];
        for arg in arguments {
            args.push(self.evaluate(&arg)?);
        }

        match callee {
            Object::Callable(ref function) => {
                if arguments.len() != function.arity() {
                    let err_type = RuntimeErrorType::InvalidArguments {
                        token: paren.clone(),
                    };

                    let stack_trace = RuntimeError::new(err_type, self.context.clone());
                    return Err(ExecSignal::RuntimeError(stack_trace));
                }

                let callframe = CallFrame::new(function.to_string(), paren.get_location());
                self.context.borrow_mut().push_call(callframe);
                let result = function.call(self, &args, self.context.clone());
                if result.is_ok() {
                    self.context.borrow_mut().pop_call();
                }

                result
            }
            _ => {
                let err_type = RuntimeErrorType::NotCallable;
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
        }
    }

    fn visit_get_expr(
        &mut self,
        object: &Box<expr::Expr>,
        name: &Token,
    ) -> Result<Object, ExecSignal> {
        let object = self.evaluate(object)?;

        match object {
            Object::Instance(ref instance) => instance.borrow().get(name),
            Object::Callable(ref callable) => {
                if let Some(class) = callable.as_any().downcast_ref::<ReiClass>() {
                    if let Some(method) = class.find_static_method(&name.lexeme) {
                        let method: Rc<dyn ReiCallable> = Rc::new(method);
                        return Ok(Object::Callable(method));
                    }
                }
                let err_type = RuntimeErrorType::UndefinedProperty {
                    token: name.clone(),
                };
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
            _ => {
                let err_type = RuntimeErrorType::UndefinedProperty {
                    token: name.clone(),
                };
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
        }
    }

    fn visit_set_expr(
        &mut self,
        object: &expr::Expr,
        name: &Token,
        value: &expr::Expr,
    ) -> Result<Object, ExecSignal> {
        let object = self.evaluate(object)?;
        match object {
            Object::Instance(ref instance) => {
                let value = self.evaluate(value)?;
                instance.borrow_mut().set(&name.lexeme, value.clone());
                Ok(value)
            }
            _ => {
                let err_type = RuntimeErrorType::PropertyError;
                let stack_trace = RuntimeError::new(err_type, self.context.clone());
                Err(ExecSignal::RuntimeError(stack_trace))
            }
        }
    }

    fn visit_meta_expr(
        &mut self,
        id: ExprId,
        _keyword: &Token,
        method: &Token,
        args: &Vec<expr::Expr>,
    ) -> Result<Object, ExecSignal> {
        match method.lexeme.as_str() {
            "typeof" => {
                if args.len() != 2 {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@typeof() expects 2 arguments".into(),
                    };
                    return Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )));
                }

                let instance_obj = self.evaluate(&args[0])?;
                let class_name_obj = self.evaluate(&args[1])?;

                if let (Object::Instance(inst), Object::Str(class_name)) =
                    (&instance_obj, &class_name_obj)
                {
                    let mut visited = vec![inst.borrow().class.clone()];
                    while let Some(klass) = visited.pop() {
                        if klass.name == *class_name {
                            return Ok(Object::Bool(true));
                        }
                        for parent in &klass.superclass_refs {
                            visited.push(parent.clone());
                        }
                    }
                    Ok(Object::Bool(false))
                } else {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@typeof() expects (instance, string)".into(),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                }
            }

            "destroy" => {
                if args.len() != 1 {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@destroy() expects 1 argument".into(),
                    };
                    return Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )));
                }

                let arg = self.evaluate(&args[0])?;

                if let Object::Str(name) = arg {
                    if let Some(&distance) = self.locals.get(&id) {
                        let maybe_this = Environment::get_at(&self.environment, distance, "this");
                        if let Ok(Object::Instance(inst)) = maybe_this {
                            let temp = inst.borrow();
                            let mut fields = temp.fields.borrow_mut();
                            if fields.remove(&name).is_some() {
                                return Ok(Object::Null);
                            } else {
                                let err_type = RuntimeErrorType::ErrorInReflection {
                                    msg: format!("Field '{}' does not exist", name),
                                };
                                return Err(ExecSignal::RuntimeError(RuntimeError::new(
                                    err_type,
                                    self.context.clone(),
                                )));
                            }
                        }
                    }

                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "Cannot use @destroy outside of instance methods".into(),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                } else {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@destroy() expects a string argument".into(),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                }
            }

            "exist" => {
                if args.len() != 1 {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@exist() expects 1 argument".into(),
                    };
                    return Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )));
                }

                let arg = self.evaluate(&args[0])?;

                if let Object::Str(name) = arg {
                    if let Some(&distance) = self.locals.get(&id) {
                        let maybe_this = Environment::get_at(&self.environment, distance, "this");
                        if let Ok(Object::Instance(inst)) = maybe_this {
                            let has = inst.borrow().fields.borrow().contains_key(&name);
                            return Ok(Object::Bool(has));
                        }
                    }

                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "Cannot use @exist outside of instance methods".into(),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                } else {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@exist() expects a string argument".into(),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                }
            }

            "mutate" => {
                if args.len() != 2 {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "@mutate() expects 2 argument".into(),
                    };
                    return Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )));
                }

                let name = match self.evaluate(&args[0])? {
                    Object::Str(s) => s,
                    _ => {
                        let err_type = RuntimeErrorType::ErrorInReflection {
                            msg: "@mutate() expects 1st argument to be string".into(),
                        };
                        return Err(ExecSignal::RuntimeError(RuntimeError::new(
                            err_type,
                            self.context.clone(),
                        )));
                    }
                };

                let value = self.evaluate(&args[1])?;

                let distance = self.locals.get(&id).ok_or_else(|| {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: "Cannot use @mutate outside of instance method".into(),
                    };
                    ExecSignal::RuntimeError(RuntimeError::new(err_type, self.context.clone()))
                })?;

                let maybe_this = Environment::get_at(&self.environment, *distance, "this")?;
                let instance = match maybe_this {
                    Object::Instance(inst) => inst,
                    _ => panic!(), // optional: replace with a proper error if desired
                };

                let inst_ref = instance.borrow();
                let mut klass = inst_ref.class.clone();
                let mut exists = inst_ref.fields.borrow().contains_key(&name);

                while !exists {
                    exists = klass.methods.contains_key(&name);
                    if exists {
                        break;
                    }
                    if let Some(super_ref) = klass.superclass_refs.first() {
                        klass = super_ref.clone();
                    } else {
                        break;
                    }
                }

                if exists {
                    drop(inst_ref);
                    instance
                        .borrow_mut()
                        .fields
                        .borrow_mut()
                        .insert(name, value);
                    Ok(Object::Null)
                } else {
                    let err_type = RuntimeErrorType::ErrorInReflection {
                        msg: format!(
                            "@mutate() failed: '{}' is not a valid class attribute",
                            name
                        ),
                    };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                }
            }

            _ => {
                let err_type = RuntimeErrorType::ErrorInReflection {
                    msg: format!("Unknown meta method '@{}'", method.lexeme),
                };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    self.context.clone(),
                )))
            }
        }
    }
}

impl stmt::Visitor<Result<(), ExecSignal>> for Interpreter {
    fn visit_exception_stmt(
        &mut self,
        do_stmts: &stmt::Stmt,
        fail_stmts: &stmt::Stmt,
        fail_binding: &Option<Box<stmt::Stmt>>,
        finish_stmts: &Option<Box<stmt::Stmt>>,
    ) -> Result<(), ExecSignal> {
        let mut result = self.execute(do_stmts);

        if let Err(signal) = result {
            match signal {
                ExecSignal::RuntimeError(err_obj) => {
                    let is_custom_msg =
                        matches!(&err_obj.err_type, RuntimeErrorType::CustomMsg { .. });

                    if is_custom_msg {
                        // Normal recoverable
                        if let Some(binding_stmt) = fail_binding {
                            let exception = Object::Exception(Box::new(err_obj));
                            self.define_fail_binding(binding_stmt, exception)?;
                        }
                        result = self.execute(fail_stmts);
                    } else {
                        // Unrecoverable runtime error, still run finish, then terminate
                        if let Some(finish_block) = finish_stmts {
                            let _ = self.execute(finish_block);
                        }
                        eprint!("{}", err_obj);
                        self.context.borrow_mut().pop_call();
                        std::process::exit(1);
                    }
                }
                other => return Err(other),
            }
        }

        if let Some(finish_block) = finish_stmts {
            // even if do/fail errored, we *still* run finish
            // if finish throws, it overrides earlier errors
            result = self.execute(finish_block);
        }

        result
    }

    fn visit_class_stmt(
        &mut self,
        name: &Token,
        superclasses: &Vec<expr::Expr>,
        methods: &Vec<stmt::Stmt>,
        static_methods: &Vec<stmt::Stmt>,
        expose: &bool,
    ) -> Result<(), ExecSignal> {
        let mut superclass_objs = Vec::new();
        let mut superclass_refs: Vec<Rc<ReiClass>> = Vec::with_capacity(superclasses.len());

        for sup_expr in superclasses.iter() {
            let evaluated = self.evaluate(sup_expr)?;
            match &evaluated {
                Object::Callable(c) => {
                    if let Some(klass) = c.as_any().downcast_ref::<ReiClass>() {
                        let rc_class = Rc::new(klass.clone());
                        superclass_refs.push(rc_class.clone());
                        superclass_objs.push(Object::Callable(rc_class));
                    } else {
                        let msg = format!("{} is not a super class", evaluated.to_string());
                        let err_type = RuntimeErrorType::ParentClassError { msg };
                        return Err(ExecSignal::RuntimeError(RuntimeError::new(
                            err_type,
                            self.context.clone(),
                        )));
                    }
                }
                _ => {
                    let msg = format!("{} is not a super class", evaluated.to_string());
                    let err_type = RuntimeErrorType::ParentClassError { msg };
                    return Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )));
                }
            }
        }

        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Null)?;

        let mut temp_env = None;
        if !superclass_objs.is_empty() {
            let env = Environment::from_enclosing(self.environment.clone(), self.context.clone());
            for base_obj in superclass_objs.into_iter() {
                env.borrow_mut().define(base_obj.to_string(), base_obj)?;
            }
            temp_env = Some(self.environment.clone()); // save old env
            self.environment = env;
        }

        let mut klass_methods = HashMap::new();
        for method in methods {
            if let stmt::Stmt::Function {
                name: method_name,
                params,
                body,
            } = method
            {
                let is_init = method_name.lexeme == "init";
                let func = ReiFunction::new(
                    method_name.clone(),
                    params.clone(),
                    body.clone(),
                    self.environment.clone(),
                    is_init,
                );
                klass_methods.insert(method_name.lexeme.clone(), func);
            }
        }

        let mut static_klass_methods = HashMap::new();
        for static_method in static_methods {
            if let stmt::Stmt::Function {
                name: method_name,
                params,
                body,
            } = static_method
            {
                let func = ReiFunction::new(
                    method_name.clone(),
                    params.clone(),
                    body.clone(),
                    self.environment.clone(),
                    false,
                );
                static_klass_methods.insert(method_name.lexeme.clone(), func);
            }
        }

        let klass = ReiClass::new(
            name.lexeme.clone(),
            superclass_refs,
            klass_methods,
            static_klass_methods,
        );
        let callable: Rc<dyn ReiCallable> = Rc::new(klass);
        if *expose {
            self.exposed_value = Some(Object::Callable(callable.clone()));
        }

        if let Some(env) = temp_env {
            self.environment = env;
        }

        self.environment
            .borrow_mut()
            .assign(name, Object::Callable(callable))
    }

    fn visit_expression_stmt(&mut self, expression: &expr::Expr) -> Result<(), ExecSignal> {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &expr::Expr) -> Result<(), ExecSignal> {
        let value = self.evaluate(expression)?;
        print!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_throw_stmt(&mut self, expression: &Box<expr::Expr>) -> Result<(), ExecSignal> {
        let obj = self.evaluate(expression)?;
        let value = format!("{}", self.stringify(&obj));

        let err_type = RuntimeErrorType::CustomMsg { msg: value };
        return Err(ExecSignal::RuntimeError(RuntimeError::new(
            err_type,
            self.context.clone(),
        )));
    }

    fn visit_fatal_stmt(&mut self, expression: &Box<expr::Expr>) -> Result<(), ExecSignal> {
        let obj = self.evaluate(expression)?;
        let value = format!("{}", self.stringify(&obj));

        let err_type = RuntimeErrorType::CustomMsgFatal { msg: value };
        return Err(ExecSignal::RuntimeError(RuntimeError::new(
            err_type,
            self.context.clone(),
        )));
    }

    fn visit_println_stmt(&mut self, expression: &expr::Expr) -> Result<(), ExecSignal> {
        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(&value));
        Ok(())
    }

    fn visit_let_stmt(&mut self, name: &Token, initializer: &expr::Expr) -> Result<(), ExecSignal> {
        let value = self.evaluate(initializer)?;
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value)?;

        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &Vec<stmt::Stmt>) -> Result<(), ExecSignal> {
        let new_env = Environment::from_enclosing(self.environment.clone(), self.context.clone());
        self.execute_block(statements, new_env)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &expr::Expr,
        then_branch: &stmt::Stmt,
        else_branch: &Option<Box<stmt::Stmt>>,
    ) -> Result<(), ExecSignal> {
        let obj = self.evaluate(condition)?;

        if self.is_truthy(&obj) {
            self.execute(then_branch)
        } else {
            match else_branch {
                Some(v) => self.execute(v),
                None => Ok(()),
            }
        }
    }

    fn visit_while_stmt(
        &mut self,
        condition: &expr::Expr,
        body: &stmt::Stmt,
    ) -> Result<(), ExecSignal> {
        loop {
            let cond = self.evaluate(condition)?;
            if !self.is_truthy(&cond) {
                break;
            }
            match self.execute(body) {
                Ok(_) => {}
                Err(ExecSignal::ControlFlow(ControlFlow::Break)) => break,
                Err(ExecSignal::ControlFlow(ControlFlow::Continue)) => continue,
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn visit_break_stmt(&mut self) -> Result<(), ExecSignal> {
        Err(ExecSignal::ControlFlow(ControlFlow::Break))
    }

    fn visit_continue_stmt(&mut self) -> Result<(), ExecSignal> {
        Err(ExecSignal::ControlFlow(ControlFlow::Continue))
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<stmt::Stmt>,
    ) -> Result<(), ExecSignal> {
        let function = ReiFunction::new(
            name.clone(),
            params.clone(),
            body.clone(),
            self.environment.clone(),
            false,
        );
        let callable: Rc<dyn ReiCallable> = Rc::new(function);
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Callable(callable))?;
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        _keyword: &Token,
        value: &Option<Box<expr::Expr>>,
    ) -> Result<(), ExecSignal> {
        let value = match value {
            Some(v) => self.evaluate(v)?,
            None => Object::Null,
        };
        Err(ExecSignal::ControlFlow(ControlFlow::Return(value)))
    }
}

impl Interpreter {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let context = Rc::new(RefCell::new(ExecContext::new()));
        let environment = Environment::global(context.clone());
        let locals = HashMap::new();
        native::register_all_native_fns(environment.borrow_mut())?;
        Ok(Interpreter {
            environment,
            locals,
            exposed_value: None,
            context,
        })
    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) {
        for stmt in statements {
            match self.execute(&stmt) {
                Ok(()) => {}
                Err(e) => {
                    eprint!("{}", e);
                    if let ExecSignal::RuntimeError(runtime_error) = &e {
                        match runtime_error.err_type {
                            RuntimeErrorType::CustomMsg { .. } => {}
                            _ => std::process::exit(1),
                        }
                    }
                    self.context.borrow_mut().pop_call();
                }
            }
        }
    }

    fn execute(&mut self, statement: &stmt::Stmt) -> Result<(), ExecSignal> {
        statement.accept(self)
    }

    pub fn resolve(&mut self, expression_id: ExprId, depth: usize) {
        self.locals.insert(expression_id, depth);
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<stmt::Stmt>,
        env: EnvRef,
    ) -> Result<(), ExecSignal> {
        self.with_env(env, |interpreter| {
            for stmt in statements {
                interpreter.execute(stmt)?;
            }
            Ok(())
        })
    }

    pub fn stringify(&mut self, object: &Object) -> String {
        match object {
            Object::Null => "null".to_string(),
            Object::Number(n) => {
                let mut s = n.to_string();
                if s.ends_with(".0") {
                    s.truncate(s.len() - 2); // yeet the ".0"
                }
                s
            }
            Object::Range(s, e) => format!("<range | {}..{}>", s, e),
            Object::MBlock(p, s) => format!("<mblock | ptr: {:p} size: {}>", p, s),
            Object::Bool(b) => b.to_string(),
            Object::Dummy => "dummy".to_string(),
            Object::Str(s) => s.clone(),
            Object::Callable(c) => c.to_string(),
            Object::Instance(i) => i.borrow().to_string(),
            Object::Vec(v) => {
                let vec_borrow = v.borrow();
                let elements: Vec<String> = vec_borrow.iter().map(|o| o.to_string()).collect();
                format!("[{}]", elements.join(", "))
            }
            Object::Exception(e) => format!("{}", e),
        }
    }

    fn evaluate(&mut self, expression: &expr::Expr) -> Result<Object, ExecSignal> {
        expression.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Bool(v) => *v,
            _ => true,
        }
    }

    fn look_up_variable(&self, expr_id: ExprId, name: &Token) -> Result<Object, ExecSignal> {
        if let Some(&distance) = self.locals.get(&expr_id) {
            Environment::get_at(&self.environment, distance, &name.lexeme)
        } else {
            self.environment.borrow_mut().get(name)
        }
    }

    fn check_number_operand(&self, operator: Token, operand: Object) -> Result<(), ExecSignal> {
        match operand {
            Object::Number(_) => Ok(()),
            _ => {
                let err_type = RuntimeErrorType::OperandMustBeNumber { token: operator };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    self.context.clone(),
                )))
            }
        }
    }

    fn check_number_operands(
        &self,
        operator: Token,
        a: Object,
        b: Object,
    ) -> Result<(), ExecSignal> {
        match (a, b) {
            (Object::Number(_), Object::Number(_)) => Ok(()),
            _ => {
                let err_type = RuntimeErrorType::OperandMustBeNumber { token: operator };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    self.context.clone(),
                )))
            }
        }
    }

    pub fn is_equal(&self, a: Object, b: Object) -> bool {
        match (a, b) {
            (Object::Null, Object::Null) => true,
            (Object::Bool(a), Object::Bool(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            _ => false,
        }
    }

    pub fn binary_number_operation<T>(
        &self,
        a: Object,
        b: Object,
        token: Token,
        op: T,
    ) -> Result<Object, ExecSignal>
    where
        T: Fn(f64, f64) -> f64,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => {
                if token.token_type == TokenType::Slash && y == 0.0 {
                    let err_type = RuntimeErrorType::DividedByZero { token };
                    Err(ExecSignal::RuntimeError(RuntimeError::new(
                        err_type,
                        self.context.clone(),
                    )))
                } else {
                    Ok(Object::Number(op(x, y)))
                }
            }
            _ => {
                let err_type = RuntimeErrorType::OperandMustBeNumber { token };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    self.context.clone(),
                )))
            }
        }
    }

    fn compare_number_operation<F>(
        &self,
        a: Object,
        b: Object,
        token: Token,
        op: F,
    ) -> Result<Object, ExecSignal>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (a, b) {
            (Object::Number(x), Object::Number(y)) => Ok(Object::Bool(op(x, y))),
            _ => {
                let err_type = RuntimeErrorType::OperandMustBeNumber { token };
                Err(ExecSignal::RuntimeError(RuntimeError::new(
                    err_type,
                    self.context.clone(),
                )))
            }
        }
    }

    pub fn with_env<F, R>(&mut self, env: EnvRef, f: F) -> R
    where
        F: FnOnce(&mut Interpreter) -> R,
    {
        let previous = self.environment.clone();
        self.environment = env;
        let result = f(self);
        self.environment = previous;
        result
    }

    fn define_fail_binding(
        &mut self,
        binding_stmt: &stmt::Stmt,
        err_value: Object,
    ) -> Result<(), ExecSignal> {
        if let stmt::Stmt::Let { name, .. } = binding_stmt {
            self.environment
                .borrow_mut()
                .define(name.lexeme.clone(), err_value)?;
            Ok(())
        } else {
            unreachable!("fail binding must always be a let stmt")
        }
    }
}
