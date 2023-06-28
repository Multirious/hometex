use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::take_while1,
    combinator::{map, verify},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::many0,
    sequence::tuple,
    Err as ErrorCase, IResult,
};

use super::*;

#[cfg(test)]
mod test;

pub fn tokens(input: &str) -> IResult<&str, Tokens<'_>> {
    // NOTE: potentially slow because double loop
    let token = map(token, |token| Some(token));
    let white_space = map(white_space, |_| None);
    let (input, output) = many0(alt((token, white_space)))(input)?;
    Ok((
        input,
        Tokens {
            tokens: output.into_iter().flatten().collect(),
        },
    ))
}

fn white_space(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| matches!(c, ' ' | '\t' | '\r' | '\n'))(input)
}

fn token(input: &str) -> IResult<&str, Token<'_>> {
    let identifier = map(identifier, Token::Identifier);
    let literal = map(literal, Token::Literal);
    let operator = map(operator, Token::Operator);
    alt((identifier, literal, operator))(input)
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
            take_while1(char_alphanumeric_or_underscore),
            first_char_not_digit,
        ),
        |str: &str| Identifier { str },
    )(input)
}

fn literal(input: &str) -> IResult<&str, Literal<'_>> {
    let integer = map(digits, Literal::Digits);
    let float = map(float, Literal::Float);
    alt((float, integer))(input)
}

fn digits(input: &str) -> IResult<&str, Digits<'_>> {
    map(take_while1(|c: char| c.is_ascii_digit()), |str: &str| {
        Digits { str }
    })(input)
}

fn float(input: &str) -> IResult<&str, Float<'_>> {
    let pre_dot = digits;
    let post_dot = digits;
    let dot = verify(take(1usize), |s: &str| s == ".");
    map(
        tuple((pre_dot, dot, post_dot)),
        |(pre_dot, _, post_dot): (Digits, &str, Digits)| Float {
            left_to_dot: pre_dot,
            right_to_dot: post_dot,
        },
    )(input)
}
