use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The value must not be blank.";

/// Requires a text to be neither empty nor made up only of whitespace.
#[derive(Debug, Clone, Default)]
pub struct NotBlankValidation {
    message: Option<String>,
}

impl NotBlankValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("VALUE_BLANK");
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

impl<T: AsRef<str> + ?Sized> BusinessLogicValidation<T> for NotBlankValidation {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if to_validate.as_ref().trim().is_empty() {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
