use validations_rs::{AssertFalseValidation, BusinessLogicValidation};

#[test]
fn accepts_false() {
    assert!(AssertFalseValidation::new().validate(&false).is_ok());
}

#[test]
fn rejects_true() {
    let error = AssertFalseValidation::new().validate(&true).unwrap_err();
    assert_eq!(error.code, AssertFalseValidation::CODE);
    assert_eq!(error.message, "The value must be false.");
}

#[test]
fn custom_message_keeps_the_code() {
    let error =
        AssertFalseValidation::new().with_message("The account must not be blocked.").validate(&true).unwrap_err();
    assert_eq!(error.code, AssertFalseValidation::CODE);
    assert_eq!(error.message, "The account must not be blocked.");
}
