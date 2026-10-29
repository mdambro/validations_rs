use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The collection must not be empty.";

/// Requires a collection to have at least one element.
///
/// Works with any type whose reference is iterable: `Vec`, slices, arrays, `HashMap`, `HashSet`,
/// `BTreeMap`, etc. For text, use [`NotBlankValidation`](super::NotBlankValidation).
#[derive(Debug, Clone, Default)]
pub struct NotEmptyValidation {
    message: Option<String>,
}

impl NotEmptyValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("COLLECTION_EMPTY");

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

impl<T: ?Sized> BusinessLogicValidation<T> for NotEmptyValidation
where
    for<'a> &'a T: IntoIterator,
{
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        if IntoIterator::into_iter(to_validate).next().is_none() {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
