use validations_rs::rules::{codes, IsFalse, IsNone, IsSome, IsTrue};
use validations_rs::Rule;

#[test]
fn is_some_requires_a_value() {
    assert!(IsSome.check(&Some("x")).is_ok());
    assert_eq!(IsSome.check(&None::<String>).unwrap_err().code(), codes::VALUE_MISSING);
}

#[test]
fn is_none_requires_absence() {
    assert!(IsNone.check(&None::<u8>).is_ok());
    assert_eq!(IsNone.check(&Some(1)).unwrap_err().code(), codes::VALUE_PRESENT);
}

#[test]
fn is_true_and_is_false() {
    assert!(IsTrue.check(&true).is_ok());
    assert_eq!(IsTrue.check(&false).unwrap_err().code(), codes::VALUE_NOT_TRUE);
    assert!(IsFalse.check(&false).is_ok());
    assert_eq!(IsFalse.check(&true).unwrap_err().code(), codes::VALUE_NOT_FALSE);
}
