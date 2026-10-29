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
