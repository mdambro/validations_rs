use std::collections::{BTreeMap, HashMap, HashSet};

use validations_rs::{BusinessLogicValidation, NotEmptyValidation};

#[test]
fn accepts_collections_with_elements() {
    let rule = NotEmptyValidation::new();
    assert!(rule.validate(&vec![1]).is_ok());
    assert!(rule.validate(&[1, 2]).is_ok());
    assert!(rule.validate(&HashSet::from(["a"])).is_ok());
    assert!(rule.validate(&BTreeMap::from([(1, "a")])).is_ok());
}

#[test]
fn rejects_empty_collections() {
    let rule = NotEmptyValidation::new();

    let error = rule.validate(&Vec::<i32>::new()).unwrap_err();
    assert_eq!(error.code, NotEmptyValidation::CODE);
    assert_eq!(error.message, "The collection must not be empty.");

    assert!(rule.validate(&[0u8; 0]).is_err());
    assert!(rule.validate(&HashMap::<i32, i32>::new()).is_err());
}

#[test]
fn works_with_slices() {
    let rule = NotEmptyValidation::new();
    let items = [1, 2, 3];
    assert!(BusinessLogicValidation::<[i32]>::validate(&rule, &items[..]).is_ok());
    assert!(BusinessLogicValidation::<[i32]>::validate(&rule, &items[..0]).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let rule = NotEmptyValidation::new().with_message("Add at least one item.");
    let error = rule.validate(&Vec::<i32>::new()).unwrap_err();
    assert_eq!(error.code, NotEmptyValidation::CODE);
    assert_eq!(error.message, "Add at least one item.");
}
