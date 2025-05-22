use std::collections::HashMap;
use super::interpreter::Interpreter;
use super::stmt::Stmt;
use crate::crux::token::Token;
use crate::frontend::expr::Expr;

// Assume Token and Interpreter are defined elsewhere (Token has a `lexeme: String` field).
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>, // stack of scopes
    current_function: FunctionType,
    loop_depth: usize,
}

#[derive(Clone, Debug)]
enum FunctionType { None, Function }

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            loop_depth: 0,
        }
    }

    /// Resolve a sequence of statements.
    pub fn resolve(&mut self, statements: &[Stmt]) {
        for stmt in statements {
            self.resolve_stmt(stmt);
        }
    }

    /// Resolve a single statement by dispatching on its variant.
    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(statements);
                self.end_scope();
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
                // Function declaration
                self.declare(name);
                self.define(name);
                let enclosing = self.current_function.clone();
                self.current_function = FunctionType::Function;
                self.begin_scope();
                for param in params {
                    self.declare(param);
                    self.define(param);
                }
                self.resolve(body);
                self.end_scope();
                self.current_function = enclosing;
            }
            Stmt::Return { keyword, value } => {
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

    /// Resolve an expression by dispatching on its variant.
    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Range { start, end } => {

            }
            Expr::Assign { name, value } => {
                self.resolve_expr(value);
                self.resolve_local(expr, name);
            }
            Expr::Binary { left, operator: _, right } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Logical { left, operator: _, right } => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Call { callee, paren: _, arguments } => {
                self.resolve_expr(callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Grouping { expression } => {
                self.resolve_expr(expression);
            }
            Expr::Unary { operator: _, right } => {
                self.resolve_expr(right);
            }
            Expr::Literal { value } => {
                // Nothing to do for literals.
            }
            Expr::Variable { name } => {
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

    /// Push a new empty scope.
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the current scope.
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// Declare a name in the current scope (mark as not yet defined).
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

    /// Define a name in the current scope (mark as initialized).
    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes.last_mut().unwrap().insert(name.lexeme.clone(), true);
    }

    /// Resolve a variable reference to its depth in the scope chain.
    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, i);
                return;
            }
        }
        // Not found: leave as global.
    }
}
