use std::future::Future;
use std::pin::Pin;

use super::AsyncValidationError;

/// Future returned by an asynchronous validation.
pub type ValidationFuture<'a> = Pin<Box<dyn Future<Output = Result<(), AsyncValidationError>> + Send + 'a>>;

/// An asynchronous business rule, useful when validation waits on I/O.
pub trait AsyncBusinessLogicValidation<T: Sync + ?Sized>: Send + Sync {
    /// Validates the object.
    ///
    /// # Errors
    ///
    /// Returns [`AsyncValidationError::Violation`] if the object breaks the rule, and
    /// [`AsyncValidationError::CheckFailed`] if the rule could not be checked (for example, the database
    /// it queries is unavailable).
    fn validate<'a>(&'a self, to_validate: &'a T) -> ValidationFuture<'a>;
}
