use std::fmt;

use super::RuleCode;

/// Business error shared by all validations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessRuleError {
    /// Stable code of the violated rule.
    pub code: RuleCode,
    /// Message readable by developers.
    pub message: String,
    /// Context data about the violation.
    pub details: Vec<(String, String)>,
}

impl BusinessRuleError {
    /// Creates an error without details.
    pub fn new(code: RuleCode, message: impl Into<String>) -> Self {
        Self { code, message: message.into(), details: Vec::new() }
    }

    /// Returns the error with one more detail.
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.details.push((key.into(), value.into()));
        self
    }

    /// Looks up a detail by key.
    pub fn detail(&self, key: &str) -> Option<&str> {
        self.details.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}

impl fmt::Display for BusinessRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for BusinessRuleError {}
