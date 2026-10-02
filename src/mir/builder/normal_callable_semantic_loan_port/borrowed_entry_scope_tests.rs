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
use crate::mir::ValueId;
use crate::parser::{NyashParser, ParserBuildConfig};
use std::{collections::BTreeMap, rc::Rc};

fn scope(actual: &str) -> Result<(ValueId, ValueId), String> {
    let text = format!("box Transport {{ birth() {{ }} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe({actual}) return 0 }} }}");
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        &text,
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
                with_selected_source_scope(
                    &mut inner,
                    lineage,
                    input,
                    BTreeMap::new(),
                    named_arrays,
                    Rc::clone(&ledger),
                    None,
                    |inner, _transport| {
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
