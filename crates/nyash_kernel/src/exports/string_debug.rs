use super::string_view::resolve_string_span_from_handle_nocache;
use std::sync::OnceLock;

fn stage1_string_debug_enabled() -> bool {
    static STAGE1_STRING_DEBUG: OnceLock<bool> = OnceLock::new();
    crate::env_flags::flag_on_cached(&STAGE1_STRING_DEBUG, "STAGE1_CLI_DEBUG")
}

fn stage1_string_handle_debug(handle: i64) -> (bool, usize, String) {
    if let Some(span) = resolve_string_span_from_handle_nocache(handle) {
        let s = span.as_text();
        let preview = if s.len() <= 48 {
            s.to_string()
        } else {
            s[..48].to_string()
        };
        return (true, s.len(), preview);
    }
    (false, 0, String::new())
}

pub(crate) fn stage1_string_debug_log_eq(a_h: i64, b_h: i64, result: i64) {
    if !stage1_string_debug_enabled() {
        return;
    }
    let (a_ok, a_len, a_preview) = stage1_string_handle_debug(a_h);
    let (b_ok, b_len, b_preview) = stage1_string_handle_debug(b_h);
    eprintln!(
        "[stage1/string_export] op=eq lhs={} lhs_ok={} lhs_len={} lhs_preview={:?} rhs={} rhs_ok={} rhs_len={} rhs_preview={:?} result={}",
        a_h, a_ok, a_len, a_preview, b_h, b_ok, b_len, b_preview, result
    );
}

pub(crate) fn stage1_string_debug_log_concat_materialize(a_h: i64, b_h: i64, out_h: i64) {
    if !stage1_string_debug_enabled() {
        return;
    }
    let (a_ok, a_len, a_preview) = stage1_string_handle_debug(a_h);
    let (b_ok, b_len, b_preview) = stage1_string_handle_debug(b_h);
    let (out_ok, out_len, out_preview) = stage1_string_handle_debug(out_h);
    eprintln!(
        "[stage1/string_export] op=concat_materialize lhs={} lhs_ok={} lhs_len={} lhs_preview={:?} rhs={} rhs_ok={} rhs_len={} rhs_preview={:?} out={} out_ok={} out_len={} out_preview={:?}",
        a_h,
        a_ok,
        a_len,
        a_preview,
        b_h,
        b_ok,
        b_len,
        b_preview,
        out_h,
        out_ok,
        out_len,
        out_preview
    );
}
