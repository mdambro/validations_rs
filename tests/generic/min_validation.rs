use validations_rs::{BusinessLogicValidation, MinValidation};

#[test]
fn accepts_values_at_or_above_the_minimum() {
    let rule = MinValidation::new(18);
    assert!(rule.validate(&18).is_ok());
    assert!(rule.validate(&100).is_ok());
}

#[test]
fn rejects_values_below_the_minimum() {
    let error = MinValidation::new(18).validate(&17).unwrap_err();
    assert_eq!(error.code, MinValidation::<i32>::CODE);
    assert_eq!(error.message, "The value is less than the minimum allowed.");
    assert_eq!(error.detail("value"), Some("17"));
    assert_eq!(error.detail("min"), Some("18"));
}

#[test]
fn works_with_floats_and_rejects_nan() {
    let rule = MinValidation::new(0.5);
    assert!(rule.validate(&0.5).is_ok());
    assert!(rule.validate(&0.4).is_err());
    assert!(rule.validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let error = MinValidation::new(18).with_message("You must be of legal age.").validate(&17).unwrap_err();
    assert_eq!(error.code, MinValidation::<i32>::CODE);
    assert_eq!(error.message, "You must be of legal age.");
}
