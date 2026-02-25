//! Naming helpers shared in MIR Interpreter handlers.

/// Normalize an optional arity suffix from a function/method name.
/// Examples:
/// - "env.get/1" -> "env.get"
/// - "hostbridge.extern_invoke/3" -> "hostbridge.extern_invoke"
/// - "print" -> "print"
#[inline]
pub fn normalize_arity_suffix(name: &str) -> &str {
    match name.split_once('/') {
        Some((base, _)) => base,
        None => name,
    }
}
