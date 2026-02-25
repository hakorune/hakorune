//! Verification and checking-related environment flags
//!
//! This module groups all verification, checking, and testing flags.
//! Use this for IDE autocomplete to discover verification flags easily.

use super::{env_bool, env_bool_default};

/// Allow verifier to skip SSA/dominance/merge checks for PHI-less MIR.
pub fn verify_allow_no_phi() -> bool {
    std::env::var("NYASH_VERIFY_ALLOW_NO_PHI").ok().as_deref() == Some("1") || mir_no_phi()
}

fn mir_no_phi() -> bool {
    env_bool("NYASH_MIR_NO_PHI")
}

/// Enable strict edge-copy policy verification in PHI-off mode.
/// When enabled, merge blocks must receive merged values via predecessor copies only,
/// and the merge block itself must not introduce a self-copy to the merged destination.
pub fn verify_edge_copy_strict() -> bool {
    env_bool("NYASH_VERIFY_EDGE_COPY_STRICT")
}

/// Enforce purity of return blocks: no side-effecting instructions allowed before Return
/// Default: OFF. Enable with NYASH_VERIFY_RET_PURITY=1 in dev/profiling sessions.
pub fn verify_ret_purity() -> bool {
    env_bool("NYASH_VERIFY_RET_PURITY")
}

/// Stage-B/selfhost 専用の dev verify トグル（SSA などの厳格チェックを一時緩和するためのスイッチ）
/// Default: ON（現行挙動）。NYASH_STAGEB_DEV_VERIFY=0 で Stage-B 経路の dev verify をスキップ。
pub fn stageb_dev_verify_enabled() -> bool {
    fn env_flag(var: &str) -> Option<bool> {
        std::env::var(var).ok().map(|v| {
            let lv = v.to_ascii_lowercase();
            lv == "1" || lv == "true" || lv == "on"
        })
    }
    env_flag("NYASH_STAGEB_DEV_VERIFY").unwrap_or(true)
}

/// Global fail-fast policy for runtime fallbacks.
/// Default: ON (true) to prohibit silent/different-route fallbacks in Rust layer.
/// Set NYASH_FAIL_FAST=0 to temporarily allow legacy fallbacks during bring-up.
pub fn fail_fast() -> bool {
    env_bool_default("NYASH_FAIL_FAST", true)
}

/// Bridge lowering: use Result-style try/throw lowering instead of MIR Catch/Throw
/// When on, try/catch is lowered using structured blocks and direct jumps,
/// without emitting MIR Throw/Catch. The thrown value is routed to catch via
/// block parameters (PHI-off uses edge-copy).
pub fn try_result_mode() -> bool {
    std::env::var("NYASH_TRY_RESULT_MODE").ok().as_deref() == Some("1")
}

/// Runner/CLI common toggles (hot-path centralization)
pub fn cli_verbose() -> bool {
    // Use cli_verbose_level from dump module
    super::dump::cli_verbose_level() > 0
}
