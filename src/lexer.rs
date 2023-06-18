use std::{cell::OnceCell, collections::HashMap, fmt, sync::OnceLock};

mod parse;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokens<'a> {
    tokens: Vec<Token<'a>>,
}

// impl<'a> fmt::Display for Tokens<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'a> {
    Identifier(Identifier<'a>),
    Literal(Literal<'a>),
    Operator(Operator),
}

impl<'a> fmt::Display for Token<'a> {
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
        Operator::map().get(str).map(|v| *v)
    }
}

/// String of ascii lowercase, uppercase alphabet, underscore, or number. Cannot start with number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier<'a> {
    str: &'a str,
}

impl<'a> fmt::Display for Identifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
    String(String<'a>),
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(x) => write!(f, "{}", x),
            Literal::Float(x) => write!(f, "{}", x),
            Literal::String(x) => write!(f, "{}", x),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integer<'a> {
    str: &'a str,
}

impl<'a> fmt::Display for Integer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Float<'a> {
    left_from_dot: Integer<'a>,
    right_from_dot: Integer<'a>,
}

impl<'a> fmt::Display for Float<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.left_from_dot.str, self.right_from_dot)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct String<'a> {
    str: &'a str,
}
impl<'a> fmt::Display for String<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.str)
    }
}

pub fn parse(str: &str) -> Result<Tokens, nom::Err<nom::error::Error<&str>>> {
    match parse::tokens(str) {
        Ok((_, tokens)) => Ok(tokens),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let result = parse("");
    }
}
