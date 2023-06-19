mod parse;

pub struct Ast<'src> {
    expressions: Vec<Expression<'src>>,
}

/// Thing that gives a value
pub enum Expression<'src> {
    Value(Value),
    Function(Function<'src>),
}

pub enum Value {
    Integer(Integer),
    Float(Float),
}

pub struct Integer(u32);
pub struct Float(i64);

pub struct Variable<'src> {
    identifier: Identifier<'src>,
}

pub struct Function<'src> {
    identifier: Identifier<'src>,
    inputs: Vec<Expression<'src>>,
}

pub struct Identifier<'src> {
    text: &'src str,
}
