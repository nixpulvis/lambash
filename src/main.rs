#![feature(non_ascii_idents, box_syntax)]
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
                println!("-p {}\n-> {}\n-η {}",
                         expression,
                         expression.normalize(false),
                         expression.normalize(true));

                // if let Some(n) = Option<u64>::from(expression.clone()) {
                let n = u64::from(expression.clone());
                if n > 0 {
                    println!("=u64 {}\n", n);
                } else {
                    println!();
                }
            } else {
                println!("err: parse failed\n");
            }
        } else {
            println!("err: reading line failed\n");
        }

        prompt(&mut stdout);
    }
}

fn prompt(stdout: &mut io::Stdout) {
    write!(stdout, "<- ").expect("failed to write");
    stdout.flush().expect("failed to flush");
}
