use super::callable_single_loop_recipe_coseal::{
    issue_callable_single_loop_recipe_v1, CallableRecipeCoSealRejectV1,
};
use super::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
use super::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_from_ledger_v1;
use super::callable_single_loop_syntax_facts::tests::unit;
#[path = "callable_single_loop_recipe_shape.rs"]
mod callable_single_loop_recipe_shape;
use crate::mir::loop_recipe_contract::{
    issue_initialized_local_input_source_set_v1, LoopInitializedLocalInputSourceRelationV1,
    LoopValueClassV1, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableSemanticSourceLedgerView, FunctionOwnerIssuerV1,
};
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
    let input = unit.root_function_input().expect("resolved input");
    let ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("ledger");
    let syntax = issue_callable_single_loop_syntax_facts_from_ledger_v1(input, &ledger)
        .expect("syntax facts");
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
    assert_eq!(co_seal.input().rows().len(), 1);
    assert_eq!(co_seal.input().rows()[0].recipe_value().raw(), 0);
    assert_eq!(co_seal.operations().len(), 7);
    assert_eq!(
        callable_single_loop_recipe_shape::callable_recipe(),
        recipe.clone()
    );
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

#[test]
fn initialized_local_input_set_rejects_incomplete_and_declaration_mismatch() {
    let unit = unit(None, integer(1));
    let (ledger, map) = issue(&unit);
    let product = issue_callable_single_loop_recipe_v1(&ledger, map).expect("co-seal");
    let core = product.co_seal().core();
    assert!(matches!(
        issue_initialized_local_input_source_set_v1(core, Vec::new()),
        Err(crate::mir::loop_recipe_contract::
            LoopInitializedLocalInputSourceSetRejectV1::InputCountMismatch { .. })
    ));

    let row = &product.co_seal().input().rows()[0];
    let mismatched_declaration = crate::mir::resolved_semantics::SourceBindingSiteV1::Local {
        statement: match row.declaration() {
            crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, .. } => {
                statement.clone()
            }
            _ => panic!("expected local declaration"),
        },
        ordinal: 99,
    };
    let relation = LoopInitializedLocalInputSourceRelationV1::new(
        mismatched_declaration,
        row.initializer().clone(),
        row.source_binding(),
        row.recipe_value(),
        row.class(),
    );
    assert!(matches!(
        issue_initialized_local_input_source_set_v1(core, vec![relation]),
        Err(crate::mir::loop_recipe_contract::
            LoopInitializedLocalInputSourceSetRejectV1::DeclarationMismatch { .. })
    ));

    let foreign_owner = FunctionOwnerIssuerV1::new_for_compilation()
        .expect("compilation brand")
        .issue()
        .expect("function owner");
    let foreign = LoopInitializedLocalInputSourceRelationV1::new(
        row.declaration().clone(),
        row.initializer().clone(),
        BindingRefV1::new(foreign_owner, hakorune_mir_core::BindingId::new(0)),
        row.recipe_value(),
        row.class(),
    );
    assert!(matches!(
        issue_initialized_local_input_source_set_v1(core, vec![foreign]),
        Err(crate::mir::loop_recipe_contract::
            LoopInitializedLocalInputSourceSetRejectV1::ForeignOwner { .. })
    ));

    let foreign_recipe = LoopInitializedLocalInputSourceRelationV1::new(
        row.declaration().clone(),
        row.initializer().clone(),
        row.source_binding(),
        LoopValueKeyV1::new(99),
        row.class(),
    );
    assert!(matches!(
        issue_initialized_local_input_source_set_v1(core, vec![foreign_recipe]),
        Err(crate::mir::loop_recipe_contract::
            LoopInitializedLocalInputSourceSetRejectV1::ForeignRecipeInput { .. })
    ));

    let class_mismatch = LoopInitializedLocalInputSourceRelationV1::new(
        row.declaration().clone(),
        row.initializer().clone(),
        row.source_binding(),
        row.recipe_value(),
        LoopValueClassV1::Bool,
    );
    assert!(matches!(
        issue_initialized_local_input_source_set_v1(core, vec![class_mismatch]),
        Err(crate::mir::loop_recipe_contract::
            LoopInitializedLocalInputSourceSetRejectV1::ClassMismatch { .. })
    ));
}
