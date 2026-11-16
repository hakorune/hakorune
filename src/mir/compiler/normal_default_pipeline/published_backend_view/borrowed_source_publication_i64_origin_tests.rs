//! The source read and its real operand remain tied after Copy/If joins.
use super::*;
use crate::mir::verification::scalar_origin::{same_i64_definition, SameI64Definition};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

const SOURCE: &str = r#"
static box Main {
    seed(): i64 { return 7 }
    other(): i64 { return 9 }
    main() {
        local value = me.seed()
        local other = me.other()
        local seen = 0
        if value == 7 {
            if value != 0 { seen = seen + 1 }
        }
        return value + 0
    }
}
"#;

fn call_result(function: &MirFunction, method: &str) -> ValueId {
    let key =
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Main", method, 0);
    let target = crate::mir::Callee::Global(key.canonical_global_target_v1().unwrap());
    function.blocks.values().flat_map(|block| block.all_instructions()).find_map(|row| match row {
        MirInstruction::Call(call) if call.callee == target => call.dst,
        MirInstruction::InvokeNormalResult { invoke_block, dst } => {
            matches!(function.blocks[invoke_block].terminator.as_ref(),
                Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. }) if call.callee == target)
                .then_some(*dst)
        }
        _ => None,
    }).expect("the actual distinct call result")
}

fn instruction_mut(
    function: &mut MirFunction,
    block: BasicBlockId,
    index: usize,
) -> &mut MirInstruction {
    let body = function.blocks.get_mut(&block).unwrap();
    if index == body.instructions.len() {
        body.terminator.as_mut().unwrap()
    } else {
        &mut body.instructions[index]
    }
}

#[test]
fn i64_call_read_join_rejects_operand_phi_and_live_definition_drift() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false).compile_normal_with_published(request(SOURCE), |view, verification| {
            require_artifact_verification(verification);
            view.issue_lifecycle_physical_abi_input()?;
            let symbol = view.retained_root().unwrap().signature.name.clone();
            let function = &view.module().functions[&symbol];
            let seed = call_result(function, "seed");
            let other = call_result(function, "other");
            assert_ne!(seed, other);
            let returned = function.blocks.values().flat_map(|b| b.all_instructions()).find_map(|row|
                match row { MirInstruction::Return { value } => *value, _ => None }).unwrap();
            let geometry = crate::mir::verification::geometry::FunctionVerificationGeometry::new(function);
            let (consumer_block, consumer_index, read) = if same_i64_definition(function, Some(view.module()), returned, seed).unwrap() == SameI64Definition::Same {
                let (block, body) = function.blocks.iter().find(|(_, b)| matches!(b.terminator, Some(MirInstruction::Return { value: Some(v) }) if v == returned)).unwrap();
                (*block, body.instructions.len(), returned)
            } else {
                let (block, index, row) = geometry.definition(returned).unwrap();
                let (block, index, row) = match row {
                    MirInstruction::InvokeNormalResult { invoke_block, .. } => (*invoke_block, function.blocks[invoke_block].instructions.len(), function.blocks[invoke_block].terminator.as_ref().unwrap()),
                    _ => (block, index, row),
                };
                let reads: Vec<_> = row.used_values().into_iter().filter(|value|
                    same_i64_definition(function, Some(view.module()), *value, seed).unwrap() == SameI64Definition::Same).collect();
                assert_eq!(reads.len(), 1, "one source read in the real result consumer");
                (block, index, reads[0])
            };
            let mut phi = read;
            while let Some((_, _, MirInstruction::Copy { src, .. })) = geometry.definition(phi) { phi = *src; }
            let (phi_block, phi_index, phi_row) = geometry.definition(phi).unwrap();
            assert!(matches!(phi_row, MirInstruction::Phi { .. }), "the actual If join is retained");
            let mut changed = view.module().clone();
            match instruction_mut(changed.functions.get_mut(&symbol).unwrap(), consumer_block, consumer_index) {
                MirInstruction::Invoke { operation, .. } => operation.rewrite_values(|value| if *value == read { *value = other }),
                MirInstruction::BinOp { lhs, rhs, .. } | MirInstruction::Compare { lhs, rhs, .. } => {
                    if *lhs == read { *lhs = other; }
                    if *rhs == read { *rhs = other; }
                }
                MirInstruction::Return { value: Some(value) } => { assert_eq!(*value, read); *value = other; }
                _ => panic!("expected the real scalar result consumer"),
            }
            reject_changed_module(&changed, view.retained_handoff);
            let mut changed = view.module().clone();
            let MirInstruction::Phi { inputs, .. } = instruction_mut(changed.functions.get_mut(&symbol).unwrap(), phi_block, phi_index) else { unreachable!() };
            inputs[0].1 = other;
            assert_eq!(same_i64_definition(&changed.functions[&symbol], Some(&changed), read, seed).unwrap(), SameI64Definition::Different);
            reject_changed_module(&changed, view.retained_handoff);
            let mut changed = view.module().clone();
            changed.functions.get_mut(&symbol).unwrap().blocks.get_mut(&phi_block).unwrap().instructions.remove(phi_index);
            reject_changed_module(&changed, view.retained_handoff);
            Ok(())
        }).unwrap();
    });
}

/// Remove a real Copy and every use, so rejection cannot rely on an undefined
/// SSA value. This is the same operand substitution as Copy elimination.
fn erase_copy(function: &mut MirFunction, read: ValueId, replacement: ValueId) {
    let mut removed = 0;
    let mut uses = 0;
    for body in function.blocks.values_mut() {
        body.instructions.retain(|row| {
            let erase = matches!(row, MirInstruction::Copy { dst, .. } if *dst == read);
            removed += usize::from(erase);
            !erase
        });
        for row in body
            .instructions
            .iter_mut()
            .chain(body.terminator.iter_mut())
        {
            let count = row
                .used_values()
                .iter()
                .filter(|value| **value == read)
                .count();
            if count == 0 {
                continue;
            }
            let mut replace = |value: &mut ValueId| {
                if *value == read {
                    *value = replacement;
                    uses += 1;
                }
            };
            match row {
                MirInstruction::Copy { src, .. } => replace(src),
                MirInstruction::Phi { inputs, .. } => {
                    for (_, value) in inputs {
                        replace(value);
                    }
                }
                MirInstruction::BinOp { lhs, rhs, .. }
                | MirInstruction::Compare { lhs, rhs, .. } => {
                    replace(lhs);
                    replace(rhs);
                }
                MirInstruction::Return { value: Some(value) } => replace(value),
                MirInstruction::Branch { condition, .. } => replace(condition),
                MirInstruction::Invoke { operation, .. } => operation.rewrite_values(replace),
                _ => panic!("the selected fixture has an uncovered Copy consumer: {row:?}"),
            }
        }
        if let Some(values) = &mut body.return_env {
            for value in values {
                if *value == read {
                    *value = replacement;
                    uses += 1;
                }
            }
        }
    }
    assert_eq!(removed, 1, "remove the actual Copy definition");
    assert!(uses > 0, "the actual read had a consumer");
    assert!(
        function.blocks.values().all(|body| {
            body.all_instructions()
                .all(|row| row.dst_value() != Some(read) && !row.used_values().contains(&read))
                && body
                    .return_env
                    .as_ref()
                    .is_none_or(|values| !values.contains(&read))
        }),
        "no definition or use of the removed read may remain"
    );
}

#[test]
fn i64_call_read_erasure_keeps_the_original_operand_and_effect_boundary() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        MirCompiler::with_options(false).compile_normal_with_published(request(SOURCE), |view, verification| {
            require_artifact_verification(verification);
            view.issue_lifecycle_physical_abi_input()?;
            let symbol = view.retained_root().unwrap().signature.name.clone();
            let function = &view.module().functions[&symbol];
            let seed = call_result(function, "seed");
            let other = call_result(function, "other");
            let geometry = crate::mir::verification::geometry::FunctionVerificationGeometry::new(function);
            // Cover both the source snapshot and operand wrappers, rather
            // than stopping at whichever Copy a block iteration finds first.
            let mut reads = std::collections::BTreeMap::new();
            for row in function.blocks.values().flat_map(|body| body.all_instructions()) {
                if !matches!(row, MirInstruction::Compare { .. }) { continue; }
                for mut read in row.used_values() {
                    while let Some((_, _, MirInstruction::Copy { src, .. })) = geometry.definition(read) {
                        if same_i64_definition(function, Some(view.module()), read, seed).unwrap() != SameI64Definition::Same { break; }
                        reads.insert(read, *src);
                        read = *src;
                    }
                }
            }
            assert!(!reads.is_empty(), "real live Copy reads in comparisons");
            let verify = |module: &crate::mir::MirModule| crate::mir::verification::MirVerifier::new()
                .verify_module_with_finalized_root(module, view.retained_handoff);
            let mut escaped = Vec::new();
            for (&read, &original) in &reads {
                for (case, replacement, must_reject) in [
                    ("erase-copy-change-operand", other, true),
                    ("erase-copy-original-operand", original, false),
                ] {
                    let mut changed = view.module().clone();
                    erase_copy(changed.functions.get_mut(&symbol).unwrap(), read, replacement);
                    assert!(same_i64_definition(&changed.functions[&symbol], Some(&changed), replacement, seed).is_ok(),
                        "the mutation retains valid CFG/SSA/Invoke structure");
                    let result = verify(&changed);
                    println!("i64-origin-boundary {case} read={read:?}: accepted={} error={result:?}", result.is_ok());
                    if must_reject && result.is_ok() { escaped.push((case, read)); }
                }
            }
            let (&read, &original) = reads.first_key_value().unwrap();
            let mut changed = view.module().clone();
            erase_copy(changed.functions.get_mut(&symbol).unwrap(), read, original);
            let function = changed.functions.get_mut(&symbol).unwrap();
            // Erase the initializer's effect without making its result undefined.
            let mut removed = 0;
            let invoke = function.blocks.iter().find_map(|(block, body)| match &body.terminator {
                Some(MirInstruction::Invoke { normal_landing, .. }) if function.blocks[normal_landing]
                    .instructions.iter().any(|row| matches!(row,
                        MirInstruction::InvokeNormalResult { invoke_block, dst } if invoke_block == block && *dst == seed))
                    => Some((*block, *normal_landing)),
                _ => None,
            });
            if let Some((block, normal)) = invoke {
                function.blocks.get_mut(&block).unwrap().set_terminator(MirInstruction::Jump {
                    target: normal, edge_args: None,
                });
                for row in &mut function.blocks.get_mut(&normal).unwrap().instructions {
                    if matches!(row, MirInstruction::InvokeNormalResult { dst, .. } if *dst == seed) {
                        *row = MirInstruction::Const { dst: seed, value: crate::mir::ConstValue::Integer(7) };
                        removed += 1;
                    }
                }
            } else {
                for body in function.blocks.values_mut() {
                    for row in &mut body.instructions {
                        if matches!(row, MirInstruction::Call(call) if call.dst == Some(seed)) {
                            *row = MirInstruction::Const { dst: seed, value: crate::mir::ConstValue::Integer(7) };
                            removed += 1;
                        }
                    }
                }
            }
            assert_eq!(removed, 1, "erase exactly the initializer Call/Invoke");
            function.update_cfg();
            let result = verify(&changed);
            println!("i64-origin-boundary erase-initializer-call: accepted={} error={result:?}", result.is_ok());
            if result.is_ok() { escaped.push(("erase-initializer-call", read)); }
            assert!(escaped.is_empty(), "negative boundary escaped (do not fix production in this round): {escaped:?}");
            Ok(())
        }).unwrap();
    });
}

#[test]
fn i64_call_read_phi_rejects_another_result_of_the_same_callee() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let source = SOURCE.replace("local other = me.other()", "local other = me.seed()");
        MirCompiler::with_options(false).compile_normal_with_published(request(&source), |view, verification| {
            require_artifact_verification(verification);
            view.issue_lifecycle_physical_abi_input()?;
            let symbol = view.retained_root().unwrap().signature.name.clone();
            let function = &view.module().functions[&symbol];
            let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Main", "seed", 0);
            let target = crate::mir::Callee::Global(key.canonical_global_target_v1().unwrap());
            let results: Vec<_> = function.blocks.values().flat_map(|body| body.all_instructions())
                .filter_map(|row| match row {
                    MirInstruction::Call(call) if call.callee == target => call.dst,
                    MirInstruction::InvokeNormalResult { invoke_block, dst } if matches!(function.blocks[invoke_block].terminator.as_ref(),
                        Some(MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { call, .. }, .. }) if call.callee == target) => Some(*dst),
                    _ => None,
                }).collect();
            assert_eq!(results.len(), 2, "two distinct source calls of exactly the same callee");
            assert_ne!(results[0], results[1]);
            let (block, index, dst, expected) = function.blocks.iter().flat_map(|(block, body)| body.instructions.iter()
                .enumerate().map(move |(index, row)| (*block, index, row)))
                .find_map(|(block, index, row)| match row {
                    MirInstruction::Phi { dst, .. } => {
                        let matching: Vec<_> = results.iter().filter(|result|
                            same_i64_definition(function, Some(view.module()), *dst, **result).unwrap() == SameI64Definition::Same).copied().collect();
                        (matching.len() == 1).then(|| (block, index, *dst, matching[0]))
                    }
                    _ => None,
                }).expect("an actual join has exactly one original call result basis");
            let other = *results.iter().find(|value| **value != expected).unwrap();
            let mut changed = view.module().clone();
            let MirInstruction::Phi { inputs, .. } = instruction_mut(changed.functions.get_mut(&symbol).unwrap(), block, index) else { unreachable!() };
            inputs[0].1 = other;
            assert_eq!(same_i64_definition(&changed.functions[&symbol], Some(&changed), dst, expected).unwrap(), SameI64Definition::Different);
            reject_changed_module(&changed, view.retained_handoff);
            Ok(())
        }).unwrap();
    });
}
