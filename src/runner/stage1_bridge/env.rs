/*!
 * Stage-1 CLI bridge - environment variable configurator
 *
 * Sets default environment variables for Stage-1 CLI child process.
 */

use crate::config::env;
use crate::config::env::stage1;
use std::process::Command;

/// Configure environment variables for Stage-1 CLI child process
///
/// Sets defaults for:
/// - Runtime behavior (NYASH_NYRT_SILENT_RESULT, NYASH_DISABLE_PLUGINS, etc.)
/// - Parser toggles (NYASH_FEATURES=stage3, legacy NYASH_PARSER_STAGE3, NYASH_ENABLE_USING, etc.)
/// - Stage-B configuration (HAKO_STAGEB_APPLY_USINGS, HAKO_STAGEB_MODULES_LIST, HAKO_STAGEB_MODULE_ROOTS_LIST, etc.)
pub(super) fn configure_stage1_env(
    cmd: &mut Command,
    entry_fn: &str,
    stage1_args: &[String],
    modules_list: Option<String>,
    module_roots_list: Option<String>,
) {
    // Child recursion guard
    cmd.env("NYASH_STAGE1_CLI_CHILD", "1");

    // Unified Stage-1 env (NYASH_STAGE1_*) — derive from legacy if unset to keep compatibility.
    if std::env::var("NYASH_STAGE1_MODE").is_err() {
        if let Some(m) = stage1::mode() {
            cmd.env("NYASH_STAGE1_MODE", m);
        }
    }

    // Runtime defaults
    if std::env::var("NYASH_NYRT_SILENT_RESULT").is_err() {
        cmd.env("NYASH_NYRT_SILENT_RESULT", "1");
    }
    if std::env::var("NYASH_DISABLE_PLUGINS").is_err() {
        cmd.env("NYASH_DISABLE_PLUGINS", "0");
    }
    if std::env::var("NYASH_FILEBOX_MODE").is_err() {
        cmd.env("NYASH_FILEBOX_MODE", "auto");
    }
    if std::env::var("NYASH_BOX_FACTORY_POLICY").is_err() {
        cmd.env("NYASH_BOX_FACTORY_POLICY", "builtin_first");
    }
    // Stage‑1 stubは静的 box 呼び出しが多く、methodize 経路だと未定義 receiver に落ちやすい。
    // 既定では methodization を切ってグローバル呼び出しのままにしておく（必要なら opt-in で上書き）。
    if std::env::var("HAKO_MIR_BUILDER_METHODIZE").is_err() {
        cmd.env("HAKO_MIR_BUILDER_METHODIZE", "0");
    }
    // Mainline lock: keep MirBuilder on internal-only route.
    // Delegate route (env.mirbuilder.emit) is treated as compatibility-only.
    cmd.env("HAKO_SELFHOST_NO_DELEGATE", "1");
    cmd.env("HAKO_MIR_BUILDER_DELEGATE", "0");

    // Stage-1 unified input/backend (fallback to legacy)
    if std::env::var("NYASH_STAGE1_INPUT").is_err() {
        if let Some(src) = stage1::input_path() {
            cmd.env("NYASH_STAGE1_INPUT", src);
        }
    }
    if std::env::var("NYASH_STAGE1_BACKEND").is_err() {
        if let Some(be) = stage1::backend_hint().or_else(stage1::backend_alias_warned) {
            cmd.env("NYASH_STAGE1_BACKEND", be);
        }
    }
    if std::env::var("NYASH_STAGE1_PROGRAM_JSON").is_err() {
        if let Some(pjson) = stage1::program_json_path() {
            cmd.env("NYASH_STAGE1_PROGRAM_JSON", pjson);
        }
    }

    // Stage-1 CLI 経路では既定で using 適用を無効化し、
    // prefix は空（HAKO_STAGEB_APPLY_USINGS=0）とする。
    // UsingResolver/UsingCollector の検証は専用テストで行い、
    // CLI 本線はシンプルな Program(JSON) 生成に集中させる。
    if std::env::var("HAKO_STAGEB_APPLY_USINGS").is_err() {
        cmd.env("HAKO_STAGEB_APPLY_USINGS", "0");
    }

    // Parser toggles
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
    // Stage-3 gate (default ON): prefer NYASH_FEATURES for propagation, but keep
    // legacy envs if parent explicitly set them.
    let stage3_enabled = env::parser_stage3_enabled();
    let merge_feature = |current: &str, feature: &str| -> String {
        let mut list: Vec<String> = current
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            })
            .collect();
        let normalized_feature = feature.replace(['-', '_'], "");
        let contains = list.iter().any(|f| {
            let n = f.to_ascii_lowercase().replace(['-', '_'], "");
            n == normalized_feature
        });
        if !contains {
            list.push(feature.to_string());
        }
        list.join(",")
    };
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
    if let Ok(val) = std::env::var("NYASH_PARSER_STAGE3") {
        cmd.env("NYASH_PARSER_STAGE3", val);
    }
    if let Ok(val) = std::env::var("HAKO_PARSER_STAGE3") {
        cmd.env("HAKO_PARSER_STAGE3", val);
    }

    // Modules list
    if std::env::var("HAKO_STAGEB_MODULES_LIST").is_err() {
        if let Some(mods) = modules_list {
            cmd.env("HAKO_STAGEB_MODULES_LIST", mods);
        }
    }

    // Module roots list (Phase 29bq+: prefix→path mapping for longest-match resolution)
    if std::env::var("HAKO_STAGEB_MODULE_ROOTS_LIST").is_err() {
        if let Some(roots) = module_roots_list {
            cmd.env("HAKO_STAGEB_MODULE_ROOTS_LIST", roots);
        }
    }

    // Entry function
    if std::env::var("NYASH_ENTRY").is_err() {
        cmd.env("NYASH_ENTRY", entry_fn);
    }

    // Backend hint
    if std::env::var("STAGE1_BACKEND").is_err() {
        let be_cli = stage1_args
            .windows(2)
            .find(|w| w[0] == "--backend")
            .map(|w| w[1].clone());
        if let Some(be) = stage1::backend_hint().or(be_cli) {
            cmd.env("STAGE1_BACKEND", be);
        }
    }
}
