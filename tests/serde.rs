#![cfg(feature = "serde")]

use serde_json::json;
use validations_rs::rules::{Length, Min};
use validations_rs::{validate_all, Rule, RuleCode, RuleExt, Violation};

#[test]
fn rule_code_serializes_as_a_string() {
    assert_eq!(serde_json::to_value(RuleCode("UNDERAGE")).unwrap(), json!("UNDERAGE"));
}

#[test]
fn violation_serializes_with_field_and_details() {
    let violation = Min(18).on("age", |age: &u8| age).check(&16).unwrap_err();
    assert_eq!(
        serde_json::to_value(&violation).unwrap(),
        json!({
            "code": "VALUE_BELOW_MIN",
            "message": "The value is less than the minimum allowed.",
            "field": "age",
            "details": { "value": "16", "min": "18" }
        })
    );
}

#[test]
fn violation_without_field_serializes_null() {
    let value = serde_json::to_value(Violation::new(RuleCode("X"), "m")).unwrap();
    assert_eq!(value["field"], json!(null));
    assert_eq!(value["details"], json!({}));
}

#[test]
fn violations_serialize_as_a_list() {
    let violations = validate_all("a", &[&Length::at_least(2), &Length::at_least(3)]).unwrap_err();
    let value = serde_json::to_value(&violations).unwrap();
    assert_eq!(value.as_array().map(Vec::len), Some(2));
    assert_eq!(value[0]["code"], json!("VALUE_LENGTH_OUT_OF_RANGE"));
}
