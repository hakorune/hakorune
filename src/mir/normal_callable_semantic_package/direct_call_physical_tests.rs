//! Source-to-selected-consumer dependency; ordinary package install stays stopped.
use super::*;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::{MirBuilder, MirInstruction, MirModule, ValueId};

#[test]
fn source_terminal_call_preserves_both_cleanup_paths_through_finishing() {
    for prefix in [
        "",
        "local a = new Page()",
        "local a = new Page() local b = new Page()",
    ] {
        for optimize in [false, true] {
            let mut package = issue(&format!(
                "box Page {{}} static box Main {{
                main() {{ {prefix} return helper(30, 5) }}
                helper(value: i64, other: i64): i64 {{ local m = %{{\"v\" => value}} return 30 }}
            }}"
            ))
            .unwrap();
            let mut loan = package.app_main_direct_call_loan.take().unwrap();
            let main = package
                .declaration_catalog()
                .source_backed_app_main()
                .unwrap();
            let declaration = package
                .batch()
                .declarations()
                .find(|row| row.identity().same_as(main.parser_identity()))
                .unwrap();
            let mut builder = MirBuilder::new();
            let mut function = package
                .batch()
                .with_lowering_input_and_source_identity(
                    declaration.batch_slot(),
                    |input, identity| {
                        builder.lower_map_dependency_for_test(
                            input,
                            SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                            main.parser_identity(),
                            identity.method_source_observation().cloned(),
                            std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                            Some(&mut loan),
                        )
                    },
                )
                .unwrap()
                .unwrap_or_else(|e| panic!("{prefix}: {e}"));
            loan.finish_empty().unwrap();
            let ledger = &package.ordinary_new_claim_ledger;
            crate::mir::verification::MirVerifier::new_strict()
                .verify_function(&function)
                .unwrap();
            let observation = ledger
                .validate_finalized_new_root(&function)
                .unwrap_or_else(|e| panic!("{prefix}: {e}"));
            function
                .install_root_ordinary_new_observation(observation)
                .unwrap();
            let mut module = MirModule::new("call-cleanup-dependency".into());
            module.functions.insert("Main.main/0".into(), function);
            if optimize {
                crate::mir::passes::simplify_cfg::simplify(&mut module);
            }
            let function = &module.functions["Main.main/0"];
            crate::mir::verification::MirVerifier::new_strict()
                .verify_function(function)
                .unwrap();
            for mutation in 0..5 {
                let mut changed = function.clone();
                let mut found = false;
                for block in changed.blocks.values_mut() {
                    if let Some(MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Call {
                                call,
                                result: InvokeCallResultKind::I64,
                            },
                        fault_frame,
                        normal_landing,
                        fault_landing,
                    }) = &mut block.terminator
                    {
                        match mutation {
                            0 => std::mem::swap(normal_landing, fault_landing),
                            1 => *fault_landing = *normal_landing,
                            2 => *fault_frame = ValueId(999),
                            3 => call.args.swap(0, 1),
                            _ => call.dst = Some(ValueId(999)),
                        }
                        found = true;
                        break;
                    }
                }
                assert!(found);
                assert!(
                    ledger.validate_after_compiler_finishing(&changed).is_err(),
                    "mutation {mutation}"
                );
            }
            let (call_id, normal, fault) = function
                .blocks
                .iter()
                .find_map(|(id, block)| match &block.terminator {
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call { .. },
                        normal_landing,
                        fault_landing,
                        ..
                    }) => Some((*id, *normal_landing, *fault_landing)),
                    _ => None,
                })
                .unwrap();
            for mutation in 0..3 {
                let mut changed = function.clone();
                match mutation {
                    0 => {
                        let mut foreign =
                            crate::mir::BasicBlock::new(crate::mir::BasicBlockId(999));
                        foreign.set_terminator(MirInstruction::Jump {
                            target: normal,
                            edge_args: None,
                        });
                        changed.add_block(foreign);
                    }
                    1 => changed.blocks.get_mut(&fault).unwrap().add_instruction(
                        MirInstruction::InvokeNormalResult {
                            invoke_block: call_id,
                            dst: ValueId(999),
                        },
                    ),
                    _ => {
                        let terminal = changed
                            .blocks
                            .values_mut()
                            .find_map(|block| {
                                matches!(block.terminator, Some(MirInstruction::ReturnFault { .. }))
                                    .then_some(&mut block.terminator)
                            })
                            .unwrap();
                        *terminal = Some(MirInstruction::Return {
                            value: Some(ValueId(999)),
                        });
                    }
                }
                assert!(
                    ledger.validate_after_compiler_finishing(&changed).is_err(),
                    "cleanup mutation {mutation}"
                );
            }
            let pending_head = follow_jumps(function, fault);
            if let Some(MirInstruction::Invoke {
                operation: InvokeOperation::HomeRelease { .. },
                normal_landing: next,
                ..
            }) = &function.blocks[&pending_head].terminator
            {
                let next = *next;
                for mutation in 0..3 {
                    let mut changed = function.clone();
                    let block = changed.blocks.get_mut(&pending_head).unwrap();
                    if mutation == 0 {
                        block.set_terminator(MirInstruction::Jump {
                            target: next,
                            edge_args: None,
                        });
                    } else if let Some(MirInstruction::Invoke {
                        operation,
                        normal_landing,
                        ..
                    }) = &mut block.terminator
                    {
                        if mutation == 1 {
                            *normal_landing = normal;
                        } else if let InvokeOperation::HomeRelease { value, .. } = operation {
                            *value = ValueId(999);
                        }
                    }
                    assert!(
                        ledger.validate_after_compiler_finishing(&changed).is_err(),
                        "pending Home mutation {mutation}"
                    );
                }
                let next_head = follow_jumps(function, next);
                if matches!(
                    function.blocks[&next_head].terminator,
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::HomeRelease { .. },
                        ..
                    })
                ) {
                    let mut changed = function.clone();
                    let Some(MirInstruction::Invoke {
                        operation: second, ..
                    }) = function.blocks[&next_head].terminator.as_ref()
                    else {
                        unreachable!()
                    };
                    let Some(MirInstruction::Invoke {
                        operation: first, ..
                    }) = function.blocks[&pending_head].terminator.as_ref()
                    else {
                        unreachable!()
                    };
                    for (id, replacement) in [(pending_head, second), (next_head, first)] {
                        let Some(MirInstruction::Invoke { operation, .. }) =
                            &mut changed.blocks.get_mut(&id).unwrap().terminator
                        else {
                            unreachable!()
                        };
                        *operation = replacement.clone();
                    }
                    assert!(
                        ledger.validate_after_compiler_finishing(&changed).is_err(),
                        "reordered Homes"
                    );
                }
            }
            ledger
                .validate_after_compiler_finishing(function)
                .unwrap_or_else(|e| panic!("{prefix}, optimize={optimize}: {e}"));
        }
    }
}

fn follow_jumps(
    function: &crate::mir::MirFunction,
    mut id: crate::mir::BasicBlockId,
) -> crate::mir::BasicBlockId {
    for _ in 0..=function.blocks.len() {
        match function.blocks[&id].terminator.as_ref() {
            Some(MirInstruction::Jump { target, .. }) => id = *target,
            _ => return id,
        }
    }
    panic!("cyclic source cleanup");
}
