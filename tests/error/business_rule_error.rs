use validations_rs::{BusinessRuleError, RuleCode};

fn assert_is_error<E: std::error::Error>(_: &E) {}

#[test]
fn starts_without_details() {
    let error = BusinessRuleError::new(RuleCode("X"), "m");
    assert_eq!(error.code, RuleCode("X"));
    assert_eq!(error.message, "m");
    assert!(error.details.is_empty());
}

#[test]
fn finds_details_by_key() {
    let error = BusinessRuleError::new(RuleCode("X"), "m").with_detail("k", "v");
    assert_eq!(error.detail("k"), Some("v"));
    assert_eq!(error.detail("z"), None);
}

#[test]
fn keeps_details_in_insertion_order() {
    let error = BusinessRuleError::new(RuleCode("X"), "m").with_detail("b", "2").with_detail("a", "1");
    assert_eq!(error.details, vec![("b".to_string(), "2".to_string()), ("a".to_string(), "1".to_string())]);
}

#[test]
fn returns_the_first_detail_when_a_key_repeats() {
    let error = BusinessRuleError::new(RuleCode("X"), "m").with_detail("k", "first").with_detail("k", "second");
    assert_eq!(error.detail("k"), Some("first"));
}

#[test]
fn displays_code_and_message() {
    let error = BusinessRuleError::new(RuleCode("X"), "m");
    assert_eq!(error.to_string(), "[X] m");
}

#[test]
fn is_a_std_error() {
    assert_is_error(&BusinessRuleError::new(RuleCode("X"), "m"));
}

#[test]
fn can_be_cloned_and_compared() {
    let error = BusinessRuleError::new(RuleCode("X"), "m").with_detail("k", "v");
    assert_eq!(error.clone(), error);
    assert_ne!(error, BusinessRuleError::new(RuleCode("X"), "m"));
}
