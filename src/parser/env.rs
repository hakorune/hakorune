//! Parser environment switch facade.
//!
//! Parser code should read feature/debug switches through this module instead
//! of importing `crate::config::env` directly. This keeps the parser surface
//! closer to a future frontend crate boundary.

fn env_flag(var: &str) -> Option<bool> {
    std::env::var(var).ok().map(|value| {
        let value = value.to_ascii_lowercase();
        value == "1" || value == "true" || value == "on"
    })
}

fn nyash_features_list() -> Option<Vec<String>> {
    let raw = std::env::var("NYASH_FEATURES").ok()?;
    let list: Vec<String> = raw
        .split(',')
        .filter_map(|item| {
            let item = item.trim();
            if item.is_empty() {
                None
            } else {
                Some(item.to_ascii_lowercase())
            }
        })
        .collect();
    if list.is_empty() {
        None
    } else {
        Some(list)
    }
}

fn feature_enabled<const N: usize>(targets: [&str; N]) -> bool {
    let Some(list) = nyash_features_list() else {
        return false;
    };
    list.into_iter().any(|item| {
        let normalized = item.replace(['-', '_'], "");
        targets.iter().any(|target| normalized == *target)
    })
}

pub(crate) fn block_postfix_catch() -> bool {
    crate::frontend_env::block_postfix_catch()
}

pub(crate) fn debug_parse_local() -> bool {
    std::env::var("NYASH_DEBUG_PARSE_LOCAL").ok().as_deref() == Some("1")
}

pub(crate) fn deprecate_this_enabled() -> bool {
    std::env::var("NYASH_DEPRECATE_THIS").ok().as_deref() == Some("1")
}

pub(crate) fn enable_map_literal() -> bool {
    std::env::var("NYASH_ENABLE_MAP_LITERAL").ok().as_deref() == Some("1")
}

pub(crate) fn force_sugar_enabled() -> bool {
    std::env::var("NYASH_FORCE_SUGAR").ok().as_deref() == Some("1")
}

pub(crate) fn grammar_diff() -> bool {
    std::env::var("NYASH_GRAMMAR_DIFF").ok().as_deref() == Some("1")
}

pub(crate) fn cli_verbose_enabled() -> bool {
    crate::config::env::cli_verbose_enabled()
}

pub(crate) fn expr_postfix_catch() -> bool {
    crate::frontend_env::expr_postfix_catch()
}

pub(crate) fn method_catch() -> bool {
    crate::frontend_env::method_catch()
}

pub(crate) fn parser_allow_semicolon() -> bool {
    match std::env::var("NYASH_PARSER_ALLOW_SEMICOLON")
        .ok()
        .as_deref()
    {
        Some("0") | Some("false") | Some("off") => false,
        Some(_) => true,
        None => true,
    }
}

pub(crate) fn parser_allow_semicolon_raw() -> bool {
    std::env::var("NYASH_PARSER_ALLOW_SEMICOLON")
        .ok()
        .map(|value| {
            let value = value.to_ascii_lowercase();
            !(value == "0" || value == "false" || value == "off")
        })
        .unwrap_or(true)
}

pub(crate) fn parser_method_body_strict_enabled() -> bool {
    env_flag("NYASH_PARSER_METHOD_BODY_STRICT").unwrap_or(false)
}

pub(crate) fn parser_method_body_strict_raw() -> bool {
    std::env::var("NYASH_PARSER_METHOD_BODY_STRICT")
        .ok()
        .as_deref()
        == Some("1")
}

pub(crate) fn parser_stage3_enabled() -> bool {
    crate::frontend_env::parser_stage3_enabled()
}

pub(crate) fn parser_static_init_strict_enabled() -> bool {
    env_flag("NYASH_PARSER_STATIC_INIT_STRICT").unwrap_or(false)
}

pub(crate) fn parser_static_seam_break_on_static_enabled() -> bool {
    env_flag("NYASH_PARSER_SEAM_BREAK_ON_STATIC").unwrap_or(false)
}

pub(crate) fn parser_static_seam_tolerant_enabled() -> bool {
    env_flag("NYASH_PARSER_SEAM_TOLERANT").unwrap_or(false)
}

pub(crate) fn parser_static_trace_enabled() -> bool {
    env_flag("NYASH_PARSER_TRACE_STATIC").unwrap_or(false)
}

pub(crate) fn parser_trace_blocks() -> bool {
    std::env::var("NYASH_PARSER_TRACE_BLOCKS")
        .ok()
        .as_deref()
        == Some("1")
}

pub(crate) fn parser_token_cursor_enabled() -> bool {
    env_flag("NYASH_PARSER_TOKEN_CURSOR").unwrap_or(false)
}

pub(crate) fn parser_try_compat_enabled() -> bool {
    !feature_enabled(["notrycompat"])
}

pub(crate) fn syntax_sugar_level_raw() -> Option<String> {
    std::env::var("NYASH_SYNTAX_SUGAR_LEVEL").ok()
}

pub(crate) fn unified_members() -> bool {
    match std::env::var("NYASH_ENABLE_UNIFIED_MEMBERS").ok() {
        Some(value) => {
            let value = value.to_ascii_lowercase();
            !(value == "0" || value == "false" || value == "off")
        }
        None => true,
    }
}
