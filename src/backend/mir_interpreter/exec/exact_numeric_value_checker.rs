use super::super::VMValue;
use crate::runtime::exact_numeric_contract::{validate_dynamic_integer, validate_exact_integer};

/// Shared subordinate value/type/range checker for exact-numeric contracts.
/// Boundary owners remain responsible for timing, carrier validation, and tags.
pub(super) fn validate_exact_numeric_runtime_value(
    value: &VMValue,
    declared_type_name: &str,
) -> Result<(), &'static str> {
    match value {
        VMValue::Integer(value) => validate_dynamic_integer(*value, declared_type_name),
        VMValue::ExactNumeric(value) => {
            validate_exact_integer(value.value, value.source_name, declared_type_name)
        }
        _ => return Err("runtime-type-mismatch"),
    }
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
