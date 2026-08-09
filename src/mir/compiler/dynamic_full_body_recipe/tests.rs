use crate::ast::ASTNode;
use crate::mir::loop_recipe_contract::{
    LoopExitKindV2, LoopItemKeyV1, LoopOperationV2, LoopRecipeItemV2, LoopValueClassV2,
};
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::CallableSemanticSourceLedgerView;
use crate::parser::NyashParser;

use super::super::dynamic_full_body_source::DynamicFullBodySourceIssuerV1;
use super::super::function_input::ResolvedFunctionLoweringInputV1;
use super::claims::DynamicFullLoopClaimTargetV2;
use super::produce_dynamic_full_loop_recipe_v2;

fn production_skip_while() -> ASTNode {
    let source =
        include_str!("../../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let program = NyashParser::parse_from_string(source).expect("source parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    statements
        .into_iter()
        .find_map(|statement| match statement {
            ASTNode::BoxDeclaration { name, methods, .. } if name == "ParserScanLoopBox" => {
                methods.get_declaration("skip_while").cloned()
            }
            _ => None,
        })
        .expect("unchanged production method")
}

fn source_inventory(
) -> super::super::dynamic_full_body_source::VerifiedDynamicLoopFullBodySourceInventoryV1 {
    let unit = Box::leak(Box::new(
        super::super::VerifiedResolvedSourceUnitV1::resolve_function(production_skip_while())
            .expect("fixture resolves"),
    ));
    let input: ResolvedFunctionLoweringInputV1<'static> =
        unit.root_function_input().expect("root input");
    let ledger = CallableSemanticSourceLedgerView::from_forest(input.forest(), input.owner())
        .expect("source ledger");
    let membership = ledger.only_loop_site().expect("one loop");
    let completion = verify_function_completion_v1(input).expect("completion");
    DynamicFullBodySourceIssuerV1::issue(input, membership, completion)
        .expect("full source inventory")
}

#[test]
fn unchanged_source_produces_the_complete_verified_v2_recipe() {
    let candidate =
        produce_dynamic_full_loop_recipe_v2(source_inventory()).expect("complete Recipe");
    let recipe = candidate.artifact().recipe().as_recipe();

    assert_eq!(recipe.loops.len(), 1);
    assert_eq!(recipe.blocks.len(), 3);
    assert_eq!(recipe.bindings.len(), 1);
    assert_eq!(recipe.inputs.len(), 4);
    assert_eq!(recipe.carriers.len(), 1);
    assert_eq!(recipe.values.len(), 18);
    assert_eq!(recipe.items.len(), 17);
    assert_eq!(recipe.exits.len(), 1);
    assert_eq!(recipe.bindings[0].class, LoopValueClassV2::Dynamic);
    assert_eq!(recipe.carriers[0].class, LoopValueClassV2::Dynamic);
    assert_eq!(
        recipe.exits[0].kind,
        LoopExitKindV2::Return {
            value: Some(crate::mir::loop_recipe_contract::LoopValueKeyV1::new(14))
        }
    );

    let calls = recipe
        .items
        .iter()
        .filter(|row| {
            matches!(
                row.item,
                LoopRecipeItemV2::Operation {
                    operation: LoopOperationV2::CallSlot { .. }
                }
            )
        })
        .count();
    assert_eq!(calls, 2);
    assert!(matches!(recipe.items[10].item, LoopRecipeItemV2::If { .. }));
    assert!(matches!(
        recipe.items[12].item,
        LoopRecipeItemV2::Exit { .. }
    ));
}

#[test]
fn producer_transfers_loop_authority_and_retains_every_other_source_fact() {
    let candidate =
        produce_dynamic_full_loop_recipe_v2(source_inventory()).expect("complete Recipe");
    assert_eq!(candidate.source().bindings().len(), 6);
    assert_eq!(candidate.source().rows().len(), 28);
    assert_eq!(candidate.source().completion().explicit_sites().len(), 2);
    assert_eq!(
        candidate.source().scope_region().scope().owner(),
        candidate.source().owner()
    );
    assert_eq!(candidate.artifact().source_binding().loops.len(), 1);
}

#[test]
fn private_claims_cover_all_source_roles_without_partial_selection_api() {
    let candidate =
        produce_dynamic_full_loop_recipe_v2(source_inventory()).expect("complete Recipe");
    let (_, _, claims) = candidate.into_parts();
    assert_eq!(claims.binding_rows().len(), 6);
    assert_eq!(claims.source_rows().len(), 28);
    assert!(claims
        .source_rows()
        .iter()
        .any(|row| { row.target == DynamicFullLoopClaimTargetV2::Item(LoopItemKeyV1::new(16)) }));
    assert_eq!(
        claims
            .source_rows()
            .iter()
            .filter(|row| {
                row.target == DynamicFullLoopClaimTargetV2::Item(LoopItemKeyV1::new(16))
            })
            .count(),
        2,
        "assignment statement and exact target expression intentionally share I16"
    );
}
