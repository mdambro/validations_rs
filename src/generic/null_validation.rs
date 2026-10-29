use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The value must be absent.";

/// Requires an `Option` to be empty.
#[derive(Debug, Clone, Default)]
pub struct NullValidation {
    message: Option<String>,
}

impl NullValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("VALUE_PRESENT");

    /// Creates the rule with the default message.
    pub fn new() -> Self {
        Self { message: None }
    }

    /// Customizes the message without changing the rule's stable code.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<T> BusinessLogicValidation<Option<T>> for NullValidation {
    fn validate(&self, to_validate: &Option<T>) -> Result<(), BusinessRuleError> {
        if to_validate.is_some() {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
