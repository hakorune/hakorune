use super::with_selected_source_scope;
use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
use crate::mir::builder::normal_callable_binding_materialization_port::{
    CallableBindingMaterializationPortV1, CallableEntryShapeV1,
};
use crate::mir::builder::raw_invocation_source_transport::RawInvocationRootLineageV1;
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::{
    CompilationContext, MirBuilder, NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1,
};
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};
use std::{collections::BTreeMap, rc::Rc};

fn scope(mode: &str) -> Result<(), String> {
    let text = "box Transport { birth() { } probe(p: usize): i64 { return 0 } } static box Main { main() { return 0 } }";
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
    let ledger = installed.ordinary_new_claim_ledger();
    let mut selected = installed.begin_lowering(&context).unwrap();
    let named_arrays = selected.named_array_emission_collector();
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
                declared_type_name: Some("usize".into()),
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
            crate::mir::MirType::Box("usize".into())
        ]
    );
    let mut invocation =
        ModuleLoweringInvocationV1::with_collector(&mut builder, ModuleDraftCollectorV1::default());
    let _owner = selected
        .with_selected_lowering_input(&key, |input| {
            let owner = input.source().owner();
            if mode == "ok" {
                use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
                use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIssuerV1};
                let (_, binding, _) = input.parameter_contracts().next().unwrap();
                let foreign_owner = FunctionOwnerIssuerV1::new_for_compilation().unwrap().issue().unwrap();
                for rows in [
                    vec![(1, binding)],
                    vec![(0, binding), (0, binding)],
                    vec![(0, BindingRefV1::new(foreign_owner, binding.binding()))],
                ] {
                    let mut state = CallableSemanticLoweringState::from_exact_source(input.source()).unwrap();
                    assert!(state.stage_exact_usize_entry_formals(owner, rows.into_boxed_slice())
                        .unwrap_err().contains("exact-usize-entry/source-binding-drift"));
                    // Rejection must not leave a partial staged row.
                    state.stage_exact_usize_entry_formals(owner, vec![(0, binding)].into_boxed_slice()).unwrap();
                }
            }
            let SelectedNormalCallableKeyV1::Cataloged(lineage_key) = input.selected_key() else {
                panic!("cataloged instance")
            };
            let lineage = RawInvocationRootLineageV1::Cataloged(lineage_key.clone());
            invocation.with_module_port(|builder, module_port| {
                let mut inner = RawInvocationChildPortV1::new(module_port);
                if mode == "ok" {
                    let state = crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState::from_exact_source(input.source()).unwrap();
                    let entry = CallableEntryShapeV1::Instance { parameter_count: 1 }.prepare_values(builder)?;
                    assert!(state.prepare_exact_usize_entry_projection(&entry, builder)?.is_empty());
                    assert_eq!(builder.function_state.current_function.as_ref().unwrap().signature.params[1], crate::mir::MirType::Box("usize".into()));
                }
                with_selected_source_scope(
                    &mut inner,
                    lineage,
                    input,
                    BTreeMap::new(),
                    named_arrays,
                    Rc::clone(&ledger),
                    None,
                    |inner, _transport| {
                        use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
                        let function = builder.function_state.current_function.as_mut().unwrap();
                        match mode {
                            "ok" => {},
                            "name" => function.metadata.declared_param_decls[1].name = "foreign".into(),
                            "type" => function.metadata.declared_param_decls[1].declared_type_name = Some("i64".into()),
                            "missing-carrier" => function.metadata.physical_param_carriers = None,
                            "short-carrier" => function.metadata.physical_param_carriers = Some(vec![Carrier::ExistingCallableI64].into_boxed_slice()),
                            "map-carrier" | "bits-carrier" | "tagged-carrier" => {
                                function.metadata.physical_param_carriers.as_mut().unwrap()[1] = match mode {
                                    "map-carrier" => Carrier::CheckedMapStorage,
                                    "bits-carrier" => Carrier::U64BitsOnI64,
                                    _ => Carrier::BorrowedTaggedValue,
                                };
                            },
                            _ => panic!("unknown mutation"),
                        }
                        let result = inner.adopt_callable_entry_values_v1(
                            builder,
                            CallableEntryShapeV1::Instance { parameter_count: 1 },
                        );
                        if mode != "ok" {
                            let err = result.expect_err("source declaration drift rejects");
                            assert!(
                                err.contains("exact-usize-entry/"),
                                "{err}"
                            );
                            assert_eq!(
                                builder
                                    .function_state
                                    .current_function
                                    .as_ref()
                                    .unwrap()
                                    .signature
                                    .params[1],
                                crate::mir::MirType::Box("usize".into())
                            );
                            return Err(err);
                        }
                        result?;
                        assert!(inner
                            .adopt_callable_entry_values_v1(
                                builder,
                                CallableEntryShapeV1::Instance { parameter_count: 1 },
                            )
                            .unwrap_err()
                            .contains("exact-usize-entry/repeated-entry"));
                        Ok(())
                    },
                )
            })?;
            Ok::<_, String>(owner)
        })
        .unwrap()?;
    drop(invocation);
    let function = builder.function_state.current_function.as_mut().unwrap();
    assert_eq!(function.params, original_params);
    assert_eq!(function.signature.params[1], crate::mir::MirType::Integer);
    assert_eq!(
        function.metadata.declared_param_decls[1]
            .declared_type_name
            .as_deref(),
        Some("usize")
    );
    crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts(
        function,
    );
    crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(function)?;
    let contract = &function.metadata.parameter_entry_contracts[0];
    assert_eq!(contract.parameter_value_id, original_params[1]);
    assert!(contract.runtime_check_required);
    assert!(!contract.proof_elision_allowed);
    function.metadata.parameter_entry_contracts.clear();
    assert!(
        crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(function)
            .unwrap_err()
            .contains("parameter_contract_carrier_missing")
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_types
            .get(&original_params[1]),
        Some(&crate::mir::MirType::Integer)
    );
    Ok(())
}

#[test]
fn selected_exact_usize_source_entry_projects_existing_value_and_preserves_checks() {
    scope("ok").unwrap();
}

#[test]
fn selected_exact_usize_source_entry_rejects_declaration_drift_before_projection() {
    assert!(scope("name")
        .unwrap_err()
        .contains("exact-usize-entry/declaration-or-value-drift"));
}

#[test]
fn selected_exact_usize_source_entry_rejects_foreign_type_and_carriers_before_projection() {
    for mode in [
        "type",
        "missing-carrier",
        "short-carrier",
        "map-carrier",
        "bits-carrier",
        "tagged-carrier",
    ] {
        let err = scope(mode).unwrap_err();
        let expected = if mode == "type" {
            "declaration-or-value-drift"
        } else {
            "carrier-"
        };
        assert!(err.contains(expected), "mode={mode}: {err}");
    }
}

#[test]
fn selected_exact_usize_wrapper_survives_real_source_finalization_with_exact_facts() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            let parsed = NyashParser::parse_normal_callable_program_with_build_config(
                "static box Width { scalar(p) { return 0 } wrapper(p: usize) { return me.scalar(p) } } static box Main { main() { return 0 } }",
                ParserBuildConfig::default(),
            ).unwrap();
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap();
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                transformed
            else {
                panic!("source-preserving fixture")
            };
            let request =
                crate::mir::compiler::NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    None,
                    std::collections::HashMap::new(),
                );
            let result = crate::mir::compiler::MirCompiler::with_options(optimize)
                .compile_normal(request)
                .expect("source usize wrapper finalizes");
            assert!(
                result.verification_result.is_ok(),
                "{:?}",
                result.verification_result
            );
            let function = &result.module.functions["Width.wrapper/1"];
            let index = function
                .metadata
                .declared_param_decls
                .iter()
                .position(|decl| decl.declared_type_name.as_deref() == Some("usize"))
                .unwrap();
            let value = function.params[index];
            assert_eq!(
                function.signature.params[index],
                crate::mir::MirType::Integer
            );
            assert_eq!(
                function.metadata.value_types.get(&value),
                Some(&crate::mir::MirType::Integer)
            );
            let fact = &function.metadata.exact_numeric_value_facts[&value];
            assert_eq!(fact.declared_type_name, "usize");
            assert!(
                matches!(&fact.source, crate::mir::exact_numeric_value_facts::ExactNumericValueFactSource::Param { index: actual, name } if *actual == index && name == "p")
            );
            let contract = function
                .metadata
                .parameter_entry_contracts
                .iter()
                .find(|row| row.parameter_value_id == value)
                .unwrap();
            assert!(contract.runtime_check_required);
            assert!(!contract.proof_elision_allowed);
            crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(
                function,
            )
            .unwrap();
        }
    });
}
