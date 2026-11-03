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

fn at(secs: u64) -> std::time::SystemTime {
    std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(secs)
}

#[test]
fn uses_the_injected_clock() {
    let rule = FutureValidation::new().with_clock(|| at(1_000));
    assert!(rule.validate(&at(1_001)).is_ok());
    assert_eq!(rule.validate(&at(999)).unwrap_err().code, FutureValidation::CODE);
}

#[test]
fn rejects_the_current_instant() {
    let rule = FutureValidation::new().with_clock(|| at(1_000));
    assert!(rule.validate(&at(1_000)).is_err());
}
