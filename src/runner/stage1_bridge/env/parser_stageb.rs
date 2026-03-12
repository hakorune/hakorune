/*!
 * Stage-1 bridge child env - parser and Stage-B toggles section.
 *
 * Keeps parser feature propagation separate from runtime defaults and Stage-1
 * alias promotion. Stage-B module payload apply stays owned by `modules.rs`.
 */

use super::Stage1ChildEnvConfig;
use crate::config::env;
use std::process::Command;

pub(super) fn apply(cmd: &mut Command, config: &Stage1ChildEnvConfig<'_>) {
    if std::env::var("HAKO_STAGEB_APPLY_USINGS").is_err() {
        cmd.env("HAKO_STAGEB_APPLY_USINGS", "0");
    }

    if std::env::var("NYASH_ENABLE_USING").is_err() {
        cmd.env(
            "NYASH_ENABLE_USING",
            if env::enable_using() { "1" } else { "0" },
        );
    }
    if std::env::var("HAKO_ENABLE_USING").is_err() {
        cmd.env(
            "HAKO_ENABLE_USING",
            if env::enable_using() { "1" } else { "0" },
        );
    }

    let stage3_enabled = env::parser_stage3_enabled();
    match std::env::var("NYASH_FEATURES") {
        Ok(current) => {
            if stage3_enabled {
                cmd.env("NYASH_FEATURES", merge_feature(&current, "stage3"));
            } else {
                cmd.env("NYASH_FEATURES", current);
            }
        }
        Err(_) if stage3_enabled => {
            cmd.env("NYASH_FEATURES", "stage3");
        }
        Err(_) => {}
    }

    if let Ok(value) = std::env::var("NYASH_PARSER_STAGE3") {
        cmd.env("NYASH_PARSER_STAGE3", value);
    }
    if let Ok(value) = std::env::var("HAKO_PARSER_STAGE3") {
        cmd.env("HAKO_PARSER_STAGE3", value);
    }
    config.module_env_lists.apply_to_command_if_missing(cmd);
}

fn merge_feature(current: &str, feature: &str) -> String {
    let mut list: Vec<String> = current
        .split(',')
        .filter_map(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect();
    let normalized_feature = feature.replace(['-', '_'], "");
    let contains = list.iter().any(|existing| {
        let normalized = existing.to_ascii_lowercase().replace(['-', '_'], "");
        normalized == normalized_feature
    });
    if !contains {
        list.push(feature.to_string());
    }
    list.join(",")
}
