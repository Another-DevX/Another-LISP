use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use std::io::{self, Write};

#[derive(Parser)]
#[grammar = "anotlisp.pest"]
struct AnotlispParser;

fn eval_op(x: u128, op: char, y: u128) -> u128 {
    match op {
        '+' => x + y,
        '-' => x - y,
        '*' => x * y,
        '/' => {
            if y == 0 {
                panic!("Division by zero");
            }
            x / y
        }
        _ => panic!("Unknown operator"),
    }
}

fn eval(pairs: &Pairs<Rule>) -> u128 {
    let mut pairs = pairs.clone();
    let first_pair = pairs.next().unwrap();

    if first_pair.as_rule() == Rule::number {
        return first_pair.as_str().parse().unwrap();
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
