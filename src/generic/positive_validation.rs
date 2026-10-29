use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The value must be greater than zero.";

/// Requires a number to be strictly greater than zero.
///
/// Works with any numeric type whose `Default` is zero. `NaN` is considered invalid.
#[derive(Debug, Clone, Default)]
pub struct PositiveValidation {
    message: Option<String>,
}

impl PositiveValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("VALUE_NOT_POSITIVE");

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

impl<T: PartialOrd + Default + std::fmt::Display> BusinessLogicValidation<T> for PositiveValidation {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        let is_valid = *to_validate > T::default();
        if !is_valid {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE))
                .with_detail("value", to_validate.to_string()));
        }
        Ok(())
    }
}
