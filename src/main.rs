#![feature(non_ascii_idents, box_syntax)]
extern crate lalrpop_lambda;

use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::ffi::CString;
use docopt::Docopt;
use nix::sys::wait::{WaitStatus};
use lalrpop_lambda::parse::ExpressionParser;
use lalrpop_lambda::{Expression, Variable, Application, Abstraction, Strategy};
use oursh::repl::{
    self,
    Prompt,
};
use oursh::job::Job;

const USAGE: &'static str = "
lambash - λ-calculus shell.

Usage:
    lambash [-vn]

Options:
    -h --help     Show this screen.
    -v --verbose  Show version.
    -n --numbers  Show number decoding (BROKEN)
";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());
    let parser = ExpressionParser::new();

    // Elementary job management.
    let jobs: Rc<RefCell<Vec<(String, Job)>>> = Rc::new(RefCell::new(vec![]));

    let handler = move |line: &String| {
        jobs.borrow_mut().retain(|job| {
            match job.1.status() {
                Ok(WaitStatus::StillAlive) => {
                    true
                },
                Ok(WaitStatus::Exited(pid, code)) => {
                    println!("[{}]+\tExit({})\t{}", job.0, code, pid);
                    false
                },
                Ok(WaitStatus::Signaled(pid, signal, _)) => {
                    println!("[{}]+\t{}\t{}", job.0, signal, pid);
                    false
                },
                Ok(_) => {
                    println!("unhandled");
                    true
                },
                Err(e) => {
                    println!("err: {:?}", e);
                    false
                }
            }
        });

        if line == "" {
            return Ok(());
        }

        if let Ok(expression) = parser.parse(&line) {
            if args.get_bool("--verbose") {
                println!("-p {}\n-> {}\n-η {}",
                         expression,
                         expression.normalize(&Strategy::Applicative(false)),
                         expression.normalize(&Strategy::Applicative(true)));
            }

            if args.get_bool("--numbers") {
                // TODO: if let Some(n) = Option<u64>::from(expression.clone())
                let n = u64::from(expression.clone());
                println!("=u64 {}", n);
            }

            // Run the reduced term.
            expression.normalize(&Strategy::Applicative(true)).run(jobs.clone());
        } else {
            println!("err: parse failed");
        }

        Ok(())
    };

    // TODO: oursh repl needs error types.
    repl::start(Prompt::new("¤- "), io::stdin(), io::stdout(), handler);
}

trait Run {
    fn run(self, jobs: Rc<RefCell<Vec<(String, Job)>>>);
}

impl Run for Expression {
    fn run(self, jobs: Rc<RefCell<Vec<(String, Job)>>>) {
        match self {
            // Running a variable is like `ls` or `~/foo`.
            Expression::Var(Variable(v, None)) => {
                let program = format!("{}", v);
                if program == "jobs" {
                    for (id, job) in jobs.borrow().iter() {
                        if let Some(pid) = job.pid() {
                            println!("[{}]\t{}", id, pid)
                        }
                    }
                } else {
                    let args = vec![CString::new(program).unwrap()];
                    let mut job = Job::new(args);
                    job.fork_and_wait().unwrap();
                }
            },
            Expression::Var(Variable(_, Some(_))) => {
                unimplemented!();
            },
            // Applying an expression is like `(ls -la)` or `date --iso-8601`.
            // TODO: `(echo -n date)` vs `(echo -n (date))`
            // NOTE: x y z = ((x y) z), 1 2 3 4 = (((1 2) 3) 4)
            e @ Expression::App(_) => {
                // TODO: match for subshells?
                let mut job = Job::new(e.args());
                job.fork_and_wait().unwrap();
            },
            Expression::Abs(Abstraction(id, body)) => {
                let mut job = Job::new(body.args());
                job.fork().unwrap();
                if let Some(pid) = job.pid() {
                    println!("[{}]\t{}", id, pid)
                }
                jobs.borrow_mut().push((format!("{}", id), job));
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
