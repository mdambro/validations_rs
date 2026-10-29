use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The value is less than the minimum allowed.";

/// Requires an ordered value to be greater than or equal to `min`.
///
/// A value that cannot be compared, such as `NaN`, is considered invalid.
#[derive(Debug, Clone)]
pub struct MinValidation<T> {
    min: T,
    message: Option<String>,
}

impl<T> MinValidation<T> {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("VALUE_BELOW_MIN");

    /// Creates the rule with the default message.
    pub fn new(min: T) -> Self {
        Self { min, message: None }
    }

    /// Customizes the message without changing the rule's stable code.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<T: PartialOrd + std::fmt::Display> BusinessLogicValidation<T> for MinValidation<T> {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        let is_valid = *to_validate >= self.min;
        if !is_valid {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE))
                .with_detail("value", to_validate.to_string())
                .with_detail("min", self.min.to_string()));
        }
        Ok(())
    }
}
