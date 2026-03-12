//! MIR-related environment flags
//!
//! This module groups all MIR feature flags and environment variable controls.
//! Use this for IDE autocomplete to discover MIR flags easily.

use super::env_bool;

// ---- MIR PHI / PHI-less (edge-copy) mode ----
/// Enable MIR PHI non-generation for Bridge compatibility mode only.
/// フェーズM.2: MirBuilder/LoopBuilderでPHI統一済み、Bridge層の互換性制御のみ
/// Default: PHI-ON (Phase 15 direction), override with NYASH_MIR_NO_PHI=1
pub fn mir_no_phi() -> bool {
    env_bool("NYASH_MIR_NO_PHI")
}

/// Global PHI type inference debug (set to any value to enable)
pub fn phi_global_debug_enabled() -> bool {
    std::env::var("NYASH_PHI_GLOBAL_DEBUG").is_ok()
}

// ---- Phase 11.8 MIR cleanup toggles ----
/// Core-13 minimal MIR mode toggle. Default ON unless NYASH_MIR_CORE13=0.
pub fn mir_core13() -> bool {
    match std::env::var("NYASH_MIR_CORE13").ok() {
        Some(v) => {
            let lv = v.to_ascii_lowercase();
            !(lv == "0" || lv == "false" || lv == "off")
        }
        None => true,
    }
}

pub fn mir_ref_boxcall() -> bool {
    std::env::var("NYASH_MIR_REF_BOXCALL").ok().as_deref() == Some("1") || mir_core13()
}

pub fn mir_array_boxcall() -> bool {
    std::env::var("NYASH_MIR_ARRAY_BOXCALL").ok().as_deref() == Some("1") || mir_core13()
}

/// Core-13 "pure" mode: only the 13 canonical ops are allowed (verifier rejects others).
pub fn mir_core13_pure() -> bool {
    env_bool("NYASH_MIR_CORE13_PURE")
}

// ---- Optimizer diagnostics ----
pub fn opt_debug() -> bool {
    std::env::var("NYASH_OPT_DEBUG").is_ok()
}

pub fn opt_diag() -> bool {
    std::env::var("NYASH_OPT_DIAG").is_ok()
}

pub fn opt_diag_forbid_legacy() -> bool {
    std::env::var("NYASH_OPT_DIAG_FORBID_LEGACY").is_ok()
}

pub fn opt_diag_fail() -> bool {
    std::env::var("NYASH_OPT_DIAG_FAIL").is_ok()
}

pub fn binop_reprop_debug_enabled() -> bool {
    env_bool("NYASH_BINOP_REPROP_DEBUG")
}

// ---- Rewriter flags (optimizer transforms)
pub fn rewrite_debug() -> bool {
    std::env::var("NYASH_REWRITE_DEBUG").ok().as_deref() == Some("1")
}

pub fn rewrite_safepoint() -> bool {
    std::env::var("NYASH_REWRITE_SAFEPOINT").ok().as_deref() == Some("1")
}

pub fn rewrite_future() -> bool {
    std::env::var("NYASH_REWRITE_FUTURE").ok().as_deref() == Some("1")
}

// ---- Operator Boxes adopt defaults ----
/// CompareOperator.apply adopt: default OFF（観測のみ; 意味論はVM側をSSOTにする）
pub fn operator_box_compare_adopt() -> bool {
    match std::env::var("NYASH_OPERATOR_BOX_COMPARE_ADOPT")
        .ok()
        .as_deref()
        .map(|v| v.to_ascii_lowercase())
    {
        Some(ref s) if s == "0" || s == "false" || s == "off" => false,
        Some(ref s) if s == "1" || s == "true" || s == "on" => true,
        _ => false, // default OFF
    }
}

/// AddOperator.apply adopt: default OFF（順次昇格のため）
pub fn operator_box_add_adopt() -> bool {
    match std::env::var("NYASH_OPERATOR_BOX_ADD_ADOPT")
        .ok()
        .as_deref()
        .map(|v| v.to_ascii_lowercase())
    {
        Some(ref s) if s == "0" || s == "false" || s == "off" => false,
        _ => true, // default ON (promoted after validation)
    }
}

// ---- Null/Missing Boxes (dev-only observe → adopt) ----
/// Enable NullBox/MissingBox observation path (no behavior change by default).
/// Default: OFF. Turn ON with `NYASH_NULL_MISSING_BOX=1`. May be auto-enabled in --dev later.
pub fn null_missing_box_enabled() -> bool {
    std::env::var("NYASH_NULL_MISSING_BOX").ok().as_deref() == Some("1")
}

/// Strict null policy for operators (when enabled): null in arithmetic/compare is an error.
/// Default: OFF (null propagates). Effective only when `null_missing_box_enabled()` is true.
pub fn null_strict() -> bool {
    std::env::var("NYASH_NULL_STRICT").ok().as_deref() == Some("1")
}

// ---- Phase 12: Nyash ABI (vtable) toggles ----
pub fn abi_vtable() -> bool {
    std::env::var("NYASH_ABI_VTABLE").ok().as_deref() == Some("1")
}

/// ABI strict diagnostics: missing vtable methods become errors when enabled.
pub fn abi_strict() -> bool {
    std::env::var("NYASH_ABI_STRICT").ok().as_deref() == Some("1")
}

// ---- Legacy compatibility (dev-only) ----
/// Enable legacy InstanceBox fields (SharedNyashBox map) for compatibility.
/// Default: OFF. Set NYASH_LEGACY_FIELDS_ENABLE=1 to materialize and use legacy fields.
pub fn legacy_fields_enable() -> bool {
    env_bool("NYASH_LEGACY_FIELDS_ENABLE")
}

// ---- GC/Runtime tracing (execution-affecting visibility) ----
pub fn gc_trace() -> bool {
    env_bool("NYASH_GC_TRACE")
}

pub fn runtime_checkpoint_trace() -> bool {
    env_bool("NYASH_RUNTIME_CHECKPOINT_TRACE")
}

pub fn gc_barrier_strict() -> bool {
    std::env::var("NYASH_GC_BARRIER_STRICT").ok().as_deref() == Some("1")
}

/// Return 0 (off) to 3 (max) for `NYASH_GC_TRACE`.
pub fn gc_trace_level() -> u8 {
    match std::env::var("NYASH_GC_TRACE").ok().as_deref() {
        Some("1") => 1,
        Some("2") => 2,
        Some("3") => 3,
        Some(_) => 1,
        None => 0,
    }
}

// ---- GC mode and instrumentation ----
/// Return current GC mode string (auto default = "rc+cycle").
/// Allowed: "auto", "rc+cycle", "off"
pub fn gc_mode() -> String {
    match std::env::var("NYASH_GC_MODE").ok() {
        Some(m) if !m.trim().is_empty() => m,
        _ => "rc+cycle".to_string(),
    }
}

/// Typed GC mode getter (SSOT parser lives in runtime::gc_mode).
pub fn gc_mode_typed(
) -> Result<crate::runtime::gc_mode::GcMode, crate::runtime::gc_mode::GcModeParseError> {
    crate::runtime::gc_mode::GcMode::from_env_result()
}

/// Brief metrics emission (text)
pub fn gc_metrics() -> bool {
    std::env::var("NYASH_GC_METRICS").ok().as_deref() == Some("1")
}

// ---- Cleanup (method-level postfix) policy toggles ----
/// Allow `return` inside a cleanup block. Default: false (0)
pub fn cleanup_allow_return() -> bool {
    match std::env::var("NYASH_CLEANUP_ALLOW_RETURN").ok() {
        Some(v) => {
            let lv = v.to_ascii_lowercase();
            !(lv == "0" || lv == "false" || lv == "off")
        }
        None => false,
    }
}

/// Allow `throw` inside a cleanup block. Default: false (0)
pub fn cleanup_allow_throw() -> bool {
    match std::env::var("NYASH_CLEANUP_ALLOW_THROW").ok() {
        Some(v) => {
            let lv = v.to_ascii_lowercase();
            !(lv == "0" || lv == "false" || lv == "off")
        }
        None => false,
    }
}

/// Run a collection every N safepoints (if Some)
pub fn gc_collect_sp_interval() -> Option<u64> {
    std::env::var("NYASH_GC_COLLECT_SP").ok()?.parse().ok()
}

/// Run a collection when allocated bytes since last >= N (if Some)
pub fn gc_collect_alloc_bytes() -> Option<u64> {
    std::env::var("NYASH_GC_COLLECT_ALLOC").ok()?.parse().ok()
}

/// Get await maximum milliseconds, centralized here for consistency.
pub fn await_max_ms() -> u64 {
    std::env::var("NYASH_AWAIT_MAX_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5000)
}
