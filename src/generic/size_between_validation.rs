use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The number of elements must be within the allowed range.";

/// Requires a collection to have between `min` and `max` elements, inclusive.
///
/// Works with any type whose reference is iterable: `Vec`, slices, arrays, `HashMap`, `HashSet`,
/// `BTreeMap`, etc. For the length of a text, use
/// [`LengthBetweenValidation`](super::LengthBetweenValidation).
#[derive(Debug, Clone)]
pub struct SizeBetweenValidation {
    min: usize,
    max: usize,
    message: Option<String>,
}

impl SizeBetweenValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("SIZE_OUT_OF_RANGE");

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

impl<T: ?Sized> BusinessLogicValidation<T> for SizeBetweenValidation
where
    for<'a> &'a T: IntoIterator,
{
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        let size = IntoIterator::into_iter(to_validate).count();
        if size < self.min || size > self.max {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE))
                .with_detail("size", size.to_string())
                .with_detail("min", self.min.to_string())
                .with_detail("max", self.max.to_string()));
        }
        Ok(())
    }
}
