use super::{MirFunction, MirInstruction, MirModule, MirOptimizer, MirType, ValueId};
use crate::mir::optimizer_stats::OptimizationStats;
use crate::runtime::get_global_ring0;

/// Map string type name to MIR type (optimizer-level helper)
#[allow(dead_code)]
fn map_type_name(name: &str) -> MirType {
    match name {
        "Integer" | "Int" | "I64" => MirType::Integer,
        "Float" | "F64" => MirType::Float,
        "Bool" | "Boolean" => MirType::Bool,
        "String" => MirType::String,
        "Void" | "Unit" => MirType::Void,
        other => MirType::Box(other.to_string()),
    }
}

#[allow(dead_code)]
fn opt_debug_enabled() -> bool {
    crate::config::env::opt_debug()
}

#[allow(dead_code)]
fn opt_debug(msg: &str) {
    if opt_debug_enabled() {
        get_global_ring0().log.debug(&format!("[OPT] {}", msg));
    }
}

/// Resolve a MIR type from a value id that should represent a type name
/// Supports: Const String("T") and NewBox(StringBox, Const String("T"))
#[allow(dead_code)]
fn resolve_type_from_value(
    function: &MirFunction,
    def_map: &std::collections::HashMap<ValueId, (crate::mir::BasicBlockId, usize)>,
    id: ValueId,
) -> Option<MirType> {
    if let Some((bb, idx)) = def_map.get(&id).copied() {
        if let Some(block) = function.blocks.get(&bb) {
            if idx < block.instructions.len() {
                match &block.instructions[idx] {
                    MirInstruction::Const {
                        value: crate::mir::ConstValue::String(s),
                        ..
                    } => {
                        return Some(map_type_name(s));
                    }
                    MirInstruction::NewBox { box_type, args, .. }
                        if box_type == "StringBox" && args.len() == 1 =>
                    {
                        let inner = args[0];
                        if let Some((sbb, sidx)) = def_map.get(&inner).copied() {
                            if let Some(sblock) = function.blocks.get(&sbb) {
                                if sidx < sblock.instructions.len() {
                                    if let MirInstruction::Const {
                                        value: crate::mir::ConstValue::String(s),
                                        ..
                                    } = &sblock.instructions[sidx]
                                    {
                                        return Some(map_type_name(s));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

/// Diagnostics: identify unlowered type-ops embedded as strings in Call
#[allow(dead_code)]
fn diagnose_unlowered_type_ops(optimizer: &MirOptimizer, module: &MirModule) -> OptimizationStats {
    let mut stats = OptimizationStats::new();
    let diag_on = optimizer.debug || crate::config::env::opt_diag();
    for (fname, function) in &module.functions {
        // def map for resolving constants
        let mut def_map: std::collections::HashMap<ValueId, (crate::mir::BasicBlockId, usize)> =
            std::collections::HashMap::new();
        for (bb_id, block) in &function.blocks {
            for (i, inst) in block.instructions.iter().enumerate() {
                if let Some(dst) = inst.dst_value() {
                    def_map.insert(dst, (*bb_id, i));
                }
            }
            if let Some(term) = &block.terminator {
                if let Some(dst) = term.dst_value() {
                    def_map.insert(dst, (*bb_id, usize::MAX));
                }
            }
        }
        let mut count = 0usize;
        for (_bb, block) in &function.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Call {
                        callee: Some(crate::mir::Callee::Method { method, .. }),
                        ..
                    } if method == "is"
                        || method == "as"
                        || method == "isType"
                        || method == "asType" =>
                    {
                        count += 1;
                    }
                    MirInstruction::Call { func, .. } => {
                        if let Some((bb, idx)) = def_map.get(func).copied() {
                            if let Some(b) = function.blocks.get(&bb) {
                                if idx < b.instructions.len() {
                                    if let MirInstruction::Const {
                                        value: crate::mir::ConstValue::String(s),
                                        ..
                                    } = &b.instructions[idx]
                                    {
                                        if s == "isType" || s == "asType" {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if count > 0 {
            stats.diagnostics_reported += count;
            if diag_on {
                get_global_ring0().log.info(&format!(
                    "[OPT][DIAG] Function '{}' has {} unlowered type-op calls",
                    fname, count
                ));
            }
        }
    }
    stats
}

/// Diagnostic: detect lowered-away instructions that should be gone after normalization.
/// When NYASH_OPT_DIAG or NYASH_OPT_DIAG_FORBID_LEGACY is set, prints diagnostics.
#[allow(dead_code)]
fn diagnose_legacy_instructions(module: &MirModule, debug: bool) -> OptimizationStats {
    let mut stats = OptimizationStats::new();
    let diag_on =
        debug || crate::config::env::opt_diag() || crate::config::env::opt_diag_forbid_legacy();
    for (fname, function) in &module.functions {
        let mut count = 0usize;
        for (_bb, block) in &function.blocks {
            for inst in &block.instructions {
                if crate::mir::contracts::backend_core_ops::lowered_away_tag(inst).is_some() {
                    count += 1;
                }
            }
            if let Some(term) = &block.terminator {
                if crate::mir::contracts::backend_core_ops::lowered_away_tag(term).is_some() {
                    count += 1;
                }
            }
        }
        if count > 0 {
            stats.diagnostics_reported += count;
            if diag_on {
                get_global_ring0().log.info(&format!(
                    "[OPT][DIAG] Function '{}' has {} legacy MIR ops: unify to Core‑13 (TypeOp/WeakRef/Barrier/BoxCall)",
                    fname, count
                ));
                if crate::config::env::opt_diag_forbid_legacy() {
                    panic!(
                        "NYASH_OPT_DIAG_FORBID_LEGACY=1: legacy MIR ops detected in '{}': {}",
                        fname, count
                    );
                }
            }
        }
    }
    stats
}
