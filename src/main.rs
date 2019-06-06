#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let parser = lalrpop_lambda::parse::ExpressionParser::new();

    write!(stdout, "> ");
    stdout.flush();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(expression) = parser.parse(&line) {
                println!("{}", expression.normalize(false));
            }
        }

        write!(stdout, "> ");
        stdout.flush();
    }
}
