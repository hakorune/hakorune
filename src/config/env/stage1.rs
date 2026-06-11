//! Stage-1 / selfhost CLI environment helpers (SSOT).

use super::mir_flags;
use crate::config::env::env_bool;

/// Shared auto/off mode for NyRT startup gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NyrtAutoOffMode {
    Auto,
    Off,
}

/// Primary toggle: enable Stage-1 stub routing.
pub fn enabled() -> bool {
    env_bool("NYASH_USE_STAGE1_CLI")
        || env_bool("HAKO_STAGE1_ENABLE")
        || env_bool("HAKO_EMIT_PROGRAM_JSON")
        || env_bool("HAKO_EMIT_MIR_JSON")
}

/// Recursion guard when Stage-1 stub calls back into the runner.
pub fn child_invocation() -> bool {
    env_bool("NYASH_STAGE1_CLI_CHILD")
}

/// NyRT exact-EXE / Stage-1 shared result-line toggle.
///
/// This is the P0 seam for `NYASH_NYRT_SILENT_RESULT`, shared by the NyRT
/// entry tail and the Stage-1 bridge runtime defaults.
pub fn nyrt_silent_result_enabled() -> bool {
    env_bool("NYASH_NYRT_SILENT_RESULT")
}

/// Returns `true` when the shared silent-result toggle is already set in the
/// process environment.
pub fn nyrt_silent_result_present() -> bool {
    std::env::var("NYASH_NYRT_SILENT_RESULT").is_ok()
}

/// NyRT exact-EXE shared GC metrics JSON toggle.
pub fn nyrt_gc_metrics_json_enabled() -> bool {
    env_bool("NYASH_GC_METRICS_JSON")
}

/// NyRT exact-EXE shared GC metrics text toggle.
pub fn nyrt_gc_metrics_text_enabled() -> bool {
    mir_flags::gc_metrics()
}

/// NyRT exact-EXE shared safepoint collection interval.
pub fn nyrt_gc_collect_sp_interval() -> Option<u64> {
    mir_flags::gc_collect_sp_interval()
}

/// NyRT exact-EXE shared allocation-based collection threshold.
pub fn nyrt_gc_collect_alloc_bytes() -> Option<u64> {
    mir_flags::gc_collect_alloc_bytes()
}

/// NyRT exact-EXE shared auto-safepoint toggle.
pub fn nyrt_llvm_auto_safepoint_enabled() -> bool {
    env_bool("NYASH_LLVM_AUTO_SAFEPOINT")
}

/// NyRT exact-EXE shared GC allocation warning threshold.
pub fn nyrt_gc_alloc_threshold_bytes() -> Option<u64> {
    std::env::var("NYASH_GC_ALLOC_THRESHOLD").ok()?.parse().ok()
}

fn parse_nyrt_auto_off_mode(key: &str, contract_tag: &str) -> Result<NyrtAutoOffMode, String> {
    let Ok(raw) = std::env::var(key) else {
        return Ok(NyrtAutoOffMode::Auto);
    };
    let value = raw.trim();
    if value.is_empty()
        || value.eq_ignore_ascii_case("auto")
        || value.eq_ignore_ascii_case("on")
        || value.eq_ignore_ascii_case("1")
        || value.eq_ignore_ascii_case("true")
    {
        return Ok(NyrtAutoOffMode::Auto);
    }
    if value.eq_ignore_ascii_case("off")
        || value.eq_ignore_ascii_case("0")
        || value.eq_ignore_ascii_case("false")
        || value.eq_ignore_ascii_case("none")
    {
        return Ok(NyrtAutoOffMode::Off);
    }
    Err(format!(
        "{} expected=auto|on|1|true|off|0|false|none got={}",
        contract_tag, value
    ))
}

/// NyRT exact-EXE minimal-startup toggle.
pub fn nyrt_minimal_startup_enabled() -> bool {
    env_bool("NYASH_NYRT_MINIMAL_STARTUP")
}

/// NyRT exact-EXE plugin-host mode.
pub fn nyrt_plugin_host_mode() -> Result<NyrtAutoOffMode, String> {
    parse_nyrt_auto_off_mode(
        "HAKO_NYRT_PLUGIN_HOST",
        "[freeze:contract][nyrt/plugin-host-mode]",
    )
}

/// NyRT exact-EXE runtime-hooks publication mode.
pub fn nyrt_runtime_hooks_mode() -> Result<NyrtAutoOffMode, String> {
    parse_nyrt_auto_off_mode(
        "NYASH_NYRT_RUNTIME_HOOKS",
        "[freeze:contract][nyrt/runtime-hooks-mode]",
    )
}

/// NyRT exact-EXE runtime builder mode.
pub fn nyrt_runtime_build_mode() -> Result<NyrtAutoOffMode, String> {
    parse_nyrt_auto_off_mode(
        "NYASH_NYRT_RUNTIME_BUILD",
        "[freeze:contract][nyrt/runtime-build-mode]",
    )
}

/// NyRT exact-EXE executable-path preparation mode.
pub fn nyrt_entry_path_prep_mode() -> Result<NyrtAutoOffMode, String> {
    parse_nyrt_auto_off_mode(
        "NYASH_NYRT_ENTRY_PATH_PREP",
        "[freeze:contract][nyrt/entry-path-prep-mode]",
    )
}

/// NyRT exact-EXE ring0 bootstrap mode.
pub fn nyrt_ring0_init_mode() -> Result<NyrtAutoOffMode, String> {
    parse_nyrt_auto_off_mode(
        "NYASH_NYRT_RING0_INIT",
        "[freeze:contract][nyrt/ring0-init-mode]",
    )
}

/// Stage-1 mode hint (emit-program / emit-mir / run).
pub fn mode() -> Option<String> {
    if let Some(m) = std::env::var("HAKO_STAGE1_MODE")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_MODE").ok())
    {
        let trimmed = m.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_ascii_lowercase().replace('_', "-"));
        }
    }
    if std::env::var("HAKO_EMIT_PROGRAM_JSON").ok().as_deref() == Some("1") {
        return Some("emit-program".into());
    }
    if std::env::var("HAKO_EMIT_MIR_JSON").ok().as_deref() == Some("1") {
        return Some("emit-mir".into());
    }
    if std::env::var("STAGE1_EMIT_PROGRAM_JSON").ok().as_deref() == Some("1") {
        return Some("emit-program".into());
    }
    if std::env::var("STAGE1_EMIT_MIR_JSON").ok().as_deref() == Some("1") {
        return Some("emit-mir".into());
    }
    if enabled() {
        return Some("run".into());
    }
    None
}

/// True when Stage-1 should emit Program(JSON v0).
pub fn emit_program_json() -> bool {
    matches!(
        mode().as_deref(),
        Some("emit-program" | "emit-program-json")
    )
}

/// True when Stage-1 should emit MIR(JSON).
pub fn emit_mir_json() -> bool {
    matches!(mode().as_deref(), Some("emit-mir" | "emit-mir-json"))
}

/// Input source path passed to Stage-1 stub (aliases included).
pub fn input_path() -> Option<String> {
    std::env::var("HAKO_STAGE1_INPUT")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_INPUT").ok())
        .or_else(|| std::env::var("STAGE1_SOURCE").ok())
        .or_else(|| std::env::var("STAGE1_INPUT").ok())
}

/// Program(JSON v0) path for Stage-1 emit-mir mode (aliases included).
pub fn program_json_path() -> Option<String> {
    std::env::var("HAKO_STAGE1_PROGRAM_JSON")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_PROGRAM_JSON").ok())
        .or_else(|| std::env::var("STAGE1_PROGRAM_JSON").ok())
}

/// Backend hint for Stage-1 run mode (aliases included).
pub fn backend_hint() -> Option<String> {
    std::env::var("HAKO_STAGE1_BACKEND")
        .ok()
        .or_else(|| std::env::var("NYASH_STAGE1_BACKEND").ok())
        .or_else(|| std::env::var("STAGE1_BACKEND").ok())
}

/// Optional override for Stage-1 CLI entry path.
pub fn entry_override() -> Option<String> {
    std::env::var("STAGE1_CLI_ENTRY")
        .ok()
        .or_else(|| std::env::var("HAKORUNE_STAGE1_ENTRY").ok())
}

/// Optional Stage-1 child args (passed through to stub).
pub fn child_args_env() -> Option<String> {
    std::env::var("NYASH_SCRIPT_ARGS_JSON").ok()
}

/// Stage-1 debug flag (verbose child stderr).
pub fn debug() -> bool {
    std::env::var("STAGE1_CLI_DEBUG").ok().as_deref() == Some("1")
}

fn parse_bool_override(raw: String) -> Option<bool> {
    let trimmed = raw.trim().to_ascii_lowercase();
    match trimmed.as_str() {
        "1" | "true" | "on" => Some(true),
        "0" | "false" | "off" => Some(false),
        _ => None,
    }
}

/// Shared binary-only direct route override.
///
/// - `Some(true)`: force binary-only direct route.
/// - `Some(false)`: disable binary-only direct route.
/// - `None`: no override (mainline default keeps direct route OFF).
pub fn binary_only_direct_override() -> Option<bool> {
    std::env::var("NYASH_STAGE1_BINARY_ONLY_DIRECT")
        .ok()
        .and_then(parse_bool_override)
}

/// Run-specific binary-only direct route override.
///
/// Precedence:
/// 1) `NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT`
/// 2) `NYASH_STAGE1_BINARY_ONLY_DIRECT`
/// 3) no override (`None`, mainline default keeps direct route OFF)
pub fn binary_only_run_direct_override() -> Option<bool> {
    std::env::var("NYASH_STAGE1_BINARY_ONLY_RUN_DIRECT")
        .ok()
        .and_then(parse_bool_override)
        .or_else(binary_only_direct_override)
}

/// Effective toggle for emit-mir binary-only direct route.
/// Mainline default is OFF unless explicit override is set to true.
pub fn binary_only_emit_direct_enabled() -> bool {
    binary_only_direct_override().unwrap_or(false)
}

/// Effective toggle for run-mode binary-only direct route.
/// Mainline default is OFF unless explicit override is set to true.
pub fn binary_only_run_direct_enabled() -> bool {
    binary_only_run_direct_override().unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvRestore {
        key: &'static str,
        old: Option<String>,
    }

    impl EnvRestore {
        fn clear(key: &'static str) -> Self {
            let old = std::env::var(key).ok();
            std::env::remove_var(key);
            Self { key, old }
        }

        fn set(key: &'static str, value: &'static str) -> Self {
            let old = std::env::var(key).ok();
            std::env::set_var(key, value);
            Self { key, old }
        }
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            if let Some(value) = &self.old {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn nyrt_silent_result_helper_tracks_presence_and_truthiness() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _clear = EnvRestore::clear("NYASH_NYRT_SILENT_RESULT");

        assert!(!nyrt_silent_result_present());
        assert!(!nyrt_silent_result_enabled());

        let _set = EnvRestore::set("NYASH_NYRT_SILENT_RESULT", "1");
        assert!(nyrt_silent_result_present());
        assert!(nyrt_silent_result_enabled());
    }

    #[test]
    fn nyrt_gc_metrics_helpers_track_shared_cluster_values() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _clear_json = EnvRestore::clear("NYASH_GC_METRICS_JSON");
        let _clear_text = EnvRestore::clear("NYASH_GC_METRICS");
        let _clear_sp = EnvRestore::clear("NYASH_GC_COLLECT_SP");
        let _clear_alloc = EnvRestore::clear("NYASH_GC_COLLECT_ALLOC");
        let _clear_auto = EnvRestore::clear("NYASH_LLVM_AUTO_SAFEPOINT");
        let _clear_threshold = EnvRestore::clear("NYASH_GC_ALLOC_THRESHOLD");

        assert!(!nyrt_gc_metrics_json_enabled());
        assert!(!nyrt_gc_metrics_text_enabled());
        assert_eq!(nyrt_gc_collect_sp_interval(), None);
        assert_eq!(nyrt_gc_collect_alloc_bytes(), None);
        assert!(!nyrt_llvm_auto_safepoint_enabled());
        assert_eq!(nyrt_gc_alloc_threshold_bytes(), None);

        let _set_json = EnvRestore::set("NYASH_GC_METRICS_JSON", "1");
        let _set_text = EnvRestore::set("NYASH_GC_METRICS", "1");
        let _set_sp = EnvRestore::set("NYASH_GC_COLLECT_SP", "11");
        let _set_alloc = EnvRestore::set("NYASH_GC_COLLECT_ALLOC", "22");
        let _set_auto = EnvRestore::set("NYASH_LLVM_AUTO_SAFEPOINT", "1");
        let _set_threshold = EnvRestore::set("NYASH_GC_ALLOC_THRESHOLD", "33");

        assert!(nyrt_gc_metrics_json_enabled());
        assert!(nyrt_gc_metrics_text_enabled());
        assert_eq!(nyrt_gc_collect_sp_interval(), Some(11));
        assert_eq!(nyrt_gc_collect_alloc_bytes(), Some(22));
        assert!(nyrt_llvm_auto_safepoint_enabled());
        assert_eq!(nyrt_gc_alloc_threshold_bytes(), Some(33));
    }

    #[test]
    fn nyrt_startup_mode_helpers_parse_auto_and_off_contracts() {
        let _lock = ENV_LOCK.lock().unwrap();
        let _clear_plugin = EnvRestore::clear("HAKO_NYRT_PLUGIN_HOST");
        let _clear_hooks = EnvRestore::clear("NYASH_NYRT_RUNTIME_HOOKS");
        let _clear_build = EnvRestore::clear("NYASH_NYRT_RUNTIME_BUILD");
        let _clear_path_prep = EnvRestore::clear("NYASH_NYRT_ENTRY_PATH_PREP");
        let _clear_ring0 = EnvRestore::clear("NYASH_NYRT_RING0_INIT");
        let _clear_minimal = EnvRestore::clear("NYASH_NYRT_MINIMAL_STARTUP");

        assert!(matches!(
            nyrt_plugin_host_mode().expect("default plugin host mode"),
            NyrtAutoOffMode::Auto
        ));
        assert!(matches!(
            nyrt_runtime_hooks_mode().expect("default runtime hooks mode"),
            NyrtAutoOffMode::Auto
        ));
        assert!(matches!(
            nyrt_runtime_build_mode().expect("default runtime build mode"),
            NyrtAutoOffMode::Auto
        ));
        assert!(matches!(
            nyrt_entry_path_prep_mode().expect("default path prep mode"),
            NyrtAutoOffMode::Auto
        ));
        assert!(matches!(
            nyrt_ring0_init_mode().expect("default ring0 mode"),
            NyrtAutoOffMode::Auto
        ));
        assert!(!nyrt_minimal_startup_enabled());

        let _set_plugin = EnvRestore::set("HAKO_NYRT_PLUGIN_HOST", "off");
        let _set_hooks = EnvRestore::set("NYASH_NYRT_RUNTIME_HOOKS", "off");
        let _set_build = EnvRestore::set("NYASH_NYRT_RUNTIME_BUILD", "off");
        let _set_path_prep = EnvRestore::set("NYASH_NYRT_ENTRY_PATH_PREP", "off");
        let _set_ring0 = EnvRestore::set("NYASH_NYRT_RING0_INIT", "off");
        let _set_minimal = EnvRestore::set("NYASH_NYRT_MINIMAL_STARTUP", "1");

        assert!(matches!(
            nyrt_plugin_host_mode().expect("off plugin host mode"),
            NyrtAutoOffMode::Off
        ));
        assert!(matches!(
            nyrt_runtime_hooks_mode().expect("off runtime hooks mode"),
            NyrtAutoOffMode::Off
        ));
        assert!(matches!(
            nyrt_runtime_build_mode().expect("off runtime build mode"),
            NyrtAutoOffMode::Off
        ));
        assert!(matches!(
            nyrt_entry_path_prep_mode().expect("off path prep mode"),
            NyrtAutoOffMode::Off
        ));
        assert!(matches!(
            nyrt_ring0_init_mode().expect("off ring0 mode"),
            NyrtAutoOffMode::Off
        ));
        assert!(nyrt_minimal_startup_enabled());
    }
}
