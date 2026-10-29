use std::sync::Mutex;

use validations_rs::{
    start_validations_async, AsyncBusinessLogicValidation, BusinessRuleError, RuleCode, ValidationFuture,
};

use crate::support::block_on;

/// Test validation that records its execution and fails when `fail` is true.
struct Probe<'a> {
    name: &'static str,
    fail: bool,
    log: &'a Mutex<Vec<&'static str>>,
}

impl AsyncBusinessLogicValidation<()> for Probe<'_> {
    fn validate<'a>(&'a self, _: &'a ()) -> ValidationFuture<'a> {
        Box::pin(async move {
            self.log.lock().unwrap().push(self.name);
            if self.fail {
                Err(BusinessRuleError::new(RuleCode(self.name), "Fails."))
            } else {
                Ok(())
            }
        })
    }
}

#[test]
fn runs_in_list_order() {
    let log = Mutex::new(vec![]);
    let a = Probe { name: "a", fail: false, log: &log };
    let b = Probe { name: "b", fail: false, log: &log };
    let c = Probe { name: "c", fail: false, log: &log };
    block_on(start_validations_async(&[&c, &a, &b], &())).unwrap();
    assert_eq!(*log.lock().unwrap(), vec!["c", "a", "b"]);
}

#[test]
fn stops_at_first_failure() {
    let log = Mutex::new(vec![]);
    let a = Probe { name: "a", fail: false, log: &log };
    let b = Probe { name: "b", fail: true, log: &log };
    let c = Probe { name: "c", fail: true, log: &log };
    let err = block_on(start_validations_async(&[&a, &b, &c], &())).unwrap_err();
    assert_eq!(err.code, RuleCode("b"));
    assert_eq!(*log.lock().unwrap(), vec!["a", "b"]);
}

#[test]
fn empty_list_passes() {
    let list: [&dyn AsyncBusinessLogicValidation<()>; 0] = [];
    assert!(block_on(start_validations_async(&list, &())).is_ok());
}

#[test]
fn reports_the_error_of_the_first_failing_rule() {
    let log = Mutex::new(vec![]);
    let ok = Probe { name: "ok", fail: false, log: &log };
    let bad = Probe { name: "ASYNC", fail: true, log: &log };
    assert!(block_on(start_validations_async(&[&ok], &())).is_ok());
    assert_eq!(block_on(start_validations_async(&[&bad, &ok], &())).unwrap_err().code, RuleCode("ASYNC"));
}
