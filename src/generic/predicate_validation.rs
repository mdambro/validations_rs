use std::fmt;
use std::marker::PhantomData;

use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The required condition is not met.";

/// Validation defined by a predicate that returns `true` when the rule is satisfied.
///
/// The predicate can be a function or a closure, including closures that capture their environment:
///
/// ```
/// use validations_rs::{BusinessLogicValidation, PredicateValidation};
///
/// let limit = 10;
/// let below_limit = PredicateValidation::new(move |n: &i32| *n < limit);
/// assert!(below_limit.validate(&3).is_ok());
/// assert!(below_limit.validate(&12).is_err());
/// ```
///
/// The second type parameter defaults to a function pointer, so `PredicateValidation<i32>` still names the
/// type when you write it out (a non-capturing closure converts to it).
pub struct PredicateValidation<T: ?Sized, F = fn(&T) -> bool> {
    message: Option<String>,
    predicate: F,
    _validates: PhantomData<fn(&T)>,
}

impl<T: ?Sized, F> PredicateValidation<T, F> {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("PREDICATE_FAILED");

    /// Customizes the message without changing the rule's stable code.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<T: ?Sized, F: Fn(&T) -> bool> PredicateValidation<T, F> {
    /// Creates the rule with the default message.
    #[must_use]
    pub fn new(predicate: F) -> Self {
        Self { message: None, predicate, _validates: PhantomData }
    }
}

impl<T: ?Sized, F: Fn(&T) -> bool> BusinessLogicValidation<T> for PredicateValidation<T, F> {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if (self.predicate)(to_validate) {
            Ok(())
        } else {
            Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)))
        }
    }
}

impl<T: ?Sized, F: Clone> Clone for PredicateValidation<T, F> {
    fn clone(&self) -> Self {
        Self { message: self.message.clone(), predicate: self.predicate.clone(), _validates: PhantomData }
    }
}

impl<T: ?Sized, F> fmt::Debug for PredicateValidation<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PredicateValidation").field("message", &self.message).finish_non_exhaustive()
    }
}
