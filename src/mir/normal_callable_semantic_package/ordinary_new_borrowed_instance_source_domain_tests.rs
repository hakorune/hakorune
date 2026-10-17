//! Instance source registration is independent of executable use selection.
use super::*;

#[test]
fn source_graph_instance_forward_keeps_complete_original_source_sum() {
    use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source::SourceValueAlternativeV1 as Value;
    use std::collections::BTreeSet;
    for (extra, expected) in [
        ("", Some(BTreeSet::from([Value::Text]))),
        ("unknown(q) { local n = me.ingest(q) return 0 }", None),
        ("other() { local n = me.ingest(true) return 0 }", Some(BTreeSet::from([Value::Text, Value::Other]))),
    ] {
        let text = format!("box Pipe {{ ingest(line) {{ local size = line.length() return 0 }} relay(stream) {{ local n = me.ingest(stream) return 0 }} {extra} }} box Other {{ ingest(line) {{ return 0 }} }} static box Main {{ seed() {{ local pipe = new Pipe() local n = pipe.relay(\"text\") return 0 }} main() {{ return 0 }} }}");
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog_selection(
            &text, crate::mir::normal_callable_semantic_package::PackagePhysicalSelectionV1::ClosedAppExecutable,
        ).unwrap();
        let source = package.ordinary_new_claim_ledger.borrowed_formal_source
            .as_ref().unwrap().as_ref().unwrap();
        let incoming: Vec<_> = source.source_incoming.source_rows_v1()
            .filter(|row| row.source.target().name() == "ingest").collect();
        assert_eq!(incoming.len(), 1 + usize::from(!extra.is_empty()));
        let owner = incoming[0].callee;
        assert!(source.source_incoming.source_registered_owners_v1().contains(&owner));
        assert!(!source.definitions.contains_key(&owner), "no executable use draft");
        let formal = package.parameter_contracts.iter().find(|row| row.owner == owner).unwrap().parameters[0].binding;
        assert_eq!(source.source_value_sum_v1(formal).map(|sum| sum.alternatives().clone()), expected, "{extra}");
    }
}

#[test]
fn source_graph_instance_registration_without_static_context_preserves_legacy_and_identity() {
    let package = package("box Pipe { ingest(p) { return 0 } relay(q) { local n = me.ingest(q) return 0 } } box Other { ingest(p) { return 0 } } static box Main { main() { local pipe = new Pipe() local n = pipe.relay(7) return 0 } }");
    let source = package.ordinary_new_claim_ledger.borrowed_formal_source
        .as_ref().unwrap().as_ref().unwrap();
    let calls = source.source_incoming.source_rows_v1()
        .filter_map(|row| row.source.require_instance().ok().map(|target| (row.call.clone(), target)))
        .collect();
    let scan = |contracts: &[_]| inventory_borrowed_incoming_calls_v1(
        package.batch(), &package.selected, &BTreeMap::new(), contracts,
        &calls, &Default::default(), None,
    );
    let inventory = scan(&package.parameter_contracts).unwrap();
    assert!(inventory.owners.is_empty());
    assert_eq!(inventory.exact_rows().count(), 0);
    assert!(inventory.project(&Default::default()).unwrap().is_empty());
    assert_eq!(inventory.source_rows_v1().count(), 2);
    assert!(inventory.sum_vetoed_owners_v1().is_empty(), "proved other class is excluded");
    let target = inventory.source_rows_v1().find(|row| row.source.target().name() == "ingest").unwrap();
    for mutation in 0..4 {
        let mut contracts: Vec<_> = package.parameter_contracts.iter().map(|row|
            OwnedCallableParameterContractDeclarationV1 {
                owner: row.owner, batch_slot: row.batch_slot, mode: row.mode,
                parameters: row.parameters.iter().map(|formal|
                    crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                        ordinal: formal.ordinal, binding: formal.binding, kind: formal.kind.clone(),
                    }).collect(),
            }).collect();
        let index = contracts.iter().position(|row| row.owner == target.callee).unwrap();
        let foreign = contracts.iter().find(|row| row.owner != target.callee && !row.parameters.is_empty()).unwrap().parameters[0].binding;
        match mutation {
            0 => contracts[index].parameters[0].ordinal += 1,
            1 => contracts[index].parameters[0].binding = foreign,
            2 => contracts[index].owner = foreign.owner(),
            3 => contracts[index].mode = CallableParameterDeclarationModeV1::StaticBoxMethod,
            _ => unreachable!(),
        }
        // A changed mode must not silently erase a selected Instance owner,
        // even when there is no Static source context.
        assert!(matches!(scan(&contracts), Err(BorrowedIncomingDraftErrorV1::SourceIdentity)), "mutation {mutation}");
    }
}
