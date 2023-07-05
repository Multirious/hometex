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

// impl<'src> fmt::Display for Tokens<'src> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         writeln!(f, "[")?;
//         for token in &self.tokens {
//             writeln!(f, "  {},", token)?;
//         }
//         writeln!(f, "]")
//     }
// }

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
pub struct Token<'src> {
    pub fragment: &'src str,
    pub kind: TokenKind,
}

impl<'src> Token<'src> {
    pub fn new(fragment: &'src str, kind: TokenKind) -> Token<'src> {
        Token { fragment, kind }
    }

    pub fn fragment_if_kind(&self, kind: TokenKind) -> Option<&'src str> {
        if self.kind == kind {
            Some(self.fragment)
        } else {
            None
        }
    }
}

// impl<'src> fmt::Display for Token<'src> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Token::Identifier(x) => write!(f, "{}", x),
//             Token::Literal(x) => write!(f, "{}", x),
//             Token::Operator(x) => write!(f, "{}", x),
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    /// String of ascii lowercase, uppercase alphabet, underscore, or number. Cannot start with number
    Identifier,
    Literal(Literal),
    Operator(Operator),
    WhiteSpace(WhiteSpace),
    Invalid,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Digits,
    Float,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WhiteSpace {
    NewLine,
    Space,
    Tab,
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
