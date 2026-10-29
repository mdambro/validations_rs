use validations_rs::{BusinessLogicValidation, NotBlankValidation};

#[test]
fn accepts_text_with_content() {
    let rule = NotBlankValidation::new();
    assert!(rule.validate("x").is_ok());
    assert!(rule.validate("  x  ").is_ok());
}

#[test]
fn rejects_empty_and_whitespace_only_text() {
    let rule = NotBlankValidation::new();
    for blank in ["", " ", "   ", "\t\n"] {
        let error = rule.validate(blank).unwrap_err();
        assert_eq!(error.code, NotBlankValidation::CODE);
        assert_eq!(error.message, "The value must not be blank.");
    }
}

#[test]
fn accepts_owned_strings() {
    let rule = NotBlankValidation::new();
    assert!(rule.validate(&String::from("ada")).is_ok());
    assert!(rule.validate(&String::from(" ")).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let rule = NotBlankValidation::new().with_message("Enter a name.");
    let error = rule.validate("  ").unwrap_err();
    assert_eq!(error.code, NotBlankValidation::CODE);
    assert_eq!(error.message, "Enter a name.");
}
