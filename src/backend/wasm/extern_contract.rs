/// Canonical extern-call contract shared by WASM codegen and runtime imports.
/// Keep this list as single source of truth for supported extern call names.
pub(crate) const EXTERN_CALL_MAP: [(&str, &str); 8] = [
    ("env.console.log", "console_log"),
    ("env.console.warn", "console_warn"),
    ("env.console.error", "console_error"),
    ("env.console.info", "console_info"),
    ("env.console.debug", "console_debug"),
    ("env.canvas.fillRect", "canvas_fillRect"),
    ("env.canvas.fillText", "canvas_fillText"),
    ("env.canvas.clear", "canvas_clear"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extern_contract_supported_name_maps_to_import() {
        assert_eq!(extern_import_name("env.console.log"), Some("console_log"));
        assert_eq!(extern_import_name("env.console.debug"), Some("console_debug"));
        assert_eq!(extern_import_name("env.canvas.fillRect"), Some("canvas_fillRect"));
        assert_eq!(extern_import_name("env.canvas.clear"), Some("canvas_clear"));
    }

    #[test]
    fn extern_contract_unsupported_name_is_none() {
        assert_eq!(extern_import_name("env.console.trace"), None);
        assert_eq!(extern_import_name("env.canvas.strokeRect"), None);
    }

    #[test]
    fn extern_contract_supported_csv_contains_known_entries() {
        let csv = supported_extern_calls_csv();
        assert!(csv.contains("env.console.log"));
        assert!(csv.contains("env.console.warn"));
        assert!(csv.contains("env.console.error"));
        assert!(csv.contains("env.console.info"));
        assert!(csv.contains("env.console.debug"));
        assert!(csv.contains("env.canvas.clear"));
    }
}
