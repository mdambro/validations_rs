use std::time::SystemTime;

use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The date must be in the future.";

/// Requires an instant to be after the current time.
///
/// Compares against `SystemTime::now()` at validation time.
#[derive(Debug, Clone, Default)]
pub struct FutureValidation {
    message: Option<String>,
}

impl FutureValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("DATE_NOT_FUTURE");

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

impl BusinessLogicValidation<SystemTime> for FutureValidation {
    fn validate(&self, to_validate: &SystemTime) -> Result<(), BusinessRuleError> {
        if *to_validate <= SystemTime::now() {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
