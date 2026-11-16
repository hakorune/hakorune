//! JoinIR development / experimental flags (SSOT).
//! Phase 72-C: Consolidate all NYASH_JOINIR_* dev flags through centralized helpers.

use crate::config::env::env_bool;

/// NYASH_JOINIR_MAINLINE_DEBUG=1 - Debug output for mainline JoinIR lowering
pub fn mainline_debug_enabled() -> bool {
    env_bool("NYASH_JOINIR_MAINLINE_DEBUG")
}

/// JoinIR debug output switch (compat).
///
/// - Primary: `HAKO_JOINIR_DEBUG=1..3` (recommended)
/// - Legacy: `NYASH_JOINIR_DEBUG=1` (deprecated, but still supported)
pub fn debug_enabled() -> bool {
    crate::config::env::joinir_trace_enabled()
}

/// HAKO_JOINIR_STRICT=1 or NYASH_JOINIR_STRICT=1 - Strict validation mode
///
/// Phase 138-P2-A: Supports both HAKO_ and NYASH_ prefixes for compatibility
pub fn strict_enabled() -> bool {
    env_bool("HAKO_JOINIR_STRICT") || env_bool("NYASH_JOINIR_STRICT")
}

/// HAKO_JOINIR_PLANNER_REQUIRED=1 - Disallow legacy fallback when planner returns None (dev/strict gate only)
///
/// Default: OFF. Use in single-case / diagnostics gates to enforce "no silent fallback" without changing release behavior.
pub fn planner_required_enabled() -> bool {
    env_bool("HAKO_JOINIR_PLANNER_REQUIRED")
}

pub fn strict_planner_required_debug_enabled() -> bool {
    strict_enabled() && planner_required_enabled() && debug_enabled()
}

/// Strict + planner_required gate mode (debug output may still be disabled).
///
/// Use this for "gate sentinel" logs that must appear in strict+planner_required
/// runs even when general debug output is turned off.
pub fn strict_planner_required_enabled() -> bool {
    strict_enabled() && planner_required_enabled()
}

/// NYASH_JOINIR_INPUT=* - Input source or mode
pub fn input_mode() -> Option<String> {
    std::env::var("NYASH_JOINIR_INPUT").ok()
}

/// HAKO_JOINIR_PRINT_TOKENS_MAIN=1 - Print tokens for main
pub fn print_tokens_main_enabled() -> bool {
    env_bool("HAKO_JOINIR_PRINT_TOKENS_MAIN")
}

/// HAKO_JOINIR_ARRAY_FILTER_MAIN=1 - Array filter main mode
pub fn array_filter_main_enabled() -> bool {
    env_bool("HAKO_JOINIR_ARRAY_FILTER_MAIN")
}

/// Phase 183: NYASH_LEGACY_LOOPBUILDER=1 - Legacy LoopBuilder 経路を明示的に opt-in
///
/// デフォルトはJoinIR優先。どうしても古いLoopBuilder経路を使う必要がある場合のみ設定。
/// 本線では使用しない開発専用フラグ。
pub fn legacy_loopbuilder_enabled() -> bool {
    env_bool("NYASH_LEGACY_LOOPBUILDER")
}
