use super::super::VMValue;
use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_mir_type_from_declared_name,
    exact_numeric_value_from_dynamic_integer, ExactNumericConversionError, NumericTarget,
};

/// Shared subordinate value/type/range checker for exact-numeric contracts.
/// Boundary owners remain responsible for timing, carrier validation, and tags.
pub(super) fn validate_exact_numeric_runtime_value(
    value: &VMValue,
    declared_type_name: &str,
) -> Result<(), &'static str> {
    let Some(exact_type) =
        exact_numeric_mir_type_from_declared_name(Some(declared_type_name), NumericTarget::host())
    else {
        return Err("unknown-exact-type");
    };

    let result = match value {
        VMValue::Integer(value) => {
            exact_numeric_value_from_dynamic_integer(*value, &exact_type).map(|_| ())
        }
        VMValue::ExactNumeric(value) if value.source_name == exact_type.source_name => {
            exact_numeric_const_from_i128(value.value, &exact_type).map(|_| ())
        }
        _ => return Err("runtime-type-mismatch"),
    };

    result.map_err(|error| match error {
        ExactNumericConversionError::NegativeToUnsigned { .. } => "negative-to-unsigned",
        ExactNumericConversionError::OutOfRange { .. } => "out-of-range",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::vm_types::ExactNumericRuntimeValue;

    #[test]
    fn accepts_dynamic_and_matching_exact_values_with_range_checks() {
        assert_eq!(
            validate_exact_numeric_runtime_value(&VMValue::Integer(255), "u8"),
            Ok(())
        );
        assert_eq!(
            validate_exact_numeric_runtime_value(&VMValue::Integer(256), "u8"),
            Err("out-of-range")
        );
        assert_eq!(
            validate_exact_numeric_runtime_value(
                &VMValue::ExactNumeric(ExactNumericRuntimeValue::new("u8", 7)),
                "u8"
            ),
            Ok(())
        );
        assert_eq!(
            validate_exact_numeric_runtime_value(
                &VMValue::ExactNumeric(ExactNumericRuntimeValue::new("i8", 7)),
                "u8"
            ),
            Err("runtime-type-mismatch")
        );
    }
}
