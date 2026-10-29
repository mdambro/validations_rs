use std::time::{Duration, SystemTime};

use validations_rs::{BusinessLogicValidation, FutureValidation};

#[test]
fn accepts_instants_after_now() {
    let in_an_hour = SystemTime::now() + Duration::from_secs(3600);
    assert!(FutureValidation::new().validate(&in_an_hour).is_ok());
}

#[test]
fn rejects_instants_before_now() {
    let an_hour_ago = SystemTime::now() - Duration::from_secs(3600);
    let error = FutureValidation::new().validate(&an_hour_ago).unwrap_err();
    assert_eq!(error.code, FutureValidation::CODE);
    assert_eq!(error.message, "The date must be in the future.");
    assert!(FutureValidation::new().validate(&SystemTime::UNIX_EPOCH).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let an_hour_ago = SystemTime::now() - Duration::from_secs(3600);
    let error =
        FutureValidation::new().with_message("The booking must be in the future.").validate(&an_hour_ago).unwrap_err();
    assert_eq!(error.code, FutureValidation::CODE);
    assert_eq!(error.message, "The booking must be in the future.");
}
