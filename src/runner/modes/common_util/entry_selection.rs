/*!
 * Entry function selection logic (Main.main → main fallback)
 *
 * NamingBox SSOT: Centralized entry point resolution with arity-aware fallback
 */

/// Select entry function for execution (Main.main → main fallback with env control)
///
/// Resolution order (NamingBox SSOT):
/// 1. Main.main/0 (arity-aware, future-proof for NYASH_BUILD_STATIC_MAIN_ENTRY=1)
/// 2. Main.main (legacy, arity-less, current default)
/// 3. main (top-level, only if NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1)
/// 4. "Main.main" (default fallback, caller handles missing function error)
pub fn select_entry_function(module: &crate::mir::MirModule) -> String {
    use crate::config::env;

    // NamingBox SSOT: Try arity-aware names first (future-proof)
    // Currently "Main.main" without arity is used by default (NYASH_BUILD_STATIC_MAIN_ENTRY=0)
    let main_with_arity = crate::mir::naming::encode_static_method("Main", "main", 0);
    if module.functions.contains_key(&main_with_arity) {
        return main_with_arity;
    }

    // Legacy: arity-less "Main.main" (current default)
    if module.functions.contains_key("Main.main") {
        return "Main.main".to_string();
    }

    // Fallback to top-level "main" if allowed
    let allow_top = env::entry_allow_toplevel_main();
    if allow_top && module.functions.contains_key("main") {
        return "main".to_string();
    }

    // Default: "Main.main" (may not exist, but caller will handle error)
    "Main.main".to_string()
}
