use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

// https://en.wikipedia.org/wiki/Shunting_yard_algorithm
fn eval(expression: &str, precedence: &HashMap<&str, u8>) -> i64 {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\d+*()]").unwrap());

    fn pop(output: &mut Vec<i64>, stack: &mut Vec<&str>) {
        let op = stack.pop().unwrap();
        let a = output.pop().unwrap();
        let b = output.pop().unwrap();
        match op {
            "+" => output.push(a + b),
            "*" => output.push(a * b),
            _ => panic!("unsupported op: {op}"),
        }
    }

    let mut output = Vec::new();
    let mut stack = Vec::new();
    for token in RE.find_iter(expression).map(|m| m.as_str()) {
        match token {
            "+" | "*" => {
                while stack
                    .last()
                    .is_some_and(|&op| op != "(" && precedence[op] >= precedence[token])
                {
                    pop(&mut output, &mut stack);
                }
                stack.push(token);
            }
            "(" => {
                stack.push(token);
            }
            ")" => {
                while stack.last().is_some_and(|&op| op != "(") {
                    pop(&mut output, &mut stack);
                }
                assert_eq!(stack.pop(), Some("("));
            }
            _ => {
                output.push(token.parse().unwrap());
            }
        }
    }
    while !stack.is_empty() {
        pop(&mut output, &mut stack);
    }
    assert_eq!(output.len(), 1);
    output[0]
}

fn eval1(expression: &str) -> i64 {
    eval(expression, &[("+", 0), ("*", 0)].into())
}

fn eval2(expression: &str) -> i64 {
    eval(expression, &[("+", 1), ("*", 0)].into())
}

pub fn part1(input: &str) -> i64 {
    input.lines().map(eval1).sum()
}

pub fn part2(input: &str) -> i64 {
    input.lines().map(eval2).sum()
}

pub fn tests() {
    assert_eq!(eval1("1+2*3+4*5+6"), 71);
    assert_eq!(eval1("1+(2*3)+(4*(5+6))"), 51);
    assert_eq!(eval1("2*3+(4*5)"), 26);
    assert_eq!(eval1("5+(8*3+9+3*4*3)"), 437);
    assert_eq!(eval1("5*9*(7*3*3+9*3+(8+6*4))"), 12240);
    assert_eq!(eval1("((2+4*9)*(6+9*8+6)+6)+2+4*2"), 13632);
    assert_eq!(eval2("1+2*3+4*5+6"), 231);
    assert_eq!(eval2("1+(2*3)+(4*(5+6))"), 51);
    assert_eq!(eval2("2*3+(4*5)"), 46);
    assert_eq!(eval2("5+(8*3+9+3*4*3)"), 1445);
    assert_eq!(eval2("5*9*(7*3*3+9*3+(8+6*4))"), 669_060);
    assert_eq!(eval2("((2+4*9)*(6+9*8+6)+6)+2+4*2"), 23340);
}
