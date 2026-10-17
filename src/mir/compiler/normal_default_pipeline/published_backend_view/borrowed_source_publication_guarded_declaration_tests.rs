//! Original guarded declaration, real outgoing packet, and independent final gates.
use super::*;

#[test]
fn guarded_zero_call_declaration_publishes_original_entry_and_rejects_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for (text, arity) in [
            ("box Token {} box Maker { make(size) { if size <= 0 { return new Token() } return new Token() } qualified() { return me.make(7) } relay(size) { if size <= 0 { return 0 } local item = me.make(size) return 0 } } static box Main { main() { return 0 } }", 1),
            ("box Token {} box Maker { make(size) { if size <= 0 { return new Token() } return new Token() } qualified() { return me.make(7) } relay(token: Token, size) { if token == null { return 0 } if size <= 0 { return 0 } local item = me.make(size) return 0 } } static box Main { main() { return 0 } }", 2),
        ] {
        MirCompiler::with_options(false).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
            require_artifact_verification(verification);
            let handoff = view.retained_handoff.ok_or("handoff missing")?;
            let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Maker", "relay", arity);
            let owner = handoff.callables().and_then(|rows| rows.owner_for_canonical_key(&key)).ok_or("original owner missing")?;
            let symbol = view.module().canonical_callable_definition_symbol(&key).ok_or("canonical symbol missing")?;
            let function = &view.module().functions[symbol];
            let source = handoff.root_source().ok_or("source missing")?;
            source.borrowed_declaration_for_function_v1(owner, &key, function)?;
            assert_eq!(source.borrowed_ordinary_entry_source_for_function_v1(owner, function)?.incoming_targets().count(), 0);
            let input = view.issue_lifecycle_physical_abi_input()?;
            let wire = crate::mir::emit_lifecycle_physical_abi_json(&input)?;
            let json: serde_json::Value = serde_json::from_str(&wire).unwrap();
            let declaration = json["functions"].as_array().unwrap().iter()
                .find(|row| row["name"].as_str() == Some(symbol)).unwrap();
            assert_eq!(declaration["params"].as_array().unwrap().len(), arity as usize);
            if arity == 2 { assert!(declaration["params"][0]["object_view"].is_number()); }
            let qualified = input.program().functions().iter().find(|row| row.name() == "Maker.qualified/0")
                .expect("original incoming caller retained without a fabricated call");
            assert!(matches!(qualified.role(), PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryHandle { .. }));
            assert!(input.program().functions().iter().any(|row| row.name() == symbol));
            crate::mir::verification::MirVerifier::new().verify_module_with_finalized_root(view.module(), Some(handoff))
                .map_err(|errors| format!("valid final module: {errors:?}"))?;
            for mutation in 0..6 {
                let mut module = view.module().clone();
                let changed = module.functions.get_mut(symbol).unwrap();
                match mutation {
                    0 => changed.params[1] = ValueId(u32::MAX),
                    1 => changed.metadata.physical_param_carriers = Some(vec![Carrier::BorrowedTaggedValue, Carrier::BorrowedTaggedValue].into_boxed_slice()),
                    2 => changed.signature.params.pop().map(|_| ()).unwrap(),
                    3 => changed.signature.params[0] = crate::mir::MirType::Integer,
                    4 => changed.signature.params[0] = crate::mir::MirType::Box("Foreign".into()),
                    _ => changed.signature.params[1] = crate::mir::MirType::Bool,
                }
                let errors = crate::mir::verification::MirVerifier::new()
                    .verify_module_with_finalized_root(&module, Some(handoff)).unwrap_err();
                assert!(format!("{errors:?}").contains("borrowed-declaration/entry-correspondence"));
                let error = super::super::super::compiled_entry_contract::verify_borrowed_call_incoming(input.program(), &module).unwrap_err();
                assert!(error.contains("borrowed-declaration/entry-correspondence"), "{error}");
            }
            let mut missing_compare = view.module().clone();
            for block in missing_compare.functions.get_mut(symbol).unwrap().blocks.values_mut() {
                block.instructions.retain(|row| !matches!(row, MirInstruction::Compare { .. }));
            }
            assert!(super::super::super::compiled_entry_contract::verify_borrowed_call_incoming(input.program(), &missing_compare).is_err());
            // Projection-only drift cannot inherit the correct module proof.
            let mut program = input.program().clone();
            let declaration = program.functions.iter_mut().find(|row| row.name() == symbol).unwrap();
            declaration.param_carriers = Some(&[Carrier::BorrowedTaggedValue, Carrier::BorrowedTaggedValue]);
            assert!(super::super::super::compiled_entry_contract::verify_borrowed_call_incoming(&program, view.module()).is_err());
            Ok(())
        }).unwrap();
        }
    });
}

#[test]
fn source_bool_terminal_publishes_original_result_through_final() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Handle { block_id: i64 birth() { me.block_id = 0 } } box Transport { flag: i64 birth() { me.flag = 0 } probe(p: Handle) { if p == null { return false } return true } } box Parent { child: Transport birth() { me.child = new Transport() } run(p: Handle) { if p == null { return false } return me.child.probe(p) } qualified() { local h = new Handle() return me.run(h) } } static box Main { main() { return 0 } }";
        MirCompiler::with_options(false).compile_normal_with_published(request(text), |view, verification| -> Result<(), String> {
            require_artifact_verification(verification);
            let handoff = view.retained_handoff.ok_or("handoff missing")?;
            crate::mir::verification::MirVerifier::new().verify_module_with_finalized_root(view.module(), Some(handoff))
                .map_err(|errors| format!("valid final module: {errors:?}"))?;
            view.issue_lifecycle_physical_abi_input()?;
            let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::instance_box_method("Parent", "run", 1);
            let symbol = view.module().canonical_callable_definition_symbol(&key).ok_or("canonical symbol missing")?;
            let function = &view.module().functions[symbol];
            assert_eq!(function.signature.return_type, crate::mir::MirType::Bool);
            for change_signature in [true, false] {
                let mut drifted = view.module().clone();
                let function = drifted.functions.get_mut(symbol).unwrap();
                if change_signature {
                    function.signature.return_type = crate::mir::MirType::Integer;
                } else {
                    let mut changed = 0;
                    for block in function.blocks.values_mut() {
                        if let Some(MirInstruction::Invoke { operation: InvokeOperation::Call { result, .. }, .. }) = block.terminator.as_mut() {
                            if *result == InvokeCallResultKind::Bool {
                                *result = InvokeCallResultKind::I64;
                                changed += 1;
                            }
                        }
                    }
                    assert_eq!(changed, 1);
                }
                let rejected = PublishedMirBackendView::try_new(&drifted).map_err(|error| error.to_string())
                    .and_then(|view| view.bind_finalized_root_handoff(Some(handoff)))
                    .and_then(|view| crate::mir::compiler::normal_default_pipeline::lifecycle_admission::admit_lifecycle(
                        view, &crate::mir::compiler::normal_default_pipeline::published_backend_view::PublishedObjectStorageProfileV1::SafeMutex))
                    .and_then(|view| view.issue_lifecycle_physical_abi_input().map(|_| ()));
                assert!(rejected.is_err(), "physical result drift must refuse");
            }
            Ok(())
        }).unwrap();
    });
}
