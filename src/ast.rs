mod parse;

pub struct Ast<'src> {
    expressions: Vec<Expression<'src>>,
}

/// Thing that gives a value
pub enum Expression<'src> {
    Value(Value),
    FunctionCall(FunctionCall<'src>),
}

pub enum Value {
    Whole(Whole),
    Float(Float),
}

pub struct Whole(u64);
pub struct Float(f64);

pub struct Variable<'src> {
    identifier: Identifier<'src>,
}

pub struct FunctionCall<'src> {
    identifier: Identifier<'src>,
    inputs: Vec<Expression<'src>>,
}

pub struct Identifier<'src> {
    text: &'src str,
}
