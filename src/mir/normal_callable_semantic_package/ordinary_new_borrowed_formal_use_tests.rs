//! Source-use closure tests; these do not activate a tagged ABI.

use super::*;

fn draft(body: &str) -> Result<BorrowedFormalUsesDraftV1, BorrowedFormalUseDraftErrorV1> {
    draft_with_contract(body, |_| {})
}

fn draft_with_contract(
    body: &str,
    change: impl FnOnce(&mut OwnedCallableParameterContractDeclarationV1),
) -> Result<BorrowedFormalUsesDraftV1, BorrowedFormalUseDraftErrorV1> {
    let source = format!(
        "box BorrowUse {{ birth() {{ }} probe(p): i64 {{ {body} }} \
         sink(q): i64 {{ return 0 }} }} static box Main {{ main() {{ return 0 }} }}"
    );
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::
        issue_with_brand_catalog(&source).expect("source-backed package");
    let contract = package
        .parameter_contracts
        .iter()
        .find(|contract| {
            package
                .batch()
                .with_lowering_input(contract.batch_slot, |input| {
                    contract.parameters.iter().any(|parameter| {
                        input
                            .function()
                            .binding(parameter.binding)
                            .is_some_and(|row| row.diagnostic_name() == "p")
                    })
                })
                .expect("exact declaration loan")
        })
        .expect("probe parameter contract");
    let mut contract = OwnedCallableParameterContractDeclarationV1 {
        batch_slot: contract.batch_slot,
        owner: contract.owner,
        mode: contract.mode,
        parameters: contract.parameters.iter().map(|row| {
            crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                ordinal: row.ordinal,
                binding: row.binding,
                kind: row.kind,
            }
        }).collect(),
    };
    change(&mut contract);
    package
        .batch()
        .with_lowering_input(contract.batch_slot, |input| {
            draft_borrowed_formal_uses_v1(input, &contract, package.instance_constructors(), None)
        })
        .expect("exact source loan")
}

#[test]
fn ignored_opaque_formal_keeps_its_source_origin_without_fabricated_uses() {
    let row = draft("return 0").expect("ignored formal");
    assert_eq!(row.origins.len(), 1);
    assert!(row.uses.is_empty());
    let (binding, origin) = row.origins.iter().next().unwrap();
    assert_eq!(binding, origin);
}

#[test]
fn copies_preserve_actual_binding_and_formal_origin_without_selecting_forward_target() {
    let row = draft("local a = p local b = a local value = me.sink(b) return 0")
        .expect("direct copies and unresolved argument");
    assert_eq!(row.origins.len(), 3);
    assert_eq!(row.uses.len(), 3);
    assert_eq!(
        row.uses
            .iter()
            .filter(|row| matches!(row.kind, BorrowedFormalUseDraftKindV1::Copy { .. }))
            .count(),
        2
    );
    let forwarded = row
        .uses
        .iter()
        .find(|row| {
            matches!(
                row.kind,
                BorrowedFormalUseDraftKindV1::UnresolvedArgument { .. }
            )
        })
        .unwrap();
    assert_ne!(forwarded.binding, forwarded.formal);
    assert_eq!(row.origins[&forwarded.binding], forwarded.formal);
    let BorrowedFormalUseDraftKindV1::UnresolvedArgument { call, ordinal } = &forwarded.kind else {
        unreachable!()
    };
    assert_eq!(*ordinal, 0);
    assert_eq!(call.owner(), forwarded.formal.owner());
    assert_ne!(call, &forwarded.site);
}

#[test]
fn opaque_subexpression_is_not_a_direct_argument_forward() {
    assert!(matches!(
        draft("local value = me.sink(p + 1) return 0"),
        Err(BorrowedFormalUseDraftErrorV1::UnsupportedUse(_))
    ));
}

#[test]
fn opaque_return_receiver_condition_and_store_are_not_transport_uses() {
    for body in [
        "return p",
        "local value = p.sink(0) return 0",
        "if p { return 1 } return 0",
        "me.value = p return 0",
    ] {
        assert!(
            matches!(
                draft(body),
                Err(BorrowedFormalUseDraftErrorV1::UnsupportedUse(_))
            ),
            "unsupported use: {body}"
        );
    }
}

#[test]
fn formal_or_copy_rebinding_rejects_even_when_the_copy_is_unread() {
    for body in ["p = 0 return 0", "local a = p a = 0 return 0"] {
        assert!(
            matches!(draft(body), Err(BorrowedFormalUseDraftErrorV1::Rebound(_))),
            "rebind: {body}"
        );
    }
}

#[test]
fn copied_formal_must_not_acquire_an_integer_annotation() {
    assert!(matches!(
        draft("local a: i64 = p return 0"),
        Err(BorrowedFormalUseDraftErrorV1::AnnotatedAlias(_))
    ));
}

#[test]
fn descendant_capture_reads_and_writes_are_not_ignored_formal_uses() {
    for body in [
        "local closure = fn() { return p } return 0",
        "local closure = fn() { p = 1 return 0 } return 0",
        "local a = p local closure = fn() { return a } return 0",
        "local closure = fn() { local inner = fn() { p = 1 return 0 } return 0 } return 0",
    ] {
        assert!(
            matches!(draft(body), Err(BorrowedFormalUseDraftErrorV1::Captured(_))),
            "capture: {body}"
        );
    }
}

#[test]
fn truncated_or_wrong_ordinal_contract_does_not_make_a_formal_ignored() {
    assert!(matches!(
        draft_with_contract("return 0", |contract| {
            contract.parameters = Box::new([]);
        }),
        Err(BorrowedFormalUseDraftErrorV1::SourceIdentity)
    ));
    assert!(matches!(
        draft_with_contract("return 0", |contract| {
            contract.parameters[0].ordinal = 1;
        }),
        Err(BorrowedFormalUseDraftErrorV1::SourceIdentity)
    ));
}

#[test]
fn lexical_shadowing_does_not_borrow_the_formal_by_name() {
    let row = draft("if true { local p = 1 local value = me.sink(p) } return 0")
        .expect("distinct shadow binding");
    assert_eq!(row.origins.len(), 1);
    assert!(row.uses.is_empty());
}

#[test]
fn checked_compare_admits_direct_if_greater_with_integer_or_origin_sibling() {
    for body in [
        "if p > 5 { return 1 } return 0",
        "if 5 > p { return 1 } return 0",
        "local q = p if p > q { return 1 } return 0",
    ] {
        let row = draft(body).expect("admitted compare operand");
        let compares = row
            .uses
            .iter()
            .filter(|row| {
                matches!(
                    row.kind,
                    BorrowedFormalUseDraftKindV1::CompareOperand { .. }
                )
            })
            .count();
        assert!(compares >= 1, "compare operand admitted: {body}");
    }
    let aliased = draft("local a = p if a > 5 { return 1 } return 0")
        .expect("alias compare operand");
    assert_eq!(aliased.uses.len(), 2);
    let compare = aliased
        .uses
        .iter()
        .find(|row| {
            matches!(
                row.kind,
                BorrowedFormalUseDraftKindV1::CompareOperand { .. }
            )
        })
        .expect("view row");
    assert_ne!(compare.binding, compare.formal);
    assert_eq!(aliased.origins[&compare.binding], compare.formal);
}

#[test]
fn checked_compare_rejects_outside_if_wrong_operator_and_unproved_sibling() {
    for body in [
        "local b = p > 5 return 0",
        "if p >= 5 { return 1 } return 0",
        "if p < 5 { return 1 } return 0",
        "if p > true { return 1 } return 0",
        "local n = 5 if p > n { return 1 } return 0",
        "loop(p > 5) { return 1 } return 0",
    ] {
        assert!(
            matches!(
                draft(body),
                Err(BorrowedFormalUseDraftErrorV1::UnsupportedUse(_))
            ),
            "rejected use: {body}"
        );
    }
}

#[test]
fn dominated_add_admits_guarded_integer_sibling_and_aliased_operands() {
    for body in [
        "if p > 5 { return 1 } local t = p + 1 return t",
        "if p > 5 { return 1 } local t = 1 + p return t",
        "local a = p if a > 5 { return 1 } local t = a + 1 return t",
        "if p > 5 { return 1 } local t = p + p return t",
    ] {
        let row = draft(body).expect("admitted add operand");
        let adds = row
            .uses
            .iter()
            .filter(|row| matches!(row.kind, BorrowedFormalUseDraftKindV1::AddOperand { .. }))
            .count();
        assert!(adds >= 1, "add operand admitted: {body}");
    }
}

#[test]
fn add_operand_rejects_unguarded_undominated_and_unproved_sibling_uses() {
    for body in [
        "local t = p + 1 return t",
        "local t = p + 1 if p > 5 { return 1 } return t",
        "if p > 5 { local t = p + 1 return t } return 0",
        "if p > 5 { return 1 } else { local t = p + 1 return t } return 0",
        "if p >= 5 { return 1 } local t = p + 1 return t",
        "if p > true { return 1 } local t = p + 1 return t",
        "if p > 5 { return 1 } local t = p - 1 return t",
        "if p > 5 { return 1 } local t = p + true return t",
        "local n = 5 if p > 5 { return 1 } local t = p + n return t",
    ] {
        assert!(
            matches!(
                draft(body),
                Err(BorrowedFormalUseDraftErrorV1::UnsupportedUse(_))
            ),
            "rejected add use: {body}"
        );
    }
}

fn forwarding_package(
    mutual: bool,
) -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1 {
    let sink = if mutual {
        "local recv = new BorrowUse() local value = recv.probe(q) return 0"
    } else {
        "return 0"
    };
    let main = if mutual {
        "return 0"
    } else {
        "local recv = new BorrowUse() local out = recv.probe(0) return 0"
    };
    let source = format!(
        "box BorrowUse {{ birth() {{ }} \
         probe(p): i64 {{ local a = p local recv = new BorrowUse() \
             local value = recv.sink(a) return 0 }} \
         sink(q): i64 {{ {sink} }} }} \
         static box Main {{ main() {{ {main} }} }}"
    );
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        &source,
    )
    .expect("exact ordinary-call source")
}

fn drafts_for_package(
    package: &crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1,
) -> BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1> {
    package
        .parameter_contracts
        .iter()
        .filter(|contract| {
            contract
                .parameters
                .iter()
                .any(|formal| formal.kind == CallableParameterContractKindV1::OpaqueHandle)
        })
        .map(|contract| {
            let draft = package
                .batch()
                .with_lowering_input(contract.batch_slot, |input| {
                    draft_borrowed_formal_uses_v1(
                        input,
                        contract,
                        package.instance_constructors(),
                        None,
                    )
                })
                .expect("exact source loan")
                .expect("transport-only uses");
            (contract.owner, draft)
        })
        .collect()
}

#[test]
fn exact_lexical_forward_join_retains_both_formal_identities() {
    let package = forwarding_package(false);
    let drafts = drafts_for_package(&package);
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let calls = slots.iter().filter_map(|(site, slot)| {
        match slot {
            crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some((site.clone(), row.source_target())),
            _ => None,
        }
    }).collect();
    let joined = join_borrowed_forward_uses_v1(&drafts, &package.parameter_contracts, &calls)
        .expect("exact source forward relation");
    assert_eq!(joined.len(), 1);
    let row = &joined[0];
    assert_ne!(row.binding, row.source_formal);
    assert_ne!(row.source_formal.owner(), row.callee_formal.owner());
    assert_eq!(row.target.owner(), "BorrowUse");
    assert_eq!(row.target.name(), "sink");
    assert_eq!(row.ordinal, 0);
    assert_eq!(row.site.owner(), row.call.owner());
}

#[test]
fn finite_mutual_forward_join_does_not_assume_unvisited_owner_success() {
    let package = forwarding_package(true);
    let mut drafts = drafts_for_package(&package);
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let calls = slots.iter().filter_map(|(site, slot)| {
        match slot {
            crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some((site.clone(), row.source_target())),
            _ => None,
        }
    }).collect();
    let joined = join_borrowed_forward_uses_v1(&drafts, &package.parameter_contracts, &calls)
        .expect("both finite source owners covered");
    assert_eq!(joined.len(), 2);
    let removed = joined[0].callee_formal.owner();
    drafts.remove(&removed);
    assert!(matches!(
        join_borrowed_forward_uses_v1(&drafts, &package.parameter_contracts, &calls),
        Err(BorrowedForwardJoinDraftErrorV1::MissingCallee(_))
    ));
}

#[test]
fn unresolved_forward_cannot_obtain_authority_from_its_argument_position() {
    let package = forwarding_package(false);
    let drafts = drafts_for_package(&package);
    assert!(matches!(
        join_borrowed_forward_uses_v1(&drafts, &package.parameter_contracts, &BTreeMap::new()),
        Err(BorrowedForwardJoinDraftErrorV1::MissingCall(_))
    ));
}

#[test]
fn all_incoming_edges_must_be_exact_and_in_the_same_ordinary_scope() {
    let package = forwarding_package(true);
    let drafts = drafts_for_package(&package);
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let calls = slots.iter().filter_map(|(site, slot)| match slot {
        crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some((site.clone(), row.source_target())),
        _ => None,
    }).collect();
    let mut scope = drafts.keys().copied().collect();
    let incoming = draft_borrowed_incoming_calls_v1(
        package.batch(),
        &package.selected,
        &drafts,
        &package.parameter_contracts,
        &calls,
        &scope,
    )
    .expect("exact finite graph");
    assert_eq!(incoming.len(), 2);
    for row in &incoming {
        assert_eq!(row.arguments.len(), 1);
        assert_eq!(row.arguments[0].2.owner(), row.callee);
        assert_ne!(row.call.owner(), row.callee);
    }
    scope.remove(&incoming[0].call.owner());
    assert!(matches!(
        draft_borrowed_incoming_calls_v1(
            package.batch(),
            &package.selected,
            &drafts,
            &package.parameter_contracts,
            &calls,
            &scope
        ),
        Err(BorrowedIncomingDraftErrorV1::OutsideOrdinaryScope(_))
    ));
    assert!(matches!(
        draft_borrowed_incoming_calls_v1(
            package.batch(),
            &package.selected,
            &drafts,
            &package.parameter_contracts,
            &BTreeMap::new(),
            &scope
        ),
        Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))
    ));
}

#[test]
fn ignored_formal_without_any_incoming_edge_does_not_authorize_an_abi_change() {
    let source = "box BorrowUse { birth() { } probe(p): i64 { return 0 } sink(q): i64 { return 0 } } static box Main { main() { return 0 } }";
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let drafts = drafts_for_package(&package);
    let slots = package
        .ordinary_new_claim_ledger
        .lexical_instance_calls
        .borrow();
    let calls = slots.iter().filter_map(|(site, slot)| match slot {
        crate::mir::normal_callable_semantic_package::disposition_slot::DispositionSlotV1::Ready(row) => Some((site.clone(), row.source_target())),
        _ => None,
    }).collect();
    let scope = drafts.keys().copied().collect();
    assert!(matches!(
        draft_borrowed_incoming_calls_v1(
            package.batch(),
            &package.selected,
            &drafts,
            &package.parameter_contracts,
            &calls,
            &scope
        ),
        Err(BorrowedIncomingDraftErrorV1::NoIncoming(_))
    ));
}
