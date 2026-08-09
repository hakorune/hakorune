use crate::ast::ASTNode;
use crate::mir::loop_recipe_contract::{
    issue_sole_root_carrier_join_closure_v2, LoopBindingKeyV1, LoopCarrierKeyV1, LoopExitKindV2,
    LoopItemKeyV1, LoopJoinBranchArmV2, LoopJoinBranchExitTargetV2, LoopJoinClosureRejectV2,
    LoopJoinEdgeRoleV1, LoopJoinEdgeV2, LoopJoinPayloadV2, LoopJoinPortBindingV2, LoopJoinPortV1,
    LoopNodeKeyV1, LoopOperationV2, LoopRecipeBindingV2, LoopRecipeCarrierV2, LoopRecipeItemV2,
    LoopRecipeV2RejectReason, LoopRecipeValueV2, LoopRecipeVerifierV2, LoopValueClassV2,
    LoopValueKeyV1,
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

#[test]
fn unchanged_dynamic_recipe_has_exact_typed_joinsig() {
    let candidate =
        produce_dynamic_full_loop_recipe_v2(source_inventory()).expect("complete Recipe");
    let closure = issue_sole_root_carrier_join_closure_v2(candidate.artifact().recipe())
        .expect("typed Dynamic JoinSig/After closure");
    let sig = closure.join_sig().as_sig();

    let b0 = LoopBindingKeyV1::new(0);
    let l0 = LoopNodeKeyV1::new(0);
    let entry = payload(b0, 1);
    let backedge = payload(b0, 17);
    assert_eq!(sig.loops.len(), 1);
    assert_eq!(sig.loops[0].key, l0);
    assert_eq!(sig.loops[0].carriers, vec![backedge.clone()]);
    assert_eq!(
        sig.loops[0].edges,
        vec![
            edge(
                LoopJoinPortV1::Preheader,
                LoopJoinPortV1::Header,
                LoopJoinEdgeRoleV1::Enter,
                entry.clone(),
            ),
            edge(
                LoopJoinPortV1::Header,
                LoopJoinPortV1::Body,
                LoopJoinEdgeRoleV1::PredicateTrue,
                entry.clone(),
            ),
            edge(
                LoopJoinPortV1::Header,
                LoopJoinPortV1::After,
                LoopJoinEdgeRoleV1::PredicateFalse,
                entry.clone(),
            ),
            edge(
                LoopJoinPortV1::Body,
                LoopJoinPortV1::FunctionExit,
                LoopJoinEdgeRoleV1::Return,
                entry.clone(),
            ),
            edge(
                LoopJoinPortV1::Body,
                LoopJoinPortV1::Header,
                LoopJoinEdgeRoleV1::Backedge,
                backedge,
            ),
        ]
    );

    assert_eq!(sig.branches.len(), 1);
    let branch = &sig.branches[0];
    assert_eq!(branch.owner_loop, l0);
    assert_eq!(branch.if_item, LoopItemKeyV1::new(10));
    assert_eq!(branch.condition, LoopValueKeyV1::new(13));
    let LoopJoinBranchArmV2::Exit(then_exit) = &branch.then_arm else {
        panic!("then arm must return to FunctionExit")
    };
    assert_eq!(then_exit.exit_item, LoopItemKeyV1::new(12));
    assert_eq!(then_exit.role, LoopJoinEdgeRoleV1::Return);
    assert_eq!(then_exit.target, LoopJoinBranchExitTargetV2::FunctionExit);
    assert_eq!(then_exit.payload, vec![entry.clone()]);
    let LoopJoinBranchArmV2::Fallthrough { payload } = &branch.else_arm else {
        panic!("else arm must fall through")
    };
    assert_eq!(payload, &vec![entry]);

    assert_eq!(
        sig.port_bindings,
        vec![
            port_binding(l0, LoopJoinPortV1::Header, b0),
            port_binding(l0, LoopJoinPortV1::After, b0),
        ]
    );
    assert!(sig
        .loops
        .iter()
        .flat_map(|row| row.edges.iter())
        .flat_map(|edge| edge.payload.iter())
        .all(|row| row.value != LoopValueKeyV1::new(10) && row.value != LoopValueKeyV1::new(14)));
}

#[test]
fn v2_branch_targets_reject_cross_family_roles() {
    let loop_target = LoopJoinBranchExitTargetV2::Loop(LoopNodeKeyV1::new(0));
    let function_exit = LoopJoinBranchExitTargetV2::FunctionExit;

    assert!(loop_target.accepts_role(LoopJoinEdgeRoleV1::Break));
    assert!(loop_target.accepts_role(LoopJoinEdgeRoleV1::Continue));
    assert!(!loop_target.accepts_role(LoopJoinEdgeRoleV1::Return));
    assert!(function_exit.accepts_role(LoopJoinEdgeRoleV1::Return));
    assert!(!function_exit.accepts_role(LoopJoinEdgeRoleV1::Break));
    assert!(!function_exit.accepts_role(LoopJoinEdgeRoleV1::Continue));
}

#[test]
fn v2_join_closure_rejects_missing_root_carrier_before_split_truth() {
    let mut recipe = super::mapping::complete_dynamic_loop_recipe_v2();
    recipe.carriers.clear();
    let verified = LoopRecipeVerifierV2::verify(recipe).expect("carrier-free wire verifies");
    assert!(matches!(
        issue_sole_root_carrier_join_closure_v2(&verified),
        Err(LoopJoinClosureRejectV2::RootCarrierCardinality {
            root,
            found: 0,
        }) if root == LoopNodeKeyV1::new(0)
    ));
}

#[test]
fn v2_join_closure_rejects_multiple_root_carriers_before_elaboration() {
    let mut recipe = super::mapping::complete_dynamic_loop_recipe_v2();
    recipe.bindings.push(LoopRecipeBindingV2 {
        key: LoopBindingKeyV1::new(1),
        label: "second".to_owned(),
        class: LoopValueClassV2::Dynamic,
    });
    recipe.values.push(LoopRecipeValueV2 {
        key: LoopValueKeyV1::new(18),
        class: LoopValueClassV2::Dynamic,
    });
    recipe.inputs.push(LoopValueKeyV1::new(18));
    recipe.carriers.push(LoopRecipeCarrierV2 {
        key: LoopCarrierKeyV1::new(1),
        owner_loop: LoopNodeKeyV1::new(0),
        binding: LoopBindingKeyV1::new(1),
        class: LoopValueClassV2::Dynamic,
        entry_value: LoopValueKeyV1::new(18),
    });
    let verified = LoopRecipeVerifierV2::verify(recipe).expect("two-carrier wire verifies");
    assert!(matches!(
        issue_sole_root_carrier_join_closure_v2(&verified),
        Err(LoopJoinClosureRejectV2::RootCarrierCardinality {
            root,
            found: 2,
        }) if root == LoopNodeKeyV1::new(0)
    ));
}

#[test]
fn v2_verifier_rejects_duplicate_and_wrong_class_carriers() {
    let mut duplicate = super::mapping::complete_dynamic_loop_recipe_v2();
    let mut second = duplicate.carriers[0];
    second.key = LoopCarrierKeyV1::new(1);
    duplicate.carriers.push(second);
    assert_eq!(
        LoopRecipeVerifierV2::verify(duplicate),
        Err(LoopRecipeV2RejectReason::DuplicateCarrierBinding {
            loop_key: LoopNodeKeyV1::new(0),
            binding: LoopBindingKeyV1::new(0),
        })
    );

    let mut wrong_class = super::mapping::complete_dynamic_loop_recipe_v2();
    wrong_class.carriers[0].class = LoopValueClassV2::Text;
    assert_eq!(
        LoopRecipeVerifierV2::verify(wrong_class),
        Err(LoopRecipeV2RejectReason::InvalidCarrierBinding {
            key: LoopCarrierKeyV1::new(0),
        })
    );
}

#[test]
fn v2_verifier_rejects_body_local_or_return_value_as_root_carrier_entry() {
    for value in [10, 14] {
        let mut recipe = super::mapping::complete_dynamic_loop_recipe_v2();
        recipe.carriers[0].entry_value = LoopValueKeyV1::new(value);
        assert_eq!(
            LoopRecipeVerifierV2::verify(recipe),
            Err(LoopRecipeV2RejectReason::CarrierEntryNotAvailable {
                key: LoopCarrierKeyV1::new(0),
            })
        );
    }
}

#[test]
fn changed_recipe_backedge_cannot_match_the_unchanged_golden() {
    let mut recipe = super::mapping::complete_dynamic_loop_recipe_v2();
    let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::WriteBinding { value, .. },
    } = &mut recipe.items[16].item
    else {
        panic!("I16 must remain the carrier write")
    };
    *value = LoopValueKeyV1::new(15);
    let verified = LoopRecipeVerifierV2::verify(recipe).expect("different valid Recipe");
    let closure =
        issue_sole_root_carrier_join_closure_v2(&verified).expect("derived JoinSig closure");
    let backedge = closure.join_sig().as_sig().loops[0]
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge)
        .expect("backedge");
    assert_eq!(backedge.payload[0].value, LoopValueKeyV1::new(15));
    assert_ne!(backedge.payload[0].value, LoopValueKeyV1::new(17));
}

fn payload(binding: LoopBindingKeyV1, value: u32) -> LoopJoinPayloadV2 {
    LoopJoinPayloadV2 {
        binding,
        value: LoopValueKeyV1::new(value),
        class: LoopValueClassV2::Dynamic,
    }
}

fn edge(
    from: LoopJoinPortV1,
    to: LoopJoinPortV1,
    role: LoopJoinEdgeRoleV1,
    payload: LoopJoinPayloadV2,
) -> LoopJoinEdgeV2 {
    LoopJoinEdgeV2 {
        from,
        to,
        role,
        payload: vec![payload],
    }
}

fn port_binding(
    loop_key: LoopNodeKeyV1,
    port: LoopJoinPortV1,
    binding: LoopBindingKeyV1,
) -> LoopJoinPortBindingV2 {
    LoopJoinPortBindingV2 {
        loop_key,
        port,
        binding,
        class: LoopValueClassV2::Dynamic,
    }
}
