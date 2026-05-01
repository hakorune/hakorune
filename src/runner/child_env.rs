/*!
 * child_env.rs — Runner helper utilities (OOB strict, quiet exit policies)
 */

pub fn pre_run_reset_oob_if_strict() {
    if crate::config::env::oob_strict_fail() {
        crate::runtime::observe::reset();
    }
}

#[allow(dead_code)]
pub fn post_run_exit_if_oob_strict_triggered() -> ! {
    if crate::config::env::oob_strict_fail() && crate::runtime::observe::oob_seen() {
        crate::runtime::get_global_ring0()
            .log
            .error("[gate-c][oob-strict] Out-of-bounds observed → exit(1)");
        std::process::exit(1);
    }
    // If not strict or no OOB, return to caller path; caller should exit(…) itself.
    // This function is defined as diverging only when it actually exits above; otherwise it does nothing.
    // To keep signature simple for callers, they should not rely on this returning.
    std::process::exit(0)
}

/// Apply a consistent child environment for selfhost/core wrapper executions.
/// - Forces JSON-only quiet pipe
/// - Disables plugins to avoid host-side side effects
/// - Disables file-based using resolution (namespace-first policy)
/// - Skips nyash.toml env injection to reduce drift
/// - Propagates Stage-3 parser flags to ensure 'local' keyword support in nested compilations
pub fn apply_core_wrapper_env(cmd: &mut std::process::Command) {
    // Remove noisy or recursive toggles
    cmd.env_remove("NYASH_USE_NY_COMPILER");
    cmd.env_remove("NYASH_CLI_VERBOSE");
    // Enforce quiet JSON capture (allow override for debug)
    if std::env::var("NYASH_JSON_ONLY").is_err() {
        cmd.env("NYASH_JSON_ONLY", "1");
    }
    // Restrict environment to avoid plugin/using drift
    cmd.env("NYASH_DISABLE_PLUGINS", "1");
    cmd.env("NYASH_SKIP_TOML_ENV", "1");
    cmd.env("NYASH_USING_AST", "0");
    cmd.env("NYASH_ALLOW_USING_FILE", "0");
    cmd.env("HAKO_ALLOW_USING_FILE", "0");

    // Phase 25.1b fix: Propagate Stage-3 parser flags to child processes
    // When selfhost builder uses `using` to load modules, the inline compiler
    // needs Stage-3 support for `local` keyword. Without this, we get:
    // "Undefined variable: local" in nested compilation.
    // Preferred propagation: NYASH_FEATURES carries Stage-3 (legacy envs kept for compatibility)
    if let Ok(val) = std::env::var("NYASH_FEATURES") {
        cmd.env("NYASH_FEATURES", val);
    } else if crate::config::env::parser_stage3_enabled() {
        cmd.env("NYASH_FEATURES", "stage3");
    }
    if let Ok(val) = std::env::var("NYASH_PARSER_STAGE3") {
        cmd.env("NYASH_PARSER_STAGE3", val);
    }
    if let Ok(val) = std::env::var("HAKO_PARSER_STAGE3") {
        cmd.env("HAKO_PARSER_STAGE3", val);
    }
    if let Ok(val) = std::env::var("NYASH_PARSER_ALLOW_SEMICOLON") {
        cmd.env("NYASH_PARSER_ALLOW_SEMICOLON", val);
    }
}

/// Apply environment for selfhost/Ny compiler processes (ParserBox/EmitterBox/MirBuilderBox).
/// - Inherits core restrictions from apply_core_wrapper_env()
/// - Re-enables `using` file resolution for module loading (lang.compiler.parser.box, etc.)
/// - Re-enables plugins (compiler.hako needs FileBox to read source files)
pub fn apply_selfhost_compiler_env(cmd: &mut std::process::Command) {
    // Phase 25.1b: Start with core wrapper restrictions
    apply_core_wrapper_env(cmd);

    // Phase 25.1b: Re-enable file-based using for selfhost compiler module resolution
    // Selfhost compiler uses `using lang.compiler.parser.box`, which requires file resolution
    // (nyash.toml [modules] mapping is primary, but file fallback ensures robustness)
    cmd.env("HAKO_ALLOW_USING_FILE", "1");

    // Phase 28.2 fix: Re-enable plugins for selfhost compiler child process.
    // compiler.hako needs FileBox to read source files via `new FileBox(path).read()`.
    // Without plugins, the [plugin/missing] warning fires and file I/O fails.
    cmd.env("NYASH_DISABLE_PLUGINS", "0");

    // Phase 28.2b fix: Pass module mappings explicitly to child process.
    // compiler.hako uses `using lang.compiler.entry.compiler_stageb as StageBMain`.
    // Child process may not have access to nyash.toml or CWD context, so we pass
    // the required mappings via NYASH_MODULES environment variable.
    cmd.env(
        "NYASH_MODULES",
        "lang.compiler.entry.compiler_stageb=lang/src/compiler/entry/compiler_stageb.hako,\
         lang.compiler.entry.compiler=lang/src/compiler/entry/compiler.hako",
    );

    // Program(JSON v0) compat pin:
    // selfhost mirbuilder entry prefers HAKO_PROGRAM_JSON when present.
    // Populate it from HAKO_PROGRAM_JSON_FILE for child compiler processes so
    // planner-required lanes do not depend on FileBox host-method routing.
    if std::env::var("HAKO_PROGRAM_JSON")
        .ok()
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        if let Ok(path) = std::env::var("HAKO_PROGRAM_JSON_FILE") {
            if !path.is_empty() {
                if let Ok(program_json) = std::fs::read_to_string(&path) {
                    if !program_json.is_empty() {
                        cmd.env("HAKO_PROGRAM_JSON", program_json);
                    }
                }
            }
        }
    }
    if std::env::var("HAKO_PROGRAM_JSON")
        .ok()
        .as_deref()
        .map(|s| !s.is_empty())
        .unwrap_or(false)
        || std::env::var("HAKO_PROGRAM_JSON_FILE")
            .ok()
            .as_deref()
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    {
        // Match the Rust-owned Phase-0 Program(JSON)->MIR contract:
        // compiler static-box calls stay on the non-methodized surface.
        cmd.env("HAKO_MIR_BUILDER_METHODIZE", "0");
        // Keep compiler/build-bridge pins on bootstrap-rust-vm-keep even under strict/dev.
        // vm-hako reference does not own compiler static-box Global calls yet.
        cmd.env("NYASH_VM_HAKO_PREFER_STRICT_DEV", "0");
    }

    // Note: NYASH_USING_AST stays 0 (AST-based using not needed for basic module resolution)
}
