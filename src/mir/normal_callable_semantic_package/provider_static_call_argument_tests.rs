//! Focused seal tests for `MIRBUILDER-APP-MIMALLOC-LITE-HEAP-PROVIDER-CALL-ARG-S0`
//! — the provider-`new` argument arm that admits a proven `Alias.m(..)`
//! qualified static call inside a Birth construction plan.
//!
//! The seal is fail-closed at `FieldContractUnsupported`: the arm requires
//! the resolver's `QualifiedUnbound` receiver shape, an exact `(caller,
//! site)` claim-index row naming the `StaticBoxMethod` target, matching
//! arity, and Integer/Bool literal actuals with i64 evidence at every
//! required-i64 ordinal.  Any deviation leaves the plan ineligible — the
//! unsupported argument is never silently sealed or skipped.

use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use super::{
    ConstructionEligibilityV1, ConstructionStoreRhsV1, ConstructionUnavailableV1,
    OrdinaryNewTrivialArgumentKindV1, QualifiedStaticCallArgumentKindV1,
};

fn parent_construction(source: &str) -> ConstructionEligibilityV1 {
    let package = issue(source).expect("package issues with the ineligible plan");
    package
        .instance_constructors()
        .rows()
        .iter()
        .find(|row| row.box_name() == "Parent")
        .expect("parent birth row")
        .construction()
        .clone()
}

fn expect_field_contract_unsupported(label: &str, provider_args: &str) {
    let source = format!(
        "static box LayoutBox {{
            class_size(unused) {{ return 8 }}
            class_id(base) {{ return base }}
            make(unused) {{ return new Page(0, 0) }}
        }}
        box Page {{
            s: i64
            birth(a, b) {{ me.s = a }}
        }}
        box Parent {{
            child: Page = new Page({provider_args})
            birth() {{ }}
        }}
        static box Main {{ main() {{ return 0 }} }}"
    );
    let construction = parent_construction(&source);
    assert!(
        matches!(
            construction,
            Err(ConstructionUnavailableV1::FieldContractUnsupported)
        ),
        "{label}: unproven provider argument keeps FieldContractUnsupported, got {construction:?}"
    );
}

#[test]
fn provider_static_call_argument_seals_target_and_literal_actuals() {
    let package = issue(
        "static box LayoutBox { class_size(unused) { if unused == 0 { return 8 } return 16 } }
        box Page {
            s: i64
            birth(a, b) { me.s = a }
        }
        box Parent {
            child: Page = new Page(0, LayoutBox.class_size(0))
            birth() { }
        }
        static box Main { main() { return 0 } }",
    ).expect("original provider package");
    let parent = package.instance_constructors().rows().iter()
        .find(|row| row.box_name() == "Parent").unwrap();
    let plan = parent.construction().as_ref().expect("sealed provider argument stays eligible");
    let rhs = plan
        .stores()
        .iter()
        .map(|store| store.rhs())
        .find(|rhs| matches!(rhs, ConstructionStoreRhsV1::ProviderConstruction { .. }))
        .expect("one provider construction store");
    let ConstructionStoreRhsV1::ProviderConstruction {
        caller, arguments, static_sources, ..
    } = rhs
    else {
        unreachable!()
    };
    assert_eq!(
        caller,
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Parent", 0),
        "the caller half of the publication row is this birth"
    );
    assert_eq!(arguments.len(), 2);
    assert_eq!(
        arguments[0].kind(),
        &OrdinaryNewTrivialArgumentKindV1::Integer(0)
    );
    let OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall {
        target,
        arguments: actuals,
    } = arguments[1].kind()
    else {
        panic!(
            "second argument seals the qualified static call, got {:?}",
            arguments[1].kind()
        )
    };
    assert_eq!(
        target,
        &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
            "LayoutBox",
            "class_size",
            1
        )
    );
    assert_eq!(
        actuals.as_ref(),
        &[QualifiedStaticCallArgumentKindV1::Integer(0)]
    );

    package.batch().with_normal_program_source_loan(|source| {
        let input = parent.lowering_input(source.program()).unwrap();
        let original = input.function().expression_source().constructions()
            .find(|row| row.class() == "Page").unwrap();
        let lender = |ordinal, claims: &super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1| {
            package.instance_constructors().provider_static_argument_source_v1(
                input, original, ordinal, claims, &package.selected)
        };
        let loan = lender(1, &package.source_static_claims_for_test).unwrap().unwrap();
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(input.owner(), original.site().clone());
        assert_eq!(static_sources.len(), 1);
        assert!(std::rc::Rc::ptr_eq(&loan, &static_sources[0]),
            "construction and formal source retain the SAME original provider");
        let store = plan.stores().iter().find(|store| std::ptr::eq(store.rhs(), rhs)).unwrap();
        assert!(loan.matches_store_v1(store.field(), original.site(), caller, arguments));
        assert!(!loan.matches_store_v1(store.field(), arguments[1].site(), caller, arguments));
        assert!(!loan.matches_store_v1(store.field(), original.site(), caller, &arguments[..1]));
        assert!(loan.matches_original(&site, original, 1));
        assert!(!loan.matches_original(&site, original, 0));
        assert!(loan.matches_constructor(package.instance_constructors()));
        assert!(lender(0, &package.source_static_claims_for_test).unwrap().is_none());
        assert!(lender(2, &package.source_static_claims_for_test).is_err());
        let foreign = issue("static box Main { main() { return 0 } }").unwrap();
        assert!(lender(1, &foreign.source_static_claims_for_test).is_err());
        assert!(!loan.matches_constructor(foreign.instance_constructors()));
    }).unwrap();
    let retained_provider = std::rc::Rc::clone(&static_sources[0]);
    let ledger = std::rc::Rc::clone(&package.ordinary_new_claim_ledger);
    let packet = ledger.prepare_provider_static_packet_v1(&retained_provider)
        .unwrap().expect("original provider borrows the completed scalar cohort");
    assert!(std::rc::Rc::ptr_eq(packet.original_source(), &retained_provider));
    assert!(matches!(packet.arguments(), [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]));
    assert_eq!(packet.incoming().target(), target);
    assert!(ledger.local_call_for_owner(packet.incoming().call_site().owner(),
        packet.incoming().call_site().site()).is_none(), "provider has no synthetic Local observation");
    let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("LayoutBox", "class_size", 1));
    let mut context = crate::mir::builder::CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    installed.begin_lowering(&context).unwrap().with_selected_lowering_input(&key, |input| {
        let entry = ledger.borrowed_ordinary_entry_source_v1(&input).unwrap().unwrap();
        let incoming = entry.incoming().unwrap();
        assert_eq!(incoming.len(), 1, "same original constructor provider incoming");
        assert!(matches!(incoming[0].1, [super::PreparedBorrowedFormalActualV1 {
            ordinal: 0, source: super::BorrowedFormalActualSourceV1::Integer(0), ..
        }]));
    }).unwrap();
    // The same source family must reach the independent final consumers.
    let text = "static box LayoutBox { class_size(i) { if i == 0 { return 8 } return 16 } }
        box Page { s: i64 birth(a, b) { me.s = a } }
        box Parent { child: Page = new Page(0, LayoutBox.class_size(0)) birth() { } }
        static box Main { main() { local parent = new Parent() return 0 } }";
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
            else { panic!("original provider source lost"); };
        let outcome = crate::mir::MirCompiler::with_options(false).compile_normal_with_published(
            crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default())
                .for_closed_app_executable(),
            |view, verification| -> Result<(), String> {
                verification.as_ref().map_err(|errors| format!("{errors:?}"))?;
                let source = view.retained_root_source().ok_or("original provider final source missing")?;
                let mut count = 0;
                let mut original = None;
                source.visit_finalized_borrowed_call_nodes_v1(view.module(), |packet, arguments, caller, coordinate, _| {
                    if packet.target().owner() == "LayoutBox" && packet.target().name() == "class_size" {
                        count += 1;
                        assert_eq!(caller.signature.name,
                            hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Parent", 0).mir_symbol_projection());
                        assert!(matches!(arguments, [crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1::BorrowedActual { ordinal: 0, .. }]));
                        source.borrowed_finalized_call_actuals_v1(packet)?.ok_or("provider actuals missing")?;
                        original = Some((caller.signature.name.clone(), coordinate));
                    }
                    Ok(())
                })?;
                assert_eq!(count, 1, "the genuine provider reaches common final incoming/participants");
                let input = view.issue_lifecycle_physical_abi_input()?;
                crate::mir::emit_lifecycle_physical_abi_json(&input)?;
                let (symbol, (block, _)) = original.unwrap();
                for mutation in 0..6 {
                    let mut altered = view.module().clone();
                    let function = altered.functions.get_mut(&symbol).unwrap();
                    match mutation {
                        0 | 1 | 5 => {
                            let crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, normal_landing, fault_landing, .. } =
                                function.blocks.get_mut(&block).unwrap().terminator.as_mut().unwrap() else { panic!("provider Invoke"); };
                            match mutation {
                                0 => call.args[0] = crate::mir::ValueId(99999),
                                1 => call.callee = crate::mir::Callee::Global(
                                    hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("LayoutBox", "other", 1)
                                        .canonical_global_target_v1()?),
                                _ => *fault_landing = *normal_landing,
                            }
                        }
                        2 => {
                            let crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. } =
                                function.blocks[&block].terminator.as_ref().unwrap() else { panic!("provider Call"); };
                            let actual = call.args[0];
                            let instruction = function.blocks.get_mut(&block).unwrap().instructions.iter_mut()
                                .find(|row| matches!(row, crate::mir::MirInstruction::Const { dst, .. } if *dst == actual)).unwrap();
                            let crate::mir::MirInstruction::Const { value, .. } = instruction else { unreachable!() };
                            *value = crate::mir::ConstValue::Integer(99);
                        }
                        3 => {
                            let instruction = function.blocks.values_mut().flat_map(|block| block.instructions.iter_mut())
                                .find(|row| matches!(row, crate::mir::MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == block)).unwrap();
                            let crate::mir::MirInstruction::InvokeNormalResult { dst, .. } = instruction else { unreachable!() };
                            *dst = crate::mir::ValueId(99999);
                        }
                        _ => {
                            let call = function.blocks.values_mut().find_map(|block| match block.terminator.as_mut() {
                                Some(crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Birth(call), .. }) => Some(call),
                                _ => None,
                            }).unwrap();
                            call.arguments[1] = crate::mir::instruction::BirthArgumentV1::Value(crate::mir::ValueId(99999));
                        }
                    }
                    assert!(source.visit_finalized_borrowed_call_nodes_v1(&altered, |_, _, _, _, _| Ok(())).is_err(),
                        "provider final source refuses independent module drift {mutation}");
                }
                Ok(())
            });
        match outcome {
            Ok(crate::mir::NormalPublishedCompileOutcome::Consumed(())) => {}
            Ok(_) => panic!("provider publication missing"),
            Err(error) => panic!("provider source through final ABI/JSON: {error}"),
        }
    });
}

#[test]
fn provider_static_call_arguments_seal_in_source_order() {
    let construction = parent_construction(
        "static box LayoutBox {
            first(unused) { return 1 }
            second(unused) { return 2 }
        }
        box Page {
            s: i64
            birth(a, b) { me.s = a }
        }
        box Parent {
            child: Page = new Page(LayoutBox.first(1), LayoutBox.second(2))
            birth() { }
        }
        static box Main { main() { return 0 } }",
    );
    let plan = construction.expect("two qualified static arguments stay eligible");
    let rhs = plan
        .stores()
        .iter()
        .map(|store| store.rhs())
        .find(|rhs| matches!(rhs, ConstructionStoreRhsV1::ProviderConstruction { .. }))
        .expect("one provider construction store");
    let ConstructionStoreRhsV1::ProviderConstruction { arguments, .. } = rhs else {
        unreachable!()
    };
    let names: Vec<&str> = arguments
        .iter()
        .map(|argument| match argument.kind() {
            OrdinaryNewTrivialArgumentKindV1::QualifiedStaticCall { target, .. } => target.name(),
            other => panic!("both arguments seal qualified static calls, got {other:?}"),
        })
        .collect();
    assert_eq!(
        names,
        ["first", "second"],
        "source order is the sealed order"
    );
}

#[test]
fn provider_static_call_argument_stays_fail_closed() {
    for (label, args) in [
        // No `StaticBoxMethod` declaration exists for the call — the claim
        // index keeps no row for the site.
        ("unresolved-method", "0, LayoutBox.missing(0)"),
        // `Page` is an instance box: no static target inventory row.
        ("non-static-receiver", "0, Page.class_size(0)"),
        // `make` returns an object — `ExactI64` membership never seals.
        ("non-i64-result", "0, LayoutBox.make(0)"),
        // A non-literal inner actual is outside the literal seal.
        ("compound-inner-arg", "0, LayoutBox.class_size(0 + 1)"),
        // `class_id` returns its parameter, so ordinal 0 is a required-i64
        // position — Bool evidence refuses the seal.
        ("bool-at-required-i64", "0, LayoutBox.class_id(true)"),
        // A bare call is not the qualified-static receiver shape.
        ("unqualified-call", "0, class_size(0)"),
    ] {
        expect_field_contract_unsupported(label, args);
    }
}
