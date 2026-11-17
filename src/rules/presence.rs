use super::codes;
use crate::error::Violation;
use crate::rule::{ensure, Rule};

/// The optional value is present.
///
/// In your own types, a required value should not be an `Option` at all; use this rule at the edges,
/// where input arrives with optional fields (a JSON body, a form).
///
/// ```
/// use validations_rs::rules::IsSome;
/// use validations_rs::Rule;
///
/// assert!(IsSome.check(&Some(1)).is_ok());
/// assert!(IsSome.check(&None::<i32>).is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IsSome;

impl<T> Rule<Option<T>> for IsSome {
    fn check(&self, value: &Option<T>) -> Result<(), Violation> {
        ensure(value.is_some(), codes::VALUE_MISSING, "The value is required.")
    }
}

/// The optional value is absent.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IsNone;

impl<T> Rule<Option<T>> for IsNone {
    fn check(&self, value: &Option<T>) -> Result<(), Violation> {
        ensure(value.is_none(), codes::VALUE_PRESENT, "The value must be absent.")
    }
}

/// The flag is `true`, such as accepted terms.
///
/// ```
/// use validations_rs::rules::IsTrue;
/// use validations_rs::Rule;
///
/// assert!(IsTrue.check(&true).is_ok());
/// assert!(IsTrue.check(&false).is_err());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IsTrue;

impl Rule<bool> for IsTrue {
    fn check(&self, value: &bool) -> Result<(), Violation> {
        ensure(*value, codes::VALUE_NOT_TRUE, "The value must be true.")
    }
}

/// The flag is `false`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct IsFalse;

impl Rule<bool> for IsFalse {
    fn check(&self, value: &bool) -> Result<(), Violation> {
        ensure(!*value, codes::VALUE_NOT_FALSE, "The value must be false.")
    }
}
