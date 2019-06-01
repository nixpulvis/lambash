#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Term(Terminal),
    Abs(Box<Abstraction>),
    App(Box<Application>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Terminal {
    Bind(Binding),

    // TODO: More to come...
    None,
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abstraction(pub Binding, pub Box<Expression>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding(pub String);

