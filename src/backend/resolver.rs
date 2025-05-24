use std::collections::HashMap;
use super::interpreter::Interpreter;
use super::stmt::Stmt;
use crate::crux::token::Token;
use crate::frontend::expr::Expr;

pub struct Resolver<'a> {

    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>, // stack of scopes
    current_function: FunctionType,
    loop_depth: usize,

}

#[derive(Clone, Debug)]
enum FunctionType { None, Function, Method }

impl<'a> Resolver<'a> {

    pub fn new(interpreter: &'a mut Interpreter) -> Self {

        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
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
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
            }
            Stmt::Class { name, methods } => {
                self.declare(name);
                self.define(name);

                for method in methods {

                    match method {
                        Stmt::Function { name: _, params, body } => {
                            self.resolve_function(params, body, FunctionType::Method);
                        },
                        _ => {}
                    }

                }

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
            Stmt::If { condition, then_branch, else_branch } => {
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
                    self.resolve_expr(val);
                }
            }
            Stmt::Print { expression } => {
                self.resolve_expr(expression);
            }
            Stmt::PrintLn { expression } => {
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
            Expr::Range { id: _, start, end } => {
                self.resolve_expr(start);
                self.resolve_expr(end);
            }
            Expr::Assign { id: _, name, value } => {
                self.resolve_expr(value);
                self.resolve_local(expr, name);
            }
            Expr::Binary { id: _, left, operator: _, right } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Logical { id: _, left, operator: _, right } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call { id: _, callee, paren: _, arguments } => {
                self.resolve_expr(callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Get { id: _, object, name: _ } => {
                self.resolve_expr(object);
            }
            Expr::Set {id: _, object, name: _, value} => {
                self.resolve_expr(value);
                self.resolve_expr(object);
            }
            Expr::Grouping { id: _, expression } => {
                self.resolve_expr(expression);
            }
            Expr::Unary { id: _, operator: _, right } => {
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
        self.scopes.last_mut().unwrap().insert(name.lexeme.clone(), true);

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
}
