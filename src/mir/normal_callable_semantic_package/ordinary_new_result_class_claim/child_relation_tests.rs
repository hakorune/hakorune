//! Source candidate and unchanged admission frontier pins.
use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::SourcePathSegmentV1 as P;

const PREFIX: &str = "box Token { value: i64 birth(value) { me.value = value } } box Other { value: i64 birth(value) { me.value = value } } box Holder { child: Token birth(child) { me.child = child } }";
fn source(body: &str) -> String {
    format!("{PREFIX} box Door {{ {body} }} static box Main {{ main() {{ return 0 }} }}")
}
fn facts(source: &str) -> OrdinaryNewResultClassClaimsV1 {
    super::super::source_result_facts_for_test(source)
}

fn rebuild(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> OrdinaryNewResultClassClaimsV1 {
    let mut draft = OrdinaryNewResultClassClaimDraftV1::new();
    for declaration in package.batch.declarations() {
        let slot = declaration.batch_slot();
        let Some(SelectedNormalCallableKeyV1::Cataloged(key)) =
            package.selected.key_for_batch_slot(slot)
        else {
            continue;
        };
        package
            .batch
            .with_lowering_input(slot, |input| draft.observe_function(input, key, slot))
            .unwrap();
    }
    draft.finish(
        package.batch.ordinary_box_coverage(),
        &package.batch,
        &package.selected,
        None,
        &package.ordinary_new_claim_ledger.field_write_claims,
        &package.parameter_contracts,
    )
}

#[test]
fn child_relation_real_page_heap_pins_all_sites_and_retains_prefix_failures() {
    let package = issue(include_str!(
        "../../../../lang/src/hako_alloc/memory/page_heap_box.hako"
    ))
    .unwrap();
    let facts = &package.ordinary_new_claim_ledger.callable_result_classes;
    let relations: Vec<_> = facts.construction_child_relations().collect();
    assert_eq!(relations.len(), 8);
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "reallocResult", 2),
    );
    let slot = package.selected.batch_slot(&key).unwrap();
    let owner = package
        .batch
        .declarations()
        .find(|row| row.batch_slot() == slot)
        .unwrap()
        .owner();
    let mut exact_frontiers = 0;
    for relation in &relations {
        assert_eq!(relation.field().declaration_ordinal(), 2);
        assert_eq!(relation.formal_ordinal(), 2);
        assert_eq!(
            relation.formal_binding().owner(),
            relation.constructor_owner()
        );
        let mut argument = relation.outer().site().node().segments().to_vec();
        argument.push(P::Argument(2));
        assert_eq!(relation.actual().site().node().segments(), argument);
        assert_eq!(relation.actual().owner(), relation.outer().owner());
        assert!(std::ptr::eq(
            *relation,
            facts
                .construction_child_relation(relation.outer(), relation.field())
                .unwrap()
        ));
        let birth = package
            .instance_constructors
            .rows()
            .iter()
            .find(|birth| birth.source_id().same_as(relation.constructor_source()))
            .unwrap();
        assert_eq!(birth.object(), relation.field().object());
        if relation.outer().owner() != owner {
            continue;
        }
        if relation.outer().site().node().segments() == [P::Body(4), P::IfThen(0), P::Value] {
            assert_eq!(relation.witnesses().len(), 1);
            assert_eq!(relation.witnesses()[0].origin(), &ResultValueOriginV1::Null);
            assert!(matches!(
                relation.witnesses()[0].step(),
                ResultWitnessStepV1::NullLiteral
            ));
            exact_frontiers += 1;
        }
        if relation.outer().site().node().segments() == [P::Body(5), P::Value] {
            let origins: BTreeSet<_> = relation
                .witnesses()
                .iter()
                .map(|w| w.origin().clone())
                .collect();
            assert_eq!(
                origins,
                BTreeSet::from([
                    ResultValueOriginV1::Null,
                    ResultValueOriginV1::Fresh("HakoAllocHandle".into()),
                    ResultValueOriginV1::ForwardFormal { ordinal: 0 }
                ])
            );
            for witness in relation.witnesses() {
                assert_eq!(witness.site(), relation.actual());
                let ResultWitnessStepV1::Call { site, key, .. } = witness.step() else {
                    panic!("replacement initializer")
                };
                assert_eq!(key.name(), "realloc");
                assert_ne!(site, relation.actual());
            }
            exact_frontiers += 1;
        }
    }
    assert_eq!(exact_frontiers, 2);
    let claims = package
        .ordinary_new_claim_ledger
        .pending_result_claims_for_test();
    let mut covered = 0;
    let mut uncovered = 0;
    for (_, claim) in claims
        .iter()
        .filter(|(_, claim)| claim.class() == "HakoAllocHandleResult")
    {
        if claim.home_prefix().is_ok() {
            covered += 1;
        } else {
            let Err(crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1::PrefixNotCovered(site)) = claim.home_prefix() else { panic!("same prefix boundary") };
            assert_eq!(site.node().segments(), &[P::Body(3)]);
            uncovered += 1;
        }
    }
    assert_eq!((covered, uncovered), (6, 2));
}

#[test]
fn child_relation_full_facade_does_not_duplicate_callee_constructions() {
    let source = format!(
        "{}\n{}",
        include_str!("../../../../lang/src/hako_alloc/memory/page_heap_box.hako"),
        include_str!("../../../../lang/src/hako_alloc/memory/allocator_facade_box.hako")
    );
    assert_eq!(facts(&source).construction_child_relations().count(), 8);
}

#[test]
fn child_relation_declared_formal_retains_null_and_anchor_provenance_only() {
    let facts = facts(&source("wrap(h: Token) { return new Holder(h) }"));
    let relation = facts.construction_child_relations().next().unwrap();
    assert_eq!(relation.formal_ordinal(), 0);
    assert_eq!(relation.witnesses().len(), 2);
    for witness in relation.witnesses() {
        assert!(matches!(
            witness.step(),
            ResultWitnessStepV1::Formal { ordinal: 0, .. }
        ));
        assert!(!matches!(witness.origin(), ResultValueOriginV1::Fresh(_)));
    }
}

#[test]
fn child_relation_wrong_class_opaque_rebind_and_unsupported_actuals_stay_unavailable() {
    for body in [
        "wrap(h: Other) { return new Holder(h) }",
        "wrap(h) { return new Holder(h) }",
        "wrap(h: Token) { h = h return new Holder(h) }",
        "wrap(h: Token) { return new Holder(new Token(1)) }",
        "wrap(h: Token) { return new Holder(h.value) }",
        "wrap(h: Token) { return new Holder(h[0]) }",
        "cycle(h: Token) { return me.cycle(h) } wrap(h: Token) { local r = me.cycle(h) return new Holder(r) }",
        "mixed(c: i64) { if c == 0 { return new Token(1) } return new Other(2) } wrap(c: i64) { local r = me.mixed(c) return new Holder(r) }",
    ] {
        assert_eq!(facts(&source(body)).construction_child_relations().count(), 0, "{body}");
    }
}

#[test]
fn child_relation_foreign_constructor_source_cannot_issue_same_spelling_field() {
    let source = include_str!("../../../../lang/src/hako_alloc/memory/page_heap_box.hako");
    let package = issue(source).unwrap();
    let foreign = issue(source).unwrap();
    let mut scratch = rebuild(&package);
    attach_source_child_relations_v1(
        &mut scratch,
        &package.batch,
        &package.selected,
        &foreign.instance_constructors,
        &package.ordinary_new_claim_ledger.field_write_claims,
        &package.parameter_contracts,
    );
    assert_eq!(scratch.construction_child_relations().count(), 0);
}

#[test]
fn child_relation_changed_forward_formal_ordinal_rejects_whole_mixed_candidate() {
    let mut package = issue(include_str!(
        "../../../../lang/src/hako_alloc/memory/page_heap_box.hako"
    ))
    .unwrap();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::instance_box_method("HakoAllocHeap", "reallocResult", 2),
    );
    let slot = package.selected.batch_slot(&key).unwrap();
    let owner = package
        .batch
        .declarations()
        .find(|row| row.batch_slot() == slot)
        .unwrap()
        .owner();
    let parameters = package
        .parameter_contracts
        .iter_mut()
        .find(|row| row.batch_slot == slot)
        .unwrap();
    parameters
        .parameters
        .iter_mut()
        .find(|p| p.ordinal == 0)
        .unwrap()
        .ordinal = 9;
    let mut scratch = rebuild(&package);
    attach_source_child_relations_v1(
        &mut scratch,
        &package.batch,
        &package.selected,
        &package.instance_constructors,
        &package.ordinary_new_claim_ledger.field_write_claims,
        &package.parameter_contracts,
    );
    let facts = &scratch;
    assert_eq!(facts.construction_child_relations().count(), 7);
    assert!(!facts
        .construction_child_relations()
        .any(|relation| relation.outer().owner() == owner
            && relation.outer().site().node().segments() == [P::Body(5), P::Value]));
}
