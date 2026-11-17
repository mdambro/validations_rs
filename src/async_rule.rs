//! Rules that wait on I/O: [`AsyncRule`], [`AsyncError`], and [`validate_async`].

use std::error::Error;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

use crate::error::Violation;

/// The future an [`AsyncRule`] returns: boxed, so rules of different types fit in one list.
pub type RuleFuture<'a> = Pin<Box<dyn Future<Output = Result<(), AsyncError>> + Send + 'a>>;

/// An asynchronous business rule, for checks that wait on I/O (a database, another service).
///
/// Such a rule usually holds its dependencies (a repository, a client), so it is a type:
///
/// ```
/// use validations_rs::{AsyncError, AsyncRule, RuleCode, RuleFuture, Violation};
///
/// trait Users: Send + Sync {
///     fn email_exists(&self, email: &str) -> Result<bool, std::io::Error>;
/// }
///
/// struct EmailNotTaken<'r> {
///     users: &'r dyn Users,
/// }
///
/// impl AsyncRule<str> for EmailNotTaken<'_> {
///     fn check<'a>(&'a self, email: &'a str) -> RuleFuture<'a> {
///         Box::pin(async move {
///             // An I/O failure is not a broken rule: report it as `CheckFailed`.
///             let taken = self.users.email_exists(email).map_err(AsyncError::check_failed)?;
///             if taken {
///                 return Err(Violation::new(RuleCode("EMAIL_TAKEN"), "the email is already registered").into());
///             }
///             Ok(())
///         })
///     }
/// }
/// ```
pub trait AsyncRule<T: ?Sized + Sync>: Send + Sync {
    /// Checks the value.
    ///
    /// # Errors
    ///
    /// Returns [`AsyncError::Violation`] if the value breaks the rule, and [`AsyncError::CheckFailed`]
    /// if the rule could not be checked.
    fn check<'a>(&'a self, value: &'a T) -> RuleFuture<'a>;
}

/// Why an asynchronous rule did not pass.
///
/// The two cases call for different answers: a broken rule is the client's problem (HTTP 422, gRPC
/// `FAILED_PRECONDITION`); a rule that could not be checked is transient (HTTP 503, gRPC `UNAVAILABLE`).
#[derive(Debug)]
pub enum AsyncError {
    /// The value breaks a business rule.
    Violation(Violation),
    /// The rule could not be checked; the cause is kept as the error source.
    CheckFailed(Box<dyn Error + Send + Sync>),
}

impl AsyncError {
    /// Wraps the reason a rule could not be checked.
    pub fn check_failed(cause: impl Into<Box<dyn Error + Send + Sync>>) -> Self {
        Self::CheckFailed(cause.into())
    }

    /// The violation, if a rule was broken.
    #[must_use]
    pub fn violation(&self) -> Option<&Violation> {
        match self {
            Self::Violation(violation) => Some(violation),
            Self::CheckFailed(_) => None,
        }
    }

    /// Consumes the error and returns the violation, if a rule was broken.
    #[must_use]
    pub fn into_violation(self) -> Option<Violation> {
        match self {
            Self::Violation(violation) => Some(violation),
            Self::CheckFailed(_) => None,
        }
    }
}

impl From<Violation> for AsyncError {
    fn from(violation: Violation) -> Self {
        Self::Violation(violation)
    }
}

impl fmt::Display for AsyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Violation(violation) => fmt::Display::fmt(violation, f),
            Self::CheckFailed(_) => f.write_str("a business rule could not be checked"),
        }
    }
}

impl Error for AsyncError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Violation(_) => None,
            Self::CheckFailed(cause) => Some(cause.as_ref()),
        }
    }
}

/// Checks the asynchronous rules in list order and stops at the first failure.
///
/// Run synchronous rules first with [`validate`](crate::validate), so cheap rules fail before any I/O.
///
/// # Errors
///
/// Returns the error of the first rule that fails, unchanged.
pub async fn validate_async<T: ?Sized + Sync>(value: &T, rules: &[&dyn AsyncRule<T>]) -> Result<(), AsyncError> {
    for rule in rules {
        rule.check(value).await?;
    }
    Ok(())
}
