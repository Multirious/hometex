use super::lexer::Literal;

mod parse;

/// Thing that gives a value
pub enum Expression<'a> {
    Literal(Literal<'a>),
    Identifier(Identifier<'a>),
    Function(Function<'a>),
}

pub struct Function<'a> {
    identifier: Identifier<'a>,
    inputs: Vec<Expression<'a>>,
}

pub struct Identifier<'a> {
    text: &'a str,
}
