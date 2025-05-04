use std::boxed::Box;

use crate::frontend::ast_printer::AstPrinter;
use crate::frontend::expr::Expr;
use crate::frontend::token::{ Token, TokenType, Object };

#[test]
pub fn test_binary_expression() {

    let mut printer = AstPrinter;

    let expr = Expr::Binary {
        left: Box::new(Expr::Literal { value: Object::Number(1.0) }),
        operator: Token::new(TokenType::Star, "*".to_string(), Object::Non, 1),
        right: Box::new(Expr::Literal { value: Object::Number(2.0) }),
    };

    assert_eq!(expr.accept(&mut printer), "(* 1 2)");

}
