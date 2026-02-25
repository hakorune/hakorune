/*!
 * Plugin guard utilities
 *
 * Centralized helper to check required plugin providers and emit
 * consistent diagnostics across runner modes.
 */

/// Core required providers (SSOT - Phase 134 P1)
///
/// These are essential for basic program execution (substring, array ops, console output).
/// Missing any of these in strict mode will cause immediate failure.
pub fn gather_core_required_providers() -> Vec<String> {
    vec![
        "StringBox".to_string(),
        "IntegerBox".to_string(),
        "ArrayBox".to_string(),
        "ConsoleBox".to_string(),
    ]
}

/// Build the list of required provider type names.
///
/// Priority:
/// - If env `NYASH_PLUGIN_OVERRIDE_TYPES` is set, use it (comma-separated).
/// - Otherwise, return a conservative default set used in VM paths.
pub fn gather_required_providers() -> Vec<String> {
    if let Ok(list) = std::env::var("NYASH_PLUGIN_OVERRIDE_TYPES") {
        let mut v: Vec<String> = list
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        v.sort();
        v.dedup();
        return v;
    }
    // Default conservative set
    let v = vec![
        "FileBox".to_string(),
        "ConsoleBox".to_string(),
        "ArrayBox".to_string(),
        "MapBox".to_string(),
        "StringBox".to_string(),
        "IntegerBox".to_string(),
    ];
    v
}

/// Return missing providers by checking the unified registry.
pub fn detect_missing_providers(required: &[String]) -> Vec<String> {
    let reg = nyash_rust::runtime::get_global_registry();
    let mut missing: Vec<String> = Vec::new();
    for t in required {
        if reg.get_provider(t).is_none() {
            missing.push(t.clone());
        }
    }
    missing
}

/// Emit hints for specific provider types.
fn emit_hints_for(missing: &[String]) {
    if missing.iter().any(|t| t == "FileBox") {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .warn("[plugin/hint] FileBox plugin is required for file I/O (new FileBox/open/read).");
        ring0.log.warn("[plugin/hint] Build and load plugin: see tools/plugin_v2_smoke.sh or configure nyash.toml [libraries.*.FileBox].");
        ring0
            .log
            .warn("[plugin/hint] Ensure LD_LIBRARY_PATH (or platform equivalent) includes the plugin directory.");
        ring0.log.warn("[plugin/hint] For analyzer runs, you can avoid FileBox via --source-file <path> <text>.");
    }
}

/// Check provider availability and emit diagnostics.
///
/// - `strict`: exit(1) when any CORE provider is missing (Phase 134 P1).
/// - `quiet_pipe`: respect quiet JSON pipelines; we still write diagnostics to stderr only.
/// - `label`: context label (e.g., "vm", "vm-fallback") for messages.
pub fn check_and_report(strict: bool, quiet_pipe: bool, label: &str) {
    // When plugins are explicitly disabled (hermetic gates, analyzer runs),
    // missing plugin providers are expected; keep stderr clean unless strict fails.
    if crate::config::env::disable_plugins() {
        return;
    }

    let required = gather_required_providers();
    let missing = detect_missing_providers(&required);
    if missing.is_empty() {
        return;
    }

    // Phase 134 P1: In strict mode, fail-fast only if CORE providers are missing
    if strict {
        let core_required = gather_core_required_providers();
        let core_missing: Vec<_> = missing
            .iter()
            .filter(|m| core_required.contains(m))
            .cloned()
            .collect();

        if !core_missing.is_empty() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.error(&format!(
                "❌ {} plugin-first strict: missing CORE providers: {:?}",
                label, core_missing
            ));
            ring0.log.error(&format!(
                "[plugin/strict] Core providers are required: {:?}",
                core_required
            ));
            emit_hints_for(&missing);
            // Do not print anything to stdout in quiet mode; just exit with 1
            std::process::exit(1);
        } else {
            // Strict mode but only non-core providers missing (e.g., FileBox, MapBox)
            // Log warning but continue
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.warn(&format!(
                "[plugin/missing] {} providers not loaded (non-core): {:?}",
                label, missing
            ));
            ring0.log.warn("[plugin/missing] hint: set NYASH_DEBUG_PLUGIN=1 or NYASH_CLI_VERBOSE=1 to see plugin init errors");
            emit_hints_for(&missing);
        }
    } else {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.warn(&format!(
            "[plugin/missing] {} providers not loaded: {:?}",
            label, missing
        ));
        ring0.log.warn("[plugin/missing] hint: set NYASH_DEBUG_PLUGIN=1 or NYASH_CLI_VERBOSE=1 to see plugin init errors");
        emit_hints_for(&missing);
        if quiet_pipe {
            // In quiet JSON mode, avoid noisy stdout; hints are on stderr already.
        }
    }
}
