use std::collections::HashSet;

use validations_rs::RuleCode;

const CONST_CODE: RuleCode = RuleCode("CONST");

#[test]
fn exposes_the_code_as_text() {
    assert_eq!(RuleCode("VALUE_BLANK").as_str(), "VALUE_BLANK");
}

#[test]
fn displays_the_code() {
    assert_eq!(RuleCode("VALUE_BLANK").to_string(), "VALUE_BLANK");
}

#[test]
fn compares_by_value() {
    assert_eq!(RuleCode("A"), RuleCode("A"));
    assert_ne!(RuleCode("A"), RuleCode("B"));
    assert!(RuleCode("A") < RuleCode("B"));
}

#[test]
fn can_be_used_as_a_hash_key() {
    let codes: HashSet<RuleCode> = [RuleCode("A"), RuleCode("B"), RuleCode("A")].into_iter().collect();
    assert_eq!(codes.len(), 2);
}

#[test]
fn can_be_declared_as_a_constant() {
    const AS_TEXT: &str = CONST_CODE.as_str();
    assert_eq!(AS_TEXT, "CONST");
}
