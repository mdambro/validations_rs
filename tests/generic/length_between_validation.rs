use validations_rs::{BusinessLogicValidation, LengthBetweenValidation};

#[test]
fn accepts_lengths_inside_the_bounds_inclusive() {
    let rule = LengthBetweenValidation::new(2, 3);
    assert!(rule.validate("ab").is_ok());
    assert!(rule.validate("abc").is_ok());
}

#[test]
fn rejects_lengths_outside_the_bounds() {
    let rule = LengthBetweenValidation::new(2, 3);

    let too_short = rule.validate("a").unwrap_err();
    assert_eq!(too_short.code, LengthBetweenValidation::CODE);
    assert_eq!(too_short.message, "The length must be within the allowed range.");

    assert_eq!(rule.validate("abcd").unwrap_err().code, LengthBetweenValidation::CODE);
    assert!(rule.validate("").is_err());
}

#[test]
fn reports_length_and_bounds_as_details() {
    let error = LengthBetweenValidation::new(2, 3).validate("a").unwrap_err();
    assert_eq!(error.detail("length"), Some("1"));
    assert_eq!(error.detail("min"), Some("2"));
    assert_eq!(error.detail("max"), Some("3"));
}

#[test]
fn counts_characters_not_bytes() {
    let rule = LengthBetweenValidation::new(2, 3);
    assert!(rule.validate("ñé").is_ok());
    assert!(rule.validate("日本語").is_ok());
    assert!(rule.validate("日本語です").is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let rule = LengthBetweenValidation::new(2, 3).with_message("Use between 2 and 3 characters.");
    let error = rule.validate("a").unwrap_err();
    assert_eq!(error.code, LengthBetweenValidation::CODE);
    assert_eq!(error.message, "Use between 2 and 3 characters.");
}
