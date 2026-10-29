use std::cell::RefCell;

use validations_rs::{
    start_validations, BusinessLogicValidation, BusinessRuleError, LengthBetweenValidation, NotBlankValidation,
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
            Err(BusinessRuleError::new(RuleCode(self.name), "Fails.").with_detail("rule", self.name))
        } else {
            Ok(())
        }
    }
}

#[test]
fn runs_in_list_order() {
    let log = RefCell::new(vec![]);
    let a = Probe { name: "a", fail: false, log: &log };
    let b = Probe { name: "b", fail: false, log: &log };
    let c = Probe { name: "c", fail: false, log: &log };
    start_validations(&[&c, &a, &b], &()).unwrap();
    assert_eq!(*log.borrow(), vec!["c", "a", "b"]);
}

#[test]
fn stops_at_first_failure() {
    let log = RefCell::new(vec![]);
    let a = Probe { name: "a", fail: false, log: &log };
    let b = Probe { name: "b", fail: true, log: &log };
    let c = Probe { name: "c", fail: true, log: &log };
    let err = start_validations(&[&a, &b, &c], &()).unwrap_err();
    assert_eq!(err.code, RuleCode("b"));
    assert_eq!(*log.borrow(), vec!["a", "b"]);
}

#[test]
fn returns_the_failing_error_unchanged() {
    let log = RefCell::new(vec![]);
    let failing = Probe { name: "x", fail: true, log: &log };
    let err = start_validations(&[&failing], &()).unwrap_err();
    assert_eq!(err.message, "Fails.");
    assert_eq!(err.detail("rule"), Some("x"));
}

#[test]
fn empty_list_passes() {
    let list: [&dyn BusinessLogicValidation<()>; 0] = [];
    assert!(start_validations(&list, &()).is_ok());
}

#[test]
fn order_decides_which_error_is_reported() {
    let not_blank = NotBlankValidation::new();
    let length = LengthBetweenValidation::new(2, 5);

    let blank_first: [&dyn BusinessLogicValidation<str>; 2] = [&not_blank, &length];
    assert_eq!(start_validations(&blank_first, " ").unwrap_err().code, NotBlankValidation::CODE);

    let length_first: [&dyn BusinessLogicValidation<str>; 2] = [&length, &not_blank];
    assert_eq!(start_validations(&length_first, " ").unwrap_err().code, LengthBetweenValidation::CODE);
}

#[test]
fn validates_unsized_values() {
    let not_blank = NotBlankValidation::new();
    let list: [&dyn BusinessLogicValidation<str>; 1] = [&not_blank];
    assert!(start_validations(&list, "hello").is_ok());
    assert!(start_validations(&list, "").is_err());
}
