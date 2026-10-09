//! One original index lends exact input contracts and a stable publication brand.
use super::*;
use crate::mir::callable_result_representation::{
    StaticCallResultPublicationTakeV1, VerifiedSameModuleCallableResultCatalogV1,
    VerifiedStaticCallResultPublicationOwnerV1,
};
use crate::mir::normal_callable_semantic_package::qualified_static_call_claim::{
    caller_key_for_function, incoming_source::StaticIncomingSourceV1,
};
use crate::mir::source_call_target::{
    VerifiedStaticImportAliasViewV1, VerifiedWholeSourceStaticCallTargetInventoryV1,
};
use std::rc::Rc;

type Package =
    crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
const MAIN: &str = "static box Layout { class_id(p) { return 0 } }
    static box Main { main() { local k = Layout.class_id(7) return 0 } }";
const HEAP: &str = "static box Layout { need(p) { return p + 1 } }
    box Heap { lookup(size) { local k = Layout.need(size) return 0 } }
    static box Main { main() { local heap = new Heap() local k = heap.lookup(7) return 0 } }";

fn issue(text: &str) -> Package {
    crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        text,
    )
    .expect("original source-issued package")
}

fn input_source(package: &Package, allow_main: bool) -> Result<Rc<StaticIncomingSourceV1>, String> {
    let main = borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .expect("original Main identity");
    for declaration in package.batch().declarations() {
        let answer = package
            .batch()
            .with_lowering_input(declaration.batch_slot(), |input| {
                let Some((site, call)) = input.function().method_calls().find(|(_, call)| {
                    call.receiver() == ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
                }) else {
                    return None;
                };
                let contract = package
                    .parameter_contracts
                    .iter()
                    .find(|row| row.owner == input.owner());
                let main = main
                    .as_ref()
                    .filter(|main| contract.is_some_and(|row| main.matches_contract(row)));
                let caller = caller_key_for_function(
                    &package.selected,
                    declaration.batch_slot(),
                    main.is_some(),
                    main.map(|main| main.catalog_key()),
                );
                let caller = caller.expect("key from original selected row or Main loan");
                let site = OwnedExprSiteV1::new(input.owner(), site.clone());
                Some(
                    package
                        .source_static_claims_for_test
                        .incoming_source(
                            &caller,
                            &site,
                            call,
                            &package.selected,
                            &package.parameter_contracts,
                            main.filter(|_| allow_main),
                        )
                        .and_then(|loan| {
                            loan.map(|loan| loan.retain())
                                .ok_or_else(|| "original source absent".to_owned())
                        }),
                )
            })
            .unwrap();
        if let Some(answer) = answer {
            return answer;
        }
    }
    panic!("qualified source call");
}

fn publications(package: &Package) -> VerifiedStaticCallResultPublicationOwnerV1 {
    let declarations = package.catalog.catalog();
    let imports = VerifiedStaticImportAliasViewV1::seal(declarations, []).unwrap();
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &imports)
        .unwrap()
        .into_targets();
    let results =
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets).unwrap();
    let source = input_source(package, false).unwrap();
    assert!(
        results
            .call_result(source.caller(), source.call_site().site())
            .is_none(),
        "exercise original exact requirement, not the general result path"
    );
    VerifiedStaticCallResultPublicationOwnerV1::issue(declarations, &targets, &results).unwrap()
}

#[test]
fn qualified_static_input_source_keeps_unselected_main_and_opaque_full_formal() {
    let package = issue(MAIN);
    let source = input_source(&package, true).unwrap();
    let main = borrow_app_main_source_v1(
        package.batch(),
        package.catalog.catalog().source_backed_app_main(),
    )
    .unwrap()
    .unwrap();
    assert_eq!(source.caller(), main.catalog_key());
    assert!(package
        .selected
        .batch_slot(&SelectedNormalCallableKeyV1::Cataloged(
            main.catalog_key().clone()
        ))
        .is_none());
    assert_eq!(source.target().owner(), "Layout");
    assert_eq!(source.target().name(), "class_id");
    assert_eq!(source.parameters().len(), 1);
    assert_eq!(source.parameters()[0].ordinal, 0);
    assert_eq!(
        source.parameters()[0].binding.owner(),
        source.callee_owner()
    );
    assert_eq!(
        source.parameters()[0].kind,
        crate::mir::callable_parameter_contract::CallableParameterContractKindV1::OpaqueHandle
    );
    assert!(source.required_i64_arguments().is_empty());
    assert!(input_source(&package, false)
        .unwrap_err()
        .contains("incoming-source-identity"));
    let foreign = issue(MAIN);
    let foreign_main = borrow_app_main_source_v1(
        package.batch(),
        foreign.catalog.catalog().source_backed_app_main(),
    );
    assert!(matches!(
        foreign_main,
        Err(OrdinaryNewCoSealIssueV1::AppMainIdentityMissing)
    ));
}

#[test]
fn qualified_static_input_source_matches_real_publication_after_catalog_move() {
    let package = issue(HEAP);
    let source = input_source(&package, false).unwrap();
    assert_eq!(source.caller().owner(), "Heap");
    assert_eq!(source.required_i64_arguments(), &[0]);
    let mut owner = publications(&package);
    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take_for_source(
            package.catalog.catalog(),
            source.caller(),
            source.call_site().site(),
        )
        .unwrap()
    else {
        panic!("selected");
    };
    assert!(source.corroborates_publication_handoff(&handoff));
    assert!(owner
        .take_for_source(
            package.catalog.catalog(),
            source.caller(),
            source.call_site().site()
        )
        .is_err());
    let original_address = package.catalog.catalog() as *const _ as usize;
    let moved = Box::new(package);
    assert_ne!(
        original_address,
        moved.catalog.catalog() as *const _ as usize
    );
    let mut owner = publications(&moved);
    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take_for_source(
            moved.catalog.catalog(),
            source.caller(),
            source.call_site().site(),
        )
        .unwrap()
    else {
        panic!("selected moved");
    };
    assert!(source.corroborates_publication_handoff(&handoff));
    let foreign = issue(HEAP);
    let mut owner = publications(&foreign);
    let StaticCallResultPublicationTakeV1::Selected(handoff) = owner
        .take_for_source(
            foreign.catalog.catalog(),
            source.caller(),
            source.call_site().site(),
        )
        .unwrap()
    else {
        panic!("foreign selected");
    };
    assert!(!source.corroborates_publication_handoff(&handoff));
}

#[test]
fn current_owner_loop_handoff_matches_only_its_original_catalog() {
    let text = "static box Size { norm(size) { if size <= 0 { return 1 } return size } run(size) { local n = me.norm(size) return 0 } } static box Main { main() { return 0 } }";
    let package = issue(text);
    let source = package
        .ordinary_new_claim_ledger
        .borrowed_formal_source
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
        .source_incoming
        .static_observations()
        .values()
        .filter_map(|row| row.as_ref().ok())
        .find(|row| row.target().name() == "norm" && row.current_owner_source().is_some())
        .unwrap();
    let declarations = package.declaration_catalog();
    let aliases = VerifiedStaticImportAliasViewV1::seal(declarations, []).unwrap();
    let targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(declarations, &aliases)
        .unwrap()
        .into_targets();
    let results =
        VerifiedSameModuleCallableResultCatalogV1::verify(declarations, &targets).unwrap();
    let owner = VerifiedStaticCallResultPublicationOwnerV1::issue(declarations, &targets, &results)
        .unwrap();
    let handoff = owner
        .selected_handoff_for_source(source.caller(), source.call_site().site())
        .unwrap();
    assert!(!source.corroborates_publication_handoff(handoff));
    assert!(source.corroborates_selected_loop_publication_handoff(handoff));

    let foreign = issue(text);
    let foreign_declarations = foreign.declaration_catalog();
    let foreign_aliases = VerifiedStaticImportAliasViewV1::seal(foreign_declarations, []).unwrap();
    let foreign_targets = VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
        foreign_declarations,
        &foreign_aliases,
    )
    .unwrap()
    .into_targets();
    let foreign_results =
        VerifiedSameModuleCallableResultCatalogV1::verify(foreign_declarations, &foreign_targets)
            .unwrap();
    let foreign_owner = VerifiedStaticCallResultPublicationOwnerV1::issue(
        foreign_declarations,
        &foreign_targets,
        &foreign_results,
    )
    .unwrap();
    let foreign_handoff = foreign_owner
        .selected_handoff_for_source(source.caller(), source.call_site().site())
        .unwrap();
    assert!(!source.corroborates_selected_loop_publication_handoff(foreign_handoff));
}

#[test]
fn qualified_static_input_source_rejects_missing_duplicate_slot_and_ordinal_contracts() {
    for drift in 0..7 {
        let mut package = issue(HEAP);
        let source = input_source(&package, false).unwrap();
        let mut contracts = std::mem::take(&mut package.parameter_contracts).into_vec();
        let owner = if drift >= 5 {
            source.call_site().owner()
        } else {
            source.callee_owner()
        };
        let index = contracts.iter().position(|row| row.owner == owner).unwrap();
        let caller_binding = contracts
            .iter()
            .find(|row| row.owner == source.call_site().owner())
            .unwrap()
            .parameters[0]
            .binding;
        match drift {
            0 | 5 => {
                contracts.remove(index);
            }
            1 | 6 => {
                let row = &contracts[index];
                let duplicate = OwnedCallableParameterContractDeclarationV1 {
                    batch_slot: row.batch_slot, owner: row.owner, mode: row.mode,
                    parameters: row.parameters.iter().map(|formal|
                        crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1 {
                            ordinal: formal.ordinal, binding: formal.binding, kind: formal.kind.clone(),
                        }).collect(),
                };
                contracts.push(duplicate);
            }
            2 => contracts[index].batch_slot += 100,
            3 => contracts[index].parameters[0].ordinal += 1,
            _ => contracts[index].parameters[0].binding = caller_binding,
        }
        package.parameter_contracts = contracts.into_boxed_slice();
        assert!(
            input_source(&package, false)
                .unwrap_err()
                .contains("incoming-source-identity"),
            "drift={drift}"
        );
    }
}

#[test]
fn qualified_static_input_source_rejects_foreign_owner_and_absent_caller_or_site() {
    let package = issue(HEAP);
    let source = input_source(&package, false).unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == source.call_site().owner())
        .unwrap();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            let (_, call) = input
                .function()
                .method_calls()
                .find(|(site, _)| *site == source.call_site().site())
                .unwrap();
            let index = &package.source_static_claims_for_test;
            let foreign =
                OwnedExprSiteV1::new(source.callee_owner(), source.call_site().site().clone());
            assert!(index
                .incoming_source(
                    source.caller(),
                    &foreign,
                    call,
                    &package.selected,
                    &package.parameter_contracts,
                    None
                )
                .is_err());
            let absent_site =
                OwnedExprSiteV1::new(input.owner(), source.argument_sites()[0].clone());
            assert!(index
                .incoming_source(
                    source.caller(),
                    &absent_site,
                    call,
                    &package.selected,
                    &package.parameter_contracts,
                    None
                )
                .unwrap()
                .is_none());
            assert!(index
                .incoming_source(
                    source.target(),
                    source.call_site(),
                    call,
                    &package.selected,
                    &package.parameter_contracts,
                    None
                )
                .unwrap()
                .is_none());
            let original = index
                .incoming_source(
                    source.caller(),
                    source.call_site(),
                    call,
                    &package.selected,
                    &package.parameter_contracts,
                    None,
                )
                .unwrap()
                .unwrap();
            assert!(original.corroborates_retained(&source));
            let foreign_package = issue(HEAP);
            let foreign_source = input_source(&foreign_package, false).unwrap();
            assert!(!original.corroborates_retained(&foreign_source));
        })
        .unwrap();
}
