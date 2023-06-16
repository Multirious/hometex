use std::{cell::OnceCell, collections::HashMap, sync::OnceLock};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tokens<'a> {
    tokens: Vec<Token<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'a> {
    Identifier(Identifier<'a>),
    Literal(Literal<'a>),
    Operator(Operator),
    Keyword(Keyword),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'a> {
    Integer(Integer<'a>),
    Float(Float<'a>),
    // String(String<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Integer<'a> {
    str: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Float<'a> {
    pre_dot: Integer<'a>,
    post_dot: Integer<'a>,
}

// pub struct String<'a> {
//     str: &'a str,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Keyword {}

pub use parse::parse;

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::take,
        bytes::complete::{take_while, take_while1},
        character::is_alphabetic,
        combinator::{map, verify},
        error::{Error as NomError, ErrorKind as NomErrorKind},
        sequence::tuple,
        Err as ErrorCase, IResult,
    };

    use super::*;

    pub fn parse(str: &str) -> Result<Tokens<'_>, ()> {
        todo!()
    }

    fn operator(input: &str) -> IResult<&str, Operator> {
        // NOTE: potentially slow and invalid code with longer operator
        let (input, output) = take(1usize)(input)?;
        match Operator::recognize(output) {
            Some(operator) => Ok((input, operator)),
            None => Err(ErrorCase::Error(NomError::new(input, NomErrorKind::Verify))),
        }
    }

    fn identifier(input: &str) -> IResult<&str, Identifier<'_>> {
        let first_char_not_digit = |s: &str| {
            let first_char = s.as_bytes()[0] as char;
            !first_char.is_ascii_digit()
        };
        let char_alphanumeric_or_underscore = |c: char| c.is_ascii_alphanumeric() || c == '_';
        map(
            verify(
                take_while(char_alphanumeric_or_underscore),
                first_char_not_digit,
            ),
            |str: &str| Identifier { str },
        )(input)
    }

    fn literal(input: &str) -> IResult<&str, Literal<'_>> {
        let integer = map(integer, Literal::Integer);
        let float = map(float, Literal::Float);
        alt((float, integer))(input)
    }

    fn integer(input: &str) -> IResult<&str, Integer<'_>> {
        map(take_while1(|c: char| c.is_ascii_digit()), |str: &str| {
            Integer { str }
        })(input)
    }

    fn float(input: &str) -> IResult<&str, Float<'_>> {
        let pre_dot = integer;
        let post_dot = integer;
        let dot = verify(take(1usize), |s: &str| s.as_bytes()[0] == b'.');
        map(
            tuple((pre_dot, dot, post_dot)),
            |(pre_dot, _, post_dot): (Integer, &str, Integer)| Float { pre_dot, post_dot },
        )(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_identifier() {
            assert_eq!(
                identifier("hello guys"),
                Ok((" guys", Identifier { str: "hello" }))
            );
            assert_eq!(
                identifier("_he_ll23o_ guys"),
                Ok((" guys", Identifier { str: "_he_ll23o_" }))
            );
            assert_eq!(
                identifier("2mama_sd guys"),
                Err(ErrorCase::Error(NomError::new(
                    "2mama_sd guys",
                    NomErrorKind::Verify
                )))
            );
        }
    }
}
