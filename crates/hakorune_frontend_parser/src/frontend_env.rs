//! Passive frontend environment boundary marker.
//!
//! Active environment reads remain in the main crate until parser/tokenizer
//! files move behind this crate root.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use crate::frontend_host::frontend_host;

static WARNED_ALIASES: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontendEnvBoundary;

impl FrontendEnvBoundary {
    pub const fn name(self) -> &'static str {
        "frontend_env"
    }
}

fn nyash_features_list() -> Option<Vec<String>> {
    let raw = std::env::var("NYASH_FEATURES").ok()?;
    let list: Vec<String> = raw
        .split(',')
        .filter_map(|item| {
            let item = item.trim();
            if item.is_empty() {
                None
            } else {
                Some(item.to_ascii_lowercase())
            }
        })
        .collect();
    if list.is_empty() {
        None
    } else {
        Some(list)
    }
}

fn feature_enabled<const N: usize>(targets: [&str; N]) -> bool {
    let Some(list) = nyash_features_list() else {
        return false;
    };
    list.into_iter().any(|item| {
        let normalized = item.replace(['-', '_'], "");
        targets.iter().any(|target| normalized == *target)
    })
}

fn env_flag(var: &str) -> Option<bool> {
    std::env::var(var).ok().map(|value| {
        let value = value.to_ascii_lowercase();
        value == "1" || value == "true" || value == "on"
    })
}

fn warn_alias_once(alias: &'static str, primary: &'static str) {
    let set = WARNED_ALIASES.get_or_init(|| Mutex::new(HashSet::new()));
    let Ok(mut set) = set.lock() else {
        return;
    };
    if !set.insert(alias) {
        return;
    }

    frontend_host().warn_alias_once(alias, primary);
}

/// Core frontend Stage-3 gate (default ON).
pub fn parser_stage3_enabled() -> bool {
    if feature_enabled(["stage3", "parserstage3"]) {
        return true;
    }
    if let Some(value) = env_flag("NYASH_PARSER_STAGE3") {
        warn_alias_once("NYASH_PARSER_STAGE3", "NYASH_FEATURES=stage3");
        return value;
    }
    if let Some(value) = env_flag("HAKO_PARSER_STAGE3") {
        warn_alias_once("HAKO_PARSER_STAGE3", "NYASH_FEATURES=stage3");
        return value;
    }
    true
}
