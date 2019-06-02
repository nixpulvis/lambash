#![feature(non_ascii_idents, box_syntax)]

mod ast;
mod syntax;

fn main() {
    let id = λ!{x.x};
    dbg!(&id);

    let zero = λ!{f.λ!{x.x}};
    dbg!(&zero);
    let one = λ!{f.λ!{x.γ!(f, x)}};
    dbg!(&one);

    let succ = λ!{n.λ!{f.λ!{x.γ!(f, γ!(n, γ!(f, x)))}}};
    dbg!(&succ);
    let add = λ!{m.λ!{n.λ!{f.λ!{x.γ!(m, γ!(f, γ!(n, γ!(f, x))))}}}};
    dbg!(&add);

    let two = app!(succ, one);
    dbg!(&two);
    let three = app!(succ, two);
    dbg!(&three);
    let four = app!(succ, three);
    dbg!(&four);

    // dbg!(app!(app!(add, one), one));

    // TODO: Primitives
    // let leet = γ!(id, 1337);
    // dbg!(&leet);
    // let inline = γ!(λ!{x.x}, 1337);
    // dbg!(&inline);
    // let inline = γ!(λ!{_x.1337}, ());
    // dbg!(&inline);
}
