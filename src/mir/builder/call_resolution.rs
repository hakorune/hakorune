/*!
 * Call Resolution Utilities - Type-safe function call helpers
 *
 * ChatGPT5 Pro Design: Stateless helpers for compile-time function resolution
 * These utilities can be used across different parts of the compiler pipeline
 */

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
