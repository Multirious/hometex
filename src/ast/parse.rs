use super::*;
use crate::lexer::{Literal, Operator::*, Token};
use nom_reinvented::*;

type TS<'a, 'src> = &'a [Token<'src>];

mod nom_reinvented {
    use crate::lexer::Token;

    use super::TS;

    pub type IResult<I, O> = std::result::Result<(I, O), ()>;

    pub fn take<T>(at: usize) -> impl Fn(&[T]) -> IResult<&[T], &[T]> {
        move |input| {
            if at > input.len() {
                Err(())
            } else {
                let (splitted_segment, the_rest) = input.split_at(at);
                Ok((the_rest, splitted_segment))
            }
        }
    }

    pub fn try_map<T, P, F, U>(parser: P, mapper: F) -> impl Fn(&[T]) -> IResult<&[T], U>
    where
        P: Fn(&[T]) -> IResult<&[T], &[T]>,
        F: Fn(&[T]) -> Option<U>,
    {
        move |input| {
            let (i, o) = parser(input)?;
            match mapper(o) {
                Some(o) => Ok((i, o)),
                None => Err(()),
            }
        }
    }

    pub fn map<T, P, PO, F, U>(parser: P, mapper: F) -> impl Fn(&[T]) -> IResult<&[T], U>
    where
        P: Fn(&[T]) -> IResult<&[T], PO>,
        F: Fn(PO) -> U,
    {
        move |input| {
            let (i, o) = parser(input)?;
            Ok((i, mapper(o)))
        }
    }

    pub fn between_leveled<T>(
        the_left: &[T],
        the_right: &[T],
    ) -> impl Fn(&[T]) -> IResult<&[T], &[T]>
    where
        T: Clone + std::cmp::PartialEq,
    {
        let the_left = the_left.to_vec();
        let the_right = the_right.to_vec();
        move |input| {
            let left_size = the_left.len();
            let (input, left) = take(left_size)(input)?;
            if left == the_left {
                return Err(());
            }

            let right_size = the_right.len();
            let mut depth = 0isize;
            let mut completed = false;
            let mut i = 0usize;
            loop {
                let the_left_range = i..(i + left_size);
                let is_the_left = match input.get(the_left_range) {
                    Some(ts) => ts == the_left,
                    None => false,
                };
                if is_the_left {
                    depth += 1;
                    continue;
                }
                let the_right_range = i..(i + right_size);
                let is_the_right = match input.get(the_right_range) {
                    Some(ts) => ts == the_right,
                    None => false,
                };
                if is_the_right && depth > 0 {
                    depth -= 1;
                }
                if is_the_right && depth == 0 {
                    completed = true;
                    break;
                }
                i += 1;
                if i == input.len() {
                    break;
                }
            }
            if completed {
                let (indeed_in_between_and_leveled, rest) = input.split_at(i);
                Ok((rest, indeed_in_between_and_leveled))
            } else {
                Err(())
            }
        }
    }
}

use nom_reinvented::take;

pub fn ast<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Ast<'src>> {
    todo!()
}

fn expression<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Expression<'src>> {
    let value = map(value, Expression::Value);
    let function_call = map(function_call, Expression::FunctionCall);
    value(input)
}

fn value<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Value> {
    let whole = map(whole, Value::Whole);
    whole(input)
}

fn whole<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Whole> {
    let (input, digits) = take(1)(input)?;
    let &[Token::Literal(Literal::Digits(digits))] = digits else {
        return Err(());
    };
    Ok((input, Whole(digits.to_u64())))
}

fn function_call<'src, 'a>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, FunctionCall<'src>> {
    let (input, ident) = take(1)(input)?;
    let &[
        Token::Identifier(ident),
    ] = ident else {
        return Err(())
    };
    let inputs = function_inputs(input)?;
    todo!()
}

fn function_inputs<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Vec<Expression<'src>>> {
    let left_bracket = &[Token::Operator(LeftRoundBracket)];
    let right_bracket = &[Token::Operator(RightRoundBracket)];
    let (input, function_inputs) = between_leveled(left_bracket, right_bracket)(input)?;
    todo!()
}
