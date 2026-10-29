use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The length must be within the allowed range.";

/// Requires the number of characters in a text to be between `min` and `max`, inclusive.
#[derive(Debug, Clone)]
pub struct LengthBetweenValidation {
    min: usize,
    max: usize,
    message: Option<String>,
}

impl LengthBetweenValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("VALUE_LENGTH_OUT_OF_RANGE");

    /// Creates the rule with the default message.
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max, message: None }
    }

    /// Customizes the message without changing the rule's stable code.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<T: AsRef<str> + ?Sized> BusinessLogicValidation<T> for LengthBetweenValidation {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        let len = to_validate.as_ref().chars().count();
        if len < self.min || len > self.max {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE))
                .with_detail("length", len.to_string())
                .with_detail("min", self.min.to_string())
                .with_detail("max", self.max.to_string()));
        }
        Ok(())
    }
}
