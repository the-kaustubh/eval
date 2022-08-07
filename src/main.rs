mod stack;
mod queue;

use stack::Stack;
use queue::Queue;
use std::env;

#[derive(Debug, PartialEq, Clone, PartialOrd)]
enum Operator {
    Invalid,
    Minus,
    Plus,
    Multiply,
    Divide,
    Exponent,
    LeftParenthesis,
    RightParenthesis,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Operator(Operator),
    Number(f64),
}

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
                is_operator_mode = false;
                number = 0.0;
                is_editing_number = false;
                is_next_negative = false;
            },
            '0'..='9' => {
                is_operator_mode = false;
                is_editing_number = true;
                number *= 10.0;
                number += c.to_digit(10u32).unwrap() as f64;
            },
            '-' => {
                if is_operator_mode {
                    is_next_negative = true;
                } else {
                    if is_editing_number {
                        if is_next_negative {
                            is_next_negative = false;
                            number *= -1.0;
                        }
                        tokens.push(Token::Number(number));
                        number = 0.0;
                        is_editing_number = false;

                    }
                    is_operator_mode = true;
                    tokens.push(Token::Operator(Operator::Minus))
                }
            },
            '+'|'*'|'/'|'^'|'('|')' => {
                if is_editing_number {
                    if is_next_negative {
                        number *= -1.0;
                    }
                    tokens.push(Token::Number(number));
                    number = 0.0;
                }
                is_operator_mode = true;
                if c == ')' {
                    is_operator_mode = false;
                }
                is_next_negative = false;
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

fn infix_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack: Stack<Operator> = Stack::new();
    // let mut stack: Stack<Token> = Stack::new();
    let mut queue: Queue<Token> = Queue::new();

    // for each token
    for tok in tokens {
        match tok {
            Token::Number(n) => {
                queue.enqueue(Token::Number(n));
            },
            Token::Operator(o) => {
                if o == Operator::LeftParenthesis || o == Operator::Exponent { //  }&& o != Operator::RightParenthesis {
                    stack.push(o.clone())
                }

                else if o == Operator::RightParenthesis {
                    while !stack.is_empty() {
                        let popped = stack.pop();
                        if popped == Operator::LeftParenthesis {
                            break;
                        }
                        queue.enqueue(Token::Operator(popped))
                    }
                } else {
                    while !stack.is_empty() && o <= *stack.peek() {
                        let popped = stack.pop();
                        if popped == Operator::LeftParenthesis {
                            break;
                        }
                        queue.enqueue(Token::Operator(popped));
                    }
                    stack.push(o);
                }
            },
        }
    }

    // pop all and enqueue
    while !stack.is_empty() {
        queue.enqueue(Token::Operator(stack.pop()));
    }

    queue.arr
}

fn eval_binary(num1: f64, num2: f64, op: Operator) -> f64 {
    match op {
        Operator::Exponent => {
            num1.powf(num2)
        },
        Operator::Divide => {
            num1 / num2
        },
        Operator::Multiply => {
            num1 * num2
        },
        Operator::Plus => {
            num1 + num2
        },
        Operator::Minus => {
            num1 - num2
        },
        _ => {
            f64::INFINITY
        }
    }
}

fn eval_postfix(epxr: Vec<Token>) -> f64 {
    let mut val_stack: Stack<Token> = Stack::new();

    for token in epxr {
        match token {
            Token::Number(x) => {
                val_stack.push(Token::Number(x));
            },
            Token::Operator(o) => {
                let mut num2: f64 = 0.0;
                let mut num1: f64 = 0.0;
                if let Token::Number(n2) = val_stack.pop() {
                    num2 = n2;
                }
                if let Token::Number(n1) = val_stack.pop() {
                    num1 = n1;
                }
                let res = eval_binary(num1, num2, o);
                val_stack.push(Token::Number(res));
            }
        }
    }


    if let Token::Number(ret_val) = val_stack.pop() {
        ret_val
    } else {
        0.0
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please pass expression");
        std::process::exit(1);
    }
    let expr: String = args[1].clone();
    let tokens = tokenize(expr);
    let pf = infix_to_postfix(tokens);
    let res = eval_postfix(pf);
    println!("{:?}", res);
    std::process::exit(0);
}
