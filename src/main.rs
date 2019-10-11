#![feature(non_ascii_idents, box_syntax)]
extern crate lalrpop_lambda;

use std::io::{self, BufRead, Write};
use std::ffi::CString;
use lalrpop_lambda::parse::ExpressionParser;
use lalrpop_lambda::{Expression, Strategy};
use oursh::job::Job;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let parser = ExpressionParser::new();

    prompt(&mut stdout);
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(expression) = parser.parse(&line) {
                println!("-p {}\n-> {}\n-η {}",
                         expression,
                         expression.normalize(&Strategy::Applicative(false)),
                         expression.normalize(&Strategy::Applicative(true)));

                // if let Some(n) = Option<u64>::from(expression.clone()) {
                let n = u64::from(expression.clone());
                if n > 0 {
                    println!("=u64 {}\n", n);
                } else {
                    println!();
                }

                match expression.normalize(&Strategy::Applicative(false)) {
                    Expression::Var(v) => {
                        let program = CString::new(format!("{}", v)).unwrap();
                        Job::new(vec![program]).run().unwrap();
                    },
                    _ => {},
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
