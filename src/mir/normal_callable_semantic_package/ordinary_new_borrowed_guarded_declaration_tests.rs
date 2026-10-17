//! Declaration and required-input correspondence use the same original source.
use super::*;

fn guarded_package() -> VerifiedNormalCallableSemanticPackageV1 {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Token {} box Maker { make(size) { if size <= 0 { return new Token() } return new Token() } qualified() { return me.make(7) } relay(size) { if size <= 0 { return 0 } local item = me.make(size) return 0 } relayHandle(token: Token, size) { if token == null { return 0 } if size <= 0 { return 0 } local item = me.make(size) return 0 } } static box Main { main() { return 0 } }",
    ).unwrap()
}

#[test]
fn guarded_declaration_binds_complete_empty_incoming_and_exact_original_ordinals() {
    let package = guarded_package();
    let ledger = &package.ordinary_new_claim_ledger;
    let source = ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let (owner, draft) = source
        .source_only_definitions
        .iter()
        .find(|(owner, _)| {
            ledger
                .checked_guarded_instance_definition_v1(source, **owner)
                .unwrap()
                .is_some()
        })
        .expect("original guarded relay declaration");
    let owner = *owner;
    let formal = *draft.origins.keys().next().unwrap();
    assert!(source
        .source_incoming
        .project_declaration_v1(owner)
        .unwrap()
        .is_empty());
    let wrong = Ok(vec![(1u32, formal, crate::mir::ValueId(71))].into_boxed_slice());
    assert!(ledger
        .validate_borrowed_ordinary_entry_values_v1(owner, &wrong)
        .unwrap_err()
        .contains("original-parameter-drift"));
    ledger
        .record_borrowed_ordinary_entry_values_v1(
            owner,
            Ok(Box::new([(0, formal, crate::mir::ValueId(71))])),
        )
        .unwrap();
    let declaration = ledger.checked_borrowed_declaration_source_v1(owner).unwrap();
    assert_eq!(declaration.formals(), &[(0, formal)]);
    assert!(
        declaration.incoming().is_err(),
        "declaration cannot lend executable actuals"
    );
    assert_eq!(declaration.incoming_targets().count(), 0);
    assert_eq!(declaration.guarded_argument_uses().unwrap().len(), 1);
    let mixed = package.parameter_contracts.iter()
        .find(|row| row.parameters.len() == 2).unwrap();
    let values: Box<[_]> = mixed.parameters.iter().enumerate()
        .map(|(index, row)| (row.ordinal, row.binding, crate::mir::ValueId(80 + index as u32)))
        .collect();
    ledger.record_borrowed_ordinary_entry_values_v1(mixed.owner, Ok(values)).unwrap();
    let entry = ledger.checked_borrowed_declaration_source_v1(mixed.owner).unwrap();
    assert_eq!(entry.formals().len(), 2);
    assert_eq!(entry.incoming_targets().count(), 0);
    assert!(entry.incoming().is_err(), "complete-empty entry cannot lend actuals");
    let view = entry.formal_object_view(mixed.parameters[0].binding).unwrap().unwrap();
    assert!(view.is_declared());
    assert_eq!(view.class(), "Token");
    assert!(entry.formal_object_view(mixed.parameters[1].binding).unwrap().is_none());
    assert!(entry.formal_object_view(formal).is_err(), "foreign declaration formal must be refused");
    assert!(ledger.borrowed_declaration_owners_v1().unwrap().contains(&mixed.owner));
    let make = source.source_incoming.exact_rows().find(|row| row.call.owner() == owner).unwrap();
    let error = ledger.checked_borrowed_declaration_source_v1(make.callee).unwrap_err();
    assert!(error.contains("borrowed-declaration/missing-retained-call"), "{error}");
}

#[test]
fn guarded_declaration_and_reconstruction_refuse_lost_original_guard_or_catalog() {
    for lose_catalog in [false, true] {
        let mut package = guarded_package();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let owner = source
            .source_only_definitions
            .keys()
            .copied()
            .find(|owner| {
                ledger
                    .checked_guarded_instance_definition_v1(source, *owner)
                    .unwrap()
                    .is_some()
            })
            .unwrap();
        if lose_catalog {
            ledger.original_parameter_contracts = None;
            let source = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_guarded_instance_definition_v1(source, owner)
                .unwrap_err()
                .contains("original-catalog-missing"));
        } else {
            ledger
                .borrowed_formal_source
                .as_mut()
                .unwrap()
                .as_mut()
                .unwrap()
                .guarded_actuals
                .clear();
            let source = ledger
                .borrowed_formal_source
                .as_ref()
                .unwrap()
                .as_ref()
                .unwrap();
            assert!(ledger
                .checked_guarded_instance_definition_v1(source, owner)
                .unwrap()
                .is_none());
            assert!(ledger.checked_borrowed_declaration_source_v1(owner).is_err());
        }
    }
}

#[test]
fn guarded_receiver_packet_never_retries_old_protocol_after_guard_loss() {
    for after_take in [false, true] {
        let mut package = guarded_package();
        let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
        let source = ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let target = source
            .source_incoming
            .exact_rows()
            .find_map(|row| {
                let target = row.source.instance()?;
                (!target.has_object_source_requirement()
                    && ledger.requires_object_packet_v1(target).unwrap())
                .then(|| target.clone())
            })
            .expect("original unqualified guarded receiver input");
        assert!(
            matches!(ledger.lexical_instance_calls.borrow().get(target.call_site()),
        Some(LexicalInstanceCallDispositionSlotV1::Ready(row)) if row.object_packet.is_some())
        );
        let consumed = if after_take {
            Some(
                ledger
                    .take_receiver_object_packet_v1(
                        target.call_site().owner(),
                        target.call_site().site(),
                        target.target(),
                    )
                    .unwrap()
                    .unwrap(),
            )
        } else {
            None
        };
        ledger
            .borrowed_formal_source
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap()
            .guarded_actuals
            .clear();
        if let Some(row) = consumed {
            assert!(ledger
                .borrowed_call_actuals_v1(&row)
                .unwrap_err()
                .contains("receiver-object/source-requirement"));
        } else {
            assert!(ledger
                .take_receiver_object_packet_v1(
                    target.call_site().owner(),
                    target.call_site().site(),
                    target.target()
                )
                .unwrap_err()
                .contains("receiver-object/source-missing"));
        }
    }
}
