use validations_rs::{BusinessLogicValidation, NotNullValidation};

#[test]
fn accepts_some() {
    assert!(NotNullValidation::new().validate(&Some(1)).is_ok());
    assert!(NotNullValidation::new().validate(&Some("")).is_ok());
}

#[test]
fn rejects_none() {
    let error = NotNullValidation::new().validate(&None::<i32>).unwrap_err();
    assert_eq!(error.code, NotNullValidation::CODE);
    assert_eq!(error.message, "The value is required.");
}

#[test]
fn custom_message_keeps_the_code() {
    let error = NotNullValidation::new().with_message("The email is missing.").validate(&None::<String>).unwrap_err();
    assert_eq!(error.code, NotNullValidation::CODE);
    assert_eq!(error.message, "The email is missing.");
}
