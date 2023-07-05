use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::take_while1,
    combinator::{map, verify},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::many0,
    sequence::tuple,
    Err as ErrorCase,
};

use super::*;

#[cfg(test)]
mod test;

type IResult<'a, O> = nom::IResult<&'a str, O>;
type Input<'a> = &'a str;

pub fn tokens(input: Input) -> IResult<Tokens<'_>> {
    let token = map(token, |t| Some(t));
    let white_space = map(white_space, |_| None);
    let (input, output) = many0(alt((white_space, token)))(input)?;
    Ok((
        input,
        Tokens {
            tokens: output.into_iter().flatten().collect(),
        },
    ))
}

fn white_space(input: Input) -> IResult<&str> {
    take_while1(|c: char| matches!(c, ' ' | '\t' | '\r' | '\n'))(input)
}

fn token(input: Input) -> IResult<Token<'_>> {
    let invalid_token = map(take(1usize), |s: &str| Token::new(s, TokenKind::Invalid));
    alt((identifier, literal, operator, invalid_token))(input)
}

fn operator(input: Input) -> IResult<Token<'_>> {
    // NOTE: potentially slow and invalid code with longer operator
    let (input, output) = take(1usize)(input)?;
    match Operator::recognize(output) {
        Some(operator) => Ok((input, Token::new(output, TokenKind::Operator(operator)))),
        None => Err(ErrorCase::Error(NomError::new(input, NomErrorKind::Verify))),
    }
}

fn identifier(input: Input) -> IResult<Token<'_>> {
    let first_char_not_digit = |s: &str| {
        let first_char = s.as_bytes()[0] as char;
        !first_char.is_ascii_digit()
    };
    let char_alphanumeric_or_underscore = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let identifier = verify(
        take_while1(char_alphanumeric_or_underscore),
        first_char_not_digit,
    );
    map(identifier, |str: &str| {
        Token::new(str, TokenKind::Identifier)
    })(input)
}

fn literal(input: Input) -> IResult<Token<'_>> {
    alt((float, digits))(input)
}

fn digits(input: Input) -> IResult<Token<'_>> {
    map(take_while1(|c: char| c.is_ascii_digit()), |str: &str| {
        Token::new(str, TokenKind::Literal(Literal::Digits))
    })(input)
}

fn dots_and_digits(input: Input) -> IResult<&str> {
    take_while1(|c: char| c.is_ascii_digit() || c == '.')(input)
}

fn float(input: Input) -> IResult<Token<'_>> {
    let float = verify(dots_and_digits, |s: &str| {
        let mut dot_count = 0usize;
        for char in s.chars() {
            if char == '.' {
                dot_count += 1;
                if dot_count > 1 {
                    return false;
                }
            }
        }
        dot_count == 1
    });
    map(float, |float| {
        Token::new(float, TokenKind::Literal(Literal::Float))
    })(input)
}
