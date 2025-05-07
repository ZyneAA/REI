use crate::backend::interpreter::Interpreter;
use crate::crux::token::Object;

#[test]
pub fn test_binary_number_object() {

    let caller = Interpreter;

    let left = Object::Number(10.0);
    let right = Object::Number(2.0);

    let obj_plus = caller.binary_number_operation(left.clone(), right.clone(), |a, b| a + b);
    let obj_minus = caller.binary_number_operation(left.clone(), right.clone(), |a, b| a - b);
    let obj_multiply = caller.binary_number_operation(left.clone(), right.clone(), |a, b| a * b);
    let obj_divide = caller.binary_number_operation(left.clone(), right.clone(), |a, b| a / b);

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

    let caller = Interpreter;

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
