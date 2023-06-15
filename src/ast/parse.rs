use nom::{
    bytes::complete::{take_while, take_while1},
    character::is_digit,
    IResult,
};
use thiserror::Error;

use super::*;

// fn equation(input: &str) -> IResult<&str, Equation> {

// }

#[derive(Debug, Error)]
#[error("{0} is not a number")]
struct NotANumberError(String);

fn digits(input: &str) -> IResult<&str, &str> {
    take_while(|c| is_digit(c as u8))(input)
}

fn number(input: &str) -> IResult<&str, Number> {
    // let (input, output) =
    todo!()
}
