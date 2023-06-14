use nom::IResult;

fn parse(input: &str) -> IResult<&str, Hometex> {}

// fn equation(input: &str) -> IResult<&str, >

struct Hometex<'a> {
    equations: Vec<Equation<'a>>,
}

struct Equation<'a> {
    left: Expression<'a>,
    right: Expression<'a>,
}

enum Expression<'a> {
    Number(Number<'a>),
    Operator(Operator<'a>),
}

struct Bracket<'a>(Expression<'a>);

enum Number<'a> {
    Real(&'a str),
    Fractional {
        divisor: Box<Number<'a>>,
        dividend: Box<Number<'a>>,
    },
}
enum Operator<'a> {
    Plus {
        left: Number<'a>,
        right: Number<'a>,
    },
    Minus {
        left: Number<'a>,
        right: Number<'a>,
    },
    Multiply {
        left: Number<'a>,
        right: Number<'a>,
    },
    Divide {
        dividend: Number<'a>,
        divisor: Number<'a>,
    },
    Modulo {
        dividend: Number<'a>,
        divisor: Number<'a>,
    },
    Root {
        index: Number<'a>,
        radicand: Number<'a>,
    },
    Exponential {
        base: Number<'a>,
        exponent: Number<'a>,
    },
}

struct Function<'a> {
    name: Identifier<'a>,
    input: Expression<'a>,
}

struct Identifier<'a> {
    prefix_super_script: Option<Box<Identifier<'a>>>,
    prefix_sub_script: Option<Box<Identifier<'a>>>,
    main: &'a str,
    suffix_super_script: Option<Box<Identifier<'a>>>,
    suffix_sub_script: Option<Box<Identifier<'a>>>,
}

fn main() {
    println!("Hello, world!");
}
