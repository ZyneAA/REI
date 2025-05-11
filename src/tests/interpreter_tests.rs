use crate::backend::stmt::Stmt;
use crate::backend::interpreter::Interpreter;
use crate::crux::token::{ Object, Token, TokenType };
use crate::frontend::expr::Expr;

#[test]
pub fn test_binary_number_object() {

    let caller = Interpreter::new();

    let left = Object::Number(10.0);
    let right = Object::Number(2.0);

    let obj_plus = caller.binary_number_operation(
        left.clone(),
        right.clone(),
        Token::new(TokenType::Plus, String::from("+"), Object::Null, 1),
        |a, b| a + b
    ).unwrap();
    let obj_minus = caller.binary_number_operation(
        left.clone(),
        right.clone(),
        Token::new(TokenType::Minus, String::from("-"), Object::Null, 2),
        |a, b| a - b
    ).unwrap();
    let obj_multiply = caller.binary_number_operation(
        left.clone(),
        right.clone(),
        Token::new(TokenType::Star, String::from("*"), Object::Null, 3),
        |a, b| a * b
    ).unwrap();
    let obj_divide = caller.binary_number_operation(
        left.clone(),
        right.clone(),
        Token::new(TokenType::Slash, String::from("/"), Object::Null, 4),
        |a, b| a / b
    ).unwrap();

    println!("binary operations on Object::Number(10.0) and Object::Number(2.0)\nplus: {}, minus: {}, multiply: {}, divide: {}", obj_plus, obj_minus, obj_multiply, obj_divide);

}

#[test]
pub fn test_string_concat() {

    let left = Object::Str(String::from("Hi "));
    let right = Object::Str(String::from("Mate"));

    let concated = match (left, right) {
        (Object::Str(v), Object::Str(b)) => { v + &b },
        _ => String::from("Im actually ")
    };

    println!("two strings concated: {}", concated);

}

#[test]
pub fn comparison_test() {

    let caller = Interpreter::new();

    let com_1 = caller.is_equal(Object::Bool(true), Object::Bool(true));
    let com_2 = caller.is_equal(Object::Bool(true), Object::Bool(false));
    let com_3 = caller.is_equal(Object::Str(String::from("Hi")), Object::Bool(true));
    let com_4 = caller.is_equal(Object::Number(1.0), Object::Number(1.0));

    println!("true == true ? {}\ntrue == false ? {}\nHi == true ? {}\n1.0 == 1.0 ? {}", com_1, com_2, com_3, com_4);

    assert_eq!(true, com_1);
    assert_eq!(false, com_2);
    assert_eq!(false, com_3);
    assert_eq!(true, com_4);


}

#[test]
pub fn environment_test() -> Result<(), Box<dyn std::error::Error>> {

    let expr = Expr::Binary {
        left: Box::new(
            Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), Object::Null, 1),
                right: Box::new(Expr::Literal { value: Object::Number(6.9) })
            }
        ),
        operator: Token::new(
            TokenType::Star, "*".to_string(), Object::Null, 1
        ),
        right: Box::new(
            Expr::Grouping {
                expression: Box::new( Expr::Literal { value: Object::Number(232.0) } )
            }
        ),
    };

    let statement = Stmt::Expression {
        expression: Box::new(expr)
    };

    let mut i = Interpreter::new();
    i.interpret(vec![statement])?;

}
