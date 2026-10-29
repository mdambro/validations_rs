use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The required condition is not met.";

/// Validation defined by a predicate that returns `true` when the rule is satisfied.
pub struct PredicateValidation<T: ?Sized> {
    message: Option<String>,
    predicate: fn(&T) -> bool,
}

impl<T: ?Sized> PredicateValidation<T> {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("PREDICATE_FAILED");

    /// Creates the rule with the default message.
    pub fn new(predicate: fn(&T) -> bool) -> Self {
        Self { message: None, predicate }
    }

    /// Customizes the message without changing the rule's stable code.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<T: ?Sized> BusinessLogicValidation<T> for PredicateValidation<T> {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if (self.predicate)(to_validate) {
            Ok(())
        } else {
            Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)))
        }
    }
}
