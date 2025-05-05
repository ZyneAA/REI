use std::boxed::Box;

use crate::frontend::ast_printer::AstPrinter;
use crate::frontend::expr::Expr;
use crate::crux::token::{ Token, TokenType, Object };

#[test]
pub fn test_binary_expression() {

    let mut printer = AstPrinter;

    let expr = Expr::Binary {
        left: Box::new(
            Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), Object::Non, 1),
                right: Box::new(Expr::Literal { value: Object::Number(6.9) })
            }
        ),
        operator: Token::new(
            TokenType::Star, "*".to_string(), Object::Non, 1
        ),
        right: Box::new(
            Expr::Grouping {
                expression: Box::new( Expr::Literal { value: Object::Number(232.0) } )
            }
        ),
    };

    let output = expr.accept(&mut printer);
    println!("-6.9 * group 232 should produce the nested expression: {}", output);
    assert_eq!(output, "(* (- 6.9) (group 232))");

}
