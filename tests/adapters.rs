use validations_rs::rules::{codes, Length, Min, NotBlank};
use validations_rs::{ensure, validate, validate_all, Rule, RuleCode, RuleExt, Violation};

struct Address {
    city: String,
}

struct Customer {
    name: String,
    age: u8,
    address: Address,
}

fn customer(name: &str, age: u8, city: &str) -> Customer {
    Customer { name: name.into(), age, address: Address { city: city.into() } }
}

#[test]
fn with_message_replaces_only_the_message() {
    let violation = NotBlank.with_message("Tell us your name.").check("").unwrap_err();
    assert_eq!(violation.code(), codes::VALUE_BLANK);
    assert_eq!(violation.message(), "Tell us your name.");
}

#[test]
fn with_code_replaces_only_the_code_and_keeps_details() {
    let violation = Min(18).with_code(RuleCode("UNDERAGE")).check(&16).unwrap_err();
    assert_eq!(violation.code().as_str(), "UNDERAGE");
    assert_eq!(violation.message(), "The value is less than the minimum allowed.");
    assert_eq!(violation.detail("min"), Some("18"));
}

#[test]
fn adapters_pass_through_success() {
    assert!(NotBlank.with_message("m").with_code(RuleCode("C")).check("ok").is_ok());
}

#[test]
fn on_checks_a_field_and_records_its_name() {
    let rule = NotBlank.on("name", |c: &Customer| &c.name);
    let violation = rule.check(&customer(" ", 30, "Rosario")).unwrap_err();
    assert_eq!(violation.field(), Some("name"));
    assert_eq!(violation.to_string(), "[VALUE_BLANK] name: The value must not be blank.");
}

#[test]
fn on_can_project_to_a_str() {
    let rule = Length::between(2, 10).on("name", |c: &Customer| c.name.as_str());
    assert!(rule.check(&customer("Ada", 30, "Rosario")).is_ok());
}

#[test]
fn nested_on_builds_a_field_path() {
    let city = NotBlank.on("city", |a: &Address| &a.city);
    let rule = city.on("address", |c: &Customer| &c.address);
    let violation = rule.check(&customer("Ada", 30, "")).unwrap_err();
    assert_eq!(violation.field(), Some("address.city"));
}

#[test]
fn a_function_rule_can_be_adapted() {
    fn even(value: &u8) -> Result<(), Violation> {
        ensure(value % 2 == 0, RuleCode("ODD"), "must be even")
    }

    let rule = even.with_message("age must be even").on("age", |c: &Customer| &c.age);
    let violation = validate(&customer("Ada", 31, "Rosario"), &[&rule]).unwrap_err();
    assert_eq!(violation.code().as_str(), "ODD");
    assert_eq!(violation.message(), "age must be even");
    assert_eq!(violation.field(), Some("age"));
}

#[test]
fn field_rules_mix_with_whole_value_rules() {
    fn adult(c: &Customer) -> Result<(), Violation> {
        ensure(c.age >= 18, RuleCode("UNDERAGE"), "must be an adult")
    }

    let violations = validate_all(
        &customer("", 16, ""),
        &[&adult, &NotBlank.on("name", |c: &Customer| &c.name), &NotBlank.on("city", |c: &Customer| &c.address.city)],
    )
    .unwrap_err();

    let fields: Vec<Option<&str>> = violations.iter().map(Violation::field).collect();
    assert_eq!(fields, [None, Some("name"), Some("city")]);
}

#[test]
fn adapted_rules_are_clone_and_debug() {
    let rule = NotBlank.on("name", |c: &Customer| &c.name);
    let copy = rule.clone();
    assert!(format!("{copy:?}").contains("name"));
    assert!(format!("{:?}", NotBlank.with_code(RuleCode("X"))).contains("X"));
}
