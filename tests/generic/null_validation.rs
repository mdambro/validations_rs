use validations_rs::{BusinessLogicValidation, NullValidation};

#[test]
fn accepts_none() {
    assert!(NullValidation::new().validate(&None::<i32>).is_ok());
}

#[test]
fn rejects_some() {
    let error = NullValidation::new().validate(&Some(1)).unwrap_err();
    assert_eq!(error.code, NullValidation::CODE);
    assert_eq!(error.message, "The value must be absent.");
}

#[test]
fn custom_message_keeps_the_code() {
    let error = NullValidation::new().with_message("Do not send this field.").validate(&Some("x")).unwrap_err();
    assert_eq!(error.code, NullValidation::CODE);
    assert_eq!(error.message, "Do not send this field.");
}
