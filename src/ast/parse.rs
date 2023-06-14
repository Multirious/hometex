use nom::{bytes::complete::take_while1, IResult};
use thiserror::Error;

use super::*;

// fn equation(input: &str) -> IResult<&str, Equation> {

// }

#[derive(Debug, Error)]
#[error("{0} is not a number")]
struct NotANumberError(String);

fn number(input: &str) -> IResult<&str, Number> {
    let dot_count = 0usize;
    let (input, output) = take_while1(|c: char| c.is_numeric() || c == '.')(input)?;
    IResult::Ok((input, Number(output)))
}
