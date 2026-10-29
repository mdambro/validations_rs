use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The value must be true.";

/// Requires a `bool` to be `true`.
#[derive(Debug, Clone, Default)]
pub struct AssertTrueValidation {
    message: Option<String>,
}

impl AssertTrueValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("VALUE_NOT_TRUE");

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

impl BusinessLogicValidation<bool> for AssertTrueValidation {
    fn validate(&self, to_validate: &bool) -> Result<(), BusinessRuleError> {
        if !*to_validate {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
