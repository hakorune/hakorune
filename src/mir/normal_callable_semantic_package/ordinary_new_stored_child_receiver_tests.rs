//! Exact stored receiver identity stays shared by incoming and result proofs.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::{
    field_write_claim, lexical_instance_call_tests::call_site_of,
};

#[test]
fn stored_child_receiver_corroboration_rejects_identity_drift() {
    let text = "box Item { value: i64 birth() { me.value = 5 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p: Item) { if p == null { return 7 } return p.value } }
        box Parent { left: Leaf right: Leaf
            birth() { me.left = new Leaf() me.right = new Leaf() }
            first(p: Item) { return me.left.read(p) }
            second(p: Item) { return me.right.read(p) } }
        static box Main { main() { local parent = new Parent() local item = new Item()
            local ignored = parent.second(item) return parent.first(item) } }";
    for change in 0..7 {
        let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text)
            .expect("healthy source-issued stored receiver");
        let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let flag_source = ledger.field_write_claims.value_source("Leaf", "flag")
            .unwrap().as_ref().expect("original flag writer/store loan");
        assert!(flag_source.same_inventory(&ledger.field_write_claims));
        let selected_owner = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap()
            .incoming.iter().find(|row| row.source.stored_receiver().is_some())
            .expect("original stored scalar incoming").callee;
        let callable = ledger.callable_scalar_result_source_v1(selected_owner).unwrap()
            .expect("original scalar participant");
        assert_eq!(callable.kind(), crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer);
        assert!(required_scalar_source_walk_v1(ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_i64_results, selected_owner, &ledger.callable_result_classes).unwrap());
        let original_result = ledger.borrowed_i64_results.remove(&selected_owner).unwrap();
        let missing = ledger.callable_scalar_result_source_v1(selected_owner).unwrap_err();
        assert!(missing.contains("scalar-loan/result-missing"), "{missing}");
        let (_, walk_missing) = required_scalar_source_walk_v1(ledger.borrowed_formal_source.as_ref().unwrap(),
            &ledger.borrowed_i64_results, selected_owner, &ledger.callable_result_classes).unwrap_err();
        assert!(walk_missing.contains("scalar-loan/result-missing"), "{walk_missing}");
        ledger.borrowed_i64_results.insert(selected_owner, original_result);
        let borrowed = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let mut prepared: Vec<_> = borrowed
            .incoming
            .iter()
            .map(|row| Ok(Some(row.source.require_instance().unwrap().clone())))
            .collect();
        borrowed.corroborate_source_targets(&prepared).unwrap();
        if change == 6 {
            let source = ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap();
            let incoming = source.incoming.iter_mut().find(|row| row.source.stored_receiver().is_some()).unwrap();
            incoming.source.reissue_stored_origin_for_test();
            assert!(source.corroborate_source_targets(&prepared).unwrap_err()
                .contains("borrowed-formal/final-original-incoming-drift"));
            continue;
        }
        let indices: Vec<_> = prepared
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                row.as_ref()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .stored_receiver()
                    .map(|_| i)
            })
            .collect();
        assert_eq!(indices.len(), 2, "both original stored terminals");
        // The shared terminal lender and packet row retain the original facts Rc.
        let original = prepared[indices[0]].as_ref().unwrap().as_ref().unwrap();
        let terminal = ledger.call_relations_for_owner(original.call_site().owner())
            .into_iter().find(|terminal| terminal.call_site() == original.call_site().site())
            .expect("original stored terminal");
        let source = ledger.verified_terminal_call_source_v1(
            original.call_site().owner(), terminal.return_site(),
        ).unwrap().unwrap();
        let expected = terminal.source_scalar().expect("stored scalar result loan");
        let mut slots = ledger.lexical_instance_calls.borrow_mut();
        let LexicalInstanceCallDispositionSlotV1::Ready(row) = slots.get_mut(original.call_site()).unwrap()
        else { panic!("ready original I64 row"); };
        assert_eq!(row.scalar_result_source_v1(), Some(expected));
        assert_eq!(source.result(), InvokeCallResultKind::I64);
        source.corroborate_row(row).unwrap();
        let retained = row.scalar_result.take();
        assert!(source.corroborate_row(row).unwrap_err().contains("scalar-source-identity"));
        row.scalar_result = retained;
        let kind = row.result.replace(InvokeCallResultKind::Bool);
        assert!(source.corroborate_row(row).unwrap_err().contains("row-identity"));
        row.result = kind;
        drop(slots);
        let sibling = prepared[indices[1]]
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone();
        let target = prepared[indices[0]].as_mut().unwrap().as_mut().unwrap();
        let owner = target.call_site().owner();
        let LexicalInstanceCallReceiverV1::StoredOwnedChild {
            parent_binding,
            parent_site,
            field,
            child,
        } = &mut target.receiver
        else {
            panic!("stored receiver")
        };
        let (other_binding, _, other_field, _) = sibling.stored_receiver().unwrap();
        match change {
            0 => *field = other_field,
            1 => *child = field.object(),
            2 => *parent_binding = other_binding,
            3 => *parent_site = target.receiver_site.clone(),
            4 => target.receiver_site = target.argument_sites[0].clone(),
            _ => target.target_batch_slot += 1,
        }
        assert!(
            borrowed
                .corroborate_source_targets(&prepared)
                .unwrap_err()
                .contains("borrowed-formal/final-incoming-drift"),
            "change={change}"
        );
        ledger.corroborate_borrowed_result_cohort_v1(&prepared, &package.result_contracts);
        assert!(
            ledger.borrowed_i64_results[&owner]
                .as_ref()
                .unwrap_err()
                .contains("borrowed-result/result-contract-mismatch"),
            "change={change}"
        );
    }
}

#[test]
fn passive_stored_dispatch_excludes_only_exact_different_targets() {
    use super::source::{
        CallTargetReferenceV1, PreparedSourceCallNeedV1, PreparedSourceNeedsV1,
        StoredReceiverSourceV1,
    };
    use crate::mir::resolved_semantics::BodyExpressionShapeV1;
    use super::borrowed_formal_uses::{
        inventory_borrowed_incoming_with_stored_dispatch_v1, BorrowedIncomingDraftErrorV1,
    };
    use std::collections::{BTreeMap, BTreeSet};
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Item { value: i64 birth() { me.value = 5 } }
        box Leaf { flag: i64 birth() { me.flag = 0 }
            read(p: Item) { if p == null { return 7 } return p.value } }
        box Parent { left: Leaf right: Leaf
            birth() { me.left = new Leaf() me.right = new Leaf() }
            first(p: Item) { return me.left.read(p) }
            second(p: Item) { return me.right.read(p) } }
        box Other { read(p) { return 0 } }
        static box Main { main() { local parent = new Parent() local item = new Item()
            local other = new Other() local z = other.read(7)
            local ignored = parent.second(item) return parent.first(item) } }").unwrap();
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let mut passive = Vec::new();
    let mut calls = BTreeMap::new();
    let mut leaf_owner = None;
    for row in source.incoming.iter() {
        let target = row.source.require_instance().unwrap();
        if let Some((parent_binding, parent_site, _, _)) = target.stored_receiver() {
            leaf_owner = Some(target.callee_owner());
            let caller = package
                .parameter_contracts
                .iter()
                .find(|c| c.owner == row.call.owner())
                .unwrap();
            let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller_key) = package
                .selected
                .key_for_batch_slot(caller.batch_slot)
                .unwrap()
            else {
                panic!("instance caller")
            };
            let field_name = package
                .batch()
                .with_lowering_input(caller.batch_slot, |input| {
                    let Some(BodyExpressionShapeV1::FieldAccess { field, .. }) = input
                        .body_shape()
                        .unwrap()
                        .expression_shape(target.receiver_site())
                    else {
                        panic!("field source")
                    };
                    field.clone()
                })
                .unwrap();
            passive.push((
                CallTargetReferenceV1::from_target(target),
                StoredReceiverSourceV1 {
                    origin: super::source::StoredReceiverSourceOriginV1::Ordinary,
                    parent_new: None,
                    parent_binding,
                    parent_site: parent_site.clone(),
                    parent_class: caller_key.owner().into(),
                    field_name,
                    child_class: target.target().owner().into(),
                },
            ));
        } else {
            calls.insert(row.call.clone(), target);
        }
    }
    assert_eq!(passive.len(), 2);
    let other_key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
        "Other", "read", 1,
    );
    let other = package
        .parameter_contracts
        .iter()
        .find(|c| {
            matches!(package.selected.key_for_batch_slot(c.batch_slot),
            Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(k)) if k == &other_key)
        })
        .unwrap()
        .owner;
    let scope = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .collect();
    let scan = |needs: Option<&PreparedSourceNeedsV1>| {
        inventory_borrowed_incoming_with_stored_dispatch_v1(
            package.batch(),
            &package.selected,
            &BTreeMap::new(),
            &source.definitions,
            &package.parameter_contracts,
            &calls,
            &scope,
            None,
            needs,
            Some(&package.instance_constructors),
            None,
        )
    };
    let prepared =
        |pairs: Vec<(CallTargetReferenceV1, StoredReceiverSourceV1)>| -> PreparedSourceNeedsV1 {
            Ok(pairs
                .into_iter()
                .map(|(reference, receiver)| {
                    Ok(Some(PreparedSourceCallNeedV1::Stored {
                        reference,
                        receiver,
                        owning_object_input: None,
                        result_requirement: super::LexicalCallSourceResultRequirementV1::ExistingBorrowedResult,
                    }))
                })
                .collect())
        };
    let missing = scan(None).unwrap();
    assert!(matches!(
        missing.project(&BTreeSet::from([other])),
        Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))
    ));
    let original = prepared(passive.clone());
    let checked = scan(Some(&original)).unwrap();
    let rows = checked.project(&BTreeSet::from([other])).unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].source.target(), &other_key);
    assert!(matches!(
        checked.project(&BTreeSet::from([leaf_owner.unwrap()])),
        Err(BorrowedIncomingDraftErrorV1::UnresolvedCaller(_))
    ));
    // Pending is a complete original source observation, never a ready call.
    let pending: Vec<_> = checked.exact_rows().filter(|row| row.callee == leaf_owner.unwrap())
        .cloned().collect();
    assert_eq!(pending.len(), 2);
    for row in &pending {
        assert!(row.source.is_pending());
        assert!(row.source.as_loan().is_err());
        assert!(row.source.instance().is_none());
        assert_eq!(row.source.argument_sites().len(), 1);
        assert!(row.source.object_return_sources().is_none());
    }
    let mut absent = scan(Some(&original)).unwrap();
    let absent_pending: Vec<_> = absent.exact_rows().filter(|row| row.source.is_pending()).cloned().collect();
    absent.finish_selected_calls_v1(&calls).unwrap();
    for row in &absent_pending {
        let retained = absent.exact_rows().find(|actual| actual.call == row.call).unwrap();
        assert!(retained.source.same_source(&row.source), "missing selected target keeps the original Rc and args");
    }
    assert!(absent.project(&BTreeSet::from([leaf_owner.unwrap()])).is_err());
    assert!(absent.vetoed_owners().contains(&leaf_owner.unwrap()));
    let all_calls: BTreeMap<_, _> = source.incoming.iter().map(|row|
        (row.call.clone(), row.source.require_instance().unwrap())
    ).collect();
    let mut finished = scan(Some(&original)).unwrap();
    let original_pending: Vec<_> = finished.exact_rows().filter(|row| row.source.is_pending()).cloned().collect();
    finished.finish_selected_calls_v1(&all_calls).unwrap();
    let completed = finished.project(&BTreeSet::from([leaf_owner.unwrap()])).unwrap();
    assert_eq!(completed.len(), 2);
    for row in &completed {
        let original = original_pending.iter().find(|original| original.call == row.call).unwrap();
        assert!(matches!((&original.source, &row.source),
            (super::borrowed_formal_uses::BorrowedIncomingSourceV1::PendingStored(before),
             super::borrowed_formal_uses::BorrowedIncomingSourceV1::SelectedStored { original: after, .. })
                if std::rc::Rc::ptr_eq(before, after)));
        assert!(row.source.as_loan().is_ok());
    }
    let first = &passive[0].0.call_site;
    for change in 0..3 {
        let mut drift = (*all_calls[first]).clone();
        match change {
            0 => drift.target_batch_slot += 1,
            1 => drift.receiver = super::LexicalInstanceCallReceiverV1::Lexical(passive[0].1.parent_binding),
            _ => drift.result_requirement = super::LexicalCallSourceResultRequirementV1::ObjectProducerDependency(Box::new([])),
        }
        let mut targets = all_calls.clone(); targets.insert(first.clone(), &drift);
        let mut incoming = scan(Some(&original)).unwrap();
        assert!(incoming.finish_selected_calls_v1(&targets).is_err(), "selected target drift {change}");
    }
    let mut duplicate = scan(Some(&original)).unwrap();
    duplicate.duplicate_object_row_for_test(first);
    assert!(duplicate.finish_selected_calls_v1(&all_calls).is_err());
    for change in 0..7 {
        let mut pairs = passive.clone();
        match change {
            0 => pairs.push(pairs[0].clone()),
            1 => pairs[0].0.receiver_site = pairs[0].0.argument_sites[0].clone(),
            2 => pairs[0].1.field_name = "missing".into(),
            3 => pairs[0].0.target_batch_slot += 1,
            4 => pairs[0].0.argument_sites[0] = pairs[0].0.receiver_site.clone(),
            5 => pairs[0].1.child_class = "Other".into(),
            _ => pairs[0].1.parent_site = pairs[0].0.receiver_site.clone(),
        }
        let changed = prepared(pairs);
        assert!(scan(Some(&changed)).is_err(), "identity change {change}");
    }
}

/// A field written with a non-`new` value carries no class claim:
/// `me.inner = 7` vetoes `inner`, so the loop call stays unarmed.
#[test]
fn lexical_instance_call_keeps_non_new_field_write_unarmed() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Outer {
    init { inner }
    birth() { me.inner = 7 }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        return o.run()
    }
}
"#,
    )
    .expect("non-new field write source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let writes = ledger.field_write_claims.writes();
    let [write] = writes else { panic!("retain the original non-object writer: {writes:?}") };
    assert_eq!(write.owner_box(), Some("Outer"));
    assert_eq!(write.field(), "inner");
    assert!(write.is_attributed() && write.in_birth());
    let birth = package.instance_constructors.rows().iter().find(|row| row.box_name() == "Outer")
        .expect("original Outer Birth");
    assert_eq!(write.birth_source(), Some(birth.source_id()));
    let plan = write.source().expect("literal RHS original plan");
    assert_eq!(plan.owner(), write.owner());
    assert_eq!(plan.root().statement(), write.assignment().statement_site());
    assert_eq!(plan.nodes()[0].site(), write.assignment().value_site());
    assert!(!ledger.field_write_claims.has_hidden_writers());
    assert!(!ledger.field_write_claims.has_unattributed_field("inner"));

    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "a non-new field write must not carry a class claim"
    );
}

/// Two writers disagreeing on the stored class veto the field:
/// `birth` stores `new Inner()` but `swap` stores `new Other()`.
#[test]
fn lexical_instance_call_vetoes_multi_class_field_writers() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Other {
    birth() { }
}
box Outer {
    init { inner }
    birth() { me.inner = new Inner() }
    swap() { me.inner = new Other() }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        return o.run()
    }
}
"#,
    )
    .expect("multi-class field writer source package");
    let ledger = &package.ordinary_new_claim_ledger;
    let writes = ledger.field_write_claims.writes();
    assert_eq!(writes.len(), 2, "retain both disagreeing original writers");
    assert_eq!(writes.iter().filter(|write| write.in_birth()).count(), 1);
    for write in writes {
        assert_eq!(write.owner_box(), Some("Outer"));
        assert_eq!(write.field(), "inner");
        assert!(write.is_attributed());
        assert_eq!(write.source().unwrap().owner(), write.owner());
    }

    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "disagreeing field writers must veto the field claim"
    );
}

/// A field name written through a receiver we cannot attribute vetoes
/// every same-named claim: `o.inner = n` in `main` is not a `me.` write,
/// so `inner` carries no provenance even though `birth` wrote `new`.
#[test]
fn lexical_instance_call_vetoes_unattributed_field_write() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        r#"
box Inner {
    birth() { }
    seal(): i64 { return 1 }
}
box Outer {
    init { inner }
    birth() { me.inner = new Inner() }
    run(): i64 {
        local inner = me.inner
        local i = 0
        loop(i < 2) {
            local r = inner.seal()
            i = i + 1
        }
        return i
    }
}
static box Main {
    main() {
        local o = new Outer()
        local n = new Inner()
        o.inner = n
        return o.run()
    }
}
"#,
    )
    .expect("unattributed field write source package");
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(ledger.field_write_claims.has_unattributed_field("inner"));
    assert!(matches!(ledger.field_write_claims.value_source("Outer", "inner"),
        Some(Err(field_write_claim::FieldValueSourceIssueV1::UnattributedWriter))));
    assert_eq!(ledger.field_write_claims.writes().len(), 2);
    let foreign = ledger.field_write_claims.writes().iter().find(|write| !write.is_attributed())
        .expect("foreign writer is retained rather than erased");
    assert_eq!(foreign.field(), "inner");
    assert!(foreign.owner_box().is_none());
    assert!(!foreign.in_birth());
    assert_eq!(foreign.source().unwrap().nodes()[0].site(), foreign.assignment().value_site());

    // Reuse the real Birth input, deliberately omitting only its shape loan.
    // One opaque participant vetoes the projection without erasing known rows.
    package.batch().with_normal_program_source_loan(|loan| {
        let birth = package.instance_constructors.rows().iter().find(|row| row.box_name() == "Outer")
            .expect("original Outer Birth");
        let input = birth.lowering_input(loan.program()).unwrap();
        let projection = crate::mir::compiler::source_projection::VerifiedSourceProjectionV1::seal_with_root_profile(
            input.source().root(), input.forest(), input.function().root_profile(),
        ).unwrap();
        let without_shape = crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            input.source().root(), input.forest(), &projection,
        ).unwrap();
        assert!(without_shape.body_shape().is_none());
        let mut draft = field_write_claim::OrdinaryNewFieldWriteClaimDraftV1::new();
        draft.observe_function(input, Some("Outer"), Some(birth.source_id()));
        draft.observe_function(without_shape, Some("Outer"), Some(birth.source_id()));
        let (claims, residences) = draft.seal_inventory().project_claims(package.batch().ordinary_box_coverage(), &package.instance_constructors);
        assert!(claims.has_hidden_writers());
        assert!(matches!(claims.value_source("Outer", "inner"),
            Some(Err(field_write_claim::FieldValueSourceIssueV1::HiddenWriters))));
        assert_eq!(claims.writes().len(), 1, "keep the known writer under the global veto");
        assert!(claims.get(&("Outer".into(), "inner".into())).is_none());
        assert!(residences.is_empty());
        // Duplicate original observations cannot issue a value loan even when
        // the old object-class projection agrees. This is a distinct boundary
        // from a wholly missing shape and must not set the old opaque veto.
        let mut duplicate = field_write_claim::OrdinaryNewFieldWriteClaimDraftV1::new();
        duplicate.observe_function(input, Some("Outer"), Some(birth.source_id()));
        duplicate.observe_function(input, Some("Outer"), Some(birth.source_id()));
        let (claims, _) = duplicate.seal_inventory().project_claims(package.batch().ordinary_box_coverage(), &package.instance_constructors);
        assert!(!claims.has_hidden_writers());
        assert_eq!(claims.get(&("Outer".into(), "inner".into())).map(|name| name.as_ref()), Some("Inner"));
        assert!(matches!(claims.value_source("Outer", "inner"),
            Some(Err(field_write_claim::FieldValueSourceIssueV1::IncompleteWriterInventory))));
        // The root's valid shape omits the nested owner's actual FieldWrite.
        // Resolve just this original method so closure execution admission is
        // irrelevant to the inverse source-inventory check.
        let parsed = crate::parser::NyashParser::parse_from_string(
            "box Outer { mutate() { me.inner = null local change = fn() { me.inner = null return 0 } return 0 } }",
        ).unwrap();
        let crate::ast::ASTNode::Program { statements, .. } = parsed else { panic!("Program") };
        let method = statements.into_iter().find_map(|node| match node {
            crate::ast::ASTNode::BoxDeclaration { methods, .. } => methods.get_declaration("mutate").cloned(),
            _ => None,
        }).unwrap();
        let unit = crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(method).unwrap();
        let nested_input = unit.root_function_input().unwrap();
        assert!(nested_input.forest().semantic_owners().any(|(owner, product)|
            owner != nested_input.owner() && product.assignment_targets().any(|(_, target)|
                matches!(target, crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::FieldWrite { .. }))));
        let mut nested = field_write_claim::OrdinaryNewFieldWriteClaimDraftV1::new();
        nested.observe_function(input, Some("Outer"), Some(birth.source_id()));
        nested.observe_function(nested_input, Some("Outer"), None);
        let (claims, _) = nested.seal_inventory().project_claims(package.batch().ordinary_box_coverage(), &package.instance_constructors);
        assert_eq!(claims.writes().len(), 2, "retain Birth and root write; nested write is absent");
        assert!(!claims.has_hidden_writers(), "root body shape is present");
        assert!(matches!(claims.value_source("Outer", "inner"),
            Some(Err(field_write_claim::FieldValueSourceIssueV1::IncompleteWriterInventory))));
    }).unwrap();
    let (owner, site) = call_site_of(&package, "seal");
    assert!(
        !ledger.lexical_instance_call_covered(owner, &site),
        "an unattributed write to the field name must veto the claim"
    );
}

#[test]
fn local_new_stored_roots_keep_original_acquisition_and_incoming() {
    let text = "box Leaf { n: i64 birth() { me.n = 7 } read() { return me.n } }
        box Parent { left: Leaf right: Leaf birth() { me.left = new Leaf() me.right = new Leaf() } }
        static box Main { main() { local p = new Parent() local q = new Parent()
            print(\"n=\" + p.left.read() + \" other=\" + q.right.read()) return 0 } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
    let roots: Vec<_> = ingress.source_incoming.exact_rows().filter_map(|incoming| {
        let target = incoming.source.instance()?;
        let (slot, root) = incoming.source.local_new_root_source_v1(target).ok()??;
        Some((target.clone(), slot, std::rc::Rc::clone(root)))
    }).collect();
    assert_eq!(roots.len(), 2, "both zero-input scalar calls retain their original Local-New roots");
    assert_ne!(roots[0].2.binding(), roots[1].2.binding());
    for (target, _, _) in &roots { ledger.recheck_stored_local_new_parent_v1(target).unwrap(); }
    let (target, slot, root) = &roots[0];
    let (binding, site, _, _) = target.stored_receiver().unwrap();
    package.batch().with_lowering_input(*slot, |input| {
        root.corroborate_input(input, site).unwrap();
        let mut receiver = source::StoredReceiverSourceV1 { origin: source::StoredReceiverSourceOriginV1::Ordinary,
            parent_new: Some((*slot, std::rc::Rc::clone(root))), parent_binding: binding, parent_site: site.clone(),
            parent_class: "Parent".into(), field_name: "left".into(), child_class: "Leaf".into() };
        receiver.local_new_root_in_input_v1(input, *slot).unwrap();
        receiver.parent_new = Some((*slot, std::rc::Rc::clone(&roots[1].2)));
        assert!(receiver.local_new_root_in_input_v1(input, *slot).is_err(), "same-class sibling acquisition cannot replace the parent");
        receiver.parent_new = Some((*slot + 1, std::rc::Rc::clone(root)));
        assert!(receiver.local_new_root_in_input_v1(input, *slot).is_err(), "caller slot must remain exact");
    }).unwrap();
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let mut claims = ledger.claims.borrow_mut();
    let foreign_declaration = claims.values().find(|row| row.destination == roots[1].2.binding()).unwrap().declaration.clone();
    let original = claims.values_mut().find(|row| row.destination == root.binding()).unwrap();
    let retained = std::mem::replace(&mut original.declaration, foreign_declaration);
    drop(claims);
    assert!(root.recheck(ledger).is_err(), "an equal-class New cannot lend a different original declaration");
    ledger.claims.borrow_mut().values_mut().find(|row| row.destination == root.binding()).unwrap().declaration = retained;
    ledger.recheck_stored_local_new_parent_v1(target).unwrap();
    ledger.borrowed_formal_source.as_mut().unwrap().as_mut().unwrap()
        .source_incoming.replace_stored_origin_for_test(target.call_site());
    assert!(ledger.recheck_stored_local_new_parent_v1(target).is_err(),
        "equal reissued incoming Rc is not the original source");
}
