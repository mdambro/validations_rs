use std::error::Error;
use std::fmt;

use crate::error::BusinessRuleError;

/// Why an asynchronous validation did not pass.
///
/// An asynchronous rule usually depends on I/O, so it can end in two very different ways: the data breaks
/// a business rule, or the rule could not be checked at all (the database is down, a request timed out).
/// Callers usually answer the first with a client error (HTTP 422) and the second with a server error
/// (HTTP 503 or 500), so this type keeps them apart.
///
/// A `BusinessRuleError` converts into this type with `?` or `.into()`:
///
/// ```
/// use validations_rs::{AsyncValidationError, BusinessRuleError, RuleCode};
///
/// let error: AsyncValidationError = BusinessRuleError::new(RuleCode("EMAIL_TAKEN"), "taken").into();
/// assert_eq!(error.violation().map(|v| v.code), Some(RuleCode("EMAIL_TAKEN")));
/// ```
#[derive(Debug)]
pub enum AsyncValidationError {
    /// The value breaks a business rule.
    Violation(BusinessRuleError),
    /// The rule could not be checked; the cause is kept as the error source.
    CheckFailed(Box<dyn Error + Send + Sync>),
}

impl AsyncValidationError {
    /// Wraps the reason a rule could not be checked.
    pub fn check_failed(cause: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        Self::CheckFailed(cause.into())
    }

    /// The business error, if a rule was violated.
    #[must_use]
    pub fn violation(&self) -> Option<&BusinessRuleError> {
        match self {
            Self::Violation(error) => Some(error),
            Self::CheckFailed(_) => None,
        }
    }

    /// Consumes the error and returns the business error, if a rule was violated.
    #[must_use]
    pub fn into_violation(self) -> Option<BusinessRuleError> {
        match self {
            Self::Violation(error) => Some(error),
            Self::CheckFailed(_) => None,
        }
    }
}

impl From<BusinessRuleError> for AsyncValidationError {
    fn from(error: BusinessRuleError) -> Self {
        Self::Violation(error)
    }
}

impl fmt::Display for AsyncValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Violation(error) => fmt::Display::fmt(error, f),
            Self::CheckFailed(_) => f.write_str("the validation could not be completed"),
        }
    }
}

impl Error for AsyncValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Violation(_) => None,
            Self::CheckFailed(cause) => Some(cause.as_ref()),
        }
    }
}
