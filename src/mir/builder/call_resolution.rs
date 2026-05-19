/*!
 * Call Resolution Utilities - Type-safe function call helpers
 *
 * ChatGPT5 Pro Design: Stateless helpers for compile-time function resolution
 * These utilities can be used across different parts of the compiler pipeline
 */

/// Check if function name is a built-in global function
/// These functions are resolved at compile-time to Callee::Global
pub fn is_builtin_function(name: &str) -> bool {
    matches!(
        name,
        // Core runtime functions
        "print" | "error" | "panic" | "exit" | "now" |
        // Type operation functions
        "isType" | "asType" |
        // Math functions (may be expanded)
        "abs" | "min" | "max"
    )
}

/// Check if function name is an external/host function
/// These functions are resolved to Callee::Extern and handled by runtime
pub fn is_extern_function(name: &str) -> bool {
    name.starts_with("nyash.") // Host functions use nyash.* namespace
}

/// Get suggested resolution for unresolved function names
/// Provides helpful error messages for common mistakes
pub fn suggest_resolution(name: &str) -> String {
    match name {
        "print" | "error" | "panic" | "exit" => {
            format!("Consider using ::{}() for global function or check if you're in a box with a {} method", name, name)
        }
        name if name.starts_with("str") || name.starts_with("string") => {
            "Consider using StringBox methods or string.* functions".to_string()
        }
        name if name.starts_with("array") || name.starts_with("arr") => {
            "Consider using ArrayBox methods or array.* functions".to_string()
        }
        _ => {
            format!(
                "Function '{}' not found. Check spelling or add explicit scope qualifier",
                name
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_function_detection() {
        assert!(is_builtin_function("print"));
        assert!(is_builtin_function("error"));
        assert!(is_builtin_function("panic"));
        assert!(is_builtin_function("isType"));
        assert!(!is_builtin_function("custom_function"));
        assert!(!is_builtin_function("nyash.console.log"));
    }

    #[test]
    fn test_extern_function_detection() {
        assert!(is_extern_function("nyash.console.log"));
        assert!(is_extern_function("nyash.fs.read"));
        assert!(!is_extern_function("print"));
        assert!(!is_extern_function("custom_function"));
    }
}
