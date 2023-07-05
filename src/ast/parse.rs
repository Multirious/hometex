use super::{Literal, *};
use crate::lexer;
use anyhow::{anyhow, bail, Context};
use nom_reinvented::*;

type TS<'a, 'src> = &'a [lexer::Token<'src>];

mod nom_reinvented;
#[cfg(test)]
mod test;

pub fn ast<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Ast<'src>> {
    todo!()
}

fn tag_tokens_kind(
    kind: &[lexer::TokenKind],
) -> impl for<'a, 'src> Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, TS<'a, 'src>> {
    let kind = kind.to_vec();
    move |input| {
        let (input, tokens) = take(kind.len())(input)?;
        for (expected_kind, token) in kind.iter().zip(tokens) {
            if token.kind != *expected_kind {
                return Err(anyhow!(Error::new(input))).with_context(|| "tag");
            }
        }
        Ok((input, tokens))
    }
}

fn expression<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Expression<'src>> {
    let literal = map(literal, Expression::Literal);
    let function_call = map(function_call, Expression::FunctionCall);
    if let Ok(o) = literal(input) {
        return Ok(o);
    }
    if let Ok(o) = function_call(input) {
        return Ok(o);
    }
    Err(anyhow!(Error::new(input)))
        .with_context(|| "expression expecting literal, or functiona call")
}

fn literal<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Literal> {
    let whole = map(whole, Literal::Whole);
    let float = map(float, Literal::Float);
    if let Ok(o) = whole(input) {
        return Ok(o);
    }
    if let Ok(o) = float(input) {
        return Ok(o);
    }

    Err(anyhow!(Error::new(input)))
        .with_context(|| "literal expecting whole number, or floating-point number")
}

fn whole<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Whole> {
    let digits_kind = lexer::TokenKind::Literal(lexer::Literal::Digits);
    let (input, digits) = tag_tokens_kind(&[digits_kind])(input)?;
    Ok((input, Whole(digits[0].fragment.parse().unwrap())))
}

fn float<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Float> {
    let float_kind = lexer::TokenKind::Literal(lexer::Literal::Float);
    let (input, float) = tag_tokens_kind(&[float_kind])(input)?;
    Ok((input, Float(float[0].fragment.parse().unwrap())))
}

fn identifier<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Identifier<'src>> {
    let identifier_kind = lexer::TokenKind::Identifier;
    let (input, identifier) = tag_tokens_kind(&[identifier_kind])(input)?;
    Ok((
        input,
        Identifier {
            text: identifier[0].fragment,
        },
    ))
}

fn function_call<'src, 'a>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, FunctionCall<'src>> {
    let (input, identifier) = identifier(input)?;
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
    let left_bracket = lexer::TokenKind::Operator(lexer::Operator::LeftRoundBracket);
    let right_bracket = lexer::TokenKind::Operator(lexer::Operator::RightRoundBracket);
    let comma = lexer::TokenKind::Operator(lexer::Operator::Comma);
    surround_seperated_items_allowed_trailing(
        expression,
        tag(&[comma]),
        tag(&[left_bracket]),
        tag(&[right_bracket]),
    )(input)
}

fn surround_seperated_items_allowed_trailing<I, IP, SP, EP, BP>(
    item_parser: IP,
    seperator: SP,
    begin: EP,
    end: BP,
) -> impl for<'a, 'src> Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, Vec<I>>
where
    IP: for<'a, 'src> Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, I>,
    SP: for<'a, 'src> Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, TS<'a, 'src>>,
    BP: for<'a, 'src> Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, TS<'a, 'src>>,
    EP: for<'a, 'src> Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, TS<'a, 'src>>,
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
                    return Err(anyhow!(Error::new(input)))
                        .with_context(|| "surround_seperated_items_allowed_trailing");
                }
                (true, true) => break,
            }
        }

        Ok((l_input, vec))
    }
}
