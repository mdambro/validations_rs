use std::fmt;

/// Stable code of a business rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleCode(pub &'static str);

impl RuleCode {
    /// Returns the code as text.
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl fmt::Display for RuleCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
