//! Catalog of NYASH environment variables (AGENTS.md 287)
//!
//! Provides runtime enumeration of all environment variables used by Hakorune.
//! Used for diagnostics, documentation, and CI validation.

use std::fmt;

/// System where the env var applies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppliesTo {
    /// Core runtime (VM, interpreter, executor)
    Runtime,
    /// Parser/Compiler (JoinIR, MIR, backend)
    Compiler,
    /// Macro System (MacroBox, engine, expand)
    Macro,
    /// Box Factory (Plugin, builtin, user-defined)
    BoxFactory,
    /// Testing (Test harness, args, filter)
    Testing,
    /// CLI (Command-line options, logging)
    CLI,
    /// Selfhost (Nyash compiler, toolchain)
    Selfhost,
    /// Unknown/Other
    Other,
}

/// Metadata for an environment variable
#[derive(Debug, Clone)]
pub struct EnvVarMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub applies_to: AppliesTo,
    pub default: Option<&'static str>,
}

impl fmt::Display for EnvVarMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.description)
    }
}

/// Enumerate all known environment variables (Phase 287)
///
/// This function MUST be kept in sync with actual usage.
/// Run `cargo test env_vars_consistency` to validate.
pub fn env_vars() -> Vec<EnvVarMeta> {
    vec![
        // Macro System (Phase 286A)
        EnvVarMeta {
            name: "NYASH_MACRO_DISABLE",
            description: "Disable macro system entirely (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_ENABLE",
            description: "Enable macro system (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_TRACE",
            description: "Trace macro expansion (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_MAX_PASSES",
            description: "Max macro expansion passes",
            applies_to: AppliesTo::Macro,
            default: Some("32"),
        },
        EnvVarMeta {
            name: "NYASH_MACRO_CYCLE_WINDOW",
            description: "Cycle detection window size",
            applies_to: AppliesTo::Macro,
            default: Some("8"),
        },
        EnvVarMeta {
            name: "NYASH_MACRO_DERIVE_ALL",
            description: "Derive all Equals+ToString (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_DERIVE",
            description: "Comma-separated derive targets (Equals,ToString)",
            applies_to: AppliesTo::Macro,
            default: Some("Equals,ToString"),
        },
        EnvVarMeta {
            name: "NYASH_MACRO_TRACE_JSONL",
            description: "Path to JSONL trace file",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_PATHS",
            description: "Comma-separated macro source paths",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_NY",
            description: "Enable Nyash MacroBox loading (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_NY_PATHS",
            description: "Legacy alias for NYASH_MACRO_PATHS",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX",
            description: "Enable MacroBox execution (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_EXAMPLE",
            description: "Enable built-in UppercasePrintMacro (1=true)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_ENABLE",
            description: "Comma-separated MacroBox names",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_STRICT",
            description: "Strict error mode (1=true)",
            applies_to: AppliesTo::Macro,
            default: Some("1"),
        },
        EnvVarMeta {
            name: "NYASH_MACRO_TOPLEVEL_ALLOW",
            description: "Allow top-level static functions (deprecated)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_CHILD_RUNNER",
            description: "Use runner script for macro expansion (deprecated)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_CHILD",
            description: "Enable child-process mode for macro expansion",
            applies_to: AppliesTo::Macro,
            default: Some("1"),
        },
        EnvVarMeta {
            name: "NYASH_MACRO_BOX_NY_IDENTITY_ROUNDTRIP",
            description: "Roundtrip identity macros via JSON",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        // Test Harness (Phase 286A)
        EnvVarMeta {
            name: "NYASH_TEST_RUN",
            description: "Enable test harness injection (1=true)",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TEST_ARGS_JSON",
            description: "JSON args for test functions",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TEST_ARGS_DEFAULTS",
            description: "Use zero defaults for missing test args (1=true)",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TEST_FILTER",
            description: "Substring filter for test names",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TEST_FORCE",
            description: "Force harness injection even with main (1=true)",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TEST_ENTRY",
            description: "Entry policy: wrap|override",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TEST_RETURN",
            description: "Return policy: tests|original",
            applies_to: AppliesTo::Testing,
            default: None,
        },
        // Box Factory (Phase 286B)
        EnvVarMeta {
            name: "NYASH_BOX_FACTORY_POLICY",
            description: "Factory policy: strict_plugin_first|compat_plugin_first|builtin_first",
            applies_to: AppliesTo::BoxFactory,
            default: Some("strict_plugin_first"),
        },
        EnvVarMeta {
            name: "NYASH_USE_PLUGIN_BUILTINS",
            description: "Allow plugins to override builtins (1=true)",
            applies_to: AppliesTo::BoxFactory,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_PLUGIN_OVERRIDE_TYPES",
            description: "Comma-separated types plugins may override",
            applies_to: AppliesTo::BoxFactory,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_DISABLE_PLUGINS",
            description: "Disable all plugins (1=true)",
            applies_to: AppliesTo::BoxFactory,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_DEBUG_PLUGIN",
            description: "Debug plugin loading (1=true)",
            applies_to: AppliesTo::BoxFactory,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_PLUGIN_EXEC_MODE",
            description: "Plugin execution mode: module_first|dynamic_only|dynamic_first",
            applies_to: AppliesTo::BoxFactory,
            default: Some("module_first"),
        },
        // Runtime (Phase 29ab)
        EnvVarMeta {
            name: "NYASH_STR_CP",
            description: "String index mode: 1=codepoint, 0=byte (default)",
            applies_to: AppliesTo::Runtime,
            default: Some("0"),
        },
        EnvVarMeta {
            name: "NYASH_HOST_HANDLE_ALLOC_POLICY",
            description: "Host handle alloc policy: lifo|none|off|no-reuse",
            applies_to: AppliesTo::Runtime,
            default: Some("lifo"),
        },
        EnvVarMeta {
            name: "NYASH_STRING_SPAN_CACHE_POLICY",
            description: "String span cache policy: on|off|enabled|disabled|1|0",
            applies_to: AppliesTo::Runtime,
            default: Some("on"),
        },
        EnvVarMeta {
            name: "NYASH_SCHED_TRACE",
            description: "Trace scheduler poll/move (1=true)",
            applies_to: AppliesTo::Runtime,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_SCHED_POLL_BUDGET",
            description: "Scheduler poll budget (tasks per poll)",
            applies_to: AppliesTo::Runtime,
            default: Some("1"),
        },
        EnvVarMeta {
            name: "NYASH_SCHED_POLL_IN_SAFEPOINT",
            description: "Enable scheduler poll in safepoint bridge (1=true, 0=false)",
            applies_to: AppliesTo::Runtime,
            default: Some("1"),
        },
        EnvVarMeta {
            name: "NYASH_RING0_LOG_LEVEL",
            description: "Ring0 log minimum level (DEBUG|INFO|WARN|ERROR)",
            applies_to: AppliesTo::Runtime,
            default: Some("INFO"),
        },
        // Selfhost (Phase 286B)
        EnvVarMeta {
            name: "NYASH_NY_COMPILER_TIMEOUT_MS",
            description: "Timeout for child Nyash compiler (ms)",
            applies_to: AppliesTo::Selfhost,
            default: Some("2000"),
        },
        EnvVarMeta {
            name: "NYASH_STAGE1_EMIT_TIMEOUT_MS",
            description: "Timeout override for Stage-1 emit routes (ms, fallback to NYASH_NY_COMPILER_TIMEOUT_MS)",
            applies_to: AppliesTo::Selfhost,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_NY_COMPILER_MIN_JSON",
            description: "Minimize JSON output from Nyash compiler (1=true)",
            applies_to: AppliesTo::Selfhost,
            default: None,
        },
        // CLI (Phase 286B)
        EnvVarMeta {
            name: "NYASH_CLI_VERBOSE",
            description: "Verbose CLI output (1=true)",
            applies_to: AppliesTo::CLI,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_ROOT",
            description: "Repo root hint for tools/path resolution (optional)",
            applies_to: AppliesTo::CLI,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_WASM_ROUTE_POLICY",
            description: "WASM route policy: default",
            applies_to: AppliesTo::Compiler,
            default: Some("default"),
        },
        EnvVarMeta {
            name: "NYASH_WASM_ROUTE_TRACE",
            description: "Emit one-line wasm route trace with shape_id (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_USING_DYLIB_AUTOLOAD",
            description: "Auto-load [using.*] dylib packages (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        // Compiler/Tokenizer (Phase 29bq)
        EnvVarMeta {
            name: "NYASH_BINOP_REPROP_DEBUG",
            description: "Trace binop type re-propagation (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_PHI_GLOBAL_DEBUG",
            description: "Trace global PHI type inference (set to any value)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_PARSER_ALLOW_SEMICOLON",
            description: "Allow optional semicolon separator (default ON; 0=disable)",
            applies_to: AppliesTo::Compiler,
            default: Some("1"),
        },
        EnvVarMeta {
            name: "NYASH_STRICT_12_7",
            description: "Tokenizer strict 12.7 mode (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_TOK_TRACE",
            description: "Tokenizer trace output (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_GRAMMAR_DIFF",
            description: "Tokenizer vs grammar diff trace (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        // Compiler/LLVM provider (Phase 29bq)
        EnvVarMeta {
            name: "NYASH_NY_LLVM_COMPILER",
            description: "Path to ny-llvmc compiler binary",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_LLVM_USE_CAPI",
            description: "Enable LLVM C-API provider (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_V1_EXTERN_PROVIDER_C_ABI",
            description: "Enable extern provider C-ABI bridge (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_BACKEND_COMPILE_RECIPE",
            description: "Backend compile recipe selector (for example pure-first)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_BACKEND_COMPAT_REPLAY",
            description: "Backend compat replay selector (for example harness)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_CAPI_PURE",
            description: "Legacy alias for historical pure C-API/FFI lowering route (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_LLVM_EMIT_PROVIDER",
            description: "Select LLVM emit compat keep (llvmlite|ny-llvmc)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_LLVM_OPT_LEVEL",
            description: "LLVM opt level (primary)",
            applies_to: AppliesTo::Compiler,
            default: Some("0"),
        },
        EnvVarMeta {
            name: "HAKO_LLVM_OPT_LEVEL",
            description: "LLVM opt level (legacy alias)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_AOT_FFI_LIB",
            description: "Path to AOT FFI library",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_AOT_USE_FFI",
            description: "Enable AOT FFI path (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_AOT_LDFLAGS",
            description: "Extra ldflags for AOT link",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "HAKO_CABI_TRACE",
            description: "Trace LLVM C-ABI bridge (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_RESOLVE_TRACE",
            description: "Trace using/prelude resolution (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_RESOLVE_SEAM_DEBUG",
            description: "Insert using boundary markers for diagnostics (1=true)",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_RESOLVE_DUMP_MERGED",
            description: "Dump merged prelude+main source to the given path",
            applies_to: AppliesTo::Compiler,
            default: None,
        },
        // Macro capabilities (Phase 286A)
        EnvVarMeta {
            name: "NYASH_MACRO_CAP_IO",
            description: "Allow IO in macros (FileBox, PathBox)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_CAP_NET",
            description: "Allow network in macros (HTTP, Socket)",
            applies_to: AppliesTo::Macro,
            default: None,
        },
        EnvVarMeta {
            name: "NYASH_MACRO_CAP_ENV",
            description: "Allow env read in macros",
            applies_to: AppliesTo::Macro,
            default: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_vars_all_have_descriptions() {
        for meta in env_vars() {
            assert!(
                !meta.description.is_empty(),
                "{} has no description",
                meta.name
            );
            assert!(!meta.name.is_empty(), "empty name found");
        }
    }

    #[test]
    fn test_env_vars_no_duplicates() {
        let names: Vec<&str> = env_vars().iter().map(|m| m.name).collect();
        let mut seen = std::collections::HashSet::new();
        for name in names {
            assert!(!seen.contains(name), "Duplicate env var: {}", name);
            seen.insert(name);
        }
    }

    #[test]
    fn test_env_vars_applies_to_valid() {
        for meta in env_vars() {
            // Just ensure it can be compared without panicking
            let _ = format!("{:?}", meta.applies_to);
        }
    }
}
