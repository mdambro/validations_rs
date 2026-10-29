use validations_rs::{BusinessLogicValidation, PositiveValidation};

#[test]
fn accepts_values_above_zero() {
    let rule = PositiveValidation::new();
    assert!(rule.validate(&1).is_ok());
    assert!(rule.validate(&0.001).is_ok());
}

#[test]
fn rejects_zero_and_negative_values() {
    let rule = PositiveValidation::new();

    let error = rule.validate(&0).unwrap_err();
    assert_eq!(error.code, PositiveValidation::CODE);
    assert_eq!(error.message, "The value must be greater than zero.");
    assert_eq!(error.detail("value"), Some("0"));

    assert!(rule.validate(&-1).is_err());
}

#[test]
fn rejects_nan() {
    assert!(PositiveValidation::new().validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let error = PositiveValidation::new().with_message("The amount must be positive.").validate(&0).unwrap_err();
    assert_eq!(error.code, PositiveValidation::CODE);
    assert_eq!(error.message, "The amount must be positive.");
}
