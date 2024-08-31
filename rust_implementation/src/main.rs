use core::fmt;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::io::{self, Write};

#[derive(Parser)]
#[grammar = "anotlisp.pest"]
struct AnotlispParser;

#[derive(Debug, Clone, Copy, PartialEq)]
enum LvalType {
    LvalNum,
    LvalErr,
}

#[derive(Debug, Clone, Copy)]
enum LvalErr {
    DivZero,
    BadOp,
    BadNum,
}

#[derive(Debug, Clone, Copy)]
struct Lval {
    lval_type: LvalType,
    num: Option<i128>,
    err: Option<LvalErr>,
}

impl Lval {
    fn new_num(value: i128) -> Self {
        Lval {
            lval_type: LvalType::LvalNum,
            num: Some(value),
            err: None,
        }
    }

    fn new_err(err: LvalErr) -> Self {
        Lval {
            lval_type: LvalType::LvalErr,
            num: None,
            err: Some(err),
        }
    }
}

impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.lval_type {
            LvalType::LvalNum => write!(f, "{}", self.num.unwrap()),
            LvalType::LvalErr => match self.err.unwrap() {
                LvalErr::DivZero => write!(f, "Error: Division by zero"),
                LvalErr::BadOp => write!(f, "Error: Invalid operator"),
                LvalErr::BadNum => write!(f, "Error: Invalid number"),
            },
        }
    }
}

fn eval_op(x: Lval, op: char, y: Lval) -> Lval {
    if x.lval_type == LvalType::LvalErr {
        return x;
    }
    if y.lval_type == LvalType::LvalErr {
        return y;
    }
    match op {
        '+' => Lval::new_num(x.num.unwrap() + y.num.unwrap()),
        '-' => Lval::new_num(x.num.unwrap() - y.num.unwrap()),
        '*' => Lval::new_num(x.num.unwrap() * y.num.unwrap()),
        '/' => {
            if y.num.unwrap() == 0 {
                Lval::new_err(LvalErr::DivZero)
            } else {
                Lval::new_num(x.num.unwrap() / y.num.unwrap())
            }
        }
        '%' => {
            if y.num.unwrap() == 0 {
                Lval::new_err(LvalErr::DivZero)
            } else {
                Lval::new_num(x.num.unwrap() % y.num.unwrap())
            }
        }
        _ => Lval::new_err(LvalErr::BadOp),
    }
}

fn eval(pairs: &Pairs<Rule>) -> Lval {
    let mut pairs = pairs.clone();
    let first_pair = pairs.next().unwrap();

    if first_pair.as_rule() == Rule::number {
        let number = first_pair.as_str().parse::<i128>();
        match number {
            Ok(num) => return Lval::new_num(num),
            Err(_) => return Lval::new_err(LvalErr::BadNum),
        }
    }

    let op = &first_pair.as_str().chars().next().unwrap();
    let mut x = eval(&pairs.next().unwrap().into_inner());
    while let Some(next_pair) = pairs.next() {
        if next_pair.as_rule() == Rule::expression {
            x = eval_op(x, *op, eval(&next_pair.into_inner()));
        } else {
            break;
        }
    }
    return x;
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Anotlisp version 0.0.0.1");
    println!("Press Ctrl+C to Exit \n");

    loop {
        print!("Anotlisp> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        let parse_result = AnotlispParser::parse(Rule::anotlisp, &buffer);

        match parse_result {
            Ok(parsed) => {
                let result = eval(&parsed.clone().next().unwrap().into_inner());

                println!("Result: {}", result);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }

        buffer.clear();
    }
}
fn format_pair(pair: &Pair<Rule>, indent: usize) -> String {
    let mut formatted = String::new();
    formatted.push_str(&" ".repeat(indent));
    formatted.push_str(&format!("{:?}: \"{}\"\n", pair.as_rule(), pair.as_str()));

    for inner_pair in pair.clone().into_inner() {
        formatted.push_str(&format_pair(&inner_pair, indent + 2));
    }

    formatted
}
