use super::with_selected_source_scope;
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use crate::mir::builder::raw_invocation_source_transport::{RawInvocationRootLineageV1, RawSourceTransportPortV1};
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::{
    CompilationContext, MirBuilder, NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1,
};
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::ValueId;
use crate::parser::{NyashParser, ParserBuildConfig};
use std::{collections::BTreeMap, rc::Rc};

fn installed_source(text: &str) -> Result<(CompilationContext, crate::mir::normal_callable_semantic_package::InstalledNormalCallableSemanticPackageV1), String> {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        text,
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
    let mut resolver = FunctionSemanticResolverSessionV1::new(992).unwrap();
    let package = issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&brands),
    )
    .map_err(|issue| format!("{issue:?}"))?;
    let mut context = CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    Ok((context, installed))
}

fn scope(actual: &str) -> Result<(ValueId, ValueId), String> {
    let text = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe({actual}) return 0 }} }}");
    let (context, installed) = installed_source(&text)?;
    let ledger = installed.ordinary_new_claim_ledger();
    let mut selected = installed.begin_lowering(&context).unwrap();
    let named_arrays = selected.named_array_emission_collector();
    let owned_array_source = selected.owned_array_lifecycle_source_v1();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Transport",
            "probe",
            1,
        ),
    );
    let mut builder = MirBuilder::new();
    let params = vec!["p".to_owned()];
    // Header-only fixture: the source's declared i64 return is projected
    // separately; the actual callable body is not lowered in this test.
    builder
        .create_method_skeleton("Transport.probe/1".into(), "Transport", &params, &[])
        .unwrap();
    builder.set_current_function_declared_signature(
        vec![
            crate::mir::function::MirParamDecl {
                name: "me".into(),
                declared_type_name: None,
                implicit_receiver: true,
            },
            crate::mir::function::MirParamDecl {
                name: "p".into(),
                declared_type_name: None,
                implicit_receiver: false,
            },
        ],
        Some("i64".into()),
    );
    builder.setup_method_params("Transport", &params).unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    let original_params = function.params.clone();
    let original_signature = function.signature.params.clone();
    assert_eq!(
        original_signature,
        vec![
            crate::mir::MirType::Box("Transport".into()),
            crate::mir::MirType::Unknown
        ]
    );
    let mut invocation =
        ModuleLoweringInvocationV1::with_collector(&mut builder, ModuleDraftCollectorV1::default());
    let owner = selected
        .with_selected_lowering_input(&key, |input| {
            let owner = input.source().owner();
            let SelectedNormalCallableKeyV1::Cataloged(lineage_key) = input.selected_key() else {
                panic!("cataloged instance")
            };
            let lineage = RawInvocationRootLineageV1::Cataloged(lineage_key.clone());
            invocation.with_module_port(|builder, module_port| {
                let mut inner = RawInvocationChildPortV1::new(module_port);
                assert!(inner.owned_array_source.is_none());
                with_selected_source_scope(
                    &mut inner,
                    lineage,
                    input,
                    BTreeMap::new(),
                    named_arrays,
                    Rc::clone(&ledger),
                    Some(owned_array_source),
                    None,
                    |inner, _transport| {
                        inner.owned_array_source.as_ref().unwrap().require_same_ledger_v1(&ledger)?;
                        let (_, foreign) = installed_source(
                            "static box Main { main() { return 0 } }")?;
                        assert!(inner.owned_array_source.as_ref().unwrap()
                            .require_same_ledger_v1(&foreign.ordinary_new_claim_ledger()).unwrap_err()
                            .contains("foreign-source-scope"));
                        {
                            let child = inner.reborrow();
                            child.owned_array_source.as_ref().unwrap().require_same_ledger_v1(&ledger)?;
                        }
                        inner.adopt_callable_entry_values_v1(
                            builder,
                            CallableEntryShapeV1::Instance { parameter_count: 1 },
                        )?;
                        let before = builder
                            .function_state
                            .current_function
                            .as_ref()
                            .unwrap()
                            .metadata
                            .physical_param_carriers
                            .clone();
                        assert!(inner
                            .adopt_callable_entry_values_v1(
                                builder,
                                CallableEntryShapeV1::Instance { parameter_count: 1 }
                            )
                            .is_err());
                        assert_eq!(
                            builder
                                .function_state
                                .current_function
                                .as_ref()
                                .unwrap()
                                .metadata
                                .physical_param_carriers,
                            before
                        );
                        Ok(())
                    },
                )
                .unwrap();
                assert!(inner.owned_array_source.is_none(), "scoped source borrow must not escape to its parent");
            });
            owner
        })
        .unwrap();
    drop(invocation);
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .params,
        original_params
    );
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    let rows = ledger.borrowed_ordinary_entry_values_v1(owner);
    let expected = if rows.is_ok() {
        [Carrier::ExistingCallableI64, Carrier::BorrowedTaggedValue]
    } else {
        [Carrier::ExistingCallableI64; 2]
    };
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .metadata
            .physical_param_carriers
            .as_deref(),
        Some(&expected[..])
    );
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .signature
            .params,
        original_signature
    );
    let rows = rows?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, 0);
    assert_eq!(rows[0].1.owner(), owner);
    Ok((original_params[1], rows[0].2))
}

#[test]
fn selected_scope_adopts_existing_borrowed_formal_value() {
    let (original, adopted) = scope("0").unwrap();
    assert_eq!(adopted, original);
}

#[test]
fn selected_scope_rejects_failed_actual_before_raw_carrier_activation() {
    let error = scope("\"unsupported\"").unwrap_err();
    assert!(
        error.contains("borrowed-actual/unsupported-or-unavailable"),
        "{error}"
    );
}

#[test]
fn owned_array_original_construction_installs_real_claim_and_checks_fault_phases() {
    use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
    use crate::mir::instruction::{InvokeOperation, OwnedArrayInvokeOperation as Array};
    use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceBindingSiteV1};
    use crate::mir::MirInstruction;
    for (nullable, literal_null) in [(false, false), (true, false), (false, true), (true, true)] {
        let result = if nullable { "if n > 0 { return new Token() } return null" } else { "return new Token()" };
        let extra_append = if literal_null { "arr.push(null)" } else { "" };
        let slot_nullable = nullable || literal_null;
        let source = format!("box Token {{}} box Maker {{ make(n: i64) {{ {result} }} consume(value) {{ if value == null {{ return 0 }} return 1 }} ignored() {{ local marker = 0 local arr = new ArrayBox() arr.push(me.make(7)) {extra_append} me.consume(arr.get(0)) return me.consume(null) }} }} static box Main {{ main() {{ return 0 }} }}");
        // Replace one existing whole-compiler input with repeated execution;
        // the separate manual writer checks retain their straight-line scope.
        let full_source = if !nullable && !literal_null {
            source.replace("arr.push(me.make(7))", "local i = 0 loop(i < 2) { arr.push(me.make(7)) i = i + 1 }")
        } else { source.clone() };
        full_owned_array_compiler_probe(&full_source);
        let (context, installed) = installed_source(&source).unwrap();
        let ledger = installed.ordinary_new_claim_ledger();
        let mut selected = installed.begin_lowering(&context).unwrap();
        let loan = selected.owned_array_lifecycle_source_v1();
        let key = SelectedNormalCallableKeyV1::Cataloged(hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "ignored", 0));
        selected.with_selected_lowering_input(&key, |input| {
            let source = input.source();
            let original = source.forest().callable_source_ledger(source.owner()).unwrap();
            let mut constructions = original.initializer_relations().filter_map(|relation| {
                let site = OwnedExprSiteV1::new(source.owner(), relation.initializer_site()?.clone());
                loan.prepare_construction_v1(&site, relation.clone()).unwrap().map(|prepared| (relation.clone(), prepared))
            });
            let (relation, prepared) = constructions.next().unwrap_or_else(|| panic!("original Array construction: {ledger:#?}"));
            assert!(constructions.next().is_none());
            let site = prepared.site();
            assert_eq!(prepared.descriptor().nullable(), slot_nullable);
            let wrong = original.initializer_relations().find(|r| r.binding() != relation.binding()).unwrap();
            assert!(loan.prepare_construction_v1(&site, wrong.clone()).unwrap_err().contains("construction-source-drift"));
            let mut state = CallableSemanticLoweringState::from_exact_source(source).unwrap();
            let mut builder = MirBuilder::new();
            builder.create_method_skeleton("Maker.ignored/0".into(), "Maker", &[], &[]).unwrap();
            builder.setup_method_params("Maker", &[]).unwrap();
            let entry = CallableEntryShapeV1::Instance { parameter_count: 0 }.prepare_values(&builder).unwrap();
            state.install_entry_values(&entry).unwrap();
            let array = crate::mir::builder::ordinary_new_admission::selected::owned_array::emit_construction(&mut builder, &mut state, &ledger, prepared).unwrap();
            assert!(ledger.owned_array_initializer_matches(&site, relation.binding(), array));
            let SourceBindingSiteV1::Local { statement, ordinal } = relation.declaration_site() else { panic!("original LOCAL") };
            ledger.complete_local_installation(source.owner(), statement.node(), &[(relation.binding(), *ordinal, array, array)]).unwrap();
            let function = builder.function_state.current_function.as_mut().unwrap();
            ledger.validate_new_emissions(source.owner(), function).unwrap();
            let healthy = function.clone();
            let claim_block = function.blocks.iter().find_map(|(b, block)| block.all_instructions().any(|i| matches!(i, MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Array::Claim { .. }), .. })).then_some(*b)).unwrap();
            let instruction = function.blocks.get_mut(&claim_block).unwrap().terminator.as_mut().unwrap();
            let MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Array::Claim { descriptor, .. }), .. } = instruction else { panic!("Claim") };
            *descriptor = crate::mir::instruction::OwnedArraySlotDescriptorV1::new(descriptor.object(), !slot_nullable);
            assert!(ledger.validate_new_emissions(source.owner(), function).is_err());
            *function = healthy.clone();
            let cleanup = function.blocks.values_mut().find_map(|block| block.instructions.iter_mut().find(|i| matches!(i, MirInstruction::ArrayResidenceRelease { .. }))).unwrap();
            *cleanup = MirInstruction::ArrayResidenceRelease { value: ValueId::new(array.0 + 1) };
            assert!(ledger.validate_new_emissions(source.owner(), function).is_err());
            *function = healthy;
            ledger.complete_new_emissions(source.owner(), function).unwrap();
            state.install_single_local_for_test(statement.node(), relation.binding(), *ordinal, array, array).unwrap();
            let appends: Vec<_> = original.method_calls().filter_map(|(site, _)| {
                loan.prepare_append_v1(&OwnedExprSiteV1::new(source.owner(), site.clone())).unwrap()
            }).collect();
            assert_eq!(appends.len(), if literal_null { 2 } else { 1 });
            for append in appends {
                let candidate = if append.null_candidate() { None } else {
                    let site = append.candidate();
                    Some(ledger.take_lexical_instance_call(site.owner(), site.site()).unwrap().unwrap())
                };
                let packet = crate::mir::builder::ordinary_new_admission::selected::owned_array::emit_append(
                    &mut builder, &mut state, &ledger, append, candidate,
                ).unwrap();
                let function = builder.function_state.current_function.as_mut().unwrap();
                packet.check_raw_v1(&ledger, function).unwrap();
                let healthy = function.clone();
                let block = packet.invoke_binding().0;
                let MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Array::Append { descriptor, .. }), .. } =
                    function.blocks.get_mut(&block).unwrap().terminator.as_mut().unwrap() else { panic!("Append") };
                *descriptor = crate::mir::instruction::OwnedArraySlotDescriptorV1::new(descriptor.object(), !slot_nullable);
                assert!(packet.check_raw_v1(&ledger, function).is_err());
                *function = healthy;
            }
            let mut reads = original.method_calls().filter_map(|(site, _)| {
                loan.prepare_read_v1(&OwnedExprSiteV1::new(source.owner(), site.clone())).unwrap()
            });
            let read_source = reads.next().expect("same original Get");
            assert!(reads.next().is_none());
            let mut calls = original.method_calls().filter(|(_, call)|
                call.arguments().iter().any(|argument| argument.site() == read_source.site().site()));
            let call = OwnedExprSiteV1::new(source.owner(), calls.next().unwrap().0.clone());
            assert!(calls.next().is_none());
            let mut bindings = Vec::new();
            let read = crate::mir::builder::ordinary_new_admission::selected::owned_array::emit_read(
                &mut builder, &mut state, &ledger, &call, read_source, &mut bindings,
            ).unwrap();
            let function = builder.function_state.current_function.as_mut().unwrap();
            read.check_raw_v1(function, &ledger).unwrap();
            let healthy = function.clone();
            let get = function.blocks.values_mut().find_map(|block| block.terminator.as_mut().filter(|row|
                matches!(row, MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Array::Get { .. }), .. }))).unwrap();
            let MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Array::Get { descriptor, .. }), .. } = get else { panic!("Get") };
            *descriptor = crate::mir::instruction::OwnedArraySlotDescriptorV1::new(descriptor.object(), !slot_nullable);
            assert!(read.check_raw_v1(function, &ledger).is_err());
            *function = healthy.clone();
            let index = function.blocks.values_mut().flat_map(|block| block.instructions.iter_mut())
                .find(|row| matches!(row, MirInstruction::Const { value: crate::mir::ConstValue::Integer(0), .. })).unwrap();
            let MirInstruction::Const { value, .. } = index else { panic!("original index") };
            *value = crate::mir::ConstValue::Integer(1);
            assert!(read.check_raw_v1(function, &ledger).is_err());
            *function = healthy;
            assert!(ledger.capture_owned_array_holders_v1(source.owner()).unwrap_err().contains("end-not-emitted"),
                "a real read and Append cannot replace missing source exit End evidence");
            // Construction evidence does not close the independent holder
            // Append/Get/End collector or publication obligations.
            assert!(ledger.begin_owned_array_construction(loan.prepare_construction_v1(&site, relation).unwrap().unwrap()).unwrap_err().contains("duplicate-construction"));
        }).unwrap();
    }
}

fn full_owned_array_compiler_probe(text: &str) {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(text, ParserBuildConfig::default()).unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || crate::r#macro::transform_normal_callable_program_v1(parsed)).unwrap()
        else { panic!("original compiler source") };
    let compiled = crate::mir::MirCompiler::with_options(false).compile_normal_with_published(
        crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
        |view, _pretransform_report| {
            // This callback is reached only after strict verification with the
            // genuine finalized root, not the earlier source-less report.
            let operations: Vec<_> = view.module().functions.values().flat_map(|function|
                function.blocks.values().flat_map(|block| block.all_instructions())).filter_map(|row| match row {
                    crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::OwnedArray(operation), .. } => Some(operation),
                    _ => None,
                }).collect();
            assert!(operations.iter().any(|operation| matches!(operation, crate::mir::instruction::OwnedArrayInvokeOperation::Claim { .. })));
            assert!(operations.iter().any(|operation| matches!(operation, crate::mir::instruction::OwnedArrayInvokeOperation::Append { .. })));
            assert!(operations.iter().any(|operation| matches!(operation, crate::mir::instruction::OwnedArrayInvokeOperation::Get { .. })));
            assert!(operations.iter().any(|operation| matches!(operation, crate::mir::instruction::OwnedArrayInvokeOperation::End { .. })));
            let _physical = view.issue_lifecycle_physical_abi_input()?;
            let rows = view.validated_named_arrays()?;
            let holder = rows.iter().find_map(|row| match row {
                crate::mir::normal_callable_semantic_package::EmittedNamedArrayRequirementV1::OwnedHolder(holder) => Some(holder),
                _ => None,
            }).expect("original whole-holder proof");
            let symbol = holder.validate(view.module())?.to_owned();
            for mutation in 0..4 {
                let mut changed = view.module().clone();
                let function = changed.functions.get_mut(&symbol).unwrap();
                if mutation < 2 {
                    let get = function.blocks.values_mut().find_map(|block| block.terminator.as_mut().filter(|row|
                        matches!(row, crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::OwnedArray(crate::mir::instruction::OwnedArrayInvokeOperation::Get { .. }), .. }))).unwrap();
                    let crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::OwnedArray(crate::mir::instruction::OwnedArrayInvokeOperation::Get { descriptor, index, .. }), .. } = get else { unreachable!() };
                    if mutation == 0 {
                        *descriptor = crate::mir::instruction::OwnedArraySlotDescriptorV1::new(descriptor.object(), !descriptor.nullable());
                    } else { *index = ValueId::new(index.0 + 10000); }
                } else {
                    let (block, end) = function.blocks.iter().find_map(|(id, block)| block.terminator.as_ref().filter(|row|
                        matches!(row, crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::OwnedArray(crate::mir::instruction::OwnedArrayInvokeOperation::End { .. }), .. })).map(|row| (*id, row.clone()))).unwrap();
                    if mutation == 2 { function.blocks.get_mut(&block).unwrap().terminator = None; }
                    else { function.blocks.get_mut(&function.entry_block).unwrap().instructions.push(end); }
                }
                assert!(holder.validate(&changed).is_err(), "finished holder drift {mutation}");
            }
            holder.validate(view.module())?;
            Ok(())
        },
    ).expect("whole original Array program must publish through its production entry");
    assert!(matches!(compiled, crate::mir::compiler::NormalPublishedCompileOutcome::Consumed(())));

}
