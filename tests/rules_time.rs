use std::time::{Duration, SystemTime};

use validations_rs::rules::{codes, Future, Past};
use validations_rs::Rule;

fn fixed_now() -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000)
}

#[test]
fn past_accepts_earlier_dates_and_rejects_now_and_later() {
    let rule = Past::new().with_clock(fixed_now);
    assert!(rule.check(&(fixed_now() - Duration::from_secs(1))).is_ok());
    assert_eq!(rule.check(&fixed_now()).unwrap_err().code(), codes::DATE_NOT_PAST);
    assert!(rule.check(&(fixed_now() + Duration::from_secs(1))).is_err());
}

#[test]
fn future_accepts_later_dates_and_rejects_now_and_earlier() {
    let rule = Future::new().with_clock(fixed_now);
    assert!(rule.check(&(fixed_now() + Duration::from_secs(1))).is_ok());
    assert_eq!(rule.check(&fixed_now()).unwrap_err().code(), codes::DATE_NOT_FUTURE);
    assert!(rule.check(&(fixed_now() - Duration::from_secs(1))).is_err());
}

#[test]
fn the_system_clock_is_the_default() {
    let an_hour = Duration::from_secs(3_600);
    assert!(Past::new().check(&(SystemTime::now() - an_hour)).is_ok());
    assert!(Future::new().check(&(SystemTime::now() + an_hour)).is_ok());
}

#[test]
fn date_rules_are_debug_and_clone() {
    let rule = Past::new().with_clock(fixed_now);
    assert!(format!("{:?}", rule.clone()).contains("custom"));
    assert!(format!("{:?}", Future::default()).contains("system"));
}
