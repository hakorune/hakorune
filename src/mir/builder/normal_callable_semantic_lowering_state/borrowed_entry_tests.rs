use super::*;
use crate::mir::builder::{
    CompilationContext, NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1,
};
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    VerifiedNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn package(actual: &str) -> VerifiedNormalCallableSemanticPackageV1 {
    package_with_parameters("p", actual)
}

fn package_with_parameters(
    parameters: &str,
    actual: &str,
) -> VerifiedNormalCallableSemanticPackageV1 {
    let text = format!("box Transport {{ birth() {{ }} probe({parameters}): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() local out = recv.probe({actual}) return 0 }} }}");
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
    issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&brands),
    )
    .unwrap()
}

fn state(actual: &str) -> CallableSemanticLoweringState {
    state_from_package(package(actual), 1)
}

fn state_from_package(
    package: VerifiedNormalCallableSemanticPackageV1,
    arity: u32,
) -> CallableSemanticLoweringState {
    let mut context = CompilationContext::new();
    let installed = package.prepare_install(&mut context).unwrap().commit();
    let ledger = installed.ordinary_new_claim_ledger();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Transport",
            "probe",
            arity,
        ),
    );
    installed
        .begin_lowering(&context)
        .unwrap()
        .with_selected_lowering_input(&key, |input| {
            let rows = ledger
                .borrowed_ordinary_entry_source_v1(&input)
                .map(|row| row.map(|row| row.formals().into()));
            let mut state =
                CallableSemanticLoweringState::from_exact_source(input.source()).unwrap();
            state.lend_ordinary_new_claim_ledger(Rc::clone(&ledger));
            state
                .stage_borrowed_entry_formals(input.source().owner(), rows)
                .unwrap();
            state
        })
        .unwrap()
}

fn entry(receiver: u32, formal: u32) -> PreparedCallableEntryValuesV1 {
    let mut builder = crate::mir::builder::MirBuilder::new();
    builder.enter_function_for_test("Transport/probe/1".into());
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .params = vec![ValueId::new(receiver), ValueId::new(formal)];
    let values = PreparedCallableEntryValuesV1::instance_method(&builder, 1).unwrap();
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .params,
        vec![ValueId::new(receiver), ValueId::new(formal)]
    );
    values
}

#[test]
fn borrowed_entry_installs_existing_formal_values_for_all_closed_actual_domains() {
    for actual in ["-7", "true", "recv"] {
        let mut state = state(actual);
        let entry = entry(51, 72);
        state.install_entry_values(&entry).unwrap();
        let rows = state
            .ordinary_new_claim_ledger
            .as_ref()
            .unwrap()
            .borrowed_ordinary_entry_values_v1(state.owner)
            .unwrap();
        assert_eq!(rows.as_ref(), &[(0, state.parameters[0], ValueId::new(72))]);
        assert_eq!(
            state.values.get(&state.parameters[0]),
            Some(&ValueId::new(72))
        );
        assert_eq!(
            state.values.get(&state.receiver.unwrap()),
            Some(&ValueId::new(51))
        );
        assert!(state
            .install_entry_values(&entry)
            .unwrap_err()
            .contains("duplicate-entry-install"));
    }
}

#[test]
fn borrowed_entry_preserves_pending_failed_actual_without_success_mapping() {
    let mut state = state("0");
    state.borrowed_entry_formals = Some(Err(
        "[freeze:contract][ordinary-new/borrowed-actual/unsupported-or-unavailable]".into(),
    ));
    state.install_entry_values(&entry(51, 72)).unwrap();
    let error = state
        .ordinary_new_claim_ledger
        .as_ref()
        .unwrap()
        .borrowed_ordinary_entry_values_v1(state.owner)
        .unwrap_err();
    assert!(
        error.contains("borrowed-actual/unsupported-or-unavailable"),
        "{error}"
    );
    assert!(state.entry_installed);
}

#[test]
fn borrowed_entry_rejects_receiver_collision_before_entry_install() {
    let mut state = state("0");
    assert!(state
        .install_entry_values(&entry(51, 51))
        .unwrap_err()
        .contains("formal-value-collision"));
    assert!(!state.entry_installed);
    assert!(state.values.is_empty());
    assert!(state
        .ordinary_new_claim_ledger
        .as_ref()
        .unwrap()
        .borrowed_ordinary_entry_values_v1(state.owner)
        .unwrap_err()
        .contains("entry-values-missing"));
}

#[test]
fn borrowed_entry_rejects_ordinal_and_binding_drift_before_entry_install() {
    for change in 0..2 {
        let mut state = state("0");
        state.borrowed_entry_formals = Some(Ok(Some(
            vec![(
                if change == 0 { 1 } else { 0 },
                if change == 0 {
                    state.parameters[0]
                } else {
                    state.receiver.unwrap()
                },
            )]
            .into_boxed_slice(),
        )));
        assert!(state
            .install_entry_values(&entry(51, 72))
            .unwrap_err()
            .contains("formal-value-identity"));
        assert!(!state.entry_installed);
        assert!(state.values.is_empty());
    }
}

#[test]
fn borrowed_entry_rejects_repeated_staging_and_recording() {
    let mut state = state("0");
    assert!(state
        .stage_borrowed_entry_formals(state.owner, Ok(None))
        .unwrap_err()
        .contains("duplicate-source-preparation"));
    state.install_entry_values(&entry(51, 72)).unwrap();
    let ledger = state.ordinary_new_claim_ledger.as_ref().unwrap();
    let rows = ledger
        .borrowed_ordinary_entry_values_v1(state.owner)
        .unwrap();
    assert!(ledger
        .record_borrowed_ordinary_entry_values_v1(state.owner, Ok(rows))
        .unwrap_err()
        .contains("duplicate-entry-values"));
}

#[test]
fn borrowed_entry_unselected_preparation_leaves_existing_entry_unrecorded() {
    let mut state = state("0");
    state.borrowed_entry_formals = Some(Ok(None));
    state.install_entry_values(&entry(51, 72)).unwrap();
    assert!(state.entry_installed);
    assert!(state
        .ordinary_new_claim_ledger
        .as_ref()
        .unwrap()
        .borrowed_ordinary_entry_values_v1(state.owner)
        .unwrap_err()
        .contains("entry-values-missing"));
}

#[test]
fn borrowed_entry_truncated_formals_reject_before_mutating_state() {
    let mut state = state_from_package(package_with_parameters("p, q", "0, true"), 2);
    let Some(Ok(Some(rows))) = &mut state.borrowed_entry_formals else {
        panic!("selected formals")
    };
    *rows = rows[..1].into();
    let mut builder = crate::mir::builder::MirBuilder::new();
    builder.enter_function_for_test("Transport/probe/2".into());
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .params = vec![ValueId::new(51), ValueId::new(72), ValueId::new(73)];
    let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 2).unwrap();
    assert!(state
        .install_entry_values(&entry)
        .unwrap_err()
        .contains("entry-values-identity"));
    assert!(state.values.is_empty());
    assert!(!state.entry_installed);
    assert!(state
        .ordinary_new_claim_ledger
        .as_ref()
        .unwrap()
        .borrowed_ordinary_entry_values_v1(state.owner)
        .unwrap_err()
        .contains("entry-values-missing"));
}

#[test]
fn borrowed_entry_rejects_collision_with_nonopaque_formal() {
    let mut state = state_from_package(package_with_parameters("p, q: i64", "true, 7"), 2);
    let mut builder = crate::mir::builder::MirBuilder::new();
    builder.enter_function_for_test("Transport/probe/2".into());
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .params = vec![ValueId::new(51), ValueId::new(72), ValueId::new(72)];
    let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 2).unwrap();
    assert!(state
        .install_entry_values(&entry)
        .unwrap_err()
        .contains("formal-value-collision"));
    assert!(state.values.is_empty());
    assert!(!state.entry_installed);
}

fn carrier_function(values: &[ValueId]) -> crate::mir::MirFunction {
    let mut builder = crate::mir::MirBuilder::new();
    builder.enter_function_for_test("Transport.probe/2".into());
    let mut function = builder.function_state.current_function.take().unwrap();
    function.params = values.to_vec();
    function.signature.params = vec![crate::mir::MirType::Integer; values.len()];
    function.signature.params[0] = crate::mir::MirType::Box("Transport".into());
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    function.metadata.physical_param_carriers =
        Some(vec![Carrier::ExistingCallableI64; values.len()].into_boxed_slice());
    function
}

#[test]
fn borrowed_carrier_preflight_preserves_mixed_original_formals_without_effects() {
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    let state = state_from_package(package_with_parameters("p, q: i64", "true, 7"), 2);
    let mut function = carrier_function(&[ValueId(51), ValueId(72), ValueId(73)]);
    let mut builder = crate::mir::MirBuilder::new();
    builder.function_state.current_function = Some(function.clone());
    let entry = PreparedCallableEntryValuesV1::instance_method(&builder, 2).unwrap();
    for ty in [crate::mir::MirType::Unknown, crate::mir::MirType::Integer] {
        function.signature.params[1] = ty.clone();
        let prepared = state
            .prepare_borrowed_entry_carriers(&entry, &function)
            .unwrap()
            .unwrap();
        assert_eq!(
            &*prepared,
            &[
                Carrier::ExistingCallableI64,
                Carrier::BorrowedTaggedValue,
                Carrier::ExistingCallableI64
            ]
        );
        assert_eq!(function.signature.params[1], ty);
    }
    assert_eq!(function.params, vec![ValueId(51), ValueId(72), ValueId(73)]);
    assert_eq!(
        function.metadata.physical_param_carriers.as_deref(),
        Some(&[Carrier::ExistingCallableI64; 3][..])
    );
    assert!(!state.entry_installed);
    assert!(state.values.is_empty());
}

#[test]
fn borrowed_carrier_preflight_rejects_missing_conflicting_and_drifted_column() {
    use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
    for change in 0..10 {
        let state = state("true");
        let mut function = carrier_function(&[ValueId(51), ValueId(72)]);
        let entry = entry(51, 72);
        match change {
            0 => function.metadata.physical_param_carriers = None,
            1 => {
                function.metadata.physical_param_carriers =
                    Some(vec![Carrier::ExistingCallableI64].into_boxed_slice())
            }
            2 => {
                function.metadata.physical_param_carriers.as_mut().unwrap()[0] =
                    Carrier::CheckedMapStorage
            }
            3 => {
                function.metadata.physical_param_carriers.as_mut().unwrap()[1] =
                    Carrier::CheckedMapStorage
            }
            4 => {
                function.metadata.physical_param_carriers.as_mut().unwrap()[1] =
                    Carrier::BorrowedTaggedValue
            }
            5 => function.params[1] = ValueId(99),
            6 => function.signature.params[0] = crate::mir::MirType::Integer,
            7 => function.signature.params[1] = crate::mir::MirType::Float,
            8 => function.signature.params[1] = crate::mir::MirType::Bool,
            9 => function.signature.params[1] = crate::mir::MirType::Box("MapBox".into()),
            _ => unreachable!(),
        }
        let original = function.metadata.physical_param_carriers.clone();
        let error = state
            .prepare_borrowed_entry_carriers(&entry, &function)
            .unwrap_err();
        assert!(error.contains("borrowed-entry/"), "{error}");
        assert_eq!(function.metadata.physical_param_carriers, original);
        assert!(!state.entry_installed);
        assert!(state.values.is_empty());
    }
}

#[test]
fn borrowed_carrier_preflight_never_promotes_pending_or_unselected_source() {
    let mut state = state("0");
    let mut function = carrier_function(&[ValueId(51), ValueId(72)]);
    function.metadata.physical_param_carriers = None;
    state.borrowed_entry_formals = Some(Err("source-pending".into()));
    assert!(state
        .prepare_borrowed_entry_carriers(&entry(51, 72), &function)
        .unwrap()
        .is_none());
    state.borrowed_entry_formals = Some(Ok(None));
    assert!(state
        .prepare_borrowed_entry_carriers(&entry(51, 72), &function)
        .unwrap()
        .is_none());
    assert!(!state.entry_installed);
    assert!(function.metadata.physical_param_carriers.is_none());
}
