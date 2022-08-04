mod stack;
mod queue;

#[allow(dead_code)]
// use stack::Stack;
// use queue::Queue;
use std::env;

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
    RightParenthesis,
    LeftParenthesis,
    Exponent,
    Invalid,
}

#[derive(Debug, PartialEq)]
enum Token {
    Operator(Operator),
    Number(f64),
}

// fn is_operator(c: char) -> bool {
//     which_operator(c) != Operator::Invalid
// }

fn which_operator(c: char) -> Operator {
    match c {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        '/' => Operator::Divide,
        '*' => Operator::Multiply,
        '(' => Operator::LeftParenthesis,
        ')' => Operator::RightParenthesis,
        '^' => Operator::Exponent,
        _   => Operator::Invalid
    }
}

fn tokenize(expression: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut number : f64 = 0.0;
    let mut is_operator_mode = false;
    let mut is_next_negative = false;
    let mut is_editing_number = false;
    for c in expression.chars() {
        match c {
            ' ' => {
                if is_next_negative {
                    number *= -1.0;
                }
                tokens.push(Token::Number(number));
                is_operator_mode = true;
                number = 0.0;
                is_editing_number = false;
            },
            '0'..='9' => {
                is_editing_number = true;
                number *= 10.0;
                number += c.to_digit(10u32).unwrap() as f64;
            },
            '-' => {
                if is_operator_mode {
                    is_operator_mode = false;
                    is_next_negative = true;
                } else {
                    tokens.push(Token::Operator(Operator::Minus))
                }
            },
            '+'|'*'|'/'|'^'|'('|')' => {
                if is_next_negative {
                    number *= -1.0;
                }
                tokens.push(Token::Number(number));
                is_operator_mode = true;
                number = 0.0;
                is_editing_number = false;
                let op = which_operator(c);
                tokens.push(Token::Operator(op));
            },
            _ => {
                tokens.push(Token::Operator(Operator::Invalid));
            }
        }

    }
    if is_editing_number {
        if is_next_negative {
            number *= -1.0;
        }
        tokens.push(Token::Number(number));
    }

    tokens
}

fn infix_to_postfix() {

    // for each char
    // if number enqueue
    // if LeftParenthesis or Exponent push
    // if operator other than RightParenthesis push
    // if RightParenthesis
    // // while stack and not LeftParenthesis
    // // pop and enqueue
    // else while stack and precedence(current) >= precedence(peek)
    // // pop and enqueue
    // push current
    // end for
    // pop all and enqueue

}

fn main() {
    let args : Vec<String> = env::args().collect();

    let expr: String = args[1].clone();
    let tokens = tokenize(expr);
    println!("{:?}", tokens)
}
