use super::*;

#[test]
fn test_identifier() {
    assert_eq!(
        identifier("hello guys"),
        Ok((" guys", Identifier { str: "hello" }))
    );
    assert_eq!(
        identifier("_he_ll23o_ guys"),
        Ok((" guys", Identifier { str: "_he_ll23o_" }))
    );
    assert_eq!(
        identifier("2mama_sd guys"),
        Err(ErrorCase::Error(NomError::new(
            "2mama_sd guys",
            NomErrorKind::Verify
        )))
    );
}

#[test]
fn test_float() {
    assert_eq!(
        float("23.13 yaya"),
        Ok((
            " yaya",
            Float {
                left_to_dot: Digits { str: "23" },
                right_to_dot: Digits { str: "13" }
            }
        ))
    );
    assert_eq!(
        float(".13 no"),
        Err(ErrorCase::Error(NomError::new(
            ".13 no",
            NomErrorKind::TakeWhile1
        )))
    );
    assert_eq!(
        identifier("34. haha"),
        Err(ErrorCase::Error(NomError::new(
            "34. haha",
            NomErrorKind::Verify
        )))
    );
}
