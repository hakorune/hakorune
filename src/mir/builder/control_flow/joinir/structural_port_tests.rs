use super::with_existing_structural_port;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;

#[test]
fn structural_port_is_callback_scoped_and_diagnostic_only() {
    let condition = ASTNode::Literal {
        value: LiteralValue::Bool(true),
        span: Span::unknown(),
    };
    let body = Vec::new();
    let context = LoopRouteContext::new(&condition, &body, "structural_port_test/0", true, false);

    let observed = with_existing_structural_port(&context, |port| {
        (port.diagnostic_label().to_owned(), port.debug_enabled())
    });

    assert_eq!(observed.0, "structural_port_test/0");
    assert!(observed.1);
}
