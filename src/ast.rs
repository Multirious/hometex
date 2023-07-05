mod parse;

type TS<'a, 'src> = &'a [crate::lexer::Token<'src>];

struct EpicWrapper(TS<'static, 'static>);

impl EpicWrapper {
    fn epicify<'a, 'src>(ts: TS<'a, 'src>) -> EpicWrapper {
        EpicWrapper(unsafe { std::slice::from_raw_parts(ts.as_ptr(), ts.len()) })
    }

    fn lmao<'a, 'src>(&'a self) -> TS<'a, 'src> {
        self.0
    }
}

impl std::fmt::Debug for EpicWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.lmao())
    }
}

impl std::fmt::Display for EpicWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.lmao())
    }
}

// the secret recipe is crime
#[derive(Debug, thiserror::Error)]
#[error("Error {{ input: {input} }}")]
pub struct Error {
    input: EpicWrapper,
}

impl Error {
    pub fn new<'a, 'src>(input: TS<'a, 'src>) -> Error {
        Error {
            input: EpicWrapper::epicify(input),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Ast<'src> {
    expressions: Vec<Expression<'src>>,
}

/// Thing that gives a value
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expression<'src> {
    Literal(Literal),
    Variable(Variable<'src>),
    FunctionCall(FunctionCall<'src>),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    Whole(Whole),
    Float(Float),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Whole(u64);
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Float(f64);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Variable<'src> {
    identifier: Identifier<'src>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FunctionCall<'src> {
    identifier: Identifier<'src>,
    inputs: Vec<Expression<'src>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Identifier<'src> {
    text: &'src str,
}
