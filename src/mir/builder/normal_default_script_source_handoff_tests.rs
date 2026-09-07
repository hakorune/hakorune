//! Real Script source through both completed-root finishing consumers.
use super::super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use super::*;
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::parser::ParserBuildConfig;

fn completed(source: &str) -> CompletedNormalDefaultRootCatalogLifecycleV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    session()
        .complete_normal_default_program_root_catalog_lifecycle(
            callable_source(source, ParserBuildConfig::default()),
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed Script completion")
}

#[test]
fn script_source_survives_scope_and_reaches_both_finishing_consumers() {
    for terminal in ["return 30", "return"] {
        let source = format!(
            "local first: Array<i8> = [10, 20]\nlocal alias = first\n\
             local second: Array<u32> = []\n{terminal}"
        );
        let diagnostic = completed(&source);
        assert!(matches!(
            &diagnostic.root_validation,
            RootValidation::Script { .. }
        ));
        let (_, module, validate) = diagnostic.into_parts();
        validate(&module).expect("diagnostic source retention");
        let artifact = completed(&source);
        assert!(matches!(
            &artifact.root_validation,
            RootValidation::Script { .. }
        ));
        let (_, module, validate) = artifact.into_artifact_parts();
        let handoff = validate(&module)
            .expect("artifact source retention")
            .expect("selected Array has a final handoff");
        let array = handoff.script_array().expect("Script variant");
        assert_eq!(array.acquisition_count(), 2);
        array
            .validate_root_binding(&module.functions[handoff.root_key()])
            .unwrap();
        assert!(handoff.root_source().is_none());
        assert!(handoff.root_result().is_none());
        assert!(handoff.births().is_none());
        assert!(handoff.birth_keys().is_none());
    }
}

#[test]
fn both_finishing_consumers_reject_missing_or_foreign_retained_root() {
    for artifact in [false, true] {
        for mutation in 0..3 {
            let completed = completed("local a: Array<i64> = [10, 20]\nreturn 30");
            let key = completed.root_validation.key().unwrap().to_owned();
            let mutate = |module: &mut MirModule| match mutation {
                0 => {
                    module.functions.remove(&key);
                }
                1 => {
                    module.functions.get_mut(&key).unwrap().entry_block =
                        crate::mir::BasicBlockId::new(u32::MAX);
                }
                2 => {
                    module.functions.get_mut(&key).unwrap().signature.name = "foreign".into();
                }
                _ => unreachable!(),
            };
            let error = if artifact {
                let (_, mut module, validate) = completed.into_artifact_parts();
                mutate(&mut module);
                validate(&module).unwrap_err()
            } else {
                let (_, mut module, validate) = completed.into_parts();
                mutate(&mut module);
                validate(&module).unwrap_err()
            };
            assert!(
                error.contains(match mutation {
                    0 => "root-missing",
                    1 => "script-root-owner-drift",
                    2 => "root-key-drift",
                    _ => unreachable!(),
                }),
                "{error}"
            );
        }
    }
}

#[test]
fn array_emission_mutations_reject_in_both_finishing_consumers() {
    use crate::mir::instruction::InvokeOperation;
    use crate::mir::{MirInstruction, ValueId};
    for artifact in [false, true] {
        for mutation in 0..21 {
            let completed = completed("local a: Array<i64> = [10, 20]\nlocal alias = a\nlocal b: Array<u8> = []\nreturn 30");
            let key = completed.root_validation.key().unwrap().to_owned();
            let mutate = |module: &mut MirModule| {
                let root = module.functions.get_mut(&key).unwrap();
                let writes: Vec<_> = root
                    .blocks
                    .iter()
                    .filter_map(|(id, block)| {
                        matches!(
                            block.terminator,
                            Some(MirInstruction::Invoke {
                                operation: InvokeOperation::ArrayElementWrite { .. },
                                ..
                            })
                        )
                        .then_some(*id)
                    })
                    .collect();
                let allocation = root
                    .blocks
                    .iter()
                    .find_map(|(id, block)| {
                        matches!(
                            block.terminator,
                            Some(MirInstruction::Invoke {
                                operation: InvokeOperation::IntrinsicArrayNew,
                                ..
                            })
                        )
                        .then_some(*id)
                    })
                    .unwrap();
                let claim = root
                    .blocks
                    .iter()
                    .find_map(|(id, block)| {
                        matches!(
                            block.terminator,
                            Some(MirInstruction::Invoke {
                                operation: InvokeOperation::ArrayStateContractClaim { .. },
                                ..
                            })
                        )
                        .then_some(*id)
                    })
                    .unwrap();
                let returned = root
                    .blocks
                    .iter()
                    .find_map(|(id, block)| {
                        matches!(block.terminator, Some(MirInstruction::Return { .. }))
                            .then_some(*id)
                    })
                    .unwrap();
                let fault_of = |root: &crate::mir::MirFunction, block| match root.blocks[&block]
                    .terminator
                    .as_ref()
                    .unwrap()
                {
                    MirInstruction::Invoke { fault_landing, .. } => *fault_landing,
                    _ => unreachable!(),
                };
                match mutation {
                    0 => {
                        if let Some(MirInstruction::Invoke { operation, .. }) =
                            &mut root.blocks.get_mut(&allocation).unwrap().terminator
                        {
                            *operation = InvokeOperation::NewBox {
                                object:
                                    hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(
                                        0,
                                    )
                                    .unwrap(),
                            };
                        }
                    }
                    1 => {
                        if let Some(MirInstruction::Invoke {
                            operation: InvokeOperation::ArrayStateContractClaim { contract_id, .. },
                            ..
                        }) = &mut root.blocks.get_mut(&claim).unwrap().terminator
                        {
                            contract_id.push_str(":foreign");
                        }
                    }
                    2 => {
                        if let Some(MirInstruction::Invoke {
                            operation: InvokeOperation::ArrayElementWrite { value, .. },
                            ..
                        }) = &mut root.blocks.get_mut(&writes[0]).unwrap().terminator
                        {
                            *value = ValueId::new(u32::MAX);
                        }
                    }
                    3 => root.blocks.get_mut(&writes[0]).unwrap().terminator = None,
                    4 => {
                        let body = root.blocks.get_mut(&writes[0]).unwrap();
                        body.instructions.push(body.terminator.clone().unwrap());
                    }
                    5 => {
                        let a = root.blocks.get_mut(&writes[0]).unwrap().terminator.take();
                        let b = root
                            .blocks
                            .get_mut(&writes[1])
                            .unwrap()
                            .terminator
                            .replace(a.unwrap());
                        root.blocks.get_mut(&writes[0]).unwrap().terminator = b;
                    }
                    6 => {
                        let body = root
                            .blocks
                            .values_mut()
                            .find(|body| {
                                body.instructions
                                    .iter()
                                    .any(|inst| matches!(inst, MirInstruction::Copy { .. }))
                            })
                            .unwrap();
                        let pos = body
                            .instructions
                            .iter()
                            .position(|inst| matches!(inst, MirInstruction::Copy { .. }))
                            .unwrap();
                        body.instructions.remove(pos);
                    }
                    7 => root.metadata.typed_array_contract_sources.clear(),
                    8 => root.blocks.get_mut(&returned).unwrap().terminator = None,
                    9 => {
                        root.blocks.get_mut(&returned).unwrap().terminator =
                            Some(MirInstruction::Return {
                                value: Some(ValueId::new(u32::MAX)),
                            })
                    }
                    10 => {
                        for inst in &mut root.blocks.get_mut(&returned).unwrap().instructions {
                            if let MirInstruction::Const {
                                value: crate::mir::ConstValue::Integer(30),
                                dst,
                            } = inst
                            {
                                *inst = MirInstruction::Const {
                                    dst: *dst,
                                    value: crate::mir::ConstValue::Integer(31),
                                };
                                break;
                            }
                        }
                    }
                    11 => {
                        let block = fault_of(root, allocation);
                        root.blocks.get_mut(&block).unwrap().instructions.push(
                            MirInstruction::ArrayResidenceRelease {
                                value: ValueId::new(u32::MAX),
                            },
                        );
                    }
                    12 => {
                        let block = fault_of(root, claim);
                        root.blocks.get_mut(&block).unwrap().instructions.remove(0);
                    }
                    13 => {
                        let body = root.blocks.get_mut(&returned).unwrap();
                        body.instructions
                            .push(body.instructions.last().unwrap().clone());
                    }
                    14 => {
                        let block = fault_of(root, claim);
                        root.blocks.get_mut(&block).unwrap().terminator =
                            Some(MirInstruction::Return { value: None });
                    }
                    15 => {
                        let mut extra =
                            crate::mir::BasicBlock::new(crate::mir::BasicBlockId::new(u32::MAX));
                        extra.set_terminator(MirInstruction::Return { value: None });
                        root.add_block(extra);
                    }
                    16 => {
                        if let Some(MirInstruction::Invoke {
                            fault_landing,
                            normal_landing,
                            ..
                        }) = &mut root.blocks.get_mut(&writes[0]).unwrap().terminator
                        {
                            *fault_landing = *normal_landing;
                        }
                    }
                    17 => {
                        let inst = root
                            .blocks
                            .get_mut(&returned)
                            .unwrap()
                            .instructions
                            .last_mut()
                            .unwrap();
                        *inst = MirInstruction::ArrayResidenceRelease {
                            value: ValueId::new(u32::MAX),
                        };
                    }
                    18 => {
                        let body = root
                            .blocks
                            .values_mut()
                            .find(|body| {
                                matches!(body.terminator, Some(MirInstruction::ReturnFault { .. }))
                                    && body.instructions.len() == 2
                            })
                            .unwrap();
                        body.instructions.swap(0, 1);
                    }
                    19 => {
                        let body = root.blocks.get_mut(&returned).unwrap();
                        let n = body.instructions.len();
                        body.instructions.swap(n - 1, n - 2);
                    }
                    20 => {
                        let normal = match root.blocks[&claim].terminator.as_ref().unwrap() {
                            MirInstruction::Invoke { normal_landing, .. } => *normal_landing,
                            _ => unreachable!(),
                        };
                        root.blocks.get_mut(&normal).unwrap().instructions.insert(
                            0,
                            MirInstruction::InvokeNormalResult {
                                dst: ValueId::new(u32::MAX),
                                invoke_block: claim,
                            },
                        );
                    }
                    _ => unreachable!(),
                }
            };
            let error = if artifact {
                let (_, mut module, validate) = completed.into_artifact_parts();
                mutate(&mut module);
                validate(&module).unwrap_err()
            } else {
                let (_, mut module, validate) = completed.into_parts();
                mutate(&mut module);
                validate(&module).unwrap_err()
            };
            assert!(
                error.contains("[script-array/emission/"),
                "mutation {mutation}: {error}"
            );
        }
    }
}

#[test]
fn array_source_binding_survives_actual_compiler_finishing_with_optimization() {
    use crate::mir::{MirCompiler, NormalCompileRequestV1};
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_v1, NormalCallableMaterializationOutcomeV1,
    };
    for optimize in [false, true] {
        for (text, expected_stop) in [
            (
                "local seed = 10\nlocal a: Array<i64> = [10, 10]\nreturn 30",
                None,
            ),
            (
                "local a: Array<u8> = []\nlocal b: Array<i64> = [1.5, 1.5]\nreturn 30",
                None,
            ),
            ("local a: Array<i64> = []\nreturn", None),
            // The original f32 probe is outside the seven admitted annotations.
            // Keep its upstream Stop; it is not finishing evidence.
            (
                "local a: Array<u8> = []\nlocal b: Array<f32> = [1.5, 1.5]\nreturn 30",
                Some("ObservationDeferred"),
            ),
        ] {
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) =
                materialize_normal_callable_program_v1(
                    text.to_owned(),
                    ParserBuildConfig::default(),
                )
                .unwrap()
            else {
                panic!("source authority required")
            };
            let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                source,
                None,
                Default::default(),
            );
            let result = MirCompiler::with_options(optimize).compile_normal(request);
            if let Some(stop) = expected_stop {
                let error = result
                    .err()
                    .expect("unsupported annotation must stop upstream");
                assert!(error.contains(stop), "{text}: {error}");
                continue;
            }
            let result =
                result.unwrap_or_else(|error| panic!("optimize={optimize}, {text}: {error}"));
            assert!(
                result.verification_result.is_ok(),
                "{:?}",
                result.verification_result
            );
            let copies = result
                .module
                .functions
                .values()
                .flat_map(|root| root.blocks.values())
                .flat_map(|block| &block.instructions)
                .filter(|instruction| {
                    matches!(instruction, crate::mir::MirInstruction::Copy { .. })
                })
                .count();
            let homes: usize = result
                .module
                .functions
                .values()
                .map(|root| root.metadata.typed_array_contract_sources.len())
                .sum();
            if optimize {
                assert_eq!(
                    copies, homes,
                    "DCE removes dead aliases but keeps every cleanup Home"
                );
            } else {
                assert!(copies >= homes && homes > 0);
            }
        }
    }
}

#[test]
fn script_array_artifact_requires_finishing_and_preserves_unissued_distinction() {
    let finished = completed("local a: Array<i64> = []\nreturn 30");
    let RootValidation::Script { source, entry, .. } = finished.root_validation else {
        panic!("Script source required")
    };
    let error = source.into_array_artifact(entry).unwrap_err();
    assert!(error.contains("artifact-before-finishing"), "{error}");
    let (_, module, validate) = completed("local scalar = 1\nreturn 30").into_artifact_parts();
    assert!(
        validate(&module).unwrap().is_none(),
        "unissued Array is not an empty Array product"
    );
}

#[test]
fn source_array_control_has_exact_fault_sets_and_reverse_terminal_homes() {
    use crate::mir::instruction::InvokeOperation;
    use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
    fn releases(root: &MirFunction, block: BasicBlockId) -> Vec<ValueId> {
        root.blocks[&block]
            .instructions
            .iter()
            .filter_map(|inst| match inst {
                MirInstruction::ArrayResidenceRelease { value } => Some(*value),
                _ => None,
            })
            .collect()
    }
    for spec in ["i8", "i16", "i32", "i64", "u8", "u16", "u32"] {
        for terminal in ["return 30", "return"] {
            let source = format!("local a: Array<{spec}> = [10, 20]\nlocal alias = a\nlocal b: Array<{spec}> = []\n{terminal}");
            let completed = completed(&source);
            let key = completed.root_validation.key().unwrap().to_owned();
            let (_, module, validate) = completed.into_artifact_parts();
            validate(&module).unwrap().unwrap();
            let root = &module.functions[&key];
            let mut allocations = root
                .blocks
                .iter()
                .filter_map(|(id, body)| match body.terminator.as_ref() {
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::IntrinsicArrayNew,
                        normal_landing,
                        fault_landing,
                        fault_frame,
                    }) => Some((*id, *normal_landing, *fault_landing, *fault_frame)),
                    _ => None,
                })
                .collect::<Vec<_>>();
            allocations.sort_by_key(|row| row.0);
            assert_eq!(allocations.len(), 2);
            let mut homes = Vec::new();
            for (origin, normal, allocation_fault, frame) in allocations {
                assert_eq!(
                    releases(root, allocation_fault),
                    homes.iter().rev().copied().collect::<Vec<_>>()
                );
                assert_eq!(
                    root.blocks[&allocation_fault].terminator,
                    Some(MirInstruction::ReturnFault { fault_frame: frame })
                );
                let allocation = match root.blocks[&normal].instructions[0] {
                    MirInstruction::InvokeNormalResult { dst, invoke_block }
                        if invoke_block == origin =>
                    {
                        dst
                    }
                    ref unexpected => panic!("normal allocation result: {unexpected:?}"),
                };
                let (mut next, acquired_fault) =
                    match root.blocks[&normal].terminator.as_ref().unwrap() {
                        MirInstruction::Invoke {
                            operation: InvokeOperation::ArrayStateContractClaim { array, .. },
                            normal_landing,
                            fault_landing,
                            fault_frame,
                        } if *array == allocation && *fault_frame == frame => {
                            (*normal_landing, *fault_landing)
                        }
                        unexpected => panic!("claim: {unexpected:?}"),
                    };
                let expected_fault: Vec<_> = std::iter::once(allocation)
                    .chain(homes.iter().rev().copied())
                    .collect();
                assert_eq!(releases(root, acquired_fault), expected_fault);
                assert_eq!(
                    root.blocks[&acquired_fault].terminator,
                    Some(MirInstruction::ReturnFault { fault_frame: frame })
                );
                while let Some(MirInstruction::Invoke {
                    operation: InvokeOperation::ArrayElementWrite { receiver, .. },
                    normal_landing,
                    fault_landing,
                    fault_frame,
                }) = root.blocks[&next].terminator.as_ref()
                {
                    assert_eq!(*receiver, allocation);
                    assert_eq!(*fault_landing, acquired_fault);
                    assert_eq!(*fault_frame, frame);
                    next = *normal_landing;
                }
                let local = root.blocks[&next]
                    .instructions
                    .iter()
                    .find_map(|inst| match inst {
                        MirInstruction::Copy { dst, src } if *src == allocation => Some(*dst),
                        _ => None,
                    })
                    .expect("Local commit follows all successful writes");
                homes.push(local);
            }
            let returns = root
                .blocks
                .iter()
                .filter(|(_, body)| matches!(body.terminator, Some(MirInstruction::Return { .. })))
                .collect::<Vec<_>>();
            let [(block, body)] = returns.as_slice() else {
                panic!("one Normal Return")
            };
            assert_eq!(
                releases(root, **block),
                homes.into_iter().rev().collect::<Vec<_>>()
            );
            if terminal == "return" {
                assert_eq!(
                    body.terminator,
                    Some(MirInstruction::Return { value: None })
                );
            }
            assert!(root.blocks.values().all(|body| !matches!(
                body.terminator,
                Some(MirInstruction::Jump { .. } | MirInstruction::Branch { .. })
            )));
        }
    }
}
