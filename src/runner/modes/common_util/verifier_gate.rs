/*!
 * VM verifier gate utilities
 *
 * Phase 29x X28:
 * - Centralize optional MIR verification for VM lanes
 * - Keep a single verification entrypoint for `vm` / `vm-fallback` / `vm-hako`
 */

use nyash_rust::mir::MirModule;
use nyash_rust::mir::VerificationError;

pub(crate) const VM_VERIFY_GATE_FREEZE_TAG: &str = "vm-route/verifier-gate";
pub(crate) const VM_VERIFY_GATE_DETAIL_TAG: &str = "vm-route/verifier-detail";
pub(crate) const VM_DIRECT_EMIT_MIR_VERIFY_TAG: &str = "emit-mir/direct-verify";
pub(crate) const VM_DIRECT_EMIT_EXE_VERIFY_TAG: &str = "emit-exe/direct-verify";

pub(crate) fn vm_verify_gate_enabled() -> bool {
    crate::config::env::env_bool("NYASH_VM_VERIFY_MIR")
}

pub(crate) fn format_vm_verify_gate_freeze(
    route: &str,
    function: &str,
    error_count: usize,
) -> String {
    format!(
        "[freeze:contract][{}] route={} function={} errors={}",
        VM_VERIFY_GATE_FREEZE_TAG, route, function, error_count
    )
}

pub(crate) fn format_vm_verify_gate_detail(route: &str, function: &str, detail: &str) -> String {
    format!(
        "[{}] route={} function={} detail={}",
        VM_VERIFY_GATE_DETAIL_TAG, route, function, detail
    )
}

pub(crate) fn format_vm_direct_emit_verify_freeze(
    contract_tag: &str,
    route: &str,
    error_count: usize,
) -> String {
    format!(
        "[freeze:contract][{}] route={} errors={}",
        contract_tag, route, error_count
    )
}

pub(crate) fn format_vm_direct_emit_verify_detail(
    contract_tag: &str,
    route: &str,
    detail: &str,
) -> String {
    format!("[{}] route={} detail={}", contract_tag, route, detail)
}

pub(crate) fn build_direct_emit_verify_lines(
    route: &str,
    contract_tag: &str,
    details: &[String],
) -> Vec<String> {
    let mut lines = Vec::with_capacity(details.len().saturating_add(1));
    lines.push(format_vm_direct_emit_verify_freeze(
        contract_tag,
        route,
        details.len(),
    ));
    for detail in details {
        lines.push(format_vm_direct_emit_verify_detail(
            contract_tag,
            route,
            detail.as_str(),
        ));
    }
    lines
}

fn collect_first_verify_failure(module: &MirModule) -> Option<(String, Vec<String>)> {
    let mut verifier = crate::mir::verification::MirVerifier::new();
    for (name, func) in module.functions.iter() {
        if let Err(errors) = verifier.verify_function(func) {
            if !errors.is_empty() {
                let rendered: Vec<String> = errors.into_iter().map(|e| e.to_string()).collect();
                return Some((name.clone(), rendered));
            }
        }
    }
    None
}

pub(crate) fn enforce_vm_verify_gate_or_exit(module: &MirModule, route: &str) {
    if !vm_verify_gate_enabled() {
        return;
    }
    let Some((function, errors)) = collect_first_verify_failure(module) else {
        return;
    };
    eprintln!(
        "{}",
        format_vm_verify_gate_freeze(route, &function, errors.len())
    );
    for detail in errors {
        eprintln!(
            "{}",
            format_vm_verify_gate_detail(route, &function, detail.as_str())
        );
    }
    std::process::exit(1);
}

pub(crate) fn enforce_direct_emit_verify_or_exit(
    verification_result: &Result<(), Vec<VerificationError>>,
    route: &str,
    contract_tag: &str,
) {
    let Err(errors) = verification_result else {
        return;
    };
    let details: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
    for line in build_direct_emit_verify_lines(route, contract_tag, details.as_slice()) {
        eprintln!("{}", line);
    }
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_vm_verify_gate_freeze_is_stable() {
        assert_eq!(
            format_vm_verify_gate_freeze("vm", "main", 2),
            "[freeze:contract][vm-route/verifier-gate] route=vm function=main errors=2"
        );
    }

    #[test]
    fn format_vm_verify_gate_detail_is_stable() {
        assert_eq!(
            format_vm_verify_gate_detail("vm-fallback", "main", "phi mismatch"),
            "[vm-route/verifier-detail] route=vm-fallback function=main detail=phi mismatch"
        );
    }

    #[test]
    fn format_vm_direct_emit_verify_freeze_is_stable() {
        assert_eq!(
            format_vm_direct_emit_verify_freeze(VM_DIRECT_EMIT_MIR_VERIFY_TAG, "vm", 3),
            "[freeze:contract][emit-mir/direct-verify] route=vm errors=3"
        );
    }

    #[test]
    fn format_vm_direct_emit_verify_detail_is_stable() {
        assert_eq!(
            format_vm_direct_emit_verify_detail(
                VM_DIRECT_EMIT_MIR_VERIFY_TAG,
                "vm",
                "dominance error"
            ),
            "[emit-mir/direct-verify] route=vm detail=dominance error"
        );
    }

    #[test]
    fn format_vm_direct_emit_exe_verify_freeze_is_stable() {
        assert_eq!(
            format_vm_direct_emit_verify_freeze(VM_DIRECT_EMIT_EXE_VERIFY_TAG, "vm", 1),
            "[freeze:contract][emit-exe/direct-verify] route=vm errors=1"
        );
    }

    #[test]
    fn build_direct_emit_verify_lines_contract_is_stable() {
        let lines = build_direct_emit_verify_lines(
            "mir",
            VM_DIRECT_EMIT_MIR_VERIFY_TAG,
            &["dominance A".to_string(), "dominance B".to_string()],
        );
        assert_eq!(
            lines,
            vec![
                "[freeze:contract][emit-mir/direct-verify] route=mir errors=2".to_string(),
                "[emit-mir/direct-verify] route=mir detail=dominance A".to_string(),
                "[emit-mir/direct-verify] route=mir detail=dominance B".to_string(),
            ]
        );
    }
}
