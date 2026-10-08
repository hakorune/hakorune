//! Corruption checks at the source witness composition boundary.
use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};

const SOURCE: &str = "box Token { value: i64 birth(value) { me.value = value } } box Door { give(h: Token) { return h } relay(h: Token) { return me.give(h) } } static box Main { main() { return 0 } }";

#[test]
fn witness_composition_rejects_foreign_call_actual_and_wrong_argument_site() {
    let package = issue(SOURCE).unwrap();
    let foreign = issue(SOURCE).unwrap();
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", "relay", 1);
    let slot = package.batch.declarations().find(|row| matches!(package.selected.key_for_batch_slot(row.batch_slot()), Some(SelectedNormalCallableKeyV1::Cataloged(found)) if found == &key)).unwrap().batch_slot();
    let foreign_owner = foreign.batch.declarations().next().unwrap().owner();
    for corruption in 0..4 {
        package
            .batch
            .with_lowering_input(slot, |input| {
                let source = verified_value_return_sites(input, input.body_shape().unwrap())
                    .unwrap()
                    .remove(0);
                let mut call_site = OwnedExprSiteV1::new(input.owner(), source.clone());
                let (callee, mut actuals) = resolve_call_key(
                    &source,
                    input.function(),
                    input.body_shape().unwrap(),
                    &key,
                    &package.batch,
                    &package.selected,
                    &package.ordinary_new_claim_ledger.field_write_claims,
                )
                .unwrap();
                match corruption {
                    0 => call_site = OwnedExprSiteV1::new(foreign_owner, source.clone()),
                    1 => {
                        actuals[0].site =
                            OwnedExprSiteV1::new(foreign_owner, actuals[0].site.site().clone())
                    }
                    2 => {
                        let mut path = source.node().segments().to_vec();
                        path.push(SourcePathSegmentV1::Argument(9));
                        actuals[0].site = OwnedExprSiteV1::new(
                            input.owner(),
                            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(path)),
                        );
                    }
                    3 => actuals[0].binding = None,
                    _ => unreachable!(),
                }
                let exits = [PendingResultExitV1 {
                    site: OwnedExprSiteV1::new(input.owner(), source),
                    exit: PendingExitV1::Fwd {
                        call_site,
                        key: callee,
                        actuals,
                    },
                }];
                assert!(matches!(
                    evaluate_row(
                        &exits,
                        &package.ordinary_new_claim_ledger.callable_result_classes,
                        &BTreeSet::new(),
                        &package.parameter_contracts,
                        slot,
                        package.batch.ordinary_box_coverage()
                    ),
                    ExitVerdictV1::Dead
                ));
                let PendingExitV1::Fwd { key, .. } = &exits[0].exit else {
                    unreachable!("forwarded source")
                };
                let pending = BTreeSet::from([key.clone()]);
                let unresolved = evaluate_row(
                    &exits,
                    &OrdinaryNewResultClassClaimsV1::new(),
                    &pending,
                    &package.parameter_contracts,
                    slot,
                    package.batch.ordinary_box_coverage(),
                );
                if corruption < 3 {
                    assert!(matches!(unresolved, ExitVerdictV1::Dead));
                } else {
                    assert!(matches!(unresolved, ExitVerdictV1::Waiting));
                }
            })
            .unwrap();
    }
}

#[test]
fn source_witness_rejects_formal_ordinal_disagreement() {
    let package = issue(SOURCE).unwrap();
    let key = CanonicalSameModuleCallableKeyV1::instance_box_method("Door", "give", 1);
    let slot = package.batch.declarations().find(|row| matches!(package.selected.key_for_batch_slot(row.batch_slot()), Some(SelectedNormalCallableKeyV1::Cataloged(found)) if found == &key)).unwrap().batch_slot();
    let mut draft = OrdinaryNewResultClassClaimDraftV1::new();
    package
        .batch
        .with_lowering_input(slot, |input| draft.observe_function(input, &key, slot))
        .unwrap();
    let ResultClassExitDraftV1::ForwardFormal { ordinal, .. } = &mut draft.rows[0].exits[0].1
    else {
        panic!("formal draft")
    };
    *ordinal = 9;
    let facts = draft.finish(
        package.batch.ordinary_box_coverage(),
        &package.batch,
        &package.selected,
        None,
        &package.ordinary_new_claim_ledger.field_write_claims,
        &package.parameter_contracts,
    );
    assert!(facts.outcomes(&key).is_none());
}

/// Exercise the production source-preparation phase before unrelated ingress.
/// No full package, field permission or executable is issued by this test loan.
pub(in crate::mir::normal_callable_semantic_package) fn source_result_facts_for_test(
    source: &str,
) -> OrdinaryNewResultClassClaimsV1 {
    with_source_claims_fixture(source, |batch, selected, constructors, parameters, main| {
        let (_, _, result, _) = super::super::coseal_issue::source_claims::prepare_source_claims(
            batch,
            selected,
            main,
            constructors,
            parameters,
        )
        .unwrap();
        result
    })
}

fn with_source_claims_fixture<R>(
    source: &str,
    test: impl FnOnce(
        &VerifiedResolvedCallableSemanticBatchV1,
        &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
        &crate::mir::normal_callable_semantic_package::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1,
        &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
        Option<&super::super::lexical_instance_call::BorrowedAppMainSourceLoanV1<'_>>,
    ) -> R,
) -> R {
    use crate::mir::builder::{
        issue_source_backed_same_module_callable_catalog_v1, NormalRootExecutionConsumerV1,
    };
    use crate::mir::callable_semantic_batch::issue_resolved_callable_semantic_batch_with_freestatic_targets_v1;
    use crate::mir::normal_callable_semantic_package::{
        instance_constructor_semantic, model, qualified_static_call_claim, selected_mapping,
    };
    use crate::parser::{NyashParser, ParserBuildConfig};
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source-backed fixture")
    };
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .unwrap()
        .into_consumed_source();
    let catalog = issue_source_backed_same_module_callable_catalog_v1(&source).unwrap();
    let static_claims = qualified_static_call_claim::QualifiedStaticCallClaimIndexV1::issue(
        catalog.catalog(),
        std::iter::empty(),
    )
    .unwrap();
    let mut resolver =
        crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(93).unwrap();
    let constructors = instance_constructor_semantic::issue_instance_constructor_semantic_batch_v1(
        &mut resolver,
        source.source(),
        Some(&brands),
        &static_claims,
    )
    .unwrap();
    let (batch, _) = source
        .consume_into_semantic_package(|source, root| {
            issue_resolved_callable_semantic_batch_with_freestatic_targets_v1(
                &mut resolver,
                source,
                Some(&brands),
            )
            .map(|batch| (batch, root))
        })
        .unwrap();
    let selected =
        selected_mapping::issue_selected_callable_batch_map_v1(&catalog, &batch).unwrap();
    let main = super::super::lexical_instance_call::borrow_app_main_source_v1(
        &batch,
        catalog.catalog().source_backed_app_main(),
    )
    .unwrap();
    let parameter_catalog =
        crate::mir::callable_parameter_contract::issue_callable_parameter_contract_v1(&batch)
            .unwrap();
    let parameters: Vec<_> = parameter_catalog
        .declarations()
        .map(|row| model::OwnedCallableParameterContractDeclarationV1 {
            batch_slot: row.batch_slot(),
            owner: row.owner(),
            mode: row.mode(),
            parameters: row
                .parameters()
                .iter()
                .map(|p| model::OwnedCallableParameterContractV1 {
                    ordinal: p.ordinal(),
                    binding: p.binding(),
                    kind: p.kind(),
                })
                .collect(),
        })
        .collect();
    test(&batch, &selected, &constructors, &parameters, main.as_ref())
}

#[path = "main_source_tests.rs"]
mod main_source_tests;
