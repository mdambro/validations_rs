use std::fmt;
use std::sync::Arc;
use std::time::SystemTime;

/// Source of the current time for date rules: the system clock unless a test or caller provides another.
#[derive(Clone, Default)]
pub(crate) struct Clock {
    custom: Option<Arc<dyn Fn() -> SystemTime + Send + Sync>>,
}

impl Clock {
    /// A clock that asks `now` for the current time.
    pub(crate) fn from_fn(now: impl Fn() -> SystemTime + Send + Sync + 'static) -> Self {
        Self { custom: Some(Arc::new(now)) }
    }

    /// The current time according to this clock.
    pub(crate) fn now(&self) -> SystemTime {
        self.custom.as_ref().map_or_else(SystemTime::now, |now| now())
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.custom.is_some() { "Clock(custom)" } else { "Clock(system)" })
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::Clock;

    #[test]
    fn defaults_to_the_system_clock() {
        let before = SystemTime::now();
        let now = Clock::default().now();
        assert!(now >= before && now <= SystemTime::now() + Duration::from_secs(1));
        assert_eq!(format!("{:?}", Clock::default()), "Clock(system)");
    }

    #[test]
    fn uses_the_given_function() {
        let fixed = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
        let clock = Clock::from_fn(move || fixed);
        assert_eq!(clock.now(), fixed);
        assert_eq!(format!("{clock:?}"), "Clock(custom)");
    }
}
