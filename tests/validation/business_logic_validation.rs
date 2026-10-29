use validations_rs::{BusinessLogicValidation, BusinessRuleError, RuleCode};

struct IsEven;

impl BusinessLogicValidation<i32> for IsEven {
    fn validate(&self, to_validate: &i32) -> Result<(), BusinessRuleError> {
        if to_validate % 2 == 0 {
            Ok(())
        } else {
            Err(BusinessRuleError::new(RuleCode("ODD"), "Is odd.").with_detail("value", to_validate.to_string()))
        }
    }
}

struct StartsWithA;

impl BusinessLogicValidation<str> for StartsWithA {
    fn validate(&self, to_validate: &str) -> Result<(), BusinessRuleError> {
        if to_validate.starts_with('a') {
            Ok(())
        } else {
            Err(BusinessRuleError::new(RuleCode("NOT_A"), "Must start with a."))
        }
    }
}

#[test]
fn validates_a_sized_type() {
    assert!(IsEven.validate(&4).is_ok());
    assert_eq!(IsEven.validate(&3).unwrap_err().code, RuleCode("ODD"));
}

#[test]
fn validates_an_unsized_type() {
    assert!(StartsWithA.validate("ada").is_ok());
    assert_eq!(StartsWithA.validate("bob").unwrap_err().code, RuleCode("NOT_A"));
}

#[test]
fn is_usable_as_a_trait_object() {
    let rule: &dyn BusinessLogicValidation<i32> = &IsEven;
    assert!(rule.validate(&2).is_ok());
    assert!(rule.validate(&1).is_err());
}

#[test]
fn returns_the_error_with_its_details() {
    let error = IsEven.validate(&7).unwrap_err();
    assert_eq!(error.message, "Is odd.");
    assert_eq!(error.detail("value"), Some("7"));
}
