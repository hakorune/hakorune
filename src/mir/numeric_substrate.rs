/*!
 * Numeric substrate vocabulary for allocator-grade MIR metadata.
 *
 * This module owns the fixed-width and pointer-sized integer type names used by
 * substrate rows. It is intentionally metadata-only today: values still execute
 * on the current dynamic Integer/i64 lane until a later row adds exact
 * width/range/overflow semantics.
 */

mod checked_ops;
mod exact_values;
mod target;

#[cfg(test)]
mod tests;

#[allow(unused_imports)]
// staged exact-op API; later verifier/backend rows consume the facade.
pub(crate) use checked_ops::{
    exact_numeric_checked_arithmetic, exact_numeric_compare, exact_numeric_logical_shr,
    ExactNumericArithmeticError, ExactNumericArithmeticOp, ExactNumericCompareError,
    ExactNumericCompareOp, ExactNumericShiftError,
};
#[allow(unused_imports)]
// staged exact-value API; later MIR fact/lowering rows consume the facade.
pub(crate) use exact_values::{
    exact_numeric_const_from_i128, exact_numeric_mir_signature_from_declared_names,
    exact_numeric_mir_type_from_declared_name,
    exact_numeric_type_requires_dynamic_integer_range_check,
    exact_numeric_value_from_dynamic_integer,
    exact_numeric_value_from_dynamic_integer_for_declared_name, ExactNumericConstValue,
    ExactNumericConversionError, ExactNumericMirSignature, ExactNumericMirType,
};
#[allow(unused_imports)]
// staged target/type vocabulary; later cross-target rows consume the facade.
pub(crate) use target::{
    classify_numeric_kind_for_target, classify_numeric_type_name, ExactNumericRange,
};
#[allow(unused_imports)] // staged target/type vocabulary; keep the pre-split facade stable.
pub use target::{
    NumericKind, NumericResolvedWidth, NumericSignedness, NumericTarget, NumericTypeName,
    NumericWidth,
};

pub(crate) fn is_numeric_integer_type_name(name: &str) -> bool {
    classify_numeric_kind_for_target(name, NumericTarget::host()).is_some()
}

pub(crate) fn is_inline_i64_storage_type_name(name: &str) -> bool {
    is_numeric_integer_type_name(name)
        || matches!(
            name,
            "IntegerBox" | "Integer" | "BoolBox" | "Bool" | "bool" | "i1"
        )
}
