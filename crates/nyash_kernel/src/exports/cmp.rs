// Comparison helpers for LLVM harness.

// Phase 275 B2: Precise Int↔Float equality helper for LLVM harness
// Returns 1 if equal (precise), 0 otherwise
// Takes f64 directly (LLVM harness emits Float constants as double, not handle)
#[export_name = "nyash.cmp.int_float_eq"]
pub extern "C" fn nyash_cmp_int_float_eq(int_val: i64, float_val: f64) -> i64 {
    // Precise Int↔Float equality (Phase 275 B2)
    if float_val.is_nan() {
        return 0; // NaN != anything
    }
    if float_val.is_finite() && float_val.fract() == 0.0 {
        // Float is integral - check exact representability
        let f_int = float_val as i64;
        if (f_int as f64) == float_val && f_int == int_val {
            return 1; // Exact match
        }
    }
    0 // Non-integral or inexact
}
