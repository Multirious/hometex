use nom::{
    branch::alt,
    bytes::complete::take,
    bytes::complete::{take_while, take_while1},
    combinator::{map, verify},
    error::{Error as NomError, ErrorKind as NomErrorKind},
    multi::many0,
    sequence::tuple,
    Err as ErrorCase, IResult,
};

use super::*;

pub fn tokens(input: &str) -> IResult<&str, Tokens<'_>> {
    // NOTE: potentially slow because double loop
    let token = map(token, |token| Some(token));
    let space = map(space, |_| None);
    map(many0(alt((token, space))), |tokens| Tokens {
        tokens: tokens.into_iter().flatten().collect(),
    })(input)
}

fn space(input: &str) -> IResult<&str, &str> {
    verify(take(1usize), |s: &str| s == " ")(input)
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
    let dot = verify(take(1usize), |s: &str| s == ".");
    map(
        tuple((pre_dot, dot, post_dot)),
        |(pre_dot, _, post_dot): (Integer, &str, Integer)| Float {
            left_from_dot: pre_dot,
            right_from_dot: post_dot,
        },
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

    #[test]
    fn test_float() {
        assert_eq!(
            float("23.13 yaya"),
            Ok((
                " yaya",
                Float {
                    left_from_dot: Integer { str: "23" },
                    right_from_dot: Integer { str: "13" }
                }
            ))
        );
        assert_eq!(
            float(".13 no"),
            Err(ErrorCase::Error(NomError::new(
                ".13 no",
                NomErrorKind::TakeWhile1
            )))
        );
        assert_eq!(
            identifier("34. haha"),
            Err(ErrorCase::Error(NomError::new(
                "haha",
                NomErrorKind::Verify
            )))
        );
    }
}
