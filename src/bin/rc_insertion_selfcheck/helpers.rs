//! Phase 29z P0: RC insertion minimal selfcheck (opt-in)
//!
//! This binary constructs a tiny synthetic MIR module containing Store overwrites
//! and asserts that the RC insertion pass inserts ReleaseStrong in the expected place.
//!
//! Build/run (opt-in):
//! - `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal`

use nyash_rust::mir::passes::rc_insertion::insert_rc_instructions;
use nyash_rust::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};
use std::collections::HashMap;

pub(super) const RC_PHI_EDGE_MISMATCH_TAG: &str =
    "[freeze:contract][rc_insertion/phi_edge_mismatch]";

pub(super) fn build_module_with_block(
    block: BasicBlock,
    func_name: &str,
    mod_name: &str,
) -> MirModule {
    let signature = FunctionSignature {
        name: func_name.to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let entry = block.id;
    let mut func = MirFunction::new(signature, entry);
    func.blocks = HashMap::from([(entry, block)]);

    let mut module = MirModule::new(mod_name.to_string());
    module.add_function(func);
    module
}

pub(super) fn build_module_with_blocks(
    blocks: Vec<BasicBlock>,
    entry: BasicBlockId,
    func_name: &str,
    mod_name: &str,
) -> MirModule {
    let signature = FunctionSignature {
        name: func_name.to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, entry);
    func.blocks = blocks.into_iter().map(|b| (b.id, b)).collect();

    let mut module = MirModule::new(mod_name.to_string());
    module.add_function(func);
    module
}

pub(super) fn assert_release_inserted(
    mut module: MirModule,
    func_name: &str,
    entry: BasicBlockId,
    expected_release: usize,
    label: &str,
) {
    let stats = insert_rc_instructions(&mut module);
    if stats.release_inserted != expected_release {
        eprintln!(
            "[FAIL] {}: expected release_inserted={}, got {}",
            label, expected_release, stats.release_inserted
        );
        std::process::exit(1);
    }

    let func = module
        .get_function(func_name)
        .expect("selfcheck function exists");
    let bb = func.blocks.get(&entry).expect("entry block exists");
    let release_count = bb
        .instructions
        .iter()
        .filter(|inst| matches!(inst, MirInstruction::ReleaseStrong { .. }))
        .count();
    if release_count != expected_release {
        eprintln!(
            "[FAIL] {}: expected ReleaseStrong count={}, got {}",
            label, expected_release, release_count
        );
        std::process::exit(1);
    }
    if bb.instructions.len() != bb.instruction_spans.len() {
        eprintln!(
            "[FAIL] {}: span count mismatch: insts={}, spans={}",
            label,
            bb.instructions.len(),
            bb.instruction_spans.len()
        );
        std::process::exit(1);
    }
}

pub(super) fn assert_release_counts_in_blocks(
    mut module: MirModule,
    func_name: &str,
    expected_release: usize,
    block_expectations: &[(BasicBlockId, usize)],
    label: &str,
) {
    let stats = insert_rc_instructions(&mut module);
    if stats.release_inserted != expected_release {
        eprintln!(
            "[FAIL] {}: expected release_inserted={}, got {}",
            label, expected_release, stats.release_inserted
        );
        std::process::exit(1);
    }

    let func = module
        .get_function(func_name)
        .expect("selfcheck function exists");

    for (bid, expected_count) in block_expectations {
        let bb = func.blocks.get(bid).expect("block exists");
        let release_count = bb
            .instructions
            .iter()
            .filter(|inst| matches!(inst, MirInstruction::ReleaseStrong { .. }))
            .count();
        if release_count != *expected_count {
            eprintln!(
                "[FAIL] {}: block {:?} expected ReleaseStrong count={}, got {}",
                label, bid, expected_count, release_count
            );
            std::process::exit(1);
        }
        if bb.instructions.len() != bb.instruction_spans.len() {
            eprintln!(
                "[FAIL] {}: block {:?} span count mismatch: insts={}, spans={}",
                label,
                bid,
                bb.instructions.len(),
                bb.instruction_spans.len()
            );
            std::process::exit(1);
        }
    }
}

pub(super) fn assert_call_overwrite_and_return_queue_order(
    mut module: MirModule,
    func_name: &str,
    block_id: BasicBlockId,
    old_value: ValueId,
    new_value: ValueId,
    label: &str,
) {
    let stats = insert_rc_instructions(&mut module);
    if stats.release_inserted != 2 {
        eprintln!(
            "[FAIL] {}: expected release_inserted=2, got {}",
            label, stats.release_inserted
        );
        std::process::exit(1);
    }

    let func = module
        .get_function(func_name)
        .expect("selfcheck function exists");
    let bb = func.blocks.get(&block_id).expect("entry block exists");
    if bb.instructions.len() != bb.instruction_spans.len() {
        eprintln!(
            "[FAIL] {}: span count mismatch: insts={}, spans={}",
            label,
            bb.instructions.len(),
            bb.instruction_spans.len()
        );
        std::process::exit(1);
    }
    if !matches!(bb.terminator.as_ref(), Some(MirInstruction::Return { .. })) {
        eprintln!("[FAIL] {}: expected Return terminator", label);
        std::process::exit(1);
    }
    if bb.instructions.len() != 5 {
        eprintln!(
            "[FAIL] {}: expected 5 instructions after insertion, got {}",
            label,
            bb.instructions.len()
        );
        std::process::exit(1);
    }

    if !matches!(bb.instructions[0], MirInstruction::Store { .. }) {
        eprintln!("[FAIL] {}: expected instruction[0] to stay Store", label);
        std::process::exit(1);
    }
    if !matches!(bb.instructions[1], MirInstruction::Call { .. }) {
        eprintln!("[FAIL] {}: expected instruction[1] to stay Call", label);
        std::process::exit(1);
    }
    match &bb.instructions[2] {
        MirInstruction::ReleaseStrong { values } if values.as_slice() == [old_value] => {}
        other => {
            eprintln!(
                "[FAIL] {}: expected instruction[2] = ReleaseStrong(old), got {:?}",
                label, other
            );
            std::process::exit(1);
        }
    }
    if !matches!(bb.instructions[3], MirInstruction::Store { .. }) {
        eprintln!("[FAIL] {}: expected instruction[3] to stay Store", label);
        std::process::exit(1);
    }
    match &bb.instructions[4] {
        MirInstruction::ReleaseStrong { values } if values.as_slice() == [new_value] => {}
        other => {
            eprintln!(
                "[FAIL] {}: expected instruction[4] = ReleaseStrong(new), got {:?}",
                label, other
            );
            std::process::exit(1);
        }
    }
}

fn panic_payload_to_string(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(msg) = payload.downcast_ref::<String>() {
        return msg.clone();
    }
    if let Some(msg) = payload.downcast_ref::<&str>() {
        return (*msg).to_string();
    }
    "<non-string panic payload>".to_string()
}

pub(super) fn assert_fail_fast_tag_from_insert(
    mut module: MirModule,
    expected_tag: &str,
    label: &str,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _stats = insert_rc_instructions(&mut module);
    }));
    match result {
        Ok(_) => {
            eprintln!(
                "[FAIL] {}: expected fail-fast panic, but insertion succeeded",
                label
            );
            std::process::exit(1);
        }
        Err(payload) => {
            let panic_msg = panic_payload_to_string(payload.as_ref());
            if !panic_msg.contains(expected_tag) {
                eprintln!(
                    "[FAIL] {}: fail-fast tag missing. expected='{}' panic='{}'",
                    label, expected_tag, panic_msg
                );
                std::process::exit(1);
            }
        }
    }
}

pub(super) fn assert_all_release_values_sorted(
    mut module: MirModule,
    func_name: &str,
    label: &str,
) {
    let _stats = insert_rc_instructions(&mut module);
    let func = module.get_function(func_name).expect("function exists");

    for (bid, bb) in &func.blocks {
        for inst in &bb.instructions {
            if let MirInstruction::ReleaseStrong { values } = inst {
                for window in values.windows(2) {
                    if window[0] > window[1] {
                        eprintln!(
                            "[FAIL] {}: block {:?} values not sorted: {:?} > {:?}",
                            label, bid, window[0], window[1]
                        );
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}
