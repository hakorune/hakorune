//! Parser environment switch facade.
//!
//! Parser code should read feature/debug switches through this module instead
//! of importing `crate::config::env` directly. This keeps the parser surface
//! closer to a future frontend crate boundary.

pub(crate) fn block_postfix_catch() -> bool {
    crate::config::env::block_postfix_catch()
}

pub(crate) fn cli_verbose_enabled() -> bool {
    crate::config::env::cli_verbose_enabled()
}

pub(crate) fn expr_postfix_catch() -> bool {
    crate::config::env::expr_postfix_catch()
}

pub(crate) fn method_catch() -> bool {
    crate::config::env::method_catch()
}

pub(crate) fn parser_allow_semicolon() -> bool {
    crate::config::env::parser_allow_semicolon()
}

pub(crate) fn parser_method_body_strict_enabled() -> bool {
    crate::config::env::parser_method_body_strict_enabled()
}

pub(crate) fn parser_stage3_enabled() -> bool {
    crate::config::env::parser_stage3_enabled()
}

pub(crate) fn parser_static_init_strict_enabled() -> bool {
    crate::config::env::parser_static_init_strict_enabled()
}

pub(crate) fn parser_static_seam_break_on_static_enabled() -> bool {
    crate::config::env::parser_static_seam_break_on_static_enabled()
}

pub(crate) fn parser_static_seam_tolerant_enabled() -> bool {
    crate::config::env::parser_static_seam_tolerant_enabled()
}

pub(crate) fn parser_static_trace_enabled() -> bool {
    crate::config::env::parser_static_trace_enabled()
}

pub(crate) fn parser_token_cursor_enabled() -> bool {
    crate::config::env::parser_token_cursor_enabled()
}

pub(crate) fn parser_try_compat_enabled() -> bool {
    crate::config::env::parser_try_compat_enabled()
}

pub(crate) fn unified_members() -> bool {
    crate::config::env::unified_members()
}
