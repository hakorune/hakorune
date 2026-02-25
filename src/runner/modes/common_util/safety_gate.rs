/*!
 * VM safety gate utilities
 *
 * Phase 29x X29:
 * - Centralize route safety checks (unsafe route boundary)
 * - Centralize lifecycle safety checks (ReleaseStrong contract)
 */

use nyash_rust::mir::{MirInstruction, MirModule};

pub(crate) const VM_SAFETY_TAG_HAKO_SOURCE: &str = "vm-route/safety-hako-source";
pub(crate) const VM_SAFETY_TAG_LIFECYCLE: &str = "vm-route/safety-lifecycle";

pub(crate) fn format_vm_hako_source_freeze(route: &str) -> String {
    format!(
        "[freeze:contract][{}] route={} require=backend:vm-hako",
        VM_SAFETY_TAG_HAKO_SOURCE, route
    )
}

pub(crate) fn format_vm_lifecycle_freeze(
    route: &str,
    function: &str,
    bb: &str,
    inst_idx: usize,
    reason: &str,
) -> String {
    format!(
        "[freeze:contract][{}] route={} fn={} bb={} inst_idx={} reason={}",
        VM_SAFETY_TAG_LIFECYCLE, route, function, bb, inst_idx, reason
    )
}

fn is_hako_like_for_vm_boundary(source: &str) -> bool {
    source.contains("static box ")
        || source.contains("using selfhost.")
        || source.contains("using hakorune.")
}

pub(crate) fn enforce_vm_source_safety_or_exit(source: &str, route: &str) {
    if !crate::runner::modes::common_util::hako::fail_fast_on_hako() {
        return;
    }
    if !is_hako_like_for_vm_boundary(source) {
        return;
    }
    eprintln!("{}", format_vm_hako_source_freeze(route));
    eprintln!(
        "❌ Hako-like source detected in Nyash VM path. Use Hakorune VM (v1 dispatcher) or Core/LLVM for MIR.\n   hint: set HAKO_VERIFY_PRIMARY=hakovm in verify path"
    );
    std::process::exit(1);
}

fn detect_lifecycle_violation(module: &MirModule) -> Option<(String, String, usize, &'static str)> {
    for (function_name, function) in module.functions.iter() {
        for (bb_id, block) in function.blocks.iter() {
            for (inst_idx, inst) in block.instructions.iter().enumerate() {
                if let MirInstruction::ReleaseStrong { values } = inst {
                    if values.is_empty() {
                        return Some((
                            function_name.clone(),
                            format!("{:?}", bb_id),
                            inst_idx,
                            "release_strong-empty-values",
                        ));
                    }
                }
            }
        }
    }
    None
}

pub(crate) fn enforce_vm_lifecycle_safety_or_exit(module: &MirModule, route: &str) {
    let Some((function_name, bb, inst_idx, reason)) = detect_lifecycle_violation(module) else {
        return;
    };
    eprintln!(
        "{}",
        format_vm_lifecycle_freeze(route, &function_name, &bb, inst_idx, reason)
    );
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_vm_hako_source_freeze_is_stable() {
        assert_eq!(
            format_vm_hako_source_freeze("vm"),
            "[freeze:contract][vm-route/safety-hako-source] route=vm require=backend:vm-hako"
        );
    }

    #[test]
    fn format_vm_lifecycle_freeze_is_stable() {
        assert_eq!(
            format_vm_lifecycle_freeze(
                "vm-fallback",
                "main",
                "BasicBlockId(1)",
                2,
                "release_strong-empty-values",
            ),
            "[freeze:contract][vm-route/safety-lifecycle] route=vm-fallback fn=main bb=BasicBlockId(1) inst_idx=2 reason=release_strong-empty-values"
        );
    }

    #[test]
    fn detect_hako_like_boundary_source() {
        assert!(is_hako_like_for_vm_boundary("static box Main { }"));
        assert!(is_hako_like_for_vm_boundary("using selfhost.vm.entry_s0 as X"));
        assert!(!is_hako_like_for_vm_boundary("box Main { main(){ return 0 } }"));
    }
}
