/*!
 * Stage-A spawn payload builders.
 *
 * RNR cleanup:
 * - Keep `selfhost.rs` focused on route sequencing.
 * - Keep Stage-A child args/env construction in one place.
 */

pub(crate) fn build_stage_a_child_extra_args() -> Vec<String> {
    let mut args = Vec::new();
    args.push("--".to_string());
    args.push("--stage-b".to_string());
    args.push("--stage3".to_string());

    if crate::config::env::ny_compiler_min_json() {
        args.push("--min-json".to_string());
    }

    // Optional: map env toggles to child args (prepasses)
    if crate::config::env::scopebox_enable() {
        args.push("--".to_string());
        args.push("--scopebox".to_string());
    }
    if crate::config::env::loopform_normalize() {
        args.push("--".to_string());
        args.push("--loopform".to_string());
    }

    // Optional: developer-provided child args passthrough (space-separated)
    if let Some(raw) = crate::config::env::ny_compiler_child_args() {
        let items: Vec<String> = raw
            .split(' ')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string())
            .collect();
        if !items.is_empty() {
            args.push("--".to_string());
            args.extend(items);
        }
    }

    args
}

pub(crate) fn build_stage_a_child_env(raw_source: &str) -> Vec<(String, String)> {
    let mut envs = Vec::new();

    // Stage-B contract expects raw source in HAKO_SRC.
    envs.push(("HAKO_SRC".to_string(), raw_source.to_string()));
    envs.push(("NYASH_DISABLE_PLUGINS".to_string(), "1".to_string()));
    envs.push(("NYASH_USING_AST".to_string(), "1".to_string()));
    envs.push(("NYASH_ALLOW_USING_FILE".to_string(), "1".to_string()));
    envs.push(("HAKO_ALLOW_USING_FILE".to_string(), "1".to_string()));
    envs.push(("NYASH_PARSER_ALLOW_SEMICOLON".to_string(), "1".to_string()));
    envs.push((
        "NYASH_FEATURES".to_string(),
        std::env::var("NYASH_FEATURES").unwrap_or_else(|_| "stage3,no-try-compat".to_string()),
    ));
    envs.push((
        "HAKO_JOINIR_STRICT".to_string(),
        std::env::var("HAKO_JOINIR_STRICT").unwrap_or_else(|_| "1".to_string()),
    ));
    envs.push((
        "HAKO_JOINIR_PLANNER_REQUIRED".to_string(),
        std::env::var("HAKO_JOINIR_PLANNER_REQUIRED").unwrap_or_else(|_| "1".to_string()),
    ));
    envs.push(("NYASH_QUIET".to_string(), "1".to_string()));
    envs.push(("HAKO_QUIET".to_string(), "1".to_string()));
    envs.push(("NYASH_CLI_VERBOSE".to_string(), "0".to_string()));

    if let Ok(modules) = std::env::var("HAKO_STAGEB_MODULES_LIST") {
        if !modules.trim().is_empty() {
            envs.push(("HAKO_STAGEB_MODULES_LIST".to_string(), modules));
        }
    }
    if let Ok(module_roots) = std::env::var("HAKO_STAGEB_MODULE_ROOTS_LIST") {
        if !module_roots.trim().is_empty() {
            envs.push(("HAKO_STAGEB_MODULE_ROOTS_LIST".to_string(), module_roots));
        }
    }

    envs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage_a_child_extra_args_have_stage_b_stage3_prefix() {
        let args = build_stage_a_child_extra_args();
        assert!(args.len() >= 3, "args must include stage-b/stage3");
        assert_eq!(args[0], "--");
        assert_eq!(args[1], "--stage-b");
        assert_eq!(args[2], "--stage3");
    }

    #[test]
    fn stage_a_child_env_includes_required_contract_keys() {
        let envs = build_stage_a_child_env("print(\"hi\")");
        let keys: Vec<&str> = envs.iter().map(|(k, _)| k.as_str()).collect();
        assert!(keys.contains(&"HAKO_SRC"));
        assert!(keys.contains(&"NYASH_DISABLE_PLUGINS"));
        assert!(keys.contains(&"HAKO_JOINIR_STRICT"));
        assert!(keys.contains(&"HAKO_JOINIR_PLANNER_REQUIRED"));
    }
}
