use std::collections::HashMap;

use super::interpreter::Interpreter;
use super::stmt::Stmt;

use crate::crux::token::Token;
use crate::frontend::expr::Expr;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>, // stack of scopes
    current_function: FunctionType,
    current_class: ClassType,
    loop_depth: usize,
}

#[derive(Clone, Debug)]
enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
    Static,
}

#[derive(Clone, Debug)]
enum ClassType {
    None,
    Class,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            loop_depth: 0,
        }
    }

    pub fn resolve(&mut self, statements: &Vec<Stmt>) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Exception {
                do_stmts,
                fail_stmts,
                fail_binding,
                finish_stmts
            } => {
                self.resolve_stmt(do_stmts);

                self.begin_scope();
                if let Some(binding) = fail_binding {
                    self.resolve_stmt(binding);
                }
                self.resolve_stmt(fail_stmts);
                self.end_scope();

                // finish block (optional)
                if let Some(finish) = finish_stmts {
                    self.resolve_stmt(finish);
                }
            }
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
            }
            Stmt::Class {
                name,
                superclass_refs,
                methods,
                static_methods,
                expose: _,
            } => {
                let enclosing_class = self.current_class.clone();
                self.current_class = ClassType::Class;

                self.declare(name);
                self.define(name);

                if !superclass_refs.is_empty() {
                    for superclass in superclass_refs {
                        if let Expr::Variable {
                            name: super_name, ..
                        } = superclass
                        {
                            if name.lexeme == super_name.lexeme {
                                panic!("A class cannot inherit from itself.");
                            }
                        }

                        self.resolve_expr(superclass);
                    }
                    self.begin_scope();
                }

                // Begin scope for `this`
                self.begin_scope();
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert("this".to_string(), true);
                }

                for method in methods {
                    if let Stmt::Function { name, params, body } = method {
                        let prev_function = self.current_function.clone();
                        self.current_function = if name.lexeme == "init" {
                            FunctionType::Initializer
                        } else {
                            FunctionType::Method
                        };

                        self.resolve_function(params, body, self.current_function.clone());
                        self.current_function = prev_function;
                    }
                }

                for static_method in static_methods {
                    if let Stmt::Function { params, body, .. } = static_method {
                        self.resolve_function(params, body, FunctionType::Static);
                    }
                }

                if !superclass_refs.is_empty() {
                    self.end_scope();
                }

                self.end_scope();
                self.current_class = enclosing_class;
            }
            Stmt::Expression { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::Let { name, initializer } => {
                // Variable declaration
                self.declare(name);
                self.resolve_expr(initializer);
                self.define(name);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(then_branch);
                if let Some(else_stmt) = else_branch {
                    self.resolve_stmt(else_stmt);
                }
            }
            Stmt::While { condition, body } => {
                self.loop_depth += 1;
                self.resolve_expr(condition);
                self.resolve_stmt(body);
                self.loop_depth -= 1;
            }
            Stmt::Function { name, params, body } => {
                self.declare(name);
                self.define(name);
                self.resolve_function(params, body, FunctionType::Function);
            }
            Stmt::Return { keyword: _, value } => {
                if let FunctionType::None = self.current_function {
                    panic!("Cannot return from top-level code.");
                }
                if let Some(val) = value {
                    match self.current_function {
                        FunctionType::Initializer => {
                            panic!("aaARRRRR")
                        }
                        _ => {}
                    }
                    self.resolve_expr(val);
                }
            }
            Stmt::Print { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::PrintLn { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::Throw { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::Break => {
                if self.loop_depth == 0 {
                    panic!("Cannot use 'break' outside of a loop.");
                }
            }
            Stmt::Continue => {
                if self.loop_depth == 0 {
                    panic!("Cannot use 'continue' outside of a loop.");
                }
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Meta {
                id,
                keyword: _,
                method,
                ..
            } => {
                if let Some(distance) = self.resolve_this_distance() {
                    self.interpreter.resolve(*id, distance);
                } else {
                    panic!("Bad: {}", method);
                }
            }
            Expr::Range { id: _, start, end } => {
                self.resolve_expr(start);
                self.resolve_expr(end);
            }
            Expr::Assign { id: _, name, value } => {
                self.resolve_expr(value);
                self.resolve_local(expr, name);
            }
            Expr::Binary {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Logical {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::This { id: _, keyword } => {
                match self.current_class {
                    ClassType::None => panic!(), // Bad Code, add a custom error thrower for this
                    ClassType::Class => {}
                }
                self.resolve_local(expr, keyword);
            }
            Expr::Get {
                id: _,
                object,
                name: _,
            } => {
                self.resolve_expr(object);
            }
            Expr::Set {
                id: _,
                object,
                name: _,
                value,
            } => {
                self.resolve_expr(value);
                self.resolve_expr(object);
            }
            Expr::Grouping { id: _, expression } => {
                self.resolve_expr(expression);
            }
            Expr::Unary {
                id: _,
                operator: _,
                right,
            } => {
                self.resolve_expr(right);
            }
            Expr::Literal { id: _, value: _ } => {
                // Nothing to do for literals.
            }
            Expr::Variable { id: _, name } => {
                // Check for use in own initializer.
                if let Some(scope) = self.scopes.last() {
                    if let Some(false) = scope.get(&name.lexeme) {
                        panic!("Cannot read local variable in its own initializer.");
                    }
                }
                self.resolve_local(expr, name);
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let scope = self.scopes.last_mut().unwrap();
        if scope.contains_key(&name.lexeme) {
            panic!("Variable with this name already declared in this scope.");
        }
        scope.insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }

    fn resolve_function(&mut self, params: &Vec<Token>, body: &Vec<Stmt>, ty: FunctionType) {
        let enclosing_function = self.current_function.clone();
        self.current_function = ty;

        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve(body);
        self.end_scope();

        self.current_function = enclosing_function;
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr.id(), i);
                return;
            }
        }
        // Not found: leave as global.
    }

    fn resolve_this_distance(&self) -> Option<usize> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key("this") {
                return Some(i);
            }
        }
        None
    }
}
