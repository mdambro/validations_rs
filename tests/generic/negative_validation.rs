use validations_rs::{BusinessLogicValidation, NegativeValidation};

#[test]
fn accepts_values_below_zero() {
    let rule = NegativeValidation::new();
    assert!(rule.validate(&-1).is_ok());
    assert!(rule.validate(&-0.001).is_ok());
}

#[test]
fn rejects_zero_and_positive_values() {
    let rule = NegativeValidation::new();

    let error = rule.validate(&0).unwrap_err();
    assert_eq!(error.code, NegativeValidation::CODE);
    assert_eq!(error.message, "The value must be less than zero.");
    assert_eq!(error.detail("value"), Some("0"));

    assert!(rule.validate(&1).is_err());
}

#[test]
fn rejects_nan() {
    assert!(NegativeValidation::new().validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let error = NegativeValidation::new().with_message("Must be a debt.").validate(&0).unwrap_err();
    assert_eq!(error.code, NegativeValidation::CODE);
    assert_eq!(error.message, "Must be a debt.");
}
