use std::result::Result;

use crate::crux::error::ParseError;
use crate::crux::token::{ Token, TokenType, Object };
use super::expr;
use super::expr::ExprId;
use crate::backend::stmt;

pub struct Parser {

    tokens: Vec<Token>,
    current: usize,
    id_counter: usize,
    pub is_error: bool,
    pub errors: Vec<ParseError>

}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, id_counter: 0, is_error: false, errors: vec![] }
    }

    pub fn parse(&mut self) -> Vec<stmt::Stmt> {

        let mut statements = Vec::new();
        while !self.is_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.is_error = true;
                    self.errors.push(e);
                    self.synchronize();
                }
            }
        }

        statements

    }

    fn statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        // std out
        if self.rmatch(&[TokenType::Print])? {
            self.print_statement()
        }
        else if self.rmatch(&[TokenType::PrintLn])? {
            self.println_statement()
        }

        // Return
        else if self.rmatch(&[TokenType::Return])? {
            self.return_statement()
        }

        // Block
        else if self.rmatch(&[TokenType::LeftBrace])? {
            self.block()
        }

        // Looping
        else if self.rmatch(&[TokenType::While])? {
            self.while_statement()
        }
        else if self.rmatch(&[TokenType::For])? {
            self.for_statement()
        }
        else if self.rmatch(&[TokenType::Loop])? {
            self.loop_statement()
        }

        // Condition
        else if self.rmatch(&[TokenType::If])? {
            self.if_statement()
        }

        // Control Signal
        else if self.rmatch(&[TokenType::Break])? {
            self.break_statement()
        }
        else if self.rmatch(&[TokenType::Continue])? {
            self.continue_statement()
        }

        // Class
        else if self.rmatch(&[TokenType::Class])? {
            self.class_declaration()
        }
        else {
            self.expression_statement()
        }

    }

    fn class_declaration(&mut self) -> Result<stmt::Stmt, ParseError> {

        let name = self.consume(&TokenType::Identifier, "Expected a class name")?.clone();
        self.consume(&TokenType::LeftBrace, "Expected { before class body")?;

        let mut methods = vec![];
        while !self.check(&TokenType::RightBrace) && !self.is_end() {
            methods.push(self.function("function")?);
        }

        self.consume(&TokenType::RightBrace, "EXpected } after class body")?;
        let class = stmt::Stmt::Class {
            name,
            methods
        };

        Ok(class)

    }

    fn return_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        let keyword = self.previous().clone();
        let value = if !self.check(&TokenType::Semicolon) {
            Some(Box::new(self.expression()?))
        }
        else {
            None
        };

        self.consume(&TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(stmt::Stmt::Return {
            keyword: keyword.clone(),
            value
        })

    }

    fn break_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        self.consume(&TokenType::Semicolon, "Expected ','")?;
        Ok(stmt::Stmt::Break)

    }

    fn continue_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        self.consume(&TokenType::Semicolon, "Expected ','")?;
        Ok(stmt::Stmt::Continue)

    }

    fn for_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        self.consume(&TokenType::LeftParen, "Expected a '(' after 'for'")?;

        let initializer = if self.rmatch(&[TokenType::Semicolon])? {
            None
        }
        else if self.rmatch(&[TokenType::Let])? {
            Some(self.var_declaration()?)
        }
        else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        }
        else { None };

        self.consume(&TokenType::Semicolon, "Expected a ';'")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        }
        else { None };

        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if let Some(inc) = increment {
            body = stmt::Stmt::Block {
                statements: vec![body, stmt::Stmt::Expression { expression: Box::new(inc) }]
            }
        };

        let cond_expr = condition.unwrap_or(expr::Expr::Literal { id: self.next_id(), value: Object::Bool(true) });
        body = stmt::Stmt::While {
            condition: Box::new(cond_expr), body: Box::new(body)
        };

        if let Some(init) = initializer {
            body = stmt::Stmt::Block {
                statements: vec![init, body]
            }
        };

        Ok(body)

    }

    fn loop_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        self.consume(&TokenType::LeftParen, "Expected '(' after 'loop'")?;
        self.consume(&TokenType::Let, "Expected 'let' in loop declaration")?;
        let name = self.consume(&TokenType::Identifier, "Expected loop variable name")?.clone();
        self.consume(&TokenType::Equal, "Expected '=' in loop declaration")?;
        let range_expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expected ; for skipping")?;
        let times = self.expression()?;

        let (start_expr, end_expr) = match range_expr {
            expr::Expr::Range { id: _, start, end } => (*start, *end),
            _ => {
                return Err(ParseError::SyntaxError {
                    token: self.peek().clone(),
                    message: "Expected range expression (like 0..10) in loop declaration".into(),
                });
            }
        };
        self.consume(&TokenType::RightParen, "Expected ')' after loop range declaration")?;

        let body = self.statement()?;

        // Same desugaring as `for`: convert to a while
        let init = stmt::Stmt::Let {
            name: name.clone(),
            initializer: Box::new(start_expr),
        };

        let direction = match times {
            expr::Expr::Literal { id: _, value: Object::Number(_) } => {
                TokenType::Less
            }
            _ => TokenType::Greater
        };

        let condition = expr::Expr::Binary {
            id: self.next_id(),
            left: Box::new(expr::Expr::Variable { id: self.next_id(), name: name.clone() }),
            operator: Token::fake(direction),
            right: Box::new(end_expr),
        };

        let increment = stmt::Stmt::Expression {
            expression: Box::new(expr::Expr::Assign {
                id: self.next_id(),
                name: name.clone(),
                value: Box::new(expr::Expr::Binary {
                    id: self.next_id(),
                    left: Box::new(expr::Expr::Variable { id: self.next_id(), name: name.clone() }),
                    operator: Token::fake(TokenType::Plus),
                    right: Box::new(times),
                }),
            }),
        };

        let while_body = stmt::Stmt::Block {
            statements: vec![body, increment],
        };

        let while_stmt = stmt::Stmt::While {
            condition: Box::new(condition),
            body: Box::new(while_body),
        };

        Ok(stmt::Stmt::Block {
            statements: vec![init, while_stmt],
        })
    }

    fn while_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        self.consume(&TokenType::LeftParen, "Expected a '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected a ')' after condition")?;
        let body = self.statement()?;

        Ok(stmt::Stmt::While {
            condition: Box::new(condition),
            body: Box::new(body)
        })

    }

    fn if_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        self.consume(&TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after if condition")?;

        let then_branch = self.statement()?;
        let else_branch = if self.rmatch(&[TokenType::Else])? {
            let a = self.statement()?;
            Some(Box::new(a))
        }
        else {
            None
        };
        Ok(stmt::Stmt::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch
        })

    }

    fn block(&mut self) -> Result<stmt::Stmt, ParseError> {

        let mut statements: Vec<stmt::Stmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expected a } after block")?;
        Ok(stmt::Stmt::Block {
            statements
        })

    }

    fn declaration(&mut self) -> Result<stmt::Stmt, ParseError> {

        let res = (|| {

            if self.rmatch(&[TokenType::Fn])? {
                return self.function("function");
            }
            if self.rmatch(&[TokenType::Let])? {
                return self.var_declaration();
            }

            self.statement()

        })();

        match res {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }

    }

    fn function(&mut self, kind: &str) -> Result<stmt::Stmt, ParseError> {

        let err = format!("Expect {} name", kind);
        let err1 = format!("Expect '(' {} name", kind);
        let err2 = format!("Expect '{{' before {} name", kind);

        let name = self.consume(&TokenType::Identifier, &err)?.clone();
        self.consume(&TokenType::LeftParen, &err1)?;

        let mut parameters = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParseError::SyntaxError {
                        token: self.peek().clone(),
                        message: "Can't have more than 255 parameters".into()
                    })
                }

                let param = self.consume(&TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(param.clone());

                if !self.rmatch(&[TokenType::Comma])? {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, &err2)?;
        self.consume(&TokenType::LeftBrace, &err2)?;
        let body = match self.block()? {
            stmt::Stmt::Block { statements } => statements,
            _ => {
                panic!("AHHHHHHHHH");
            }
        };

        Ok(stmt::Stmt::Function {
            name: name.clone(),
            params: parameters,
            body
        })

    }

    fn print_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        let value = self.expression()?;

        if self.peek().token_type == TokenType::DotDot {

            self.consume(&TokenType::DotDot, "Expected '.' in Range")?;
            let end = self.expression()?;
            self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration")?;

            let range = expr::Expr::Range {
                id: self.next_id(),
                start: Box::new(value),
                end: Box::new(end)
            };
            return Ok(stmt::Stmt::Print {
                expression: Box::new(range)
            })
        }

        match self.consume(&TokenType::Semicolon, "Expected ; after value") {
            Ok(_) => {
                Ok(stmt::Stmt::Print { expression: Box::new(value) })
            },
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }

    }

    fn println_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        let value = self.expression()?;

        if self.peek().token_type == TokenType::DotDot {

            self.consume(&TokenType::DotDot, "Expected '.' in Range")?;
            let end = self.expression()?;
            self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration")?;

            let range = expr::Expr::Range {
                id: self.next_id(),
                start: Box::new(value),
                end: Box::new(end)
            };
            return Ok(stmt::Stmt::PrintLn {
                expression: Box::new(range)
            })
        }

        match self.consume(&TokenType::Semicolon, "Expected ; after value") {
            Ok(_) => {
                Ok(stmt::Stmt::PrintLn { expression: Box::new(value) })
            }
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }

    }

    fn var_declaration(&mut self) -> Result<stmt::Stmt, ParseError> {

        let name = self.consume(&TokenType::Identifier, "Expect variable name")?.clone();

        let initializer = if self.rmatch(&[TokenType::Equal])? {
            Some(self.expression()?)
        }
        else {
            None
        };

        if self.peek().token_type == TokenType::DotDot {
            let initializer = initializer.unwrap();
            self.consume(&TokenType::DotDot, "Expect '..' for Range type")?;
            let end = self.expression()?;
            self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration")?;

            let range = stmt::Stmt::Let {
                name,
                initializer: Box::new(expr::Expr::Range {
                    id: self.next_id(),
                    start: Box::new(initializer),
                    end: Box::new(end)
                })
            };

            return Ok(range);

        }

        self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration")?;

        Ok(stmt::Stmt::Let {
            name,
            initializer: Box::new(initializer.unwrap_or(expr::Expr::Literal {
                id: self.next_id(),
                value: Object::Null,
            })),
        })

    }

    fn expression_statement(&mut self) -> Result<stmt::Stmt, ParseError> {

        let expr = self.expression()?;
        match self.consume(&TokenType::Semicolon, "Expected ; after expression") {
            Ok(_) => Ok(stmt::Stmt::Expression { expression: Box::new(expr) }),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }

    }

    fn expression(&mut self) -> Result<expr::Expr, ParseError> {
        self.range()
    }

    fn range(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.assignment()?;

        while self.rmatch(&[TokenType::DotDot])? {
            let right = self.equality()?;
            expr = expr::Expr::Range {
                id: self.next_id(),
                start: Box::new(expr),
                end: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<expr::Expr, ParseError> {

        let expr = self.or()?;

        if self.rmatch(&[TokenType::Setter])? {
            let name = self.consume(&TokenType::Identifier, "Expect property name after '<-'")?.clone();
            self.consume(&TokenType::Equal, "Expect '=' after property name")?;
            let value = self.assignment()?;

            return Ok(expr::Expr::Set {
                id: self.next_id(),
                object: Box::new(expr),
                name,
                value: Box::new(value)
            });
        }

        if self.rmatch(&[TokenType::Equal])? {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                expr::Expr::Variable { id: _, name } => {
                    Ok(expr::Expr::Assign {
                        id: self.next_id(),
                        name,
                        value: Box::new(value),
                    })
                }
                expr::Expr::Get { id: _, object, name } => {
                    Ok(expr::Expr::Set {
                        id: self.next_id(),
                        object,
                        name,
                        value: Box::new(value)
                    })
                }
                _ => {
                    Err(ParseError::SyntaxError {
                        token: equals.clone(),
                        message: "Invalid assignment target ".into(), }
                    )
                }
            }

        }
        else {
            Ok(expr)
        }

    }

    fn or(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.and()?;

        while self.rmatch(&[TokenType::Or])? {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = expr::Expr::Logical {
                id: self.next_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)

    }

    fn and(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.equality()?;

        while self.rmatch(&[TokenType::And])? {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = expr::Expr::Logical {
                id: self.next_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right)
            }
        }

        Ok(expr)

    }

    fn equality(&mut self) -> Result<expr::Expr, ParseError>  {

        let mut expr = self.comparison()?;

        while self.rmatch(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison()?);
            expr = expr::Expr::Binary {
                id: self.next_id(),
                left: Box::new(expr),
                operator,
                right
            };
        }

        Ok(expr)

    }

    fn comparison(&mut self) -> Result<expr::Expr, ParseError>  {

        let mut expr = self.term()?;

        while self.rmatch(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual])? {
            let operator = self.previous().clone();
            let right = Box::new(self.term()?);
            expr = expr::Expr::Binary {
                id: self.next_id(),
                left: Box::new(expr),
                operator,
                right
            };
        }

        Ok(expr)

    }

    fn term(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.factor()?;

        while self.rmatch(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous().clone();
            let right = Box::new(self.factor()?);
            expr = expr::Expr::Binary {
                id: self.next_id(),
                left: Box::new(expr),
                operator,
                right
            }
        }

        Ok(expr)

    }

    fn factor(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.unary()?;

        while self.rmatch(&[TokenType::Slash, TokenType::Star])? {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            expr = expr::Expr::Binary {
                id: self.next_id(),
                left: Box::new(expr),
                operator,
                right
            }
        }

        Ok(expr)

    }

    fn unary(&mut self) -> Result<expr::Expr, ParseError> {

        if self.rmatch(&[TokenType::Bang, TokenType::Minus])? {
            let operator= self.previous().clone();
            let right = Box::new(self.unary()?);
            return Ok(expr::Expr::Unary {
                id: self.next_id(),
                operator,
                right
            })
        }

        self.call()

    }

    fn call(&mut self)-> Result<expr::Expr, ParseError> {

        let mut expr = self.primary()?;

        loop {
            if self.rmatch(&[TokenType::LeftParen])? {
                expr = self.finish_call(&expr)?;
            }
            else if self.rmatch(&[TokenType::Dot, TokenType::Getter])? {
                let name = self.consume(&TokenType::Identifier, "Expected property name")?.clone();
                expr = expr::Expr::Get {
                    id: self.next_id(),
                    object: Box::new(expr),
                    name
                }
            }
            else {
                break;
            }
        }

        Ok(expr)

    }

    fn finish_call(&mut self, callee: &expr::Expr) -> Result<expr::Expr, ParseError> {

        let mut arguments = vec![];

        if !self.check(&TokenType::RightParen) {
            loop {

                if arguments.len() > 255 {
                    return Err(ParseError::SyntaxError {
                        token: self.peek().clone(),
                        message: "Can't have more than 255 arguments.".into(),
                    })
                }
                if self.peek().token_type == TokenType::DotDot {
                    println!("321313123");
                }
                arguments.push(self.expression().unwrap());
                if !self.rmatch(&[TokenType::Comma])? {
                    break;
                }

            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments")?.clone();
        Ok(expr::Expr::Call {
            id: self.next_id(),
            callee: Box::new(callee.clone()),
            paren,
            arguments,
        })

    }

    fn primary(&mut self) -> Result<expr::Expr, ParseError> {

        if self.rmatch(&[TokenType::False])? {
            return Ok(expr::Expr::Literal {
                id: self.next_id(),
                value: Object::Bool(false),
            });
        }

        if self.rmatch(&[TokenType::True])? {
            return Ok(expr::Expr::Literal {
                id: self.next_id(),
                value: Object::Bool(true),
            });
        }

        if self.rmatch(&[TokenType::Null])? {
            return Ok(expr::Expr::Literal {
                id: self.next_id(),
                value: Object::Null,
            });
        }

        if self.rmatch(&[TokenType::Number, TokenType::String])? {
            return Ok(expr::Expr::Literal {
                id: self.next_id(),
                value: self.previous().literal.clone(),
            });
        }

        if self.rmatch(&[TokenType::This])? {
            return Ok(expr::Expr::This {
                id: self.next_id(),
                keyword: self.previous().clone()
            });
        }

        if self.rmatch(&[TokenType::Identifier])? {
            return Ok(expr::Expr::Variable {
                id: self.next_id(),
                name: self.previous().clone()
            });
        }

        if self.rmatch(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ) after expression")?;
            return Ok(expr::Expr::Grouping {
                id: self.next_id(),
                expression: Box::new(expr),
            });
        }

        Err(ParseError::SyntaxError {
            token: self.previous().clone(),
            message: "Expected expression".into(),
        })

    }

    fn synchronize(&mut self) {

        self.advance();
        while !self.is_end() {

            if self.previous().token_type == TokenType::Semicolon { return }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::Loop
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::PrintLn
                | TokenType::Return => return,
                _ => {},
            }

            self.advance();

        }

    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {

        if self.check(token_type) {
            Ok(self.advance())
        }
        else {
            Err(ParseError::SyntaxError {
                token: self.previous().clone(),
                message: message.to_string(),
            })
        }

    }

    fn rmatch(&mut self, types: &[TokenType]) -> Result<bool, ParseError> {

        for ty in types {
            if self.check(ty) {
                self.advance();
                return Ok(true);
            }
        }

        Ok(false)

    }

    fn check(&self, token_type: &TokenType) -> bool {

        if self.is_end() {
            false
        }
        else {
            &self.peek().token_type == token_type
        }

    }

    fn advance(&mut self) -> &Token{

        if !self.is_end() {
            self.current += 1;
        }
        self.previous()

    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn next_id(&mut self) -> ExprId {

        let id = self.id_counter;
        self.id_counter += 1;
        ExprId(id)

    }

}
