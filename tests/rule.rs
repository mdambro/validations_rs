use validations_rs::rules::NotBlank;
use validations_rs::{ensure, validate, validate_all, Rule, RuleCode, Violation};

struct Order {
    quantity: u32,
    express: bool,
    weight_kg: u32,
}

const QUANTITY_ZERO: RuleCode = RuleCode("QUANTITY_ZERO");
const EXPRESS_TOO_HEAVY: RuleCode = RuleCode("EXPRESS_TOO_HEAVY");

fn has_quantity(order: &Order) -> Result<(), Violation> {
    ensure(order.quantity > 0, QUANTITY_ZERO, "the order needs at least one item")
}

fn express_is_light(order: &Order) -> Result<(), Violation> {
    ensure(!order.express || order.weight_kg <= 20, EXPRESS_TOO_HEAVY, "express orders weigh 20 kg or less")
}

#[test]
fn ensure_is_ok_when_the_condition_holds() {
    assert!(ensure(true, QUANTITY_ZERO, "unused").is_ok());
}

#[test]
fn ensure_reports_the_code_and_message_when_the_condition_fails() {
    let violation = ensure(false, QUANTITY_ZERO, "the order needs at least one item").unwrap_err();
    assert_eq!(violation.code(), QUANTITY_ZERO);
    assert_eq!(violation.message(), "the order needs at least one item");
    assert_eq!(violation.field(), None);
}

#[test]
fn ensure_accepts_an_owned_message() {
    let limit = 20;
    let violation = ensure(false, EXPRESS_TOO_HEAVY, format!("limit is {limit} kg")).unwrap_err();
    assert_eq!(violation.message(), "limit is 20 kg");
}

#[test]
fn a_function_is_a_rule() {
    let order = Order { quantity: 0, express: false, weight_kg: 1 };
    assert_eq!(has_quantity.check(&order).unwrap_err().code(), QUANTITY_ZERO);
}

#[test]
fn a_list_of_plain_functions_needs_no_annotations() {
    let order = Order { quantity: 1, express: true, weight_kg: 30 };
    let violation = validate(&order, &[&has_quantity, &express_is_light]).unwrap_err();
    assert_eq!(violation.code(), EXPRESS_TOO_HEAVY);
}

#[test]
fn a_capturing_closure_is_a_rule() {
    let max_weight = 10;
    let light = |order: &Order| ensure(order.weight_kg <= max_weight, EXPRESS_TOO_HEAVY, "too heavy");
    let order = Order { quantity: 1, express: false, weight_kg: 11 };
    assert!(validate(&order, &[&has_quantity, &light]).is_err());
}

#[test]
fn a_custom_type_can_implement_rule() {
    struct MaxItems(u32);

    impl Rule<Order> for MaxItems {
        fn check(&self, order: &Order) -> Result<(), Violation> {
            ensure(order.quantity <= self.0, RuleCode("TOO_MANY_ITEMS"), "too many items")
                .map_err(|v| v.with_detail("max", self.0))
        }
    }

    let order = Order { quantity: 5, express: false, weight_kg: 1 };
    let violation = validate(&order, &[&has_quantity, &MaxItems(3)]).unwrap_err();
    assert_eq!(violation.code().as_str(), "TOO_MANY_ITEMS");
    assert_eq!(violation.detail("max"), Some("3"));
}

#[test]
fn a_rule_works_behind_a_box_and_a_reference() {
    let boxed: Box<dyn Rule<str>> = Box::new(NotBlank);
    assert!(boxed.check("x").is_ok());
    assert!(validate_all("  ", &[boxed.as_ref()]).is_err());
}
