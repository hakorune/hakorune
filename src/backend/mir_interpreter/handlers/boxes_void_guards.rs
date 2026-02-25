// Void/VoidBox graceful method guards
// Extracted from boxes.rs to eliminate duplication (28 lines → 6 lines)

use super::super::VMValue;

/// Handle common methods on Void/VoidBox with graceful fallback values.
/// Used for short-circuit patterns like `A or not last.is_eof()` where `last` may be absent.
///
/// Returns Some(VMValue) if the method is a known void-safe method, None otherwise.
pub(super) fn handle_void_method(method: &str) -> Option<VMValue> {
    match method {
        "is_eof" => Some(VMValue::Bool(false)),
        "length" => Some(VMValue::Integer(0)),
        "substring" => Some(VMValue::String(String::new())),
        "push" => Some(VMValue::Void),
        "get_position" => Some(VMValue::Integer(0)),
        "get_line" => Some(VMValue::Integer(1)),
        "get_column" => Some(VMValue::Integer(1)),
        _ => None,
    }
}
