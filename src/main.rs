#![feature(non_ascii_idents, box_syntax)]

#[macro_use]
extern crate lalrpop_lambda;

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let parser = lalrpop_lambda::parse::ExpressionParser::new();

    prompt(&mut stdout);
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(expression) = parser.parse(&line) {
                println!("{} -> {}",
                         expression,
                         expression.normalize(false));
            } else {
                println!("err: parse failed");
            }
        } else {
            println!("err: reading line failed");
        }

        prompt(&mut stdout);
    }
}

fn prompt(stdout: &mut io::Stdout) {
    write!(stdout, "> ").expect("failed to write");
    stdout.flush().expect("failed to flush");
}
