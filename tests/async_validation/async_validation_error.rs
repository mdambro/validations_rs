use std::error::Error;
use std::fmt;

use validations_rs::{AsyncValidationError, BusinessRuleError, RuleCode};

#[derive(Debug)]
struct DatabaseDown;

impl fmt::Display for DatabaseDown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("database down")
    }
}

impl Error for DatabaseDown {}

#[test]
fn a_business_error_converts_into_a_violation() {
    let error: AsyncValidationError = BusinessRuleError::new(RuleCode("TAKEN"), "Taken.").into();
    assert_eq!(error.violation().map(|v| v.code), Some(RuleCode("TAKEN")));
    assert_eq!(error.to_string(), "[TAKEN] Taken.");
    assert!(error.source().is_none());
    assert_eq!(error.into_violation().map(|v| v.message), Some("Taken.".to_string()));
}

#[test]
fn a_check_failure_keeps_its_cause_as_source() {
    let error = AsyncValidationError::check_failed(DatabaseDown);
    assert!(error.violation().is_none());
    assert_eq!(error.to_string(), "the validation could not be completed");
    assert_eq!(error.source().map(ToString::to_string), Some("database down".to_string()));
    assert!(error.into_violation().is_none());
}

#[test]
fn a_check_failure_accepts_a_plain_message() {
    let error = AsyncValidationError::check_failed("timeout after 2s");
    assert_eq!(error.source().map(ToString::to_string), Some("timeout after 2s".to_string()));
}

#[test]
fn is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AsyncValidationError>();
}
