use std::fmt;
use std::sync::Arc;
use std::time::SystemTime;

use super::codes;
use crate::error::Violation;
use crate::rule::Rule;

/// Where the date rules get the current time: the system clock, or a function set with `with_clock`.
#[derive(Clone, Default)]
struct Clock(Option<Arc<dyn Fn() -> SystemTime + Send + Sync>>);

impl Clock {
    fn now(&self) -> SystemTime {
        self.0.as_ref().map_or_else(SystemTime::now, |now| now())
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.0.is_some() { "Clock(custom)" } else { "Clock(system)" })
    }
}

/// The date is before the current time.
///
/// Uses the system clock unless [`with_clock`](Self::with_clock) sets another source, which makes tests
/// deterministic:
///
/// ```
/// use std::time::{Duration, SystemTime};
/// use validations_rs::rules::Past;
/// use validations_rs::Rule;
///
/// let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
/// let rule = Past::new().with_clock(move || now);
/// assert!(rule.check(&(now - Duration::from_secs(1))).is_ok());
/// assert!(rule.check(&now).is_err());
/// ```
#[derive(Debug, Clone, Default)]
pub struct Past {
    clock: Clock,
}

impl Past {
    /// Compares against the system clock.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Compares against the time `now` returns.
    #[must_use]
    pub fn with_clock(mut self, now: impl Fn() -> SystemTime + Send + Sync + 'static) -> Self {
        self.clock = Clock(Some(Arc::new(now)));
        self
    }
}

impl Rule<SystemTime> for Past {
    fn check(&self, value: &SystemTime) -> Result<(), Violation> {
        if *value < self.clock.now() {
            Ok(())
        } else {
            Err(Violation::new(codes::DATE_NOT_PAST, "The date must be in the past."))
        }
    }
}

/// The date is after the current time. See [`Past`] for the clock.
#[derive(Debug, Clone, Default)]
pub struct Future {
    clock: Clock,
}

impl Future {
    /// Compares against the system clock.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Compares against the time `now` returns.
    #[must_use]
    pub fn with_clock(mut self, now: impl Fn() -> SystemTime + Send + Sync + 'static) -> Self {
        self.clock = Clock(Some(Arc::new(now)));
        self
    }
}

impl Rule<SystemTime> for Future {
    fn check(&self, value: &SystemTime) -> Result<(), Violation> {
        if *value > self.clock.now() {
            Ok(())
        } else {
            Err(Violation::new(codes::DATE_NOT_FUTURE, "The date must be in the future."))
        }
    }
}
