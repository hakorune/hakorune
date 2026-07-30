//! JoinIR-related environment flags
//!
//! This module groups all JoinIR feature flags and environment variable controls.
//! Use this for IDE autocomplete to discover JoinIR flags easily.

use super::{env_bool, env_flag, warn_alias_once};

// ---- Phase 29/30 JoinIR toggles ----
/// JoinIR experiment mode. Required for JoinIR-related experimental paths.
/// Set NYASH_JOINIR_EXPERIMENT=1 to enable.
pub fn joinir_experiment_enabled() -> bool {
    env_bool("NYASH_JOINIR_EXPERIMENT")
}

/// JoinIR core policy: **always ON** after LoopBuilder removal.
/// - `NYASH_JOINIR_CORE` is deprecated（0 を指定しても警告して無視する）
/// - JoinIR を OFF にするモードは提供しない（Fail-Fast 原則、フォールバックなし）
pub fn joinir_core_enabled() -> bool {
    if let Some(v) = env_flag("NYASH_JOINIR_CORE") {
        if !v {
            warn_joinir_core_off_ignored();
        }
    }
    true
}

fn warn_joinir_core_off_ignored() {
    use std::sync::Once;
    static WARNED_JOINIR_CORE_OFF: Once = Once::new();
    WARNED_JOINIR_CORE_OFF.call_once(|| {
        let ring0 = crate::runtime::ring0::get_global_ring0();
        ring0.log.warn(
            "[deprecate/env] NYASH_JOINIR_CORE=0 is ignored; JoinIR core is always on (LoopBuilder is removed)"
        );
    });
}

/// Explicit VM compatibility bridge mode.
///
/// In a `vm-reference` build, the explicit VM keep route may attempt the
/// selected JoinIR → VM bridge when `NYASH_JOINIR_VM_BRIDGE=1` is set.
/// `NYASH_JOINIR_EXPERIMENT` is not an activation predicate for this route.
pub fn joinir_vm_bridge_enabled() -> bool {
    joinir_core_enabled() && env_bool("NYASH_JOINIR_VM_BRIDGE")
}

/// JoinIR strict mode: when enabled, JoinIR 対象のフォールバックを禁止する。
/// 既定OFF。NYASH_JOINIR_STRICT=1 のときのみ有効。
pub fn joinir_strict_enabled() -> bool {
    env_flag("NYASH_JOINIR_STRICT").unwrap_or(false)
}

/// JoinIR VM bridge debug output. Enables verbose logging of JoinIR→MIR conversion.
/// Set NYASH_JOINIR_VM_BRIDGE_DEBUG=1 to enable.
pub fn joinir_vm_bridge_debug() -> bool {
    env_bool("NYASH_JOINIR_VM_BRIDGE_DEBUG")
}

/// Phase 33-8: JoinIR debug log level (0-3)
/// - 0: No logs (default)
/// - 1: Basic logs (which functions were lowered)
/// - 2: Route/shape matching details (CFG analysis)
/// - 3: Full dump (all variables, all instructions)
pub fn joinir_debug_level() -> u8 {
    // Primary: HAKO_JOINIR_DEBUG
    if let Ok(v) = std::env::var("HAKO_JOINIR_DEBUG") {
        return v.parse().unwrap_or(0);
    }
    // Fallback: NYASH_JOINIR_DEBUG (deprecated)
    if let Ok(v) = std::env::var("NYASH_JOINIR_DEBUG") {
        warn_alias_once("NYASH_JOINIR_DEBUG", "HAKO_JOINIR_DEBUG");
        return v.parse().unwrap_or(0);
    }
    0
}

/// JoinIR plan trace / debug logging enabled (SSOT).
///
/// This is a level-based check (0 disables logs) and is equivalent to
/// `joinir_dev::debug_enabled()`.
pub fn joinir_trace_enabled() -> bool {
    joinir_debug_level() > 0
}

/// Dev-only convenience switch to bundle experimental JoinIR knobs.
/// - NYASH_JOINIR_DEV=1 enables
/// - Otherwise inherits from joinir_debug_level()>0 (opt-in debug)
pub fn joinir_dev_enabled() -> bool {
    env_bool("NYASH_JOINIR_DEV") || joinir_debug_level() > 0
}

/// LoopForm normalize flag (NYASH_LOOPFORM_NORMALIZE=1).
pub fn loopform_normalize() -> bool {
    std::env::var("NYASH_LOOPFORM_NORMALIZE").ok().as_deref() == Some("1")
}

/// JoinIR debug logging enabled check (SSOT).
///
/// Uses the numeric debug level:
/// - `HAKO_JOINIR_DEBUG=0` disables logs
/// - `HAKO_JOINIR_DEBUG=1..` enables logs
///
/// Legacy alias: `NYASH_JOINIR_DEBUG` (deprecated).
pub fn is_joinir_debug() -> bool {
    joinir_debug_level() > 0
}

/// JoinIR structure-only routing mode (Phase 196+).
///
/// When enabled (default), routes loops based purely on structure analysis,
/// skipping the legacy function name whitelist.
///
/// - Default: ON (structure_only = true) - all loops use JoinIR route-shape analysis
/// - To revert to whitelist-only: `NYASH_JOINIR_STRUCTURE_ONLY=0` or `=off`
///
/// # Compatibility
///
/// - `NYASH_JOINIR_STRUCTURE_ONLY=0` or `=off` → false
/// - Any other value (including unset) → true
///
/// # Usage
///
/// ```rust
/// if joinir_structure_only_enabled() {
///     // Route all loops through JoinIR route-shape analysis
/// } else {
///     // Use legacy whitelist routing
/// }
/// ```
pub fn joinir_structure_only_enabled() -> bool {
    match std::env::var("NYASH_JOINIR_STRUCTURE_ONLY").ok().as_deref() {
        Some("0") | Some("off") => false,
        _ => true,
    }
}
