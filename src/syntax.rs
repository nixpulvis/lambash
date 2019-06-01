#[macro_export]
macro_rules! binding {
    ($b:ident) => {
        Binding(stringify!($b).into())
    }
}

#[macro_export]
macro_rules! bind {
    ($b:ident) => {
        Expression::Term(Terminal::Bind(binding!($b)))
    }
}

#[macro_export]
macro_rules! abs {
    {$arg:ident . $body:expr} => {{
        use $crate::ast::*;
        // TODO: close over env and everything.
        Expression::Abs(box Abstraction(binding!($arg),
                                        box $body.clone()))
    }};
}

#[macro_export]
macro_rules! app {
    ($func:expr, $arg:expr) => {{
        use $crate::ast::*;
        Expression::App(box Application(box $func.clone(),
                                        box $arg.clone()))
    }};
}

#[macro_export]
macro_rules! λ {
    {$arg:ident . $body:ident} => {
        abs!($arg . bind!($body))
    };
    {$arg:ident . $body:expr} => {{
        abs!($arg . $body)
    }};
}

#[macro_export]
macro_rules! γ {
    ($func:ident, $arg:ident) => {
        app!(bind!($func), bind!($arg))
    };
    ($func:ident, $arg:expr) => {
        app!(bind!($func), $arg)
    };
    ($func:expr, $arg:ident) => {
        app!($func, bind!($arg))
    };
    ($func:expr, $arg:expr) => {
        app!($func, $arg)
    };
}
