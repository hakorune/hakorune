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
