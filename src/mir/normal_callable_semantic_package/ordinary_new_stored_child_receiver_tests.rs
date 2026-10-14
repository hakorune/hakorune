//! Exact stored receiver identity stays shared by incoming and result proofs.
use super::*;

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
            &BTreeSet::new(),
            &source.definitions,
            &package.parameter_contracts,
            &calls,
            &scope,
            None,
            needs,
            Some(&package.instance_constructors),
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
