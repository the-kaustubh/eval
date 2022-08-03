mod stack;
mod queue;

use stack::Stack;
use queue::Queue;
use std::env;

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
    RightParenthesis,
    LeftParenthesis,
    Invalid,
    // Raise,
}

fn is_operator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '/' => true,
        '*' => true,
        '(' => true,
        ')' => true,
        _   => false
    }
}

fn which_operator(c: char) -> Operator {
    match c {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        '/' => Operator::Divide,
        '*' => Operator::Multiply,
        '(' => Operator::RightParenthesis,
        ')' => Operator::LeftParenthesis,
        _   => Operator::Invalid
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let mut operand_stack: Stack<i32> = Stack::new();
    let mut operator_stack : Stack<Operator> = Stack::new();

    let mut operand: i32 = 0;
    for c in args[1].chars() {
        if c == ' ' {
            continue
        }
        if is_operator(c) {
            if c == ')' {

            }
            operand_stack.push(operand);
            operand = 0;
            let op = which_operator(c);
            operator_stack.push(op);
            continue
        }
        println!("visiting {}", c);
        operand *= 10;
        operand += c.to_digit(10u32).unwrap() as i32;
    }
    operand_stack.push(operand);

    println!("{:?}", operator_stack.peek());
    println!("{:?}", operand_stack);
}
