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

/// JoinIR VM bridge mode. When enabled with NYASH_JOINIR_EXPERIMENT=1,
/// specific functions can be executed via JoinIR → VM bridge instead of direct MIR → VM.
/// Set NYASH_JOINIR_VM_BRIDGE=1 to enable.
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

/// JoinIR LLVM experiment mode. When enabled with NYASH_JOINIR_EXPERIMENT=1,
/// enables experimental JoinIR→MIR'→LLVM path for specific functions (e.g., Main.skip/1).
/// This is a dev-only toggle for testing PHI normalization via JoinIR in the LLVM path.
/// Set NYASH_JOINIR_LLVM_EXPERIMENT=1 to enable.
pub fn joinir_llvm_experiment_enabled() -> bool {
    joinir_core_enabled() && env_bool("NYASH_JOINIR_LLVM_EXPERIMENT")
}

/// Phase 33: JoinIR If Select 実験の有効化
/// Primary: HAKO_JOINIR_IF_SELECT (Phase 33-8+).
pub fn joinir_if_select_enabled() -> bool {
    // Core ON なら既定で有効化（JoinIR 本線化を優先）
    if joinir_core_enabled() {
        return true;
    }
    // Primary: HAKO_JOINIR_IF_SELECT
    if let Some(v) = env_flag("HAKO_JOINIR_IF_SELECT") {
        return v;
    }
    false
}

/// Phase 33-8: JoinIR Stage-1 rollout toggle
/// Set HAKO_JOINIR_STAGE1=1 to enable JoinIR lowering for Stage-1 functions.
pub fn joinir_stage1_enabled() -> bool {
    // Primary: HAKO_JOINIR_STAGE1
    if let Some(v) = env_flag("HAKO_JOINIR_STAGE1") {
        return v;
    }
    false
}

/// Phase 33-8: JoinIR debug log level (0-3)
/// - 0: No logs (default)
/// - 1: Basic logs (which functions were lowered)
/// - 2: Pattern matching details (CFG analysis)
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

/// Phase 61-2: If-in-loop JoinIR dry-run有効化
///
/// `HAKO_JOINIR_IF_IN_LOOP_DRYRUN=1` でdry-runモードを有効化
///
/// dry-runモード:
/// - JoinIR経路でPHI仕様を計算
/// - PhiBuilderBox経路と比較
/// - 実際のPHI生成はPhiBuilderBoxを使用（安全）
pub fn joinir_if_in_loop_dryrun_enabled() -> bool {
    env_bool("HAKO_JOINIR_IF_IN_LOOP_DRYRUN")
}

/// Phase 61-3: If-in-loop JoinIR本番経路有効化
///
/// `HAKO_JOINIR_IF_IN_LOOP_ENABLE=1` でJoinIR本番経路を有効化
///
/// 動作:
/// - ON: JoinIR + IfInLoopPhiEmitter経路（PhiBuilderBox不使用）
/// - OFF: PhiBuilderBox経路（既存フォールバック）
///
/// 前提条件:
/// - JoinIR IfSelect 基盤（Phase 33）の有効化
/// - dry-runモードとは独立（HAKO_JOINIR_IF_IN_LOOP_DRYRUN）
///
/// デフォルト: OFF（安全第一）
pub fn joinir_if_in_loop_enable() -> bool {
    env_bool("HAKO_JOINIR_IF_IN_LOOP_ENABLE")
}

/// Phase 61-4: ループ外If JoinIR経路有効化
///
/// `HAKO_JOINIR_IF_TOPLEVEL=1` でループ外IfのJoinIR経路を有効化
///
/// 動作:
/// - ON: try_lower_if_to_joinir経路（if_form.rsで使用）
/// - OFF: PhiBuilderBox経路（既存）
///
/// 前提条件:
/// - HAKO_JOINIR_IF_SELECT=1（Phase 33基盤）
///
/// デフォルト: OFF（安全第一）
pub fn joinir_if_toplevel_enabled() -> bool {
    env_bool("HAKO_JOINIR_IF_TOPLEVEL")
}

/// Phase 61-4: ループ外If JoinIR dry-run有効化
///
/// `HAKO_JOINIR_IF_TOPLEVEL_DRYRUN=1` でdry-runモードを有効化
///
/// dry-runモード:
/// - JoinIR経路を試行しログ出力
/// - 実際のPHI生成は既存経路を使用（安全）
pub fn joinir_if_toplevel_dryrun_enabled() -> bool {
    env_bool("HAKO_JOINIR_IF_TOPLEVEL_DRYRUN")
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
/// - Default: ON (structure_only = true) - all loops use JoinIR patterns
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
///     // Route all loops through JoinIR pattern analysis
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
