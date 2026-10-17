//! Source-only nullable Return and field-write requirements.
use super::*;

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
    // Qualified nullable-return source stays separate from ordinary transport.
    for field_write in [false, true] {
        let body = if field_write {
            "if p == null { return null } if n <= 0 { return null } p.value = n return p"
        } else { "if p == null { return null } return p" };
        let text = format!("box Item {{ value: i64 birth() {{ me.value = 5 }} }} box Leaf {{ birth() {{}} maybe(p: Item, n) {{ {body} }} }} static box Main {{ main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let source = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Leaf", "maybe", 2);
        let owner = package.parameter_contracts.iter().find(|row|
            package.selected.key_for_batch_slot(row.batch_slot)
                == Some(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone()))).unwrap().owner;
        let draft = source.source_only_definitions.get(&owner).expect("complete original source draft");
        assert!(!source.definitions.contains_key(&owner));
        let returns: Vec<_> = draft.uses.iter().filter_map(|row| match &row.kind {
            BorrowedFormalUseDraftKindV1::BorrowedFormalReturn { source, .. } => Some((row, source)), _ => None,
        }).collect();
        assert_eq!(returns.len(), 1);
        let (row, returned) = returns[0];
        ledger.callable_result_classes.recheck_borrowed_formal_return_source_v1(returned).unwrap();
        assert_eq!(returned.value(), &row.site);
        assert_eq!(returned.formal(), row.formal);
        assert_eq!(returned.ordinal(), 0);
        let roots = ledger.callable_result_classes.checked_original_source_roots_v1(&key, owner).unwrap();
        assert_eq!(returned.roots().len(), roots.len());
        assert!(returned.roots().iter().zip(roots).all(|(a, b)| Rc::ptr_eq(a, b)));
        // Source carrier classification is independent of LOCAL input demand.
        let result = ledger.callable_result_source_v1(owner).unwrap().expect("original nullable result");
        let nullable = result.borrowed_nullable().expect("same nullable result owner");
        nullable.assert_source_drifts_refuse_for_test();
        let receiver = draft.uses.iter().find_map(|row| match &row.kind {
            BorrowedFormalUseDraftKindV1::FieldWriteReceiver { source, .. } => Some(source), _ => None,
        });
        let value = draft.uses.iter().find_map(|row| match &row.kind {
            BorrowedFormalUseDraftKindV1::FieldWriteValue { source, reach } => {
                package.batch.with_lowering_input(package.parameter_contracts.iter().find(|row| row.owner == owner).unwrap().batch_slot,
                    |input| assert!(reach.corroborates(input))).unwrap();
                Some(source)
            }, _ => None,
        });
        assert_eq!(receiver.is_some(), field_write);
        assert_eq!(value, receiver);
        if let Some(write) = receiver {
            ledger.field_write_claims.recheck_original_write_source_v1(write).unwrap();
            let flow = ledger.completion_for_owner(owner).unwrap().cleanup().root_flow()
                .expect("original nullable Return selects the full Home walk");
            let frames: Vec<_> = flow.expression_plans().iter()
                .filter_map(|plan| plan.borrowed_field_write_frame()).collect();
            assert_eq!(frames.len(), 1);
            assert!(Rc::ptr_eq(frames[0].source(), write.write().source().unwrap()));
            assert!(frames[0].prior_homes().is_empty());
            let formals: Vec<_> = package.parameter_contracts.iter().find(|row| row.owner == owner).unwrap()
                .parameters.iter().map(|row| (row.ordinal, row.binding)).collect();
            use super::super::super::borrowed_formal_entry::field_write_source::checked_field_write_pair_v1;
            let pair = checked_field_write_pair_v1(owner, &formals, draft, frames[0]).unwrap();
            assert_eq!(pair.original, write);
            ledger.recheck_borrowed_field_write_frame_v1(frames[0], write).unwrap();
            let reconstructed = Rc::new(frames[0].as_ref().clone());
            assert!(Rc::ptr_eq(reconstructed.source(), frames[0].source()));
            assert!(ledger.recheck_borrowed_field_write_frame_v1(&reconstructed, write).is_err());

            assert_eq!(pair.receiver.2.site(), write.write().receiver_site());
            assert_eq!(pair.value.2.site(), write.write().assignment().value_site());
            assert!(checked_field_write_pair_v1(owner, &[], draft, frames[0]).is_err());
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
            let foreign_ledger = &foreign.ordinary_new_claim_ledger;
            let foreign_source = foreign_ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
            let foreign_contract = foreign.parameter_contracts.iter().find(|row|
                foreign.selected.key_for_batch_slot(row.batch_slot)
                    == Some(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key.clone()))).unwrap();
            let foreign_flow = foreign_ledger.completion_for_owner(foreign_contract.owner).unwrap().cleanup().root_flow().unwrap();
            let foreign_frame = foreign_flow.expression_plans().iter().find_map(|plan| plan.borrowed_field_write_frame()).unwrap();
            assert!(foreign_ledger.recheck_borrowed_field_write_frame_v1(frames[0], write).is_err());
            assert!(ledger.recheck_borrowed_field_write_frame_v1(foreign_frame, write).is_err());

            assert!(checked_field_write_pair_v1(owner, &formals, draft, foreign_frame).is_err());
            assert!(checked_field_write_pair_v1(owner, &formals,
                foreign_source.source_only_definitions.get(&foreign_contract.owner).unwrap(), frames[0]).is_err());

            let prefixes = flow.lifecycle_prefixes(&[]).unwrap();
            assert_eq!(prefixes.len(), 1);
            assert!(matches!(&prefixes[0], crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1::BorrowedFieldWrite { frame, .. }
                if Rc::ptr_eq(frame, frames[0])));
            assert!(!write.write().is_attributed());
            assert!(ledger.field_write_claims.has_unattributed_field("value"));
            assert!(!draft.uses.iter().any(|row| matches!(row.kind,
                BorrowedFormalUseDraftKindV1::FieldReadOperand { .. })));
        }
        let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
        assert!(foreign.ordinary_new_claim_ledger.callable_result_classes.recheck_borrowed_formal_return_source_v1(returned).is_err());
        if let Some(write) = receiver {
            assert!(foreign.ordinary_new_claim_ledger.field_write_claims.recheck_original_write_source_v1(write).is_err());
        }
    }
}
