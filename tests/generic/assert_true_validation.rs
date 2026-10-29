use validations_rs::{AssertTrueValidation, BusinessLogicValidation};

#[test]
fn accepts_true() {
    assert!(AssertTrueValidation::new().validate(&true).is_ok());
}

#[test]
fn rejects_false() {
    let error = AssertTrueValidation::new().validate(&false).unwrap_err();
    assert_eq!(error.code, AssertTrueValidation::CODE);
    assert_eq!(error.message, "The value must be true.");
}

#[test]
fn custom_message_keeps_the_code() {
    let error = AssertTrueValidation::new().with_message("You must accept the terms.").validate(&false).unwrap_err();
    assert_eq!(error.code, AssertTrueValidation::CODE);
    assert_eq!(error.message, "You must accept the terms.");
}
