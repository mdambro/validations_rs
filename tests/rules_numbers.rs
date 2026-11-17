use validations_rs::rules::{codes, InRange, Max, Min, Negative, NegativeOrZero, Positive, PositiveOrZero};
use validations_rs::Rule;

#[test]
fn min_is_inclusive_and_reports_both_values() {
    assert!(Min(18).check(&18).is_ok());
    let violation = Min(18).check(&17).unwrap_err();
    assert_eq!(violation.code(), codes::VALUE_BELOW_MIN);
    assert_eq!(violation.detail("value"), Some("17"));
    assert_eq!(violation.detail("min"), Some("18"));
}

#[test]
fn max_is_inclusive_and_reports_both_values() {
    assert!(Max(10).check(&10).is_ok());
    let violation = Max(10).check(&11).unwrap_err();
    assert_eq!(violation.code(), codes::VALUE_ABOVE_MAX);
    assert_eq!(violation.detail("max"), Some("10"));
}

#[test]
fn min_and_max_work_with_any_ordered_type() {
    assert!(Min('b').check(&'c').is_ok());
    assert!(Max("m").check(&"z").is_err());
    assert!(Min(String::from("b")).check(&String::from("a")).is_err());
}

#[test]
fn bounds_reject_nan() {
    assert!(Min(0.0).check(&f64::NAN).is_err());
    assert!(Max(0.0).check(&f64::NAN).is_err());
    assert!(InRange::new(0.0, 1.0).check(&f64::NAN).is_err());
}

#[test]
fn in_range_is_inclusive_on_both_ends() {
    let rule = InRange::new(1, 10);
    assert!(rule.check(&1).is_ok());
    assert!(rule.check(&10).is_ok());
    let violation = rule.check(&0).unwrap_err();
    assert_eq!(violation.code(), codes::VALUE_OUT_OF_RANGE);
    assert_eq!(violation.detail("min"), Some("1"));
    assert_eq!(violation.detail("max"), Some("10"));
    assert!(rule.check(&11).is_err());
}

#[test]
fn positive_rejects_zero_and_negatives() {
    assert!(Positive.check(&1_i32).is_ok());
    assert!(Positive.check(&0.1_f64).is_ok());
    assert_eq!(Positive.check(&0_u8).unwrap_err().code(), codes::VALUE_NOT_POSITIVE);
    assert!(Positive.check(&-1_i64).is_err());
}

#[test]
fn positive_or_zero_accepts_zero() {
    assert!(PositiveOrZero.check(&0_i32).is_ok());
    assert!(PositiveOrZero.check(&0.0_f32).is_ok());
    assert_eq!(PositiveOrZero.check(&-1_i8).unwrap_err().code(), codes::VALUE_NEGATIVE);
}

#[test]
fn negative_rejects_zero_and_positives() {
    assert!(Negative.check(&-1_i32).is_ok());
    assert_eq!(Negative.check(&0_i32).unwrap_err().code(), codes::VALUE_NOT_NEGATIVE);
    assert!(Negative.check(&1.0_f64).is_err());
}

#[test]
fn negative_or_zero_accepts_zero() {
    assert!(NegativeOrZero.check(&0_isize).is_ok());
    assert!(NegativeOrZero.check(&-0.5_f64).is_ok());
    let violation = NegativeOrZero.check(&2_u32).unwrap_err();
    assert_eq!(violation.code(), codes::VALUE_POSITIVE);
    assert_eq!(violation.detail("value"), Some("2"));
}

#[test]
fn sign_rules_reject_nan() {
    assert!(Positive.check(&f64::NAN).is_err());
    assert!(PositiveOrZero.check(&f32::NAN).is_err());
    assert!(Negative.check(&f64::NAN).is_err());
    assert!(NegativeOrZero.check(&f64::NAN).is_err());
}

#[test]
fn sign_rules_cover_every_primitive() {
    assert!(Positive.check(&1_u128).is_ok());
    assert!(Positive.check(&1_i128).is_ok());
    assert!(Positive.check(&1_usize).is_ok());
    assert!(Positive.check(&1_u16).is_ok());
    assert!(Positive.check(&1_i16).is_ok());
    assert!(Positive.check(&1_u64).is_ok());
}
