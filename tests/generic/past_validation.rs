use std::time::{Duration, SystemTime};

use validations_rs::{BusinessLogicValidation, PastValidation};

#[test]
fn accepts_instants_before_now() {
    let an_hour_ago = SystemTime::now() - Duration::from_secs(3600);
    assert!(PastValidation::new().validate(&an_hour_ago).is_ok());
    assert!(PastValidation::new().validate(&SystemTime::UNIX_EPOCH).is_ok());
}

#[test]
fn rejects_instants_after_now() {
    let in_an_hour = SystemTime::now() + Duration::from_secs(3600);
    let error = PastValidation::new().validate(&in_an_hour).unwrap_err();
    assert_eq!(error.code, PastValidation::CODE);
    assert_eq!(error.message, "The date must be in the past.");
}

#[test]
fn custom_message_keeps_the_code() {
    let in_an_hour = SystemTime::now() + Duration::from_secs(3600);
    let error = PastValidation::new()
        .with_message("The date of birth must already have occurred.")
        .validate(&in_an_hour)
        .unwrap_err();
    assert_eq!(error.code, PastValidation::CODE);
    assert_eq!(error.message, "The date of birth must already have occurred.");
}

fn at(secs: u64) -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(secs)
}

#[test]
fn uses_the_injected_clock() {
    let rule = PastValidation::new().with_clock(|| at(1_000));
    assert!(rule.validate(&at(999)).is_ok());
    assert_eq!(rule.validate(&at(1_001)).unwrap_err().code, PastValidation::CODE);
}

#[test]
fn rejects_the_current_instant() {
    let rule = PastValidation::new().with_clock(|| at(1_000));
    assert!(rule.validate(&at(1_000)).is_err());
}

#[test]
fn the_clock_can_capture_state() {
    let now = at(5_000);
    let rule = PastValidation::new().with_clock(move || now).with_message("custom");
    let error = rule.validate(&at(6_000)).unwrap_err();
    assert_eq!(error.message, "custom");
}
