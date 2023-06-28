use crate::{
    ast::{Expression, Float, FunctionCall, Identifier as AstIdentifier, Literal, Variable, Whole},
    lexer::{Identifier as LexIdentifier, Token},
};

fn lex_it(str: &str) -> crate::lexer::Tokens<'_> {
    crate::lexer::parse(str).unwrap()
}

#[test]
fn expression() {
    assert_eq!(
        super::expression(lex_it("23.43 aha").as_slice()),
        Ok((
            [Token::Identifier(LexIdentifier { str: "aha" })].as_slice(),
            Expression::Literal(Literal::Float(Float(23.43)))
        ))
    )
}

#[test]
fn function_call() {
    assert_eq!(
        super::function_call(lex_it("hello(lmao, 2132, 32.3) nextthing").as_slice()),
        Ok((
            [Token::Identifier(LexIdentifier { str: "nextthing" })].as_slice(),
            FunctionCall {
                identifier: AstIdentifier { text: "hello" },
                inputs: vec![
                    Expression::Variable(Variable {
                        identifier: AstIdentifier { text: "lmao" }
                    }),
                    Expression::Literal(Literal::Whole(Whole(2132))),
                    Expression::Literal(Literal::Float(Float(32.3)))
                ]
            }
        ))
    )
}
