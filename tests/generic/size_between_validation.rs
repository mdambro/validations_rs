use std::collections::HashMap;

use validations_rs::{BusinessLogicValidation, SizeBetweenValidation};

#[test]
fn accepts_sizes_inside_the_bounds_inclusive() {
    let rule = SizeBetweenValidation::new(1, 3);
    assert!(rule.validate(&vec![1]).is_ok());
    assert!(rule.validate(&vec![1, 2]).is_ok());
    assert!(rule.validate(&vec![1, 2, 3]).is_ok());
}

#[test]
fn rejects_sizes_outside_the_bounds() {
    let rule = SizeBetweenValidation::new(1, 3);

    let empty = rule.validate(&Vec::<i32>::new()).unwrap_err();
    assert_eq!(empty.code, SizeBetweenValidation::CODE);
    assert_eq!(empty.message, "The number of elements must be within the allowed range.");

    assert!(rule.validate(&vec![1, 2, 3, 4]).is_err());
}

#[test]
fn reports_size_and_bounds_as_details() {
    let error = SizeBetweenValidation::new(1, 3).validate(&vec![1, 2, 3, 4]).unwrap_err();
    assert_eq!(error.detail("size"), Some("4"));
    assert_eq!(error.detail("min"), Some("1"));
    assert_eq!(error.detail("max"), Some("3"));
}

#[test]
fn works_with_arrays_slices_and_maps() {
    let rule = SizeBetweenValidation::new(2, 2);
    assert!(rule.validate(&[1, 2]).is_ok());
    assert!(rule.validate(&HashMap::from([(1, "a"), (2, "b")])).is_ok());
    assert!(rule.validate(&HashMap::from([(1, "a")])).is_err());

    let items = [1, 2, 3];
    assert!(BusinessLogicValidation::<[i32]>::validate(&rule, &items[..2]).is_ok());
    assert!(BusinessLogicValidation::<[i32]>::validate(&rule, &items[..]).is_err());
}

#[test]
fn custom_message_keeps_the_code() {
    let rule = SizeBetweenValidation::new(1, 3).with_message("Choose between 1 and 3 options.");
    let error = rule.validate(&Vec::<i32>::new()).unwrap_err();
    assert_eq!(error.code, SizeBetweenValidation::CODE);
    assert_eq!(error.message, "Choose between 1 and 3 options.");
}
