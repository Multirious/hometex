mod parse;

/// A page is the whole thing
pub struct Page<'a> {
    equation_blocks: Vec<EquationBlock<'a>>,
}

/// An equation block is a group of equation.
/// May be use to explain deriving an equation or solving one.
/// Has align function like in LaTex.
pub struct EquationBlock<'a> {
    equations: Vec<Equation<'a>>,
}

pub struct Equation<'a> {
    expressions: Vec<Expression<'a>>,
}

/// Thing that returns a value
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
    Number(Number<'a>),
    // Operator(Operator<'a>),
    Bracket(Bracket<'a>),
}

pub struct Number<'a>(&'a str);
pub struct Bracket<'a>(Box<Expression<'a>>);

// /// it's basically a special kind of function that you can actually type with normal keyboard.
// pub enum Operator<'a> {
//     Plus {
//         left: Expression<'a>,
//         right: Expression<'a>,
//     },
//     Minus {
//         left: Expression<'a>,
//         right: Expression<'a>,
//     },
//     Multiply {
//         left: Expression<'a>,
//         right: Expression<'a>,
//     },
//     Divide {
//         dividend: Expression<'a>,
//         divisor: Expression<'a>,
//     },
//     Root {
//         index: Expression<'a>,
//         radicand: Expression<'a>,
//     },
//     Power {
//         base: Expression<'a>,
//         exponent: Expression<'a>,
//     },
// }

pub struct Function<'a> {
    name: Identifier<'a>,
    inputs: Vec<Expression<'a>>,
    use_symbol: bool,
}

type SubSuperScriptType<'a> = Option<Box<Identifier<'a>>>;
pub struct Identifier<'a> {
    prefix_super_script: SubSuperScriptType<'a>,
    prefix_sub_script: SubSuperScriptType<'a>,
    main: &'a str,
    suffix_super_script: SubSuperScriptType<'a>,
    suffix_sub_script: SubSuperScriptType<'a>,
}
