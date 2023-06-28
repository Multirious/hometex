use super::{Literal as AstLiteral, *};
use crate::lexer::{Literal as LexLiteral, Operator::*, Token};
use nom_reinvented::*;

type TS<'a, 'src> = &'a [Token<'src>];

mod nom_reinvented;
#[cfg(test)]
mod test;

pub fn ast<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Ast<'src>> {
    todo!()
}

fn expression<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Expression<'src>> {
    let literal = map(literal, Expression::Literal);
    let function_call = map(function_call, Expression::FunctionCall);
    if let Ok(o) = literal(input) {
        return Ok(o);
    }
    match function_call(input) {
        Ok(o) => Ok(o),
        Err(e) => Err(e),
    }
}

fn literal<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Literal> {
    let whole = map(whole, AstLiteral::Whole);
    let float = map(float, AstLiteral::Float);
    if let Ok(o) = whole(input) {
        return Ok(o);
    }
    if let Ok(o) = float(input) {
        Ok(o)
    } else {
        Err(Error::with_msg(input, "whole or float in literal"))
    }
}

fn whole<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Whole> {
    let (input, digits) = take(1)(input)?;
    let &[Token::Literal(LexLiteral::Digits(digits))] = digits else {
        return Err(Error::with_msg(input, "whole"));
    };
    Ok((input, Whole(digits.to_u64())))
}

fn float<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Float> {
    let (input, float) = take(1)(input)?;
    let &[Token::Literal(LexLiteral::Float(float))] = float else {
        return Err(Error::with_msg(input, "float"));
    };
    Ok((input, Float(float.to_f64())))
}

fn function_call<'src, 'a>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, FunctionCall<'src>> {
    let (input, identifier) = take(1)(input)?;
    let &[
        Token::Identifier(ident),
    ] = identifier else {
        return Err(Error::with_msg(input, "function call"))
    };
    let identifier = Identifier { text: ident.str() };
    let (input, function_call_inputs) = function_call_inputs(input)?;
    Ok((
        input,
        FunctionCall {
            identifier,
            inputs: function_call_inputs,
        },
    ))
}

fn function_call_inputs<'a, 'src>(
    input: TS<'a, 'src>,
) -> IResult<TS<'a, 'src>, Vec<Expression<'src>>> {
    expressions_in_round_bracket_seperated_by_comma(input)
}

fn expressions_in_round_bracket_seperated_by_comma<'a, 'src>(
    input: TS<'a, 'src>,
) -> IResult<TS<'a, 'src>, Vec<Expression<'src>>> {
    let left_bracket = Token::Operator(LeftRoundBracket);
    let right_bracket = Token::Operator(RightRoundBracket);
    let comma = Token::Operator(Comma);
    surround_seperated_items_allowed_trailing(
        expression,
        tag(&[comma]),
        tag(&[left_bracket]),
        tag(&[right_bracket]),
    )(input)
}

fn surround_seperated_items_allowed_trailing<T, I, IP, SP, EP, BP>(
    item_parser: IP,
    seperator: SP,
    begin: EP,
    end: BP,
) -> impl Fn(&[T]) -> IResult<&[T], Vec<I>>
where
    IP: Fn(&[T]) -> IResult<&[T], I>,
    SP: Fn(&[T]) -> IResult<&[T], &[T]>,
    BP: Fn(&[T]) -> IResult<&[T], &[T]>,
    EP: Fn(&[T]) -> IResult<&[T], &[T]>,
{
    move |input| {
        let mut vec = vec![];
        let (input, _) = begin(input)?;
        let mut l_input = input;
        loop {
            let input = l_input;

            let (input, output) = item_parser(input)?;
            vec.push(output);

            let (input, is_seperator) = if let Ok((input, _)) = seperator(input) {
                (input, true)
            } else {
                (input, false)
            };
            let (input, is_end) = if let Ok((input, _)) = end(input) {
                (input, true)
            } else {
                (input, false)
            };

            l_input = input;

            match (is_seperator, is_end) {
                (true, false) => continue,
                (false, true) => break,
                (false, false) => {
                    return Err(Error::with_msg(
                        input,
                        "surround_seperated_items_allowed_trailing",
                    ))
                }
                (true, true) => break,
            }
        }

        Ok((l_input, vec))
    }
}
