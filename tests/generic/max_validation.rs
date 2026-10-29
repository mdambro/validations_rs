use validations_rs::{BusinessLogicValidation, MaxValidation};

#[test]
fn accepts_values_at_or_below_the_maximum() {
    let rule = MaxValidation::new(120);
    assert!(rule.validate(&120).is_ok());
    assert!(rule.validate(&0).is_ok());
}

#[test]
fn rejects_values_above_the_maximum() {
    let error = MaxValidation::new(120).validate(&121).unwrap_err();
    assert_eq!(error.code, MaxValidation::<i32>::CODE);
    assert_eq!(error.message, "The value is greater than the maximum allowed.");
    assert_eq!(error.detail("value"), Some("121"));
    assert_eq!(error.detail("max"), Some("120"));
}

#[test]
fn works_with_floats_and_rejects_nan() {
    let rule = MaxValidation::new(0.5);
    assert!(rule.validate(&0.5).is_ok());
    assert!(rule.validate(&0.6).is_err());
    assert!(rule.validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let error = MaxValidation::new(120).with_message("The age is not credible.").validate(&121).unwrap_err();
    assert_eq!(error.code, MaxValidation::<i32>::CODE);
    assert_eq!(error.message, "The age is not credible.");
}
