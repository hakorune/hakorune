//! Argument purpose uses original topology, not an enclosing Print/Return root.
use super::*;
use crate::mir::resolved_value_profile::expression_plan::{
    call_argument_expression_source_plan_v1, ExpressionPlanRootV1,
};

#[test]
fn original_call_argument_plan_keeps_outer_identity_ordinal_and_nearest_statement() {
    let source = "static box Helpers { value(q) { local x = Sink.take(q.substring(0,1), 7) if true { return Sink.take(q,8) } return 0 } }";
    let root = NyashParser::parse_from_string(source).unwrap();
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(method_declaration(&root, "Helpers", "value")).unwrap();
    let input = unit.root_function_input().unwrap();
    let mut checked = 0;
    let mut statements = std::collections::BTreeSet::new();
    for (site, _) in input.function().method_calls().filter(|(_, call)| call.selector() == "take") {
        let call = input.function().method_call_source_v1(site).unwrap();
        for ordinal in 0..2 {
            let plan = call_argument_expression_source_plan_v1(input, call, ordinal).unwrap();
            let ExpressionPlanRootV1::CallArgument { statement, call: retained, ordinal: retained_ordinal } = plan.root() else { panic!("argument purpose") };
            assert!(std::sync::Arc::ptr_eq(call, retained));
            assert_eq!(*retained_ordinal, ordinal);
            assert_eq!(plan.nodes()[0].site(), call.arguments()[ordinal as usize].site());
            assert!(!plan.nodes().iter().any(|node| node.site() == site));
            assert_eq!(plan.ordered_postorder_v1().unwrap().len(), plan.nodes().len());
            assert_eq!(input.function().source_site_inventory().containing_statement(site), Some(statement));
            assert!(plan.root().initializer().is_none());
            statements.insert(statement.clone());
            checked += 1;
        }
        assert!(call_argument_expression_source_plan_v1(input, call, 2).is_err());
        let foreign = VerifiedResolvedSourceUnitV1::resolve_function(method_declaration(&root, "Helpers", "value")).unwrap();
        assert!(call_argument_expression_source_plan_v1(foreign.root_function_input().unwrap(), call, 0).is_err());
    }
    assert_eq!(checked, 4);
    assert_eq!(statements.len(), 2, "nested Return belongs to its own registered statement");
}
