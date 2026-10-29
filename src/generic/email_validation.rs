use crate::error::{BusinessRuleError, RuleCode};
use crate::validation::BusinessLogicValidation;

const DEFAULT_MESSAGE: &str = "The email address does not have a valid format.";
const MAX_EMAIL_LEN: usize = 254;
const MAX_LOCAL_LEN: usize = 64;
const MAX_LABEL_LEN: usize = 63;
const LOCAL_SPECIAL_CHARS: &str = "!#$%&'*+/=?^_`{|}~.-";

/// Requires a text to look like an email address.
///
/// This is a pragmatic structural check, not a full RFC 5322 implementation: it requires a local part
/// and a domain with at least two dot-separated labels, with no spaces or consecutive dots. It does not
/// verify that the domain or the mailbox exist.
#[derive(Debug, Clone, Default)]
pub struct EmailValidation {
    message: Option<String>,
}

impl EmailValidation {
    /// Default business code for this rule.
    pub const CODE: RuleCode = RuleCode("EMAIL_INVALID");

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

impl<T: AsRef<str> + ?Sized> BusinessLogicValidation<T> for EmailValidation {
    fn validate(&self, to_validate: &T) -> Result<(), BusinessRuleError> {
        let email = to_validate.as_ref();
        if !is_valid_email(email) {
            return Err(BusinessRuleError::new(Self::CODE, self.message.as_deref().unwrap_or(DEFAULT_MESSAGE))
                .with_detail("email", email));
        }
        Ok(())
    }
}

fn is_valid_email(email: &str) -> bool {
    if email.len() > MAX_EMAIL_LEN {
        return false;
    }
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    is_valid_local_part(local) && is_valid_domain(domain)
}

fn is_valid_local_part(local: &str) -> bool {
    !local.is_empty()
        && local.len() <= MAX_LOCAL_LEN
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..")
        && local.chars().all(|c| c.is_alphanumeric() || LOCAL_SPECIAL_CHARS.contains(c))
}

fn is_valid_domain(domain: &str) -> bool {
    let mut labels = 0;
    for label in domain.split('.') {
        let is_valid_label = !label.is_empty()
            && label.len() <= MAX_LABEL_LEN
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label.chars().all(|c| c.is_alphanumeric() || c == '-');
        if !is_valid_label {
            return false;
        }
        labels += 1;
    }
    labels >= 2
}
