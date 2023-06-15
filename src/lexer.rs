use crate::hometex_std::*;

pub struct Tokens<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> Tokens<'a> {
    pub fn parse(str: &str) -> Result<Tokens<'a>, ()> {
        todo!()
    }
}

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
            const OPERATOR_ARRAY: &'static [Operator] = &[
                $(Operator::$ident,)+
            ];
            const VALUE_ARRAY: &'static [&'static str] = &[
                $($value,)+
            ];

            fn value(&self) -> &'static str {
                match *self {
                    $(
                        $ident => $value,
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

/// String of ascii lowercase, uppercase alphabet, underscore, or number. Cannot start with number
pub struct Identifier<'a> {
    str: &'a str,
}

pub enum Literal<'a> {
    Integer(Integer),
    Float(Float),
    Marker(std::marker::PhantomData<&'a str>), // String(String<'a>),
}

// pub struct String<'a> {
//     str: &'a str,
// }

pub enum Keyword {}
