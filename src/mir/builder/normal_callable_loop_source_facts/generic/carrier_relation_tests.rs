use std::collections::{BTreeMap, BTreeSet};

use super::carrier_relation::CallableLoopCarrierRelationRejectV1 as Reject;
use super::*;
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopBindingProjectionDispositionV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationRootLineageV1;
use crate::mir::builder::raw_loop_child_entry::PreparedLocatedRawLoopChildEntryV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableFunctionSyntaxViewV1, FunctionOwnerIssuerV1,
    FunctionSemanticResolverSessionV1, ResolveSelectedCallableForestsOutcomeV1,
    SourceBindingSiteV1, SourceBodyKindV1, SourcePathV1,
};
use crate::parser::NyashParser;
use hakorune_mir_core::BindingId;

// Exact declaration in the older structural test witness, whose local is unread.
pub(super) fn unread_tmp_declaration(
    owner: FunctionOwnerIdV1,
) -> BTreeMap<BindingRefV1, SourceBindingSiteV1> {
    BTreeMap::from([(
        BindingRefV1::new(owner, BindingId::new(1)),
        SourceBindingSiteV1::Local {
            statement: SourcePathV1::root_body(0)
                .child(SourcePathSegmentV1::LoopBody(0))
                .stmt(),
            ordinal: 0,
        },
    )])
}

const IF_SOURCE: &str = r#"
function probe(limit) {
    local i = 0
    local sum = 0
    loop(i < limit) {
        local tmp = 1
        tmp = tmp + 1
        if i < 2 { sum = sum + tmp } else { sum = sum + 1 }
        i = i + 1
    }
    return sum
}
"#;

fn direct_source() -> String {
    IF_SOURCE.replace(
        "if i < 2 { sum = sum + tmp } else { sum = sum + 1 }",
        "sum = sum + tmp\n        sum = sum + 1",
    )
}

fn with_disposition<R>(
    source: &str,
    f: impl for<'source> FnOnce(CallableGenericLoopSourceFactsDispositionV1<'source>) -> R,
) -> R {
    let ASTNode::Program { mut statements, .. } = NyashParser::parse_from_string(source).unwrap()
    else {
        panic!("program")
    };
    let function = statements.remove(0);
    let ASTNode::FunctionDeclaration { body, .. } = &function else {
        panic!("function")
    };
    let loop_node = body[2].clone();
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function).unwrap();
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .unwrap()
    else {
        panic!("resolved fixture")
    };
    let forest = forests.into_vec().pop().unwrap();
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .unwrap();
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .unwrap();
    let owner = input.owner();
    let state = CallableSemanticLoweringState::from_exact_source(input).unwrap();
    let loop_site = SourcePathV1::root_body(2).node();
    let disposition = state
        .loop_binding_source_projection()
        .project_disposition(loop_site.clone())
        .unwrap();
    assert!(matches!(
        disposition,
        CallableLoopBindingProjectionDispositionV1::ReadyWithBodyOnly(_)
    ));
    let parent = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: loop_site,
        body_kind: Some(SourceBodyKindV1::Function),
    };
    let prepared =
        PreparedLocatedRawLoopChildEntryV1::prepare(&parent, loop_node, Some(disposition)).unwrap();
    let policy =
        GenericLoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true);
    let payload = prepared
        .into_callable_generic_loop_source_facts_payload(owner, "probe", false, false, policy)
        .unwrap();
    f(CallableGenericLoopSourceFactsIssuerV1::issue_once(payload))
}

fn with_receipt<R>(
    source: &str,
    f: impl for<'source> FnOnce(CallableGenericLoopSourceFactsReceiptV1<'source>) -> R,
) -> R {
    with_disposition(source, |disposition| match disposition {
        CallableGenericLoopSourceFactsDispositionV1::Ready(ready) => f(ready.claim_all().unwrap()),
        other => panic!("expected selected GenericLoop source: {other:?}"),
    })
}

#[test]
fn conditional_update_if_selects_loop_cond_and_requires_source_identity() {
    with_disposition(IF_SOURCE, |disposition| {
        assert!(
            matches!(
                disposition,
                CallableGenericLoopSourceFactsDispositionV1::RouteNotFrontSelected(
                    CallableGenericLoopSourceFactsRouteErrorV1::LoopCondRouteRejected(
                        CallableLoopSourceRouteRejectV1::SourceIdentityMissing
                    )
                )
            ),
            "unexpected If selection: {disposition:?}"
        );
    });
}

#[test]
fn natural_source_coseals_induction_extra_carrier_and_local_rebind() {
    with_receipt(&direct_source(), |receipt| {
        let local_bindings = receipt
            .pre_effect()
            .local_declarations()
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let recipe = receipt.into_semantic_recipe().unwrap();
        recipe
            .with_source_relation_view_once(|view| {
                let relation = view.carrier_relation();
                assert_eq!(relation.carriers().len(), 2);
                assert_eq!(relation.assignment_bindings().len(), 4);
                let induction = relation
                    .carriers()
                    .iter()
                    .find(|row| row.slot() == relation.induction())
                    .unwrap();
                assert_eq!(induction.label(), "i");
                assert_eq!(induction.targets().len(), 1);
                assert_eq!(
                    induction.targets()[0],
                    SourcePathV1::root_body(2)
                        .child(SourcePathSegmentV1::LoopBody(4))
                        .child(SourcePathSegmentV1::Target)
                        .node()
                );
                let extra = relation
                    .carriers()
                    .iter()
                    .find(|row| row.slot() != relation.induction())
                    .unwrap();
                assert_eq!(extra.label(), "sum");
                assert_eq!(extra.targets().len(), 2);
                assert_ne!(extra.binding(), induction.binding());
                assert!(relation
                    .carriers()
                    .iter()
                    .all(|row| !local_bindings.contains(&row.binding())));
                assert_eq!(
                    relation
                        .assignment_bindings()
                        .values()
                        .filter(|binding| local_bindings.contains(binding))
                        .count(),
                    1
                );
            })
            .unwrap();
    });
}

#[test]
fn source_local_shadow_does_not_become_an_extra_loop_carrier() {
    let source = direct_source()
        .replace("local tmp = 1", "local sum = 1")
        .replace("tmp = tmp + 1", "sum = sum + 1")
        .replace("sum = sum + tmp", "sum = sum + 2");
    with_receipt(&source, |receipt| {
        let local_bindings = receipt
            .pre_effect()
            .local_declarations()
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let recipe = receipt.into_semantic_recipe().unwrap();
        recipe
            .with_source_relation_view_once(|view| {
                assert_eq!(view.carrier_relation().carriers().len(), 1);
                assert!(view
                    .carrier_relation()
                    .carriers()
                    .iter()
                    .all(|row| !local_bindings.contains(&row.binding())));
            })
            .unwrap();
    });
}

#[test]
fn rejects_target_declaration_and_induction_drift_before_physical_work() {
    with_receipt(&direct_source(), |mut receipt| {
        receipt
            .outcome
            .facts
            .as_mut()
            .unwrap()
            .facts
            .generic_loop_v1
            .as_mut()
            .unwrap()
            .body
            .body
            .pop();
        assert_eq!(
            receipt.into_semantic_recipe().unwrap_err(),
            CallableGenericLoopV1SemanticRecipeRejectV1::CarrierRelation(Reject::TargetCoverage)
        );
    });
    with_receipt(&direct_source(), |mut receipt| {
        let generic = receipt
            .outcome
            .facts
            .as_mut()
            .unwrap()
            .facts
            .generic_loop_v1
            .as_mut()
            .unwrap();
        let ASTNode::Local { variables, .. } = &mut generic.body.body[0] else {
            panic!("local")
        };
        variables.clear();
        assert_eq!(
            receipt.into_semantic_recipe().unwrap_err(),
            CallableGenericLoopV1SemanticRecipeRejectV1::CarrierRelation(
                Reject::DeclarationCoverage
            )
        );
    });
    with_receipt(&direct_source(), |mut receipt| {
        receipt
            .outcome
            .facts
            .as_mut()
            .unwrap()
            .facts
            .generic_loop_v1
            .as_mut()
            .unwrap()
            .increment_index = Some(1);
        assert_eq!(
            receipt.into_semantic_recipe().unwrap_err(),
            CallableGenericLoopV1SemanticRecipeRejectV1::CarrierRelation(Reject::InductionMismatch)
        );
    });
}

#[test]
fn rejects_foreign_owner_before_slot_issuance() {
    with_receipt(&direct_source(), |mut receipt| {
        receipt.owner = FunctionOwnerIssuerV1::new_for_compilation()
            .unwrap()
            .issue()
            .unwrap();
        assert_eq!(
            receipt.into_semantic_recipe().unwrap_err(),
            CallableGenericLoopV1SemanticRecipeRejectV1::CarrierRelation(Reject::ForeignOwner)
        );
    });
}

#[test]
fn source_carrier_projection_rejects_missing_physical_label() {
    with_receipt(&direct_source(), |receipt| {
        receipt
            .into_semantic_recipe()
            .unwrap()
            .with_source_relation_view_once(|view| {
                let binding = view.carrier_relation().induction_binding().unwrap();
                let error = view
                    .carrier_relation()
                    .physical_value_for_binding(binding, &BTreeMap::new())
                    .unwrap_err();
                assert!(
                    error.contains("source-carrier-physical-missing"),
                    "unexpected missing physical carrier error: {error}"
                );
            })
            .unwrap();
    });
}

#[test]
fn source_carrier_projection_keeps_non_carrier_binding_unmapped() {
    with_receipt(&direct_source(), |receipt| {
        receipt
            .into_semantic_recipe()
            .unwrap()
            .with_source_relation_view_once(|view| {
                let foreign_binding = BindingRefV1::new(view.owner(), BindingId::new(999_999));
                assert_eq!(
                    view.carrier_relation()
                        .physical_value_for_binding(foreign_binding, &BTreeMap::new())
                        .unwrap(),
                    None
                );
            })
            .unwrap();
    });
}

pub(in crate::mir::builder) fn source_final_values_for_test(
) -> crate::mir::builder::control_flow::plan::CoreLoopFinalValuesV1 {
    with_receipt(&direct_source(), |receipt| {
        receipt
            .into_semantic_recipe()
            .unwrap()
            .with_source_relation_view_once(|view| {
                let values = view
                    .carrier_relation()
                    .carriers()
                    .iter()
                    .enumerate()
                    .map(|(i, row)| (row.slot(), crate::mir::ValueId::new(100 + i as u32)))
                    .collect::<Vec<_>>();
                crate::mir::builder::control_flow::plan::CoreLoopFinalValuesV1::from_source_view(
                    &view, &values,
                )
                .unwrap()
            })
            .unwrap()
    })
}

#[test]
fn source_final_values_reject_missing_and_duplicate_slots() {
    with_receipt(&direct_source(), |receipt| {
        receipt
            .into_semantic_recipe()
            .unwrap()
            .with_source_relation_view_once(|view| {
                use crate::mir::builder::control_flow::plan::CoreLoopFinalValuesV1;
                assert!(CoreLoopFinalValuesV1::from_source_view(&view, &[])
                    .unwrap_err()
                    .contains("slot-coverage"));
                let slot = view.carrier_relation().induction();
                assert!(CoreLoopFinalValuesV1::from_source_view(
                    &view,
                    &[
                        (slot, crate::mir::ValueId::new(100)),
                        (slot, crate::mir::ValueId::new(101))
                    ]
                )
                .unwrap_err()
                .contains("duplicate-slot"));
            })
            .unwrap()
    });
}
