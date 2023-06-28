use super::super::TS;
use anyhow::{anyhow, Context};

use super::super::Error;

pub type IResult<I, O> = std::result::Result<(I, O), anyhow::Error>;

pub fn take<'a, 'src>(
    at: usize,
) -> impl for<'b, 'bsrc> Fn(TS<'b, 'bsrc>) -> IResult<TS<'b, 'bsrc>, TS<'b, 'bsrc>> {
    move |input| {
        if at > input.len() {
            Err(anyhow!(Error::new(input)))
        } else {
            let (splitted_segment, the_rest) = input.split_at(at);
            Ok((the_rest, splitted_segment))
        }
    }
}

pub fn try_map<'a, 'src, P, PO, M, U>(
    parser: P,
    mapper: M,
) -> impl Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, U>
where
    P: for<'b, 'bsrc> Fn(TS<'b, 'bsrc>) -> IResult<TS<'b, 'bsrc>, PO>,
    M: Fn(PO) -> Option<U>,
{
    move |input| {
        let (i, o) = parser(input)?;
        match mapper(o) {
            Some(o) => Ok((i, o)),
            None => Err(Error::new(input)),
        }
    }
}

pub fn map<'a, 'src, P, PO, M, U>(
    parser: P,
    mapper: M,
) -> impl for<'b, 'bsrc> Fn(TS<'b, 'bsrc>) -> IResult<TS<'b, 'bsrc>, U>
where
    P: for<'b, 'bsrc> Fn(TS<'b, 'bsrc>) -> IResult<TS<'b, 'bsrc>, PO>,
    M: Fn(PO) -> U,
{
    move |input| {
        let (i, o) = parser(input)?;
        Ok((i, mapper(o)))
    }
}

// pub fn between_leveled<T>(the_left: TS<'a, 'src>, the_right: TS<'a, 'src>) -> impl Fn(TS<'a, 'src>) -> IResult<TS<'a, 'src>, TS<'a, 'src>>
// where
//     T: Clone + std::cmp::PartialEq,
// {
//     let the_left = the_left.to_vec();
//     let the_right = the_right.to_vec();
//     move |input| {
//         let left_size = the_left.len();
//         let (input, left) = take(left_size)(input)?;
//         if left == the_left {
//             return Err(Error::new(input));
//         }

//         let right_size = the_right.len();
//         let mut depth = 0isize;
//         let mut completed = false;
//         let mut i = 0usize;
//         loop {
//             let the_left_range = i..(i + left_size);
//             let is_the_left = match input.get(the_left_range) {
//                 Some(ts) => ts == the_left,
//                 None => false,
//             };
//             if is_the_left {
//                 depth += 1;
//                 continue;
//             }
//             let the_right_range = i..(i + right_size);
//             let is_the_right = match input.get(the_right_range) {
//                 Some(ts) => ts == the_right,
//                 None => false,
//             };
//             if is_the_right && depth > 0 {
//                 depth -= 1;
//             }
//             if is_the_right && depth == 0 {
//                 completed = true;
//                 break;
//             }
//             i += 1;
//             if i == input.len() {
//                 break;
//             }
//         }
//         if completed {
//             let (indeed_in_between_and_leveled, rest) = input.split_at(i);
//             Ok((rest, indeed_in_between_and_leveled))
//         } else {
//             Err(Error::new(input))
//         }
//     }
// }

pub fn tag<'a, 'src>(
    this: TS<'a, 'src>,
) -> impl for<'b, 'bsrc> Fn(TS<'b, 'bsrc>) -> IResult<TS<'b, 'bsrc>, TS<'b, 'bsrc>> {
    let this = this.to_vec();
    move |input| {
        let slice = &input[..this.len()];
        if slice == this {
            Ok((input, slice))
        } else {
            Err(Error::new(input))
        }
    }
}
