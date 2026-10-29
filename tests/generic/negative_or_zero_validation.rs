use validations_rs::{BusinessLogicValidation, NegativeOrZeroValidation};

#[test]
fn accepts_zero_and_negative_values() {
    let rule = NegativeOrZeroValidation::new();
    assert!(rule.validate(&0).is_ok());
    assert!(rule.validate(&-1).is_ok());
    assert!(rule.validate(&0.0).is_ok());
}

#[test]
fn rejects_positive_values() {
    let error = NegativeOrZeroValidation::new().validate(&1).unwrap_err();
    assert_eq!(error.code, NegativeOrZeroValidation::CODE);
    assert_eq!(error.message, "The value must not be positive.");
    assert_eq!(error.detail("value"), Some("1"));
}

#[test]
fn rejects_nan() {
    assert!(NegativeOrZeroValidation::new().validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let error =
        NegativeOrZeroValidation::new().with_message("The balance must not be positive.").validate(&1).unwrap_err();
    assert_eq!(error.code, NegativeOrZeroValidation::CODE);
    assert_eq!(error.message, "The balance must not be positive.");
}
