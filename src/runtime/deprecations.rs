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
        "[deprecate] --emit-program-json-v0 is compat-only and retire-target; prefer MIR-first routes such as --hako-emit-mir-json or --emit-mir-json",
    );
}

pub fn warn_stage1_bridge_program_json_route_once() {
    warn_once(
        &STAGE1_BRIDGE_PROGRAM_JSON_WARN_ONCE,
        "[deprecate] stage1 bridge emit-program-json route is compat-only; use dedicated compat probes for Program(JSON) proof and prefer MIR-first bootstrap routes otherwise",
    );
}
