use std::cell::Cell;

use validations_rs::rules::{Length, NotBlank};
use validations_rs::{ensure, validate, validate_all, Rule, RuleCode, Violation};

fn fails(code: &'static str) -> impl Fn(&str) -> Result<(), Violation> {
    move |_: &str| Err(Violation::new(RuleCode(code), "failed"))
}

#[test]
fn validate_is_ok_with_no_rules() {
    assert!(validate("anything", &[]).is_ok());
    assert!(validate_all("anything", &[]).is_ok());
}

#[test]
fn validate_is_ok_when_every_rule_passes() {
    assert!(validate("Ada", &[&NotBlank, &Length::between(1, 5)]).is_ok());
}

#[test]
fn validate_returns_the_first_violation_in_list_order() {
    let (first, second) = (fails("FIRST"), fails("SECOND"));
    let violation = validate("x", &[&first, &second]).unwrap_err();
    assert_eq!(violation.code().as_str(), "FIRST");
}

#[test]
fn validate_does_not_run_rules_after_the_first_failure() {
    let calls = Cell::new(0);
    let counted = |_: &str| {
        calls.set(calls.get() + 1);
        Ok(())
    };
    let first = fails("FIRST");
    let _ = validate("x", &[&first, &counted]);
    assert_eq!(calls.get(), 0);
}

#[test]
fn validate_all_collects_every_violation_in_list_order() {
    let (first, second) = (fails("FIRST"), fails("SECOND"));
    let violations = validate_all("x", &[&first, &NotBlank, &second]).unwrap_err();
    let codes: Vec<&str> = violations.iter().map(|v| v.code().as_str()).collect();
    assert_eq!(codes, ["FIRST", "SECOND"]);
    assert_eq!(violations.first().code().as_str(), "FIRST");
}

#[test]
fn validate_all_runs_every_rule() {
    let calls = Cell::new(0);
    let counted = |_: &str| {
        calls.set(calls.get() + 1);
        ensure(false, RuleCode("COUNTED"), "counted")
    };
    let _ = validate_all("x", &[&counted, &counted, &counted]);
    assert_eq!(calls.get(), 3);
}

#[test]
fn rules_can_be_kept_in_a_vec() {
    let rules: Vec<Box<dyn Rule<str>>> = vec![Box::new(NotBlank), Box::new(Length::at_least(3))];
    let refs: Vec<&dyn Rule<str>> = rules.iter().map(AsRef::as_ref).collect();
    assert!(validate_all("ab", &refs).is_err());
    assert!(validate_all("abc", &refs).is_ok());
}

#[test]
fn validate_works_with_owned_and_unsized_values() {
    let name = String::from("Ada");
    assert!(validate(&name, &[&NotBlank]).is_ok());
    assert!(validate(name.as_str(), &[&NotBlank]).is_ok());
}
