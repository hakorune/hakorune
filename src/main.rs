// Minimal CLI entry point for Nyash.
// Delegates to the library crate (`nyash_rust`) for all functionality.

use nyash_rust::cli::CliConfig;
use nyash_rust::config::env as env_config;
use nyash_rust::runner::NyashRunner;

fn maybe_enable_stack_overflow_backtrace() {
    if env_config::env_string("NYASH_DEBUG_STACK_OVERFLOW").as_deref() != Some("1") {
        return;
    }
    #[cfg(not(windows))]
    unsafe {
        let _ = backtrace_on_stack_overflow::enable();
    }
}

fn maybe_pin_program_json_from_file_env() {
    let current = std::env::var("HAKO_PROGRAM_JSON").unwrap_or_default();
    if !current.is_empty() {
        return;
    }
    let Ok(path) = std::env::var("HAKO_PROGRAM_JSON_FILE") else {
        return;
    };
    if path.is_empty() {
        return;
    }
    if let Ok(program_json) = std::fs::read_to_string(&path) {
        if !program_json.is_empty() {
            std::env::set_var("HAKO_PROGRAM_JSON", program_json);
        }
    }
}

fn maybe_pin_phase0_program_json_builder_env() {
    let has_program_json = std::env::var("HAKO_PROGRAM_JSON")
        .ok()
        .as_deref()
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    let has_program_json_file = std::env::var("HAKO_PROGRAM_JSON_FILE")
        .ok()
        .as_deref()
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    if has_program_json || has_program_json_file {
        // Keep Phase-0 Program(JSON)->MIR on the legacy/static-box call surface.
        // Rust host-providers already pin this route to methodize=0.
        std::env::set_var("HAKO_MIR_BUILDER_METHODIZE", "0");
        // Program(JSON) contract pins are compiler/build-bridge checks.
        // Under strict/dev, `backend=vm` normally prefers vm-hako reference,
        // but that lane does not own compiler static-box Global calls yet.
        // Keep this narrow bridge on bootstrap-rust-vm-keep until vm-hako grows the same surface.
        std::env::set_var("NYASH_VM_HAKO_PREFER_STRICT_DEV", "0");
    }
}

/// Thin entry point - delegates to CLI parsing and runner execution
fn main() {
    // Optional: enable backtrace on stack overflow for deep debug runs.
    // Guarded by env to keep default behavior unchanged.
    maybe_enable_stack_overflow_backtrace();

    // hv1 direct (primary): earliest possible check before any bootstrap/log init
    // If NYASH_VERIFY_JSON is present and route is requested, execute and exit.
    // This avoids plugin host/registry initialization and keeps output minimal.
    let has_json = env_config::env_present("NYASH_VERIFY_JSON");
    let route = nyash_rust::config::env::verify_primary_is_hakovm();
    // Force flag may allow hv1-inline without route
    let force_hv1_flag = nyash_rust::config::env::env_bool("HAKO_VERIFY_V1_FORCE_HAKOVM");
    if has_json && (route || force_hv1_flag) {
        let json = env_config::env_string("NYASH_VERIFY_JSON").unwrap_or_default();
        // Option A: force hv1 inline (dev) — bypass parser entirely
        if force_hv1_flag {
            let rc = nyash_rust::runner::hv1_inline::run_json_v1_inline(&json);
            println!("{}", rc);
            std::process::exit(rc);
        }
        // Minimal runner (no plugin init here); config parse is cheap and has no side effects.
        let cfg = CliConfig::parse();
        let runner = NyashRunner::new(cfg);
        if env_config::cli_verbose_enabled() {
            let ring0 = nyash_rust::runtime::get_global_ring0();
            ring0.log.debug("[hv1-direct] early-exit (main)");
        }
        let rc = nyash_rust::runner::core_executor::execute_json_artifact(&runner, &json);
        println!("{}", rc);
        std::process::exit(rc);
    }

    // Bootstrap env overrides from nyash.toml [env] early (管理棟)
    env_config::bootstrap_from_toml_env();
    maybe_pin_program_json_from_file_env();
    maybe_pin_phase0_program_json_builder_env();
    // Parse command-line arguments
    let config = CliConfig::parse();
    // Ensure Ring0 before any deprecation logging path that may call get_global_ring0().
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    // Deprecation notice when invoked via legacy binary name
    if let Ok(exe) = std::env::current_exe() {
        if let Some(name) = exe.file_name().and_then(|s| s.to_str()) {
            if name.eq_ignore_ascii_case("nyash") {
                let ring0 = nyash_rust::runtime::get_global_ring0();
                ring0
                    .log
                    .warn("[deprecate] 'nyash' binary is deprecated. Please use 'hakorune'.");
            }
        }
    }
    // Legacy binary deprecation: prefer 'hakorune'
    if let Ok(exe) = std::env::current_exe() {
        if let Some(name) = exe.file_name().and_then(|s| s.to_str()) {
            let allow_legacy = env_config::env_string("HAKO_ALLOW_NYASH").as_deref() == Some("1")
                || env_config::env_string("NYASH_ALLOW_NYASH").as_deref() == Some("1");
            if name.eq_ignore_ascii_case("nyash") && !allow_legacy {
                let ring0 = nyash_rust::runtime::get_global_ring0();
                ring0
                    .log
                    .warn("[deprecate] 'nyash' binary is deprecated. Please use 'hakorune'.");
                std::process::exit(2);
            }
        }
    }
    // Create and run the execution coordinator
    let runner = NyashRunner::new(config);
    runner.run();
}

#[cfg(test)]
mod tests {

    use nyash_rust::box_trait::{NyashBox, StringBox};

    #[test]
    fn test_main_functionality() {
        // Smoke: library module path wiring works
        let string_box = StringBox::new("test".to_string());
        assert_eq!(string_box.to_string_box().value, "test");
        let _ = (); // CLI wiring exists via nyash_rust::cli
    }
}
