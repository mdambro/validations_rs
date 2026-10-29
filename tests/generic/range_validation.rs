use validations_rs::{BusinessLogicValidation, RangeValidation};

#[test]
fn accepts_values_inside_the_bounds_inclusive() {
    let rule = RangeValidation::new(0, 10);
    assert!(rule.validate(&0).is_ok());
    assert!(rule.validate(&5).is_ok());
    assert!(rule.validate(&10).is_ok());
}

#[test]
fn rejects_values_outside_the_bounds() {
    let rule = RangeValidation::new(0, 10);

    let above = rule.validate(&11).unwrap_err();
    assert_eq!(above.code, RangeValidation::<i32>::CODE);
    assert_eq!(above.message, "The value is outside the allowed range.");

    assert!(rule.validate(&-1).is_err());
}

#[test]
fn reports_value_and_bounds_as_details() {
    let error = RangeValidation::new(0, 10).validate(&11).unwrap_err();
    assert_eq!(error.detail("value"), Some("11"));
    assert_eq!(error.detail("min"), Some("0"));
    assert_eq!(error.detail("max"), Some("10"));
}

#[test]
fn works_with_floats() {
    let rule = RangeValidation::new(0.5, 1.5);
    assert!(rule.validate(&1.0).is_ok());
    assert!(rule.validate(&1.6).is_err());
}

#[test]
fn rejects_nan() {
    let rule = RangeValidation::new(0.0, 1.0);
    assert!(rule.validate(&f64::NAN).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let rule = RangeValidation::new(0, 10).with_message("The age is not allowed.");
    let error = rule.validate(&11).unwrap_err();
    assert_eq!(error.code, RangeValidation::<i32>::CODE);
    assert_eq!(error.message, "The age is not allowed.");
}
