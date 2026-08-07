use super::callable_single_loop_recipe_coseal::{
    issue_callable_single_loop_recipe_v1, CallableRecipeCoSealRejectV1,
};
use super::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
use super::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
use super::callable_single_loop_syntax_facts::tests::{input_loop_and_context, unit};
use crate::mir::loop_recipe_contract::LoopValueKeyV1;
use crate::mir::resolved_semantics::{BindingRefV1, CallableSemanticSourceLedgerView};
use hakorune_mir_core::BindingId;

fn integer(value: i64) -> crate::ast::ASTNode {
    crate::ast::ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(value),
        span: crate::ast::Span::unknown(),
    }
}

fn issue(
    unit: &crate::mir::compiler::VerifiedResolvedSourceUnitV1,
) -> (
    CallableSemanticSourceLedgerView<'_>,
    super::callable_single_loop_source_map::VerifiedCallableSingleLoopSourceMapV1,
) {
    let (input, loop_stmt, context) = input_loop_and_context(unit);
    let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context)
        .expect("syntax facts");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).expect("source map");
    (ledger, map)
}

#[test]
fn co_seals_common_recipe_and_callable_boundary_once() {
    let unit = unit(None, integer(1));
    let (ledger, map) = issue(&unit);
    let product = issue_callable_single_loop_recipe_v1(&ledger, map).expect("co-seal");
    let co_seal = product.co_seal();
    let recipe = co_seal.core().recipe().as_recipe();

    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.items.len(), 7);
    assert_eq!(recipe.inputs, vec![LoopValueKeyV1::new(0)]);
    assert_eq!(recipe.carriers.len(), 1);
    assert_eq!(recipe.exits.len(), 0);
    assert_eq!(co_seal.input().recipe_value().raw(), 0);
    assert_eq!(co_seal.operations().len(), 7);
    assert_eq!(co_seal.continuation().after().binding().raw(), 0);
    assert_ne!(
        product.tail().binding(),
        BindingRefV1::new(product.prelude().owner(), BindingId::new(0))
    );
    assert_eq!(product.prelude().binding(), product.tail().binding());
}

#[test]
fn product_survives_source_unit_drop() {
    let product = {
        let unit = unit(None, integer(1));
        let (ledger, map) = issue(&unit);
        issue_callable_single_loop_recipe_v1(&ledger, map).expect("co-seal")
    };
    assert_eq!(product.co_seal().operations().len(), 7);
}

#[test]
fn rejects_prefix_tail_binding_mismatch_before_recipe_issue() {
    let unit = unit(None, integer(1));
    let (ledger, map) = issue(&unit);
    let foreign_binding = BindingRefV1::new(ledger.owner(), BindingId::new(99));
    let map = map.replace_prefix_binding_for_test(foreign_binding);
    assert!(matches!(
        issue_callable_single_loop_recipe_v1(&ledger, map),
        Err(CallableRecipeCoSealRejectV1::PrefixTailBindingMismatch)
    ));
}

#[test]
fn rejects_tail_continuation_fusion_before_recipe_issue() {
    let unit = unit(None, integer(1));
    let (ledger, map) = issue(&unit);
    let carrier_binding = map
        .rows()
        .iter()
        .find(|row| {
            row.role()
                == super::callable_single_loop_source_map::CallableSourceMapRoleV1::InitialCarrier
        })
        .and_then(|row| row.target().initial_carrier().map(|(binding, _)| binding))
        .expect("carrier binding");
    let map = map
        .replace_prefix_binding_for_test(carrier_binding)
        .replace_tail_binding_for_test(carrier_binding);
    assert!(matches!(
        issue_callable_single_loop_recipe_v1(&ledger, map),
        Err(CallableRecipeCoSealRejectV1::TailContinuationFusion)
    ));
}
