use super::*;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::compiler::callable_single_loop_syntax_facts::tests::{
    input_loop_and_context, unit,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn positive() -> crate::mir::compiler::VerifiedResolvedSourceUnitV1 {
    unit(None, integer(1))
}

fn issue(
    unit: &crate::mir::compiler::VerifiedResolvedSourceUnitV1,
) -> (
    CallableSemanticSourceLedgerView<'_>,
    VerifiedCallableSingleLoopSourceMapV1,
) {
    let (input, loop_stmt, context) = input_loop_and_context(unit);
    let syntax = super::super::callable_single_loop_syntax_facts::
        issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("syntax facts");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).expect("map");
    (ledger, map)
}

#[test]
fn seals_nine_rows_plus_prefix_with_resolver_identity() {
    let unit = positive();
    let (_, map) = issue(&unit);
    assert_eq!(map.rows().len(), 9);
    assert_eq!(map.prefix().role(), CallableSourceMapRoleV1::PrefixBoundary);
    assert_eq!(map.loop_source().function_origin(), map.origin());
    assert_eq!(map.loop_source().source_kind(), map.source_kind());
    assert_eq!(
        map.rows()[0].role(),
        CallableSourceMapRoleV1::InitialCarrier
    );
    assert_eq!(
        map.rows()[8].role(),
        CallableSourceMapRoleV1::TailReturnRead
    );
    assert_eq!(map.scope_region().scope().owner(), map.owner());
    assert_eq!(map.scope_region().region().owner(), map.owner());
}

#[test]
fn rejects_foreign_syntax_owner_before_rows() {
    let first = positive();
    let second = positive();
    let (input, loop_stmt, context) = input_loop_and_context(&first);
    let syntax = super::super::callable_single_loop_syntax_facts::
        issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("syntax facts");
    let other_input: ResolvedFunctionLoweringInputV1<'_> =
        second.root_function_input().expect("other input");
    let ledger = other_input
        .forest()
        .callable_source_ledger(other_input.owner())
        .expect("other ledger");
    assert_eq!(
        issue_callable_single_loop_source_map_v1(&ledger, syntax),
        Err(CallableSourceMapRejectV1::ForeignOwner)
    );
}

#[test]
fn rejects_condition_bound_outside_selected_profile() {
    let unit = unit(None, integer(2));
    let (input, loop_stmt, context) = input_loop_and_context(&unit);
    let syntax = super::super::callable_single_loop_syntax_facts::
        issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("syntax facts");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    assert_eq!(
        issue_callable_single_loop_source_map_v1(&ledger, syntax),
        Err(CallableSourceMapRejectV1::UnsupportedLiteral(
            CallableSourceMapRoleV1::ConditionBound,
        ))
    );
}

#[test]
fn product_survives_source_unit_drop() {
    let map = {
        let unit = positive();
        let (_, map) = issue(&unit);
        map
    };
    assert_eq!(map.rows().len(), 9);
    assert_eq!(map.prefix().role(), CallableSourceMapRoleV1::PrefixBoundary);
}
