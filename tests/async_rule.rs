use std::future::Future;
use std::io;
use std::pin::pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

use validations_rs::{validate_async, AsyncError, AsyncRule, RuleCode, RuleFuture, Violation};

/// Runs a future to completion on the current thread; enough for futures that never wait on I/O.
fn block_on<F: Future>(future: F) -> F::Output {
    struct NoopWaker;
    impl Wake for NoopWaker {
        fn wake(self: Arc<Self>) {}
    }

    let waker = Waker::from(Arc::new(NoopWaker));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}

trait Users: Send + Sync {
    fn exists(&self, email: &str) -> Result<bool, io::Error>;
}

struct InMemoryUsers(Vec<&'static str>);

impl Users for InMemoryUsers {
    fn exists(&self, email: &str) -> Result<bool, io::Error> {
        Ok(self.0.contains(&email))
    }
}

struct BrokenUsers;

impl Users for BrokenUsers {
    fn exists(&self, _: &str) -> Result<bool, io::Error> {
        Err(io::Error::new(io::ErrorKind::ConnectionRefused, "database down"))
    }
}

const EMAIL_TAKEN: RuleCode = RuleCode("EMAIL_TAKEN");

struct EmailNotTaken<'r> {
    users: &'r dyn Users,
}

impl AsyncRule<str> for EmailNotTaken<'_> {
    fn check<'a>(&'a self, email: &'a str) -> RuleFuture<'a> {
        Box::pin(async move {
            if self.users.exists(email).map_err(AsyncError::check_failed)? {
                return Err(Violation::new(EMAIL_TAKEN, "the email is already registered").into());
            }
            Ok(())
        })
    }
}

struct Counted<'c>(&'c AtomicUsize);

impl AsyncRule<str> for Counted<'_> {
    fn check<'a>(&'a self, _: &'a str) -> RuleFuture<'a> {
        Box::pin(async move {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
    }
}

#[test]
fn passes_when_every_rule_passes() {
    let users = InMemoryUsers(vec!["grace@example.com"]);
    assert!(block_on(validate_async("ada@example.com", &[&EmailNotTaken { users: &users }])).is_ok());
}

#[test]
fn a_broken_rule_is_a_violation() {
    let users = InMemoryUsers(vec!["ada@example.com"]);
    let error = block_on(validate_async("ada@example.com", &[&EmailNotTaken { users: &users }])).unwrap_err();
    assert_eq!(error.violation().map(Violation::code), Some(EMAIL_TAKEN));
    assert_eq!(error.to_string(), "[EMAIL_TAKEN] the email is already registered");
    assert_eq!(error.into_violation().map(|v| v.code()), Some(EMAIL_TAKEN));
}

#[test]
fn a_failed_check_is_not_a_violation_and_keeps_its_cause() {
    let error = block_on(validate_async("ada@example.com", &[&EmailNotTaken { users: &BrokenUsers }])).unwrap_err();
    assert!(matches!(error, AsyncError::CheckFailed(_)));
    assert!(error.violation().is_none());
    assert_eq!(error.to_string(), "a business rule could not be checked");
    let source = std::error::Error::source(&error).expect("the cause is the source");
    assert_eq!(source.to_string(), "database down");
}

#[test]
fn stops_at_the_first_failure() {
    let calls = AtomicUsize::new(0);
    let users = InMemoryUsers(vec!["ada@example.com"]);
    let rules: [&dyn AsyncRule<str>; 2] = [&EmailNotTaken { users: &users }, &Counted(&calls)];
    assert!(block_on(validate_async("ada@example.com", &rules)).is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
}

#[test]
fn runs_rules_in_order_with_no_rules_passing() {
    let calls = AtomicUsize::new(0);
    assert!(block_on(validate_async("x", &[&Counted(&calls), &Counted(&calls)])).is_ok());
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert!(block_on(validate_async::<str>("x", &[])).is_ok());
}

#[test]
fn a_violation_converts_into_an_async_error() {
    let error: AsyncError = Violation::new(EMAIL_TAKEN, "taken").into();
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn the_future_is_send() {
    fn assert_send<T: Send>(_: &T) {}
    let users = InMemoryUsers(vec![]);
    let rule = EmailNotTaken { users: &users };
    let rules: [&dyn AsyncRule<str>; 1] = [&rule];
    let future = validate_async("ada@example.com", &rules);
    assert_send(&future);
}
