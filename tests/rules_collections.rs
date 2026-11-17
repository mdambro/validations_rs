use std::collections::{BTreeSet, HashMap};

use validations_rs::rules::{codes, Count, NotEmpty};
use validations_rs::Rule;

#[test]
fn not_empty_works_with_common_collections() {
    assert!(NotEmpty.check(&vec![1]).is_ok());
    assert!(NotEmpty.check(&[1, 2]).is_ok());
    assert!(NotEmpty.check(&[1][..]).is_ok());
    assert!(NotEmpty.check(&HashMap::from([("k", 1)])).is_ok());
    assert!(NotEmpty.check(&BTreeSet::from(["a"])).is_ok());
}

#[test]
fn not_empty_rejects_empty_collections() {
    assert_eq!(NotEmpty.check(&Vec::<u8>::new()).unwrap_err().code(), codes::COLLECTION_EMPTY);
    assert!(NotEmpty.check(&HashMap::<u8, u8>::new()).is_err());
    assert!(NotEmpty.check(&[0_u8; 0][..]).is_err());
}

#[test]
fn count_bounds_are_inclusive_and_reported() {
    let rule = Count::between(1, 2);
    assert!(rule.check(&vec!['a']).is_ok());
    assert!(rule.check(&vec!['a', 'b']).is_ok());
    let violation = rule.check(&vec!['a', 'b', 'c']).unwrap_err();
    assert_eq!(violation.code(), codes::SIZE_OUT_OF_RANGE);
    assert_eq!(violation.detail("size"), Some("3"));
    assert_eq!(violation.detail("min"), Some("1"));
    assert_eq!(violation.detail("max"), Some("2"));
    assert!(rule.check(&Vec::<char>::new()).is_err());
}

#[test]
fn count_at_least_and_at_most() {
    assert!(Count::at_least(2).check(&[1, 2, 3]).is_ok());
    assert!(Count::at_least(2).check(&[1]).is_err());
    assert!(Count::at_most(1).check(&BTreeSet::<u8>::new()).is_ok());
    assert!(Count::at_most(1).check(&[1, 2]).is_err());
}
