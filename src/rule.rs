//! The [`Rule`] trait, functions as rules, and [`ensure`].

use std::borrow::Cow;

use crate::error::{RuleCode, Violation};

/// A synchronous business rule over a value of type `T`.
///
/// Every function or closure with the signature `Fn(&T) -> Result<(), Violation>` is already a rule, so
/// a rule without configuration is just a function:
///
/// ```
/// use validations_rs::{ensure, validate, RuleCode, Violation};
///
/// struct Signup {
///     age: u8,
///     accepted_terms: bool,
/// }
///
/// fn adult(signup: &Signup) -> Result<(), Violation> {
///     ensure(signup.age >= 18, RuleCode("UNDERAGE"), "you must be of legal age")
/// }
///
/// fn terms_accepted(signup: &Signup) -> Result<(), Violation> {
///     ensure(signup.accepted_terms, RuleCode("TERMS_NOT_ACCEPTED"), "you must accept the terms")
/// }
///
/// let signup = Signup { age: 16, accepted_terms: true };
/// let violation = validate(&signup, &[&terms_accepted, &adult]).unwrap_err();
/// assert_eq!(violation.code(), RuleCode("UNDERAGE"));
/// ```
///
/// Implement the trait on a type when the rule carries configuration, as the catalog in
/// [`rules`](crate::rules) does.
pub trait Rule<T: ?Sized> {
    /// Checks the value.
    ///
    /// # Errors
    ///
    /// Returns the [`Violation`] if the value breaks the rule.
    fn check(&self, value: &T) -> Result<(), Violation>;
}

impl<T: ?Sized, F> Rule<T> for F
where
    F: Fn(&T) -> Result<(), Violation>,
{
    fn check(&self, value: &T) -> Result<(), Violation> {
        self(value)
    }
}

/// Returns `Ok(())` if `condition` holds, and a [`Violation`] with `code` and `message` otherwise.
///
/// The building block of rules written as functions:
///
/// ```
/// use validations_rs::{ensure, RuleCode};
///
/// assert!(ensure(2 + 2 == 4, RuleCode("MATH"), "math is broken").is_ok());
/// assert_eq!(ensure(false, RuleCode("NOPE"), "never").unwrap_err().code(), RuleCode("NOPE"));
/// ```
///
/// # Errors
///
/// Returns the violation when `condition` is `false`.
pub fn ensure(condition: bool, code: RuleCode, message: impl Into<Cow<'static, str>>) -> Result<(), Violation> {
    if condition {
        Ok(())
    } else {
        Err(Violation::new(code, message))
    }
}
