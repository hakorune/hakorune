/*!
 * Stage-1 bridge child env - Stage-1 alias propagation section.
 *
 * Mirrors the unified `NYASH_STAGE1_*` contract for child processes while
 * keeping legacy alias handling out of the facade.
 */

use super::Stage1ChildEnvConfig;
use crate::config::env::stage1;
use std::process::Command;

pub(super) fn apply(cmd: &mut Command, config: &Stage1ChildEnvConfig<'_>) {
    cmd.env("NYASH_STAGE1_CLI_CHILD", "1");

    if std::env::var("NYASH_STAGE1_MODE").is_err() {
        if let Some(mode) = stage1::mode() {
            cmd.env("NYASH_STAGE1_MODE", mode);
        }
    }

    if std::env::var("NYASH_STAGE1_INPUT").is_err() {
        if let Some(src) = stage1::input_path() {
            cmd.env("NYASH_STAGE1_INPUT", src);
        }
    }

    if std::env::var("NYASH_STAGE1_BACKEND").is_err() {
        if let Some(backend) = config.backend_hint {
            cmd.env("NYASH_STAGE1_BACKEND", backend);
        }
    }

    if std::env::var("NYASH_STAGE1_PROGRAM_JSON").is_err() {
        if let Some(program_json) = stage1::program_json_path() {
            cmd.env("NYASH_STAGE1_PROGRAM_JSON", program_json);
        }
    }

    if std::env::var("STAGE1_CLI_ENTRY").is_err() {
        if let Some(entry_path) = config.entry_path {
            cmd.env("STAGE1_CLI_ENTRY", entry_path);
        }
    }

    if std::env::var("HAKORUNE_STAGE1_ENTRY").is_err() {
        if let Some(entry_path) = config.entry_path {
            cmd.env("HAKORUNE_STAGE1_ENTRY", entry_path);
        }
    }

    if std::env::var("NYASH_ENTRY").is_err() {
        cmd.env("NYASH_ENTRY", config.entry_fn);
    }

    if std::env::var("STAGE1_BACKEND").is_err() {
        if let Some(backend) = config.backend_hint {
            cmd.env("STAGE1_BACKEND", backend);
        }
    }
}
