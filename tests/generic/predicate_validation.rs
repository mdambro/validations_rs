use validations_rs::{BusinessLogicValidation, PredicateValidation};

#[test]
fn accepts_when_the_predicate_holds() {
    let rule: PredicateValidation<i32> = PredicateValidation::new(|n| n % 2 == 0);
    assert!(rule.validate(&4).is_ok());
}

#[test]
fn rejects_when_the_predicate_fails() {
    let rule: PredicateValidation<i32> = PredicateValidation::new(|n| n % 2 == 0);
    let error = rule.validate(&3).unwrap_err();
    assert_eq!(error.code, PredicateValidation::<i32>::CODE);
    assert_eq!(error.message, "The required condition is not met.");
}

#[test]
fn works_with_unsized_types() {
    let rule: PredicateValidation<str> = PredicateValidation::new(|s| s.starts_with('a'));
    assert!(rule.validate("ada").is_ok());
    assert!(rule.validate("bob").is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let rule = PredicateValidation::new(|n: &i32| *n > 0).with_message("Must be positive.");
    let error = rule.validate(&0).unwrap_err();
    assert_eq!(error.code, PredicateValidation::<i32>::CODE);
    assert_eq!(error.message, "Must be positive.");
}

#[test]
fn accepts_closures_that_capture_their_environment() {
    let limit = 10;
    let below_limit = PredicateValidation::new(move |n: &i32| *n < limit);
    assert!(below_limit.validate(&3).is_ok());
    assert_eq!(below_limit.validate(&12).unwrap_err().code, PredicateValidation::<i32>::CODE);
}

#[test]
fn captured_values_are_read_at_validation_time() {
    let blocked = ["admin".to_string(), "root".to_string()];
    let not_blocked = PredicateValidation::new(move |name: &str| !blocked.iter().any(|b| b == name));
    assert!(not_blocked.validate("ada").is_ok());
    assert!(not_blocked.validate("root").is_err());
}

#[test]
fn can_be_cloned_and_debug_printed() {
    let rule = PredicateValidation::new(|n: &i32| *n > 0).with_message("Must be positive.");
    let copy = rule.clone();
    assert!(copy.validate(&1).is_ok());
    assert!(format!("{rule:?}").contains("Must be positive."));
}
