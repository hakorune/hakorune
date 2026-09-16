//! Root/Birth pair ordering and complete-program pins for the physical
//! lifecycle projection; sibling of `physical_program.rs`.
use super::*;
use crate::mir::compiler::normal_default_pipeline::{MirCompiler, NormalCompileRequestV1};
use crate::parser::NyashParser;
use std::collections::HashMap;

fn request(source: &str) -> NormalCompileRequestV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("exact callable parse");
    let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
        .expect("exact callable transform");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
    else {
        panic!("source identity must remain intact");
    };
    NormalCompileRequestV1::for_mir_mode_callable_source(source, None, HashMap::new())
}

#[test]
fn final_view_issues_complete_pair_program_in_root_then_birth_order() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler.compile_normal_with_published(
            request(include_str!(
                "../../../../../apps/typed-object-birth-min/main.hako"
            )),
            |view, _| -> Result<(), String> {
                let program = view.issue_lifecycle_physical_program()?;
                let contract = view.issue_lifecycle_compiled_entry_contract()?;
                let [root, birth] = program.functions() else {
                    panic!("Pair must retain root and one Birth function");
                };
                assert!(matches!(
                    root.role(),
                    PublishedLifecyclePhysicalFunctionRoleV1::Root {
                        result: CompiledEntryRootResultV1::I64,
                        ..
                    }
                ));
                let [entry_birth] = contract.births() else {
                    panic!("Pair must retain one compiled Birth contract");
                };
                assert_eq!(entry_birth.function_index(), 1);
                assert_eq!(entry_birth.formals().len(), 3);
                assert!(entry_birth.formals()[0].source_ordinal().is_none());
                assert_eq!(entry_birth.formals()[1].source_ordinal(), Some(0));
                assert!(entry_birth.formals()[1].disposition().is_some());
                let [birth_call] = contract.birth_calls() else {
                    panic!("Pair must retain one exact Birth call");
                };
                assert_eq!(birth_call.function_index(), 1);
                assert_eq!(birth_call.arguments().len(), 2);
                assert!(!contract.cleanup().is_empty());
                assert!(matches!(
                    birth.role(),
                    PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi }
                        if abi.abi().source_arity() == 2 && birth.params().len() == 3
                ));
                assert!(root
                    .blocks()
                    .windows(2)
                    .all(|blocks| blocks[0].id() < blocks[1].id()));
                let all = root.blocks().iter().flat_map(|block| {
                    block
                        .instructions()
                        .iter()
                        .copied()
                        .chain(std::iter::once(block.terminator()))
                });
                assert!(all.clone().any(|row| matches!(
                    row.instruction(),
                    MirInstruction::Const {
                        value: ConstValue::Integer(10 | 20),
                        ..
                    }
                )));
                assert!(all.clone().any(|row| matches!(
                    row.instruction(),
                    MirInstruction::BinOp {
                        op: BinaryOp::Add,
                        ..
                    }
                )));
                assert_eq!(
                    all.filter(|row| matches!(
                        row.instruction(),
                        MirInstruction::ObjectFieldGet { .. }
                    ))
                    .count(),
                    2,
                );
                Err("[freeze:contract][published-lifecycle/consumer-pending]".into())
            },
        );
        match result {
            Err(error) if error.contains("consumer-pending") => {}
            Err(error) => panic!("unexpected selected consumer error: {error}"),
            Ok(_) => panic!("selected consumer must propagate pending terminal"),
        }
    });
}

#[test]
fn nested_ordinary_call_chain_emits_per_function_call_rows() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let mut compiler = MirCompiler::with_options(false);
        let result = compiler.compile_normal_with_published(
            request(
                "static function inner(): i64 { return 7 }
                 static function helper(value: i64): i64 { return value }
                 static box Main { main() { return helper(10) } }",
            ),
            |view, _| -> Result<(), String> {
                // The producer lane cannot yet seal a call edge inside an
                // ordinary callee; inject the nested edge on a module clone
                // so the physical projection is pinned at its own layer.
                let handoff = view.retained_handoff.expect("one borrowed handoff");
                let mut nested = view.module().clone();
                let symbol_of = |fragment: &str| {
                    nested
                        .canonical_callable_definitions
                        .keys()
                        .map(|key| key.mir_symbol_projection())
                        .find(|symbol| symbol.contains(fragment))
                        .unwrap_or_else(|| panic!("{fragment} catalog symbol"))
                };
                let helper_symbol = symbol_of("helper");
                let inner_symbol = symbol_of("inner");
                let helper = nested.functions.get_mut(&helper_symbol).unwrap();
                let invoke_block = *helper.blocks.keys().next().unwrap();
                let normal = BasicBlockId::new(90);
                let fault_block = BasicBlockId::new(91);
                let nested_result = ValueId::new(900);
                let nested_call = MirCall::new(
                    None,
                    Callee::Global(
                        hakorune_mir_defs::CanonicalGlobalTargetV1::new_free_function(
                            "inner".into(),
                            0,
                        )
                        .unwrap(),
                    ),
                    vec![],
                );
                helper
                    .blocks
                    .get_mut(&invoke_block)
                    .unwrap()
                    .set_terminator(MirInstruction::Invoke {
                        operation: InvokeOperation::Call {
                            call: nested_call.clone(),
                            result: InvokeCallResultKind::I64,
                        },
                        fault_frame: ValueId::INVALID,
                        normal_landing: normal,
                        fault_landing: fault_block,
                    });
                let mut normal_block = crate::mir::BasicBlock::new(normal);
                normal_block.add_instruction(MirInstruction::InvokeNormalResult {
                    invoke_block,
                    dst: nested_result,
                });
                normal_block.set_terminator(MirInstruction::Return {
                    value: Some(nested_result),
                });
                helper.blocks.insert(normal, normal_block);
                let mut fault_block = crate::mir::BasicBlock::new(fault_block);
                fault_block.set_terminator(MirInstruction::ReturnFault {
                    fault_frame: ValueId::INVALID,
                });
                helper.blocks.insert(fault_block.id, fault_block);

                let admitted = super::super::super::lifecycle_admission::admit_lifecycle(
                    PublishedMirBackendView::try_new(&nested)
                        .map_err(|error| error.to_string())?
                        .bind_finalized_root_handoff(Some(handoff))?,
                    &super::super::PublishedObjectStorageProfileV1::SafeMutex,
                )?;
                let program = admitted.issue_lifecycle_physical_program()?;
                let contract = admitted.issue_lifecycle_compiled_entry_contract()?;
                assert_eq!(program.functions().len(), 3, "{:?}", program.functions());
                let index_of = |fragment: &str| {
                    program
                        .functions()
                        .iter()
                        .position(|function| function.name().contains(fragment))
                        .unwrap_or_else(|| panic!("{fragment} emitted function"))
                };
                let helper_index = index_of("helper");
                let inner_index = index_of("inner");
                assert!(matches!(
                    program.functions()[0].role(),
                    PublishedLifecyclePhysicalFunctionRoleV1::Root {
                        result: CompiledEntryRootResultV1::I64,
                        ..
                    }
                ));
                for (index, symbol) in [(helper_index, "helper"), (inner_index, "inner")] {
                    assert!(
                        matches!(
                            program.functions()[index].role(),
                            PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 {
                                key,
                                ..
                            } if key.mir_symbol_projection().contains(symbol)
                        ),
                        "{symbol} must keep its ordinary i64 role"
                    );
                }
                assert_eq!(program.functions()[helper_index].name(), helper_symbol);
                assert_eq!(program.functions()[inner_index].name(), inner_symbol);
                // The callee's own edge stays inside its emitted body.
                let nested_rows: Vec<_> = program.functions()[helper_index]
                    .blocks()
                    .iter()
                    .flat_map(|block| {
                        block
                            .instructions()
                            .iter()
                            .copied()
                            .chain(std::iter::once(block.terminator()))
                    })
                    .filter_map(|row| match row.instruction() {
                        MirInstruction::Invoke {
                            operation: InvokeOperation::Call { call, result },
                            ..
                        } => Some((call, *result)),
                        _ => None,
                    })
                    .collect();
                assert_eq!(
                    nested_rows,
                    [(&nested_call, InvokeCallResultKind::I64)],
                    "helper must carry exactly its own sealed call row"
                );
                // Each contract row names its exact caller.
                let mut callers: Vec<_> = contract
                    .ordinary_calls()
                    .iter()
                    .map(|call| (call.caller_function_index(), call.function_index()))
                    .collect();
                callers.sort();
                assert_eq!(
                    callers,
                    [
                        (0, helper_index as u32),
                        (helper_index as u32, inner_index as u32)
                    ],
                    "{callers:?}"
                );
                Err("[freeze:contract][published-lifecycle/consumer-pending]".into())
            },
        );
        match result {
            Err(error) if error.contains("consumer-pending") => {}
            Err(error) => panic!("unexpected selected consumer error: {error}"),
            Ok(_) => panic!("selected consumer must propagate pending terminal"),
        }
    });
}
