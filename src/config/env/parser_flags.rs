//! Parser-related environment flags
//!
//! This module groups all parser and language feature flags.
//! Use this for IDE autocomplete to discover parser flags easily.

use super::warn_alias_once;

fn nyash_features_list() -> Option<Vec<String>> {
    let raw = std::env::var("NYASH_FEATURES").ok()?;
    let list: Vec<String> = raw
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_ascii_lowercase())
            }
        })
        .collect();
    if list.is_empty() {
        None
    } else {
        Some(list)
    }
}

fn feature_stage3_enabled() -> bool {
    feature_enabled(["stage3", "parserstage3"])
}

fn feature_rune_enabled() -> bool {
    feature_enabled(["rune"])
}

fn feature_enabled<const N: usize>(targets: [&str; N]) -> bool {
    if let Some(list) = nyash_features_list() {
        for item in list {
            let normalized = item.replace(['-', '_'], "");
            for target in targets.iter() {
                if normalized == *target {
                    return true;
                }
            }
        }
    }
    false
}

/// Parser gate for optimization annotations.
///
/// Enabled when NYASH_FEATURES includes one of:
/// - opt-annotations
/// - opt_annotations
/// - optannotations
///
/// Default: OFF.
pub fn parser_opt_annotations_enabled() -> bool {
    feature_enabled(["optannotations"])
}

/// Parser gate for Rune v0 contract annotations.
///
/// Enabled when NYASH_FEATURES includes one of:
/// - rune
///
/// Default: OFF.
pub fn parser_rune_enabled() -> bool {
    feature_rune_enabled()
}

/// Unified parser gate for declaration metadata annotations during the compat window.
///
/// Canonical syntax is `@rune ...`, while `opt-annotations` remains a compat alias gate
/// until the legacy surface is retired.
pub fn parser_metadata_annotations_enabled() -> bool {
    parser_rune_enabled() || parser_opt_annotations_enabled()
}

fn env_flag(var: &str) -> Option<bool> {
    std::env::var(var).ok().map(|v| {
        let lv = v.to_ascii_lowercase();
        lv == "1" || lv == "true" || lv == "on"
    })
}

/// Core (Rust) parser Stage-3 gate (default ON).
/// Precedence:
/// 1) NYASH_FEATURES contains `stage3`/`parser-stage3`
/// 2) Legacy env aliases (NYASH_PARSER_STAGE3 / HAKO_PARSER_STAGE3)
/// 3) Default true (Stage-3 is standard syntax)
pub fn parser_stage3_enabled() -> bool {
    if feature_stage3_enabled() {
        return true;
    }
    if let Some(v) = env_flag("NYASH_PARSER_STAGE3") {
        warn_alias_once("NYASH_PARSER_STAGE3", "NYASH_FEATURES=stage3");
        return v;
    }
    if let Some(v) = env_flag("HAKO_PARSER_STAGE3") {
        warn_alias_once("HAKO_PARSER_STAGE3", "NYASH_FEATURES=stage3");
        return v;
    }
    true
}

/// Parser compatibility gate for legacy `try` statement surface syntax.
///
/// Default: ON (compatibility preserved during migration).
/// Disable explicitly via `NYASH_FEATURES=no-try-compat` to enforce postfix
/// `catch/cleanup` only.
pub fn parser_try_compat_enabled() -> bool {
    if feature_enabled(["notrycompat"]) {
        return false;
    }
    true
}

#[deprecated(note = "Use parser_stage3_enabled() instead")]
pub fn parser_stage3() -> bool {
    parser_stage3_enabled()
}

/// Parser gate for Block‑Postfix Catch acceptance
/// Enabled when either NYASH_BLOCK_CATCH=1 or Stage‑3 gate is on.
/// Phase 15.5 allows parsing a standalone `{ ... }` block optionally followed by
/// a single `catch (...) { ... }` and/or `finally { ... }`, which is folded into
/// ASTNode::TryCatch with the preceding block as the try body.
pub fn block_postfix_catch() -> bool {
    std::env::var("NYASH_BLOCK_CATCH").ok().as_deref() == Some("1") || parser_stage3_enabled()
}

/// Parser gate for method-level postfix catch/finally acceptance on method definitions.
/// Enabled when either NYASH_METHOD_CATCH=1 or Stage‑3 gate is on.
pub fn method_catch() -> bool {
    std::env::var("NYASH_METHOD_CATCH").ok().as_deref() == Some("1") || parser_stage3_enabled()
}

/// Parser gate for expression-level postfix catch/cleanup acceptance.
/// Enabled when Stage-3 gate is on (NYASH_FEATURES=stage3 or legacy aliases). Separate gate can
/// be introduced in future if needed, but we keep minimal toggles now.
pub fn expr_postfix_catch() -> bool {
    parser_stage3_enabled()
}

/// Parser gate for Unified Members (stored/computed/once/birth_once).
/// Default: ON during Phase-15 (set NYASH_ENABLE_UNIFIED_MEMBERS=0|false|off to disable).
pub fn unified_members() -> bool {
    match std::env::var("NYASH_ENABLE_UNIFIED_MEMBERS").ok() {
        Some(v) => {
            let lv = v.to_ascii_lowercase();
            !(lv == "0" || lv == "false" || lv == "off")
        }
        None => true,
    }
}

/// Unicode decode toggle for string literals (\uXXXX, optional surrogate pairs).
/// Enabled when either HAKO_PARSER_DECODE_UNICODE=1 or NYASH_PARSER_DECODE_UNICODE=1.
/// Default: OFF (for strict backward compatibility).
pub fn parser_decode_unicode() -> bool {
    env_flag("HAKO_PARSER_DECODE_UNICODE")
        .or_else(|| env_flag("NYASH_PARSER_DECODE_UNICODE"))
        .unwrap_or(false)
}

/// Entry policy: allow top-level `main` resolution in addition to `Main.main`.
/// Default: true (prefer `Main.main` when both exist; otherwise accept `main`).
pub fn entry_allow_toplevel_main() -> bool {
    match std::env::var("NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN").ok() {
        Some(v) => {
            let v = v.to_ascii_lowercase();
            v == "1" || v == "true" || v == "on"
        }
        None => true,
    }
}

/// Macro pre-expand mode for selfhost (NYASH_MACRO_SELFHOST_PRE_EXPAND).
/// Returns "1", "auto", or None.
pub fn macro_selfhost_pre_expand() -> Option<String> {
    std::env::var("NYASH_MACRO_SELFHOST_PRE_EXPAND").ok()
}

/// ScopeBox enable flag (NYASH_SCOPEBOX_ENABLE=1).
pub fn scopebox_enable() -> bool {
    std::env::var("NYASH_SCOPEBOX_ENABLE").ok().as_deref() == Some("1")
}
