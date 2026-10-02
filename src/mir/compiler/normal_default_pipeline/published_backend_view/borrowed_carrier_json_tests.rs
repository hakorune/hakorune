//! Negative mutation of a real published Map program: neither writer
//! branch may turn a reserved borrowed carrier into an i64 transport.
use super::*;
use crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1 as Carrier;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};

#[test]
fn borrowed_carrier_writer_refuses_both_parameter_and_actual_encoding() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = "static box Helpers { read_k(m: MapBox): i64 { return m.get(\"k\") } } static box Main { main() { return read_k(%{\"k\" => 7}) } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source-backed")
        };
        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            None,
            std::collections::HashMap::new(),
        );
        MirCompiler::with_options(false)
            .compile_normal_with_published(request, |view, verification| -> Result<(), String> {
                assert!(verification.is_ok(), "{verification:?}");
                let input = view.issue_lifecycle_physical_abi_input()?;
                let mut program = input.program().clone();
                let callee = program
                    .functions
                    .iter_mut()
                    .find(|f| f.name == "Helpers.read_k/1")
                    .unwrap();
                assert_eq!(
                    callee.param_carriers,
                    Some(&[Carrier::CheckedMapStorage][..])
                );
                // Spoof only the transport metadata; this is deliberately not
                // a valid source-authorized borrowed call.
                callee.param_carriers = Some(&[Carrier::BorrowedTaggedValue]);
                for parameter_first in [true, false] {
                    program.functions.sort_by_key(|f| {
                        if parameter_first {
                            f.name == "main"
                        } else {
                            f.name != "main"
                        }
                    });
                    let first = &program.functions[0];
                    assert_eq!(first.name == "main", !parameter_first);
                    let error =
                        super::super::physical_program_json::emit_lifecycle_physical_program_value(
                            &program, None,
                        )
                        .unwrap_err();
                    assert!(
                        error.contains("borrowed-carrier-activation-missing"),
                        "{error}"
                    );
                }
                Ok(())
            })
            .unwrap();
    });
}

#[test]
fn borrowed_incoming_rejects_metadata_spoof_and_unit_handle_result_before_filter() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    // The existing fixture owns its environment lock; do not reacquire it.
    {
        let (module, handoff) =
            crate::mir::builder::lexical_call_projection_finished_artifact_fixture();
        // These are the existing source-backed builder/finalization owner
        // invocations. This does not claim the broader compiler/EXE route.
        let view = super::super::PublishedMirBackendView::try_new(&module)
            .unwrap()
            .bind_finalized_root_handoff(Some(&handoff))
            .unwrap();
        let profile =
            super::super::PublishedObjectStorageProfileV1::from_runtime_name(None).unwrap();
        let view =
            super::super::super::lifecycle_admission::admit_lifecycle(view, &profile).unwrap();
        let program = view.issue_lifecycle_physical_program().unwrap();
        let mut visited = 0;
        view.retained_root_source().unwrap().visit_finalized_lexical_call_nodes_v1(view.module(),
                |_, context, _, _, caller, coordinate, _copies| {
                    use crate::mir::normal_callable_semantic_package::FinalizedLexicalCallContextV1 as Context;
                    assert!(matches!(context, Context::Local { .. }));
                    let MirInstruction::Invoke { operation: InvokeOperation::Call { call, result }, .. } =
                        caller.blocks[&coordinate.0].all_instructions().nth(coordinate.1).unwrap()
                        else { panic!("actual Invoke"); };
                    super::super::compiled_entry_contract::corroborate_final_call(call, *result, caller, coordinate)?;
                    for mutation in 0..4 {
                        let mut published = call.clone();
                        let mut published_result = *result;
                        match mutation {
                            0 => published.callee = Callee::SameModuleInstance {
                                key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Pool", "foreign", 1),
                                receiver: match &call.callee { Callee::SameModuleInstance { receiver, .. } => *receiver, _ => panic!("receiver") },
                            },
                            1 => { let Callee::SameModuleInstance { receiver, .. } = &mut published.callee else { panic!("receiver") }; *receiver = ValueId(999); },
                            2 => {
                                if let Some(argument) = published.args.first_mut() { *argument = ValueId(998); }
                                else { published.args.push(ValueId(998)); }
                            },
                            _ => published_result = InvokeCallResultKind::Handle,
                        }
                        assert!(super::super::compiled_entry_contract::corroborate_final_call(
                            &published, published_result, caller, coordinate).unwrap_err().contains("actual-call-mismatch"));
                    }
                    visited += 1;
                    Ok(())
                }).unwrap();
        assert_eq!(visited, 5);
        super::super::compiled_entry_contract::verify_borrowed_call_incoming(
            &program,
            view.module(),
        )
        .unwrap();
        for result in [
            InvokeCallResultKind::I64,
            InvokeCallResultKind::Unit,
            InvokeCallResultKind::Handle,
        ] {
            let mut mutated = program.clone();
            let callee = mutated
                .functions
                .iter_mut()
                .find(|f| f.name == "Pool.give/1")
                .unwrap();
            callee.param_carriers =
                Some(&[Carrier::ExistingCallableI64, Carrier::BorrowedTaggedValue]);
            let caller = mutated
                .functions
                .iter_mut()
                .find(|f| f.name == "main")
                .unwrap();
            let block = caller.blocks.iter_mut().find(|b| matches!(b.terminator.instruction,
                    MirInstruction::Invoke { operation: InvokeOperation::Call { call, .. }, .. }
                    if matches!(&call.callee, Callee::SameModuleInstance { key, .. } if key == &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Pool", "give", 1))))
                    .unwrap();
            let mut altered = block.terminator.instruction.clone();
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Call {
                        result: observed, ..
                    },
                ..
            } = &mut altered
            else {
                panic!("Call");
            };
            *observed = result;
            block.terminator.instruction = &altered;
            let error = super::super::compiled_entry_contract::verify_borrowed_call_incoming(
                &mutated,
                view.module(),
            )
            .unwrap_err();
            assert!(
                error.contains(if result == InvokeCallResultKind::I64 {
                    "entry-values-missing"
                } else {
                    "borrowed-incoming/result-mismatch"
                }),
                "{error}"
            );
        }
    }
}
