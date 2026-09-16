//! Mutation witnesses for the finalization-owned physical admission boundary.
//! The handoff is issued from normal source; mutated modules never reach an artifact.

use super::super::tests::published_request;
use super::*;
use crate::mir::compiler::MirCompiler;
use crate::mir::{BasicBlock, BasicBlockId, EffectMask};
use hakorune_mir_defs::CanonicalSameModuleCallableKeyV1;
use std::collections::BTreeSet;

#[test]
fn scalar_local_then_terminal_call_reaches_published_lifecycle() {
    use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
    use crate::mir::MirInstruction;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for source in [
            "static box Main { main() { local first = helper(10) return helper(20) }
             helper(value: i64): i64 { return value } }",
            "static function helper(value: i64): i64 { return value }
             static box Main { main() { local first = helper(10) return helper(20) } }",
        ] {
            for optimize in [false, true] {
                let result = MirCompiler::with_options(optimize)
                    .compile_normal_with_published(
                        published_request(source),
                        |view, verification| {
                            assert!(verification.is_ok(), "{verification:?}");
                            let root = view.retained_root().expect("source root");
                            let instructions: Vec<_> = root
                                .blocks
                                .values()
                                .flat_map(|block| block.all_instructions())
                                .collect();
                            let mut results = std::collections::BTreeMap::new();
                            for (origin, block) in &root.blocks {
                                let Some(MirInstruction::Invoke {
                                    operation:
                                        InvokeOperation::Call {
                                            call,
                                            result: InvokeCallResultKind::I64,
                                        },
                                    normal_landing,
                                    ..
                                }) = &block.terminator
                                else {
                                    continue;
                                };
                                assert_eq!(call.args.len(), 1);
                                let literal = instructions
                                    .iter()
                                    .find_map(|instruction| match instruction {
                                        MirInstruction::Const {
                                            dst,
                                            value: crate::mir::ConstValue::Integer(value),
                                        } if *dst == call.args[0] => Some(*value),
                                        _ => None,
                                    })
                                    .expect("source literal operand");
                                let result = root.blocks[normal_landing]
                                    .all_instructions()
                                    .find_map(|instruction| match instruction {
                                        MirInstruction::InvokeNormalResult {
                                            invoke_block,
                                            dst,
                                        } if invoke_block == origin => Some(*dst),
                                        _ => None,
                                    })
                                    .expect("each Call retains its exact Normal projection");
                                assert!(results.insert(literal, result).is_none());
                            }
                            assert_eq!(results.keys().copied().collect::<Vec<_>>(), [10, 20]);
                            assert!(
                                instructions.iter().any(|instruction| matches!(
                                    instruction, MirInstruction::Return { value: Some(value) }
                                        if *value == results[&20]
                                )),
                                "return must use the second Call result"
                            );
                            view.issue_lifecycle_physical_abi_input()?;
                            Ok::<(), String>(())
                        },
                    )
                    .expect("scalar local/terminal source must publish");
                assert!(matches!(
                    result,
                    super::super::NormalPublishedCompileOutcome::Consumed(())
                ));
            }
        }
    });
}

#[test]
fn scalar_local_then_plain_return_preserves_ordinary_call_through_publication() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            let result = MirCompiler::with_options(optimize)
                .compile_normal(published_request(
                    "static box Main { main() { local first = helper(10) return 0 }
                 helper(value: i64): i64 { return value } }",
                ))
                .expect("Plain source still finishes and publishes");
            assert!(
                result.verification_result.is_ok(),
                "{:?}",
                result.verification_result
            );
            let instructions: Vec<_> = result
                .module
                .functions
                .values()
                .flat_map(|function| function.blocks.values())
                .flat_map(|block| block.all_instructions())
                .collect();
            assert_eq!(
                instructions
                    .iter()
                    .filter(|instruction| matches!(instruction, MirInstruction::Call(_)))
                    .count(),
                1
            );
            assert!(
                !instructions.iter().any(|instruction| matches!(
                    instruction,
                    MirInstruction::Invoke {
                        operation: crate::mir::instruction::InvokeOperation::Call { .. },
                        ..
                    }
                )),
                "Plain must not acquire lifecycle local Call bindings"
            );
            assert!(
                !instructions
                    .iter()
                    .any(|instruction| matches!(instruction, MirInstruction::ReturnFault { .. })),
                "empty-Home Plain must not publish an orphan Fault terminal"
            );
        }
    });
}

#[test]
fn lifecycle_admission_preserves_foreign_return_only_and_birth_call_rejections() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(
                published_request(include_str!(
                    "../../../../apps/typed-object-birth-min/main.hako"
                )),
                |view, _| {
                    let no_ordinary = BTreeSet::new();
                    let root = view.retained_root().unwrap().signature.name.as_str();
                    let births = view.retained_birth_abi().unwrap();
                    let key = births[0].target();
                    let symbol = view
                        .module()
                        .canonical_callable_definition_symbol(key)
                        .unwrap();
                    assert_eq!(
                        validate_functions(view.module(), Some(root), births, &no_ordinary),
                        Ok(())
                    );

                    let mut foreign = view.module().clone();
                    foreign.canonical_callable_definitions.remove(key);
                    assert_eq!(
                        validate_functions(&foreign, Some(root), births, &no_ordinary),
                        Err(fault("function-not-cataloged"))
                    );

                    let mut non_birth = view.module().clone();
                    non_birth.canonical_callable_definitions.remove(key);
                    non_birth.canonical_callable_definitions.insert(
                        CanonicalSameModuleCallableKeyV1::free_function("foreign", 0),
                        symbol.to_owned(),
                    );
                    assert_eq!(
                        validate_functions(&non_birth, Some(root), births, &no_ordinary),
                        Err(fault("function-not-birth"))
                    );

                    let mut return_only = view.module().clone();
                    let function = return_only.functions.get_mut(symbol).unwrap();
                    function.blocks.clear();
                    let mut block = BasicBlock::new(BasicBlockId::new(0));
                    block.terminator = Some(MirInstruction::Return { value: None });
                    function.blocks.insert(block.id, block);
                    assert_eq!(
                        validate_functions(&return_only, Some(root), births, &no_ordinary),
                        Ok(())
                    );
                    return_only
                        .functions
                        .get_mut(symbol)
                        .unwrap()
                        .signature
                        .name
                        .push_str("-drift");
                    assert_eq!(
                        validate_functions(&return_only, Some(root), births, &no_ordinary),
                        Err(fault("function-not-birth"))
                    );

                    for caller in [root, symbol] {
                        let mut bad_call = view.module().clone();
                        bad_call
                            .functions
                            .get_mut(caller)
                            .unwrap()
                            .blocks
                            .values_mut()
                            .next()
                            .unwrap()
                            .add_instruction(MirInstruction::call(
                                None,
                                Callee::BirthConstructor {
                                    key: key.clone(),
                                    receiver: ValueId::INVALID,
                                },
                                vec![],
                                EffectMask::PURE,
                            ));
                        assert_eq!(
                            validate_functions(&bad_call, Some(root), births, &no_ordinary),
                            Err(fault("birth-call-drift"))
                        );
                    }
                    Ok(())
                },
            )
            .expect("normal source reaches the admission observer once");
    });
}

#[test]
fn lifecycle_admission_collects_call_edges_module_wide() {
    use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false)
            .compile_normal_with_published(
                published_request(
                    "static function inner(): i64 { return 7 }
                     static function helper(value: i64): i64 { return value }
                     static box Main { main() { return helper(10) } }",
                ),
                |view, _| {
                    let root = view.retained_root().unwrap().signature.name.as_str();
                    let births = view.retained_birth_abi().unwrap_or_default();
                    let names = ordinary_call_names(view.module())?;
                    assert_eq!(names.len(), 1, "{names:?}");
                    let helper_symbol = names.iter().next().unwrap().clone();
                    assert!(helper_symbol.contains("helper"), "{helper_symbol}");
                    assert_eq!(
                        validate_functions(view.module(), Some(root), births, &names),
                        Ok(())
                    );

                    // A nested edge inside a non-root caller carries the same
                    // membership authority as a root edge, for either result
                    // kind. `inner` gains an Invoke row so it must itself be
                    // an admitted call target rather than a stray function.
                    for result in [InvokeCallResultKind::I64, InvokeCallResultKind::Map] {
                        let mut nested = view.module().clone();
                        let nested_call = |target: &str, arity: u32| MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                call: crate::mir::definitions::MirCall::new(
                                    None,
                                    Callee::Global(
                                        hakorune_mir_defs::CanonicalGlobalTargetV1::new_free_function(
                                            target.into(),
                                            arity,
                                        )
                                        .unwrap(),
                                    ),
                                    vec![],
                                ),
                                result,
                            },
                            fault_frame: ValueId::INVALID,
                            normal_landing: BasicBlockId::new(90),
                            fault_landing: BasicBlockId::new(91),
                        };
                        nested
                            .functions
                            .get_mut(&helper_symbol)
                            .unwrap()
                            .blocks
                            .values_mut()
                            .next()
                            .unwrap()
                            .set_terminator(nested_call("inner", 0));
                        let inner_symbol = nested
                            .canonical_callable_definitions
                            .keys()
                            .find(|key| key.mir_symbol_projection().contains("inner"))
                            .map(|key| key.mir_symbol_projection())
                            .expect("inner catalog key");
                        nested
                            .functions
                            .get_mut(&inner_symbol)
                            .unwrap()
                            .blocks
                            .values_mut()
                            .next()
                            .unwrap()
                            .set_terminator(nested_call("helper", 1));
                        let names = ordinary_call_names(&nested)?;
                        assert!(
                            names.iter().any(|name| name.contains("inner")),
                            "{result:?} edge inside a non-root caller must admit inner: {names:?}"
                        );
                        assert_eq!(
                            validate_functions(&nested, Some(root), births, &names),
                            Ok(()),
                            "{result:?} nested callee membership must pass"
                        );
                        // Without an entry exemption every lifecycle function
                        // needs edge membership or Birth namespace evidence.
                        assert!(
                            validate_functions(&nested, None, births, &names).is_err(),
                            "rootless validation must not exempt the entry-shaped root"
                        );
                    }
                    Ok(())
                },
            )
            .expect("nested-edge source must reach the admission observer");
    });
}
