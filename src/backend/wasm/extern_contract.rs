/// Canonical extern-call contract shared by WASM codegen and runtime imports.
/// Keep this list as single source of truth for supported extern call names.
pub(crate) const EXTERN_CALL_MAP: [(&str, &str); 7] = [
    ("env.console.log", "console_log"),
    ("env.console.warn", "console_warn"),
    ("env.console.error", "console_error"),
    ("env.console.info", "console_info"),
    ("env.console.debug", "console_debug"),
    ("env.canvas.fillRect", "canvas_fillRect"),
    ("env.canvas.fillText", "canvas_fillText"),
];

pub(crate) fn extern_import_name(extern_name: &str) -> Option<&'static str> {
    EXTERN_CALL_MAP
        .iter()
        .find_map(|(name, import)| (*name == extern_name).then_some(*import))
}

pub(crate) fn supported_extern_calls_csv() -> String {
    EXTERN_CALL_MAP
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>()
        .join(", ")
}
