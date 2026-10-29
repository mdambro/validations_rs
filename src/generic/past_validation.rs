use std::time::SystemTime;

use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The date must be in the past.";

/// Requires an instant to be before the current time.
///
/// Compares against `SystemTime::now()` at validation time.
#[derive(Debug, Clone, Default)]
pub struct PastValidation {
    message: Option<String>,
}

impl PastValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("DATE_NOT_PAST");

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

impl BusinessLogicValidation<SystemTime> for PastValidation {
    fn validate(&self, to_validate: &SystemTime) -> Result<(), BusinessRuleError> {
        if *to_validate >= SystemTime::now() {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
