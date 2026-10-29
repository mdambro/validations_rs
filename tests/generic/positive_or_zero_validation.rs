use validations_rs::{BusinessLogicValidation, PositiveOrZeroValidation};

#[test]
fn accepts_zero_and_positive_values() {
    let rule = PositiveOrZeroValidation::new();
    assert!(rule.validate(&0).is_ok());
    assert!(rule.validate(&1).is_ok());
    assert!(rule.validate(&0.0).is_ok());
}

#[test]
fn rejects_negative_values() {
    let error = PositiveOrZeroValidation::new().validate(&-1).unwrap_err();
    assert_eq!(error.code, PositiveOrZeroValidation::CODE);
    assert_eq!(error.message, "The value must not be negative.");
    assert_eq!(error.detail("value"), Some("-1"));
}

#[test]
fn rejects_nan() {
    assert!(PositiveOrZeroValidation::new().validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let error =
        PositiveOrZeroValidation::new().with_message("The stock must not be negative.").validate(&-1).unwrap_err();
    assert_eq!(error.code, PositiveOrZeroValidation::CODE);
    assert_eq!(error.message, "The stock must not be negative.");
}
