//! Pruebas del ejecutor y de las validaciones genéricas.

use std::cell::RefCell;
use validations::*;

/// Validación de prueba que registra su paso y falla si `fail` es verdadero.
struct Probe<'a> {
    order: i16,
    name: &'static str,
    fail: bool,
    log: &'a RefCell<Vec<&'static str>>,
}

impl BusinessLogicValidation<()> for Probe<'_> {
    fn execution_order(&self) -> i16 {
        self.order
    }
    fn validate(&self, _: &()) -> Result<(), BusinessRuleError> {
        self.log.borrow_mut().push(self.name);
        if self.fail {
            Err(BusinessRuleError::new(RuleCode(self.name), "falla"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn runs_in_ascending_order_with_stable_ties() {
    let log = RefCell::new(vec![]);
    let a = Probe { order: 20, name: "a", fail: false, log: &log };
    let b = Probe { order: 10, name: "b", fail: false, log: &log };
    let c = Probe { order: 10, name: "c", fail: false, log: &log };
    start_validations(&[&a, &b, &c], &()).unwrap();
    assert_eq!(*log.borrow(), vec!["b", "c", "a"]);
}

#[test]
fn stops_at_first_failure() {
    let log = RefCell::new(vec![]);
    let a = Probe { order: 10, name: "a", fail: false, log: &log };
    let b = Probe { order: 20, name: "b", fail: true, log: &log };
    let c = Probe { order: 30, name: "c", fail: true, log: &log };
    let err = start_validations(&[&c, &b, &a], &()).unwrap_err();
    assert_eq!(err.code, RuleCode("b"));
    assert_eq!(*log.borrow(), vec!["a", "b"]);
}

#[test]
fn empty_list_passes() {
    let list: [&dyn BusinessLogicValidation<()>; 0] = [];
    assert!(start_validations(&list, &()).is_ok());
}

#[test]
fn not_blank_length_range_predicate() {
    let nb = NotBlankValidation { order: 1, code: RuleCode("BLANK") };
    assert!(nb.validate("x").is_ok());
    assert!(BusinessLogicValidation::<str>::validate(&nb, "   ").is_err());

    let len = LengthBetweenValidation { order: 1, min: 2, max: 3, code: RuleCode("LEN") };
    assert!(BusinessLogicValidation::<str>::validate(&len, "ab").is_ok());
    assert!(BusinessLogicValidation::<str>::validate(&len, "a").is_err());
    assert!(BusinessLogicValidation::<str>::validate(&len, "abcd").is_err());
    // cuenta caracteres, no bytes
    assert!(BusinessLogicValidation::<str>::validate(&len, "ñé").is_ok());

    let range = RangeValidation { order: 1, min: 0, max: 10, code: RuleCode("RNG") };
    assert!(range.validate(&10).is_ok());
    assert!(range.validate(&11).is_err());

    let pred: PredicateValidation<i32> =
        PredicateValidation { order: 1, code: RuleCode("EVEN"), message: "par", predicate: |n| n % 2 == 0 };
    assert!(pred.validate(&4).is_ok());
    assert_eq!(pred.validate(&3).unwrap_err().code, RuleCode("EVEN"));
}

#[test]
fn error_details() {
    let e = BusinessRuleError::new(RuleCode("X"), "m").with_detail("k", "v");
    assert_eq!(e.detail("k"), Some("v"));
    assert_eq!(e.detail("z"), None);
    assert_eq!(e.to_string(), "[X] m");
}

struct AsyncProbe(i16, bool);

impl AsyncBusinessLogicValidation<()> for AsyncProbe {
    fn execution_order(&self) -> i16 {
        self.0
    }
    fn validate<'a>(&'a self, _: &'a ()) -> ValidationFuture<'a> {
        let fail = self.1;
        Box::pin(async move {
            if fail {
                Err(BusinessRuleError::new(RuleCode("ASYNC"), "falla"))
            } else {
                Ok(())
            }
        })
    }
}

/// Ejecutor mínimo de futuros para probar sin dependencias.
fn block_on<F: std::future::Future>(f: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn noop_raw() -> RawWaker {
        fn clone(_: *const ()) -> RawWaker {
            noop_raw()
        }
        fn noop(_: *const ()) {}
        static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(std::ptr::null(), &VT)
    }
    let waker = unsafe { Waker::from_raw(noop_raw()) };
    let mut cx = Context::from_waker(&waker);
    let mut f = Box::pin(f);
    loop {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            return v;
        }
    }
}

#[test]
fn async_runner_stops_at_first_failure() {
    let ok = AsyncProbe(10, false);
    let bad = AsyncProbe(20, true);
    assert!(block_on(start_validations_async(&[&ok], &())).is_ok());
    assert_eq!(block_on(start_validations_async(&[&bad, &ok], &())).unwrap_err().code, RuleCode("ASYNC"));
}
