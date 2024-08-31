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
    LvalSym,
    LvalSexpr,
}

#[derive(Debug, Clone)]
struct Lval {
    lval_type: LvalType,
    num: Option<i128>,
    err: Option<String>,
    sym: Option<String>,
    cell: Vec<Lval>,
}

impl Default for Lval {
    fn default() -> Self {
        Lval {
            lval_type: LvalType::LvalNum,
            num: None,
            err: None,
            sym: None,
            cell: Vec::new(),
        }
    }
}

impl Lval {
    fn new_num(value: i128) -> Self {
        Lval {
            lval_type: LvalType::LvalNum,
            num: Some(value),
            ..Default::default()
        }
    }

    fn new_err(err: String) -> Self {
        Lval {
            lval_type: LvalType::LvalErr,
            err: Some(err),
            ..Default::default()
        }
    }

    fn new_sym(sym: String) -> Self {
        Lval {
            lval_type: LvalType::LvalSym,
            sym: Some(sym),
            ..Default::default()
        }
    }

    fn new_sexpr() -> Self {
        Lval {
            lval_type: LvalType::LvalSexpr,
            ..Default::default()
        }
    }

    fn add(&mut self, value: Lval) -> &mut Self {
        self.cell.push(value);
        self
    }

    fn pop(&mut self, index: usize) -> Lval {
        self.cell.remove(index)
    }

    fn builtin_op(&mut self, op: &String) -> Lval {
        for lval in &self.cell {
            if lval.lval_type != LvalType::LvalNum {
                return Lval::new_err(String::from("Cannot operate on non-number!"));
            }
        }

        let mut x = self.pop(0);

        if op == "-" && self.cell.is_empty() {
            match x.num {
                Some(ref mut num) => *num = -*num,
                None => return Lval::new_err(String::from("Cannot negate non-number!")),
            }
        }

        while !self.cell.is_empty() {
            let y = self.pop(0);
            if let (Some(x_num), Some(y_num)) = (x.num.as_mut(), y.num) {
                match op.as_str() {
                    "+" => *x_num += y_num,
                    "-" => *x_num -= y_num,
                    "*" => *x_num *= y_num,
                    "/" => {
                        if y_num == 0 {
                            return Lval::new_err("Division by zero!".to_string());
                        }
                        *x_num /= y_num;
                    }
                    "%" => {
                        if y_num == 0 {
                            return Lval::new_err("Division by zero!".to_string());
                        }
                        *x_num %= y_num;
                    }
                    _ => return Lval::new_err("Invalid operator!".to_string()),
                };
            } else {
                return Lval::new_err("Invalid number!".to_string());
            }
        }

        x
    }
    fn eval(&mut self) -> Lval {
        if self.lval_type == LvalType::LvalSexpr {
            return self.eval_sexpr();
        }
        self.clone()
    }
    fn eval_sexpr(&mut self) -> Lval {
        for lval in &mut self.cell {
            *lval = lval.eval();
        }

        for lval in &self.cell {
            if lval.lval_type == LvalType::LvalErr {
                return lval.clone();
            }
        }

        if self.cell.is_empty() {
            return self.clone();
        }
        if self.cell.len() == 1 {
            return self.pop(0);
        }

        let f = self.pop(0);

        if f.lval_type != LvalType::LvalSym {
            return Lval::new_err(String::from("S-Expression does not start with symbol!"));
        }

        self.builtin_op(f.sym.as_ref().unwrap())
    }

    fn read_num(pair: Pair<Rule>) -> Lval {
        let num = pair.as_str().parse::<i128>();
        match num {
            Ok(num) => Lval::new_num(num),
            Err(_) => Lval::new_err(String::from("Invalid number!")),
        }
    }
    fn read(pair: Pair<Rule>) -> Lval {
        let mut x;
        match pair.as_rule() {
            Rule::number => return Lval::read_num(pair),
            Rule::symbol => return Lval::new_sym(pair.as_str().to_string()),
            Rule::anotlisp => x = Lval::new_sexpr(),
            Rule::sexpression => x = Lval::new_sexpr(),

            _ => return Lval::new_err(String::from("Invalid rule!")),
        };

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::regex => continue,
                _ => x.add(Lval::read(inner_pair)),
            };
        }
        x
    }
}
impl fmt::Display for Lval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.lval_type {
            LvalType::LvalNum => write!(f, "{}", self.num.unwrap()),
            LvalType::LvalSym => write!(f, "{}", self.sym.as_ref().unwrap()),
            LvalType::LvalSexpr => {
                write!(f, "(")?;
                for (i, lval) in self.cell.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", lval)?;
                }
                write!(f, ")")
            }
            LvalType::LvalErr => write!(f, "Error: {}", self.err.as_ref().unwrap()),
        }
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    println!("Anotlisp version 0.0.0.1");
    println!("Press Ctrl+C to Exit \n");

    loop {
        print!("Anotlisp> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;

        let formated = &format!("{} {} {}", "regex", &buffer, "regex");
        let parse_result = AnotlispParser::parse(Rule::anotlisp, formated);

        match parse_result {
            Ok(parsed) => {
                let mut parsed = Lval::read(parsed.clone().next().unwrap());
                let result = parsed.eval();

                println!("Result: {}", result);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }

        buffer.clear();
    }
}

// fn eval_op(x: Lval, op: char, y: Lval) -> Lval {
//     if x.lval_type == LvalType::LvalErr {
//         return x;
//     }
//     if y.lval_type == LvalType::LvalErr {
//         return y;
//     }
//     match op {
//         '+' => Lval::new_num(x.num.unwrap() + y.num.unwrap()),
//         '-' => Lval::new_num(x.num.unwrap() - y.num.unwrap()),
//         '*' => Lval::new_num(x.num.unwrap() * y.num.unwrap()),
//         '/' => {
//             if y.num.unwrap() == 0 {
//                 Lval::new_err(LvalErr::DivZero)
//             } else {
//                 Lval::new_num(x.num.unwrap() / y.num.unwrap())
//             }
//         }
//         '%' => {
//             if y.num.unwrap() == 0 {
//                 Lval::new_err(LvalErr::DivZero)
//             } else {
//                 Lval::new_num(x.num.unwrap() % y.num.unwrap())
//             }
//         }
//         _ => Lval::new_err(LvalErr::BadOp),
//     }
// }

// fn eval(pairs: &Pairs<Rule>) -> Lval {
//     let mut pairs = pairs.clone();
//     let first_pair = pairs.next().unwrap();
//
//     if first_pair.as_rule() == Rule::number {
//         let number = first_pair.as_str().parse::<i128>();
//         match number {
//             Ok(num) => return Lval::new_num(num),
//             Err(_) => return Lval::new_err(LvalErr::BadNum),
//         }
//     }
//
//     let op = &first_pair.as_str().chars().next().unwrap();
//     let mut x = eval(&pairs.next().unwrap().into_inner());
//     while let Some(next_pair) = pairs.next() {
//         if next_pair.as_rule() == Rule::expression {
//             x = eval_op(x, *op, eval(&next_pair.into_inner()));
//         } else {
//             break;
//         }
//     }
//     return x;
// }
