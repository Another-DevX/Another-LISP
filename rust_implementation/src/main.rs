use pest::Parser;
use pest_derive::Parser;
use std::io::{self, Write};

#[derive(Parser)]
#[grammar = "anotlisp.pest"]
struct AnotlispParser;

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
                println!("Parse succesful: {:?}", parsed)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }

        buffer.clear();
    }
}
