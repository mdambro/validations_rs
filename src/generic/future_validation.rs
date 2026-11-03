use std::time::SystemTime;

use super::clock::Clock;
use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The date must be in the future.";

/// Requires an instant to be after the current time.
///
/// Compares against the clock at validation time: `SystemTime::now()` by default, or the function given to
/// [`with_clock`](Self::with_clock), which makes the rule deterministic in tests.
#[derive(Debug, Clone, Default)]
pub struct FutureValidation {
    message: Option<String>,
    clock: Clock,
}

impl FutureValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("DATE_NOT_FUTURE");

    /// Creates the rule with the default message.
    #[must_use]
    pub fn new() -> Self {
        Self { message: None, clock: Clock::default() }
    }

    /// Uses `now` instead of the system clock to get the current time.
    ///
    /// ```
    /// use std::time::{Duration, SystemTime};
    /// use validations_rs::{BusinessLogicValidation, FutureValidation};
    ///
    /// let fixed_now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
    /// let rule = FutureValidation::new().with_clock(move || fixed_now);
    /// assert!(rule.validate(&(SystemTime::UNIX_EPOCH + Duration::from_secs(2_000))).is_ok());
    /// ```
    #[must_use]
    pub fn with_clock(mut self, now: impl Fn() -> SystemTime + Send + Sync + 'static) -> Self {
        self.clock = Clock::from_fn(now);
        self
    }

    /// Customizes the message without changing the rule's stable code.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl BusinessLogicValidation<SystemTime> for FutureValidation {
    fn validate(&self, to_validate: &SystemTime) -> Result<(), BusinessRuleError> {
        if *to_validate <= self.clock.now() {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE)));
        }
        Ok(())
    }
}
