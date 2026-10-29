use std::future::Future;
use std::pin::Pin;

use crate::error::BusinessRuleError;

/// Future returned by an asynchronous validation.
pub type ValidationFuture<'a> = Pin<Box<dyn Future<Output = Result<(), BusinessRuleError>> + Send + 'a>>;

/// An asynchronous business rule, useful when validation waits on I/O.
pub trait AsyncBusinessLogicValidation<T: Sync + ?Sized>: Send + Sync {
    /// Validates the object; returns the business error if the rule is violated.
    fn validate<'a>(&'a self, to_validate: &'a T) -> ValidationFuture<'a>;
}
