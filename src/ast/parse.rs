use nom::combinator::verify;

use super::*;
use crate::lexer::{Operator::*, Token, Literal};

type IResult<I, O> = std::result::Result<(I, O), ()>;
type TS<'a, 'src> = &'a [Token<'src>];

pub fn ast<'src>(input: &[Token<'src>]) -> Result<Ast<'src>, ()> {
    todo!()
}

fn between_leveled<
    'a, 'src,
    'l, 'lsrc,
    'r, 'rsrc
>(
    input: TS<'a, 'src>,
    the_left: TS<'l, 'lsrc>,
    the_right: TS<'r, 'rsrc>
) -> IResult<TS<'a, 'src>, TS<'a, 'src>> {
    let left_size = the_left.len();
    let (left, rest) = input.split_at(left_size);
    if left == the_left {
        return Err(())
    }
    
    let right_size = the_right.len();
    let mut depth = 0isize;
    let mut completed = false;
    let mut i = 0usize;
    loop {
        let the_left_range = i..(i+left_size);
        let is_the_left = match rest.get(the_left_range) {
            Some(ts) => ts == the_left,
            None => false,
        };
        if is_the_left {
            depth += 1;
            continue;
        }
        let the_right_range = i..(i+right_size);
        let is_the_right = match rest.get(the_right_range) {
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
        if i == rest.len() {
            break;
        }
    }
    if completed {
        let (indeed_in_between_and_leveled, rest) = rest.split_at(i);
        Ok((rest, indeed_in_between_and_leveled))
    } else {
        Err(())
    }
}

fn integer<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Integer> {
    let (integer, rest) = input.split_at(1);
    let &[Token::Literal(Literal::Digits(integer))] = integer else {
        return Err(());
    };
    integer.to_u32();
}

fn function<'src, 'a>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Function<'src>> {
    let (this, rest) = input.split_at(2);
    let &[
        Token::Identifier(ident),
        Token::Operator(LeftRoundBracket)
    ] = this else {
        return Err(())
    };
    let inputs = function_inputs(rest)?
    todo!()
}

fn function_inputs<'a, 'src>(input: TS<'a, 'src>) -> IResult<TS<'a, 'src>, Vec<Expression<'src>>> {
    let left_bracket = &[Token::Operator(LeftRoundBracket)];
    let right_bracket = &[Token::Operator(RightRoundBracket)];
    let (input, output) = between_leveled(input, left_bracket, right_bracket)?;
    todo!()
}
