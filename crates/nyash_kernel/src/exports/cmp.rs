//! Comparison helpers for the LLVM harness.

/// The LLVM harness supplies an f64 directly and reads the Bool as an i64.
#[export_name = "nyash.cmp.int_float_eq"]
pub extern "C" fn nyash_cmp_int_float_eq(int_val: i64, float_val: f64) -> i64 {
    i64::from(nyash_rust::runtime::semantics::int_float_eq(int_val, float_val))
}

#[cfg(test)]
mod tests {
    use super::nyash_cmp_int_float_eq;

    #[test]
    fn llvm_int_float_equality_checks_the_i64_range() {
        for (integer, float, expected) in [
            (i64::MAX, 9_223_372_036_854_775_808.0, 0),
            (i64::MIN, -9_223_372_036_854_775_808.0, 1),
            (3, f64::NAN, 0),
            (3, 3.0, 1),
        ] {
            assert_eq!(nyash_cmp_int_float_eq(integer, float), expected);
        }
    }
}
