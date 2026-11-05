use std::cell::RefCell;

use validations_rs::{
    collect_validations, BusinessLogicValidation, BusinessRuleError, LengthBetweenValidation, NotBlankValidation,
    RuleCode,
};

/// Test validation that records its execution and fails when `fail` is true.
struct Probe<'a> {
    name: &'static str,
    fail: bool,
    log: &'a RefCell<Vec<&'static str>>,
}

impl BusinessLogicValidation<()> for Probe<'_> {
    fn validate(&self, _: &()) -> Result<(), BusinessRuleError> {
        self.log.borrow_mut().push(self.name);
        if self.fail {
            Err(BusinessRuleError::new(RuleCode(self.name), "Fails."))
        } else {
            Ok(())
        }
    }
}

#[test]
fn runs_every_rule_and_returns_all_failures_in_order() {
    let log = RefCell::new(vec![]);
    let a = Probe { name: "a", fail: true, log: &log };
    let b = Probe { name: "b", fail: false, log: &log };
    let c = Probe { name: "c", fail: true, log: &log };
    let errors = collect_validations(&[&a, &b, &c], &()).unwrap_err();
    assert_eq!(errors.iter().map(|e| e.code).collect::<Vec<_>>(), vec![RuleCode("a"), RuleCode("c")]);
    assert_eq!(*log.borrow(), vec!["a", "b", "c"]);
}

#[test]
fn passes_when_no_rule_fails() {
    let log = RefCell::new(vec![]);
    let a = Probe { name: "a", fail: false, log: &log };
    assert!(collect_validations(&[&a], &()).is_ok());
}

#[test]
fn empty_list_passes() {
    let list: [&dyn BusinessLogicValidation<()>; 0] = [];
    assert!(collect_validations(&list, &()).is_ok());
}

#[test]
fn works_with_catalog_rules_on_unsized_values() {
    let errors = collect_validations::<str>(&[&NotBlankValidation::new(), &LengthBetweenValidation::new(2, 40)], " ")
        .unwrap_err();
    assert_eq!(errors[0].code, NotBlankValidation::CODE);
    assert_eq!(errors[1].code, LengthBetweenValidation::CODE);
}
