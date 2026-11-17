use std::collections::HashSet;

use validations_rs::{RuleCode, Violation, Violations};

const CODE: RuleCode = RuleCode("SOME_RULE");

#[test]
fn rule_code_exposes_its_text() {
    assert_eq!(CODE.as_str(), "SOME_RULE");
    assert_eq!(CODE.to_string(), "SOME_RULE");
    assert_eq!(AsRef::<str>::as_ref(&CODE), "SOME_RULE");
}

#[test]
fn rule_code_can_be_used_as_a_key() {
    let set: HashSet<RuleCode> = [CODE, CODE, RuleCode("OTHER")].into_iter().collect();
    assert_eq!(set.len(), 2);
    assert!(RuleCode("A") < RuleCode("B"));
}

#[test]
fn violation_keeps_code_message_and_details_in_order() {
    let violation = Violation::new(CODE, "message").with_detail("min", 1).with_detail("max", "10");
    assert_eq!(violation.code(), CODE);
    assert_eq!(violation.message(), "message");
    assert_eq!(violation.details().collect::<Vec<_>>(), [("min", "1"), ("max", "10")]);
    assert_eq!(violation.detail("max"), Some("10"));
    assert_eq!(violation.detail("missing"), None);
}

#[test]
fn violation_can_be_rewritten() {
    let violation = Violation::new(CODE, "old").with_message("new").with_code(RuleCode("NEW"));
    assert_eq!(violation.message(), "new");
    assert_eq!(violation.code().as_str(), "NEW");
}

#[test]
fn violation_display_includes_the_field_when_there_is_one() {
    assert_eq!(Violation::new(CODE, "broken").to_string(), "[SOME_RULE] broken");
    assert_eq!(Violation::new(CODE, "broken").at_field("name").to_string(), "[SOME_RULE] name: broken");
}

#[test]
fn at_field_builds_a_path_from_the_inside_out() {
    let violation = Violation::new(CODE, "m").at_field("city").at_field("address").at_field("customer");
    assert_eq!(violation.field(), Some("customer.address.city"));
}

#[test]
fn violation_is_comparable_and_an_error() {
    let violation = Violation::new(CODE, "m");
    assert_eq!(violation.clone(), violation);
    let error: Box<dyn std::error::Error + Send + Sync> = Box::new(violation);
    assert_eq!(error.to_string(), "[SOME_RULE] m");
}

#[test]
fn violations_cannot_be_empty() {
    assert!(Violations::from_vec(Vec::new()).is_none());
    let violations = Violations::from_vec(vec![Violation::new(CODE, "m")]).unwrap();
    assert_eq!(violations.len(), 1);
    assert!(!violations.is_empty());
}

#[test]
fn violations_iterate_in_order_and_display_joined() {
    let violations = Violations::from_vec(vec![
        Violation::new(RuleCode("A"), "first").at_field("x"),
        Violation::new(RuleCode("B"), "second"),
    ])
    .unwrap();

    assert_eq!(violations.first().code().as_str(), "A");
    assert_eq!(violations.as_slice().len(), 2);
    assert_eq!(violations.to_string(), "[A] x: first; [B] second");

    let by_ref: Vec<&str> = (&violations).into_iter().map(Violation::message).collect();
    assert_eq!(by_ref, ["first", "second"]);

    let owned: Vec<Violation> = violations.clone().into_iter().collect();
    assert_eq!(owned, violations.into_vec());
}

#[test]
fn a_single_violation_converts_into_violations() {
    let violations: Violations = Violation::new(CODE, "m").into();
    assert_eq!(violations.len(), 1);
}

#[test]
fn errors_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<Violation>();
    assert_send_sync::<Violations>();
    assert_send_sync::<validations_rs::AsyncError>();
}
