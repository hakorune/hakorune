//! Deprecation warnings with "warn once" guards
use crate::runtime::ring0::GLOBAL_RING0;
use std::sync::OnceLock;

fn warn_once(flag: &'static OnceLock<()>, msg: &str) {
    if flag.get().is_none() {
        let _ = flag.set(());
        if let Some(ring0) = GLOBAL_RING0.get() {
            ring0.log.warn(msg);
        } else {
            eprintln!("{}", msg);
        }
    }
}

static NYASH_TOML_WARN_ONCE: OnceLock<()> = OnceLock::new();
static EMIT_PROGRAM_JSON_V0_CLI_WARN_ONCE: OnceLock<()> = OnceLock::new();
static HAKO_EMIT_PROGRAM_JSON_CLI_WARN_ONCE: OnceLock<()> = OnceLock::new();
static PROGRAM_JSON_TO_MIR_CLI_WARN_ONCE: OnceLock<()> = OnceLock::new();
static STAGE1_BRIDGE_PROGRAM_JSON_WARN_ONCE: OnceLock<()> = OnceLock::new();

/// Warn once per process when nyash.toml is used while hako.toml is absent.
pub fn warn_nyash_toml_used_once() {
    warn_once(
        &NYASH_TOML_WARN_ONCE,
        "[deprecate] using nyash.toml; please rename to hako.toml",
    );
}

pub fn warn_emit_program_json_v0_cli_once() {
    warn_once(
        &EMIT_PROGRAM_JSON_V0_CLI_WARN_ONCE,
        "[deprecate] --emit-program-json-v0 is compat-only; prefer MIR-first routes such as --emit-mir-json",
    );
}

pub fn warn_hako_emit_program_json_cli_once() {
    warn_once(
        &HAKO_EMIT_PROGRAM_JSON_CLI_WARN_ONCE,
        "[deprecate] --hako-emit-program-json is compat-only; prefer --hako-emit-mir-json",
    );
}

pub fn warn_program_json_to_mir_cli_once() {
    warn_once(
        &PROGRAM_JSON_TO_MIR_CLI_WARN_ONCE,
        "[deprecate] Program(JSON v0) -> MIR(JSON) CLI conversion is compat-only; prefer source -> MIR(JSON) routes",
    );
}

pub fn warn_stage1_bridge_program_json_route_once() {
    warn_once(
        &STAGE1_BRIDGE_PROGRAM_JSON_WARN_ONCE,
        "[deprecate] stage1 bridge emit-program-json route is compat-only; prefer MIR-first bootstrap routes",
    );
}
