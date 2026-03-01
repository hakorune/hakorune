//! VM and backend-related environment flags
//!
//! This module groups all VM execution and backend selection flags.
//! Use this for IDE autocomplete to discover VM/backend flags easily.

use super::{env_bool, env_flag, warn_alias_once};

// ---- LLVM harness toggle (llvmlite) ----
pub fn llvm_use_harness() -> bool {
    // Phase 15: デフォルトON（LLVMバックエンドはPythonハーネス使用）
    // NYASH_LLVM_USE_HARNESS=0 で明示的に無効化可能
    if let Some(v) = env_flag("NYASH_LLVM_USE_HARNESS") {
        return v;
    }
    // Fallback to alias (with deprecation warning)
    if let Some(v) = env_flag("HAKO_LLVM_USE_HARNESS") {
        warn_alias_once("HAKO_LLVM_USE_HARNESS", "NYASH_LLVM_USE_HARNESS");
        return v;
    }
    true // デフォルト: ON（ハーネス使用）
}

/// LLVM opt level (primary: NYASH_LLVM_OPT_LEVEL; alias: HAKO_LLVM_OPT_LEVEL)
/// Returns string level (e.g., "0", "1", ...). Default: "0" when unset.
pub fn llvm_opt_level() -> String {
    if let Some(v) = std::env::var("NYASH_LLVM_OPT_LEVEL").ok() {
        return v;
    }
    if let Some(v) = std::env::var("HAKO_LLVM_OPT_LEVEL").ok() {
        warn_alias_once("HAKO_LLVM_OPT_LEVEL", "NYASH_LLVM_OPT_LEVEL");
        return v;
    }
    "0".to_string()
}

/// Raw opt level env (if set via NYASH_LLVM_OPT_LEVEL or HAKO_LLVM_OPT_LEVEL).
pub fn llvm_opt_level_env() -> Option<String> {
    std::env::var("NYASH_LLVM_OPT_LEVEL")
        .ok()
        .or_else(|| std::env::var("HAKO_LLVM_OPT_LEVEL").ok())
}

/// Raw opt level envs (HAKO first, NYASH second) for diagnostics.
pub fn llvm_opt_level_envs() -> (Option<String>, Option<String>) {
    (
        std::env::var("HAKO_LLVM_OPT_LEVEL").ok(),
        std::env::var("NYASH_LLVM_OPT_LEVEL").ok(),
    )
}

/// (Deprecated) use dispatch-based VM route; currently disabled.
pub fn vm_use_dispatch() -> bool {
    false
}

/// Force VM fallback interpreter route (dev-only escape hatch).
pub fn vm_use_fallback() -> bool {
    env_bool("NYASH_VM_USE_FALLBACK")
}

/// Compat fallback policy shared by runtime/kernel internals.
///
/// Contract:
/// - `NYASH_VM_USE_FALLBACK=0` => compat fallback prohibited
/// - otherwise => compat fallback allowed by policy (subject to additional gates)
pub fn vm_compat_fallback_allowed() -> bool {
    std::env::var("NYASH_VM_USE_FALLBACK").ok().as_deref() != Some("0")
}

/// Trace VM route selection decisions.
pub fn vm_route_trace() -> bool {
    env_bool("NYASH_VM_ROUTE_TRACE")
}

/// VM fast-path mode (hot-path oriented execution behavior).
pub fn vm_fast_enabled() -> bool {
    env_bool("NYASH_VM_FAST")
}

/// VM fast register-file mode (bench/profile only).
/// Uses a dense ValueId-indexed slot vector for register writes/reads on hot paths.
pub fn vm_fast_regfile_enabled() -> bool {
    env_bool("NYASH_VM_FAST_REGFILE")
}

/// VM trace for instruction-level diagnostics.
pub fn vm_trace_enabled() -> bool {
    env_bool("NYASH_VM_TRACE") || env_bool("NYASH_VM_TRACE_EXEC")
}

/// VM trace for PHI diagnostics.
pub fn vm_trace_phi_enabled() -> bool {
    env_bool("NYASH_VM_TRACE_PHI")
}

/// Lightweight VM execution counters output (inst/branch/compare).
pub fn vm_stats_enabled() -> bool {
    env_bool("NYASH_VM_STATS")
}

/// Box-level trace output toggle.
pub fn vm_box_trace_enabled() -> bool {
    env_bool("NYASH_BOX_TRACE")
}

/// Dev-time tolerance for undefined/void-like arithmetic and compare behavior.
pub fn vm_tolerate_void_enabled() -> bool {
    env_bool("NYASH_VM_TOLERATE_VOID")
        || env_bool("HAKO_PHI_VERIFY")
        || env_bool("NYASH_PHI_VERIFY")
}

/// PHI safety valve: allow undefined PHI input as Void.
pub fn vm_phi_tolerate_undefined_enabled() -> bool {
    env_bool("NYASH_VM_PHI_TOLERATE_UNDEFINED")
}

/// PHI strict mode (default ON). Respects HAKO alias first for compatibility.
pub fn vm_phi_strict_enabled() -> bool {
    if std::env::var("HAKO_VM_PHI_STRICT").is_ok() {
        return env_bool("HAKO_VM_PHI_STRICT");
    }
    if std::env::var("NYASH_VM_PHI_STRICT").is_ok() {
        return env_bool("NYASH_VM_PHI_STRICT");
    }
    true
}

/// Emit concise VM error location lines.
pub fn vm_error_loc_enabled() -> bool {
    env_bool("HAKO_VM_ERROR_LOC")
}

/// Whether VM should capture last-instruction context for diagnostics.
pub fn vm_capture_last_inst_enabled() -> bool {
    vm_error_loc_enabled()
        || crate::config::env::joinir_dev::debug_enabled()
        || vm_trace_enabled()
        || vm_trace_phi_enabled()
        || !vm_fast_enabled()
}

/// Prefer vm-hako lane when strict/dev gate is active.
///
/// Default: ON under strict/dev gate.
/// Override: `NYASH_VM_HAKO_PREFER_STRICT_DEV=0|1`.
pub fn vm_hako_prefer_strict_dev() -> bool {
    if let Some(v) = env_flag("NYASH_VM_HAKO_PREFER_STRICT_DEV") {
        return v;
    }
    crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled()
}

/// Policy: allow VM to fallback-dispatch user Instance BoxCall (dev only by default).
/// - prod: default false (disallow)
/// - dev/ci: default true (allow, with WARN)
/// Override with NYASH_VM_USER_INSTANCE_BOXCALL={0|1}
pub fn vm_allow_user_instance_boxcall() -> bool {
    if let Some(v) = env_flag("NYASH_VM_USER_INSTANCE_BOXCALL") {
        return v;
    }
    !using_is_prod()
}

// Import using profile functions needed for vm_allow_user_instance_boxcall
fn using_is_prod() -> bool {
    crate::config::env::using_profile().eq_ignore_ascii_case("prod")
}

pub fn nyvm_core_wrapper() -> bool {
    env_flag("HAKO_NYVM_CORE")
        .or_else(|| env_flag("NYASH_NYVM_CORE"))
        .unwrap_or(false)
}

pub fn nyvm_bridge_inject_singleton() -> bool {
    env_flag("HAKO_BRIDGE_INJECT_SINGLETON")
        .or_else(|| env_flag("NYASH_BRIDGE_INJECT_SINGLETON"))
        .unwrap_or(false)
}

pub fn nyvm_bridge_early_phi_materialize() -> bool {
    env_flag("HAKO_BRIDGE_EARLY_PHI_MATERIALIZE")
        .or_else(|| env_flag("NYASH_BRIDGE_EARLY_PHI_MATERIALIZE"))
        .unwrap_or(false)
}

pub fn nyvm_v1_downconvert() -> bool {
    env_flag("HAKO_NYVM_V1_DOWNCONVERT")
        .or_else(|| env_flag("NYASH_NYVM_V1_DOWNCONVERT"))
        .unwrap_or(false)
}

/// Gate‑C(Core) route request (primary: NYASH_GATE_C_CORE; alias: HAKO_GATE_C_CORE)
pub fn gate_c_core() -> bool {
    if env_bool("NYASH_GATE_C_CORE") {
        return true;
    }
    if env_bool("HAKO_GATE_C_CORE") {
        warn_alias_once("HAKO_GATE_C_CORE", "NYASH_GATE_C_CORE");
        return true;
    }
    false
}

/// Gate‑C(Core) strict OOB handling: when enabled, any observed OOB tag
/// (emitted by runtime during ArrayBox get/set with HAKO_OOB_STRICT=1) should
/// cause non‑zero exit at the end of JSON→VM execution.
pub fn oob_strict_fail() -> bool {
    env_flag("HAKO_OOB_STRICT_FAIL")
        .or_else(|| env_flag("NYASH_OOB_STRICT_FAIL"))
        .unwrap_or(false)
}

/// Primary verification route: return true when Hakorune VM is requested as primary.
/// Accepts HAKO_VERIFY_PRIMARY=hakovm (preferred) or legacy HAKO_ROUTE_HAKOVM=1 (deprecated, warns).
pub fn verify_primary_is_hakovm() -> bool {
    if std::env::var("HAKO_VERIFY_PRIMARY").ok().as_deref() == Some("hakovm") {
        return true;
    }
    if env_bool("HAKO_ROUTE_HAKOVM") {
        warn_alias_once("HAKO_ROUTE_HAKOVM", "HAKO_VERIFY_PRIMARY=hakovm");
        return true;
    }
    false
}
