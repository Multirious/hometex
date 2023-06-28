use std::{collections::HashMap, fmt, ops, slice, sync::OnceLock};

mod parse;

pub fn parse(str: &str) -> Result<Tokens, nom::Err<nom::error::Error<&str>>> {
    match parse::tokens(str) {
        Ok((_, tokens)) => Ok(tokens),
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokens<'src> {
    tokens: Vec<Token<'src>>,
}

impl<'src, I> ops::Index<I> for Tokens<'src>
where
    I: slice::SliceIndex<[Token<'src>]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.tokens[index]
    }
}

impl<'src> fmt::Display for Tokens<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for token in &self.tokens {
            writeln!(f, "  {},", token)?;
        }
        writeln!(f, "]")
    }
}

impl<'src> AsRef<[Token<'src>]> for Tokens<'src> {
    fn as_ref(&self) -> &[Token<'src>] {
        &self.tokens
    }
}

impl<'src> Tokens<'src> {
    pub fn as_slice(&self) -> &[Token<'src>] {
        &self.tokens
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'src> {
    Identifier(Identifier<'src>),
    Literal(Literal<'src>),
    Operator(Operator),
}

impl<'src> Token<'src> {
    pub fn is_identifer(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self, Token::Literal(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, Token::Operator(_))
    }
}

impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(x) => write!(f, "{}", x),
            Token::Literal(x) => write!(f, "{}", x),
            Token::Operator(x) => write!(f, "{}", x),
        }
    }
}

macro_rules! operators {
    ($($ident:ident => $value:literal,)+) => {
        /// Specifical character(s) that do things
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Operator {
            $($ident,)+
        }

        impl Operator {
            pub const OPERATOR_ARRAY: &'static [Operator] = &[
                $(Operator::$ident,)+
            ];
            pub const VALUE_ARRAY: &'static [&'static str] = &[
                $($value,)+
            ];

            pub fn value(&self) -> &'static str {
                match *self {
                    $(
                        Operator::$ident => $value,
                    )+
                }
            }
        }
    };
}

operators! {
    // DoubleAsterisk => "**",
    // RightThickArrow => "=>",
    // LeftThickArrow => "<=",

    // LeftCurlyBracket => "{",
    // RightCurlyBracket => "}",
    // LeftSquareBracket => "[",
    // RightSquareBracket => "]",
    LeftRoundBracket => "(",
    RightRoundBracket => ")",
    // LeftAngleBracket => "<",
    // RightAngleBracket => ">",

    // Equal => "=",
    // Plus => "+",
    // Minus => "-",
    // Asterisk => "*",
    // ForwardSlash => "/",
    // BackSlash => "\\",
    // Percentage => "%",
    // Caret => "^",
    // DollarSign => "$",
    // Hash => "#",
    // At => "@",
    // Bang => "!",
    // QuestionMark => "?",
    // Ampersand => "&",
    // Semicolon => ";",
    // Colon => ":",
    Comma => ",",
    // Dot => ".",
    // Tilde => "~",
    // Backtick => "`",
    // Pipe => "|",
    // SingleQuote => "'",
    // DoubleQuote => "\"",
}

impl Operator {
    pub fn map() -> &'static HashMap<&'static str, Operator> {
        static MAP: OnceLock<HashMap<&'static str, Operator>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from_iter(
                Operator::VALUE_ARRAY
                    .iter()
                    .zip(Operator::OPERATOR_ARRAY)
                    .map(|(v, o)| (*v, *o)),
            )
        })
    }

    pub fn recognize(str: &str) -> Option<Operator> {
        Operator::map().get(str).copied()
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.value())
    }
}

/// String of ascii lowercase, uppercase alphabet, underscore, or number. Cannot start with number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier<'src> {
    pub str: &'src str,
}

impl<'src> Identifier<'src> {
    pub fn str(&self) -> &'src str {
        self.str
    }
}

impl<'src> fmt::Display for Identifier<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'src> {
    Digits(Digits<'src>),
    Float(Float<'src>),
    String(String<'src>),
}

impl<'src> fmt::Display for Literal<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Digits(x) => write!(f, "{}", x),
            Literal::Float(x) => write!(f, "{}", x),
            Literal::String(x) => write!(f, "{}", x),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digits<'src> {
    str: &'src str,
}

impl<'src> Digits<'src> {
    pub fn to_u64(&self) -> u64 {
        self.str.parse().unwrap()
    }
}

impl<'src> fmt::Display for Digits<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Float<'src> {
    left_to_dot: Digits<'src>,
    right_to_dot: Digits<'src>,
}

impl<'src> Float<'src> {
    pub fn to_f64(&self) -> f64 {
        format!("{}.{}", self.left_to_dot, self.right_to_dot)
            .parse()
            .unwrap()
    }
}

impl<'src> fmt::Display for Float<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.left_to_dot.str, self.right_to_dot)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<'src> {
    str: &'src str,
}
impl<'src> fmt::Display for String<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let result = parse("std(hello(23,23.8), 2398)").unwrap();
        panic!("{result}");
    }
}
