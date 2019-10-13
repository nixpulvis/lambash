#![feature(non_ascii_idents, box_syntax)]
extern crate lalrpop_lambda;

use std::io::{self, BufRead, Write};
use std::ffi::CString;
use lalrpop_lambda::parse::ExpressionParser;
use lalrpop_lambda::{Expression, Variable, Application, Abstraction, Strategy};
use oursh::repl::{
    self,
    Prompt,
};
use oursh::job::Job;

fn main() {
    let parser = ExpressionParser::new();
    let handler = move |line: &String| {
        if let Ok(expression) = parser.parse(&line) {
            println!("-p {}\n-> {}\n-η {}",
                     expression,
                     expression.normalize(&Strategy::Applicative(false)),
                     expression.normalize(&Strategy::Applicative(true)));

            // TODO: if let Some(n) = Option<u64>::from(expression.clone())
            // let n = u64::from(expression.clone());
            // if n > 0 {
            //     println!("=u64 {}", n);
            // }
            expression.normalize(&Strategy::Applicative(true)).run();
        } else {
            println!("err: parse failed");
        }

        Ok(())
    };

    // TODO: oursh repl needs error types.
    let prompt = Prompt::new();
    repl::start(prompt, io::stdin(), io::stdout(), handler);
}

trait Run {
    fn run(self);
}

impl Run for Expression {
    fn run(self) {
        match self {
            // Running a variable is like `ls` or `~/foo`.
            Expression::Var(Variable(v, None)) => {
                let program = CString::new(format!("{}", v)).unwrap();
                Job::new(vec![program]).run().unwrap();
            },
            Expression::Var(Variable(_, Some(_))) => {
                unimplemented!();
            },
            // Applying an expression is like `(ls -la)` or `date --iso-8601`.
            // TODO: `(echo -n date)` vs `(echo -n (date))`
            // NOTE: x y z = ((x y) z), 1 2 3 4 = (((1 2) 3) 4)
            e @ Expression::App(_) => {
                // TODO: match for subshells?
                Job::new(e.args()).run().unwrap();
            },
            Expression::Abs(Abstraction(_id, body)) => {
                // TODO: Job::new(body.args()).run_background(id).unwrap();
                Job::new(body.args()).run_background().unwrap();
            },
        }
    }
}

trait Args {
    fn args(self) -> Vec<CString>;
}

impl Args for Expression {
    fn args(self) -> Vec<CString> {
        match self {
            // Running a variable is like `ls` or `~/foo`.
            Expression::Var(Variable(v, None)) => {
                let v = CString::new(format!("{}", v)).unwrap();
                vec![v]
            },
            Expression::App(Application(e1, e2)) => {
                let left = (*e1).args();
                let right = (*e2).args();
                [&left[..], &right[..]].concat()
            },
            _ => vec![],
        }
    }
}

fn prompt(stdout: &mut io::Stdout) {
    write!(stdout, "¤- ").expect("failed to write");
    stdout.flush().expect("failed to flush");
}
