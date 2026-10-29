use validations_rs::{AsyncBusinessLogicValidation, BusinessRuleError, RuleCode, ValidationFuture};

use crate::support::block_on;

struct IsEven;

impl AsyncBusinessLogicValidation<i32> for IsEven {
    fn validate<'a>(&'a self, to_validate: &'a i32) -> ValidationFuture<'a> {
        Box::pin(async move {
            if to_validate % 2 == 0 {
                Ok(())
            } else {
                Err(BusinessRuleError::new(RuleCode("ODD"), "Is odd.").with_detail("value", to_validate.to_string()))
            }
        })
    }
}

fn assert_send<T: Send>(_: &T) {}

#[test]
fn validates_asynchronously() {
    assert!(block_on(IsEven.validate(&4)).is_ok());
    let error = block_on(IsEven.validate(&3)).unwrap_err();
    assert_eq!(error.code, RuleCode("ODD"));
    assert_eq!(error.detail("value"), Some("3"));
}

#[test]
fn is_usable_as_a_trait_object() {
    let rule: &dyn AsyncBusinessLogicValidation<i32> = &IsEven;
    assert!(block_on(rule.validate(&2)).is_ok());
    assert!(block_on(rule.validate(&1)).is_err());
}

#[test]
fn the_future_can_be_sent_across_threads() {
    let value = 2;
    let future = IsEven.validate(&value);
    assert_send(&future);
    assert!(block_on(future).is_ok());
}
