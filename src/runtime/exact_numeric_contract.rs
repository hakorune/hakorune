use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_mir_type_from_declared_name,
    exact_numeric_value_from_dynamic_integer, ExactNumericConversionError, NumericTarget,
};

pub(crate) fn validate_dynamic_integer(
    value: i64,
    declared_type: &str,
) -> Result<(), &'static str> {
    let exact_type = resolve(declared_type)?;
    exact_numeric_value_from_dynamic_integer(value, &exact_type)
        .map(|_| ())
        .map_err(classify_conversion_error)
}

pub(crate) fn validate_exact_integer(
    value: i128,
    source_type: &str,
    declared_type: &str,
) -> Result<(), &'static str> {
    let exact_type = resolve(declared_type)?;
    if source_type != exact_type.source_name {
        return Err("runtime-type-mismatch");
    }
    exact_numeric_const_from_i128(value, &exact_type)
        .map(|_| ())
        .map_err(classify_conversion_error)
}

fn resolve(
    declared_type: &str,
) -> Result<crate::mir::numeric_substrate::ExactNumericMirType, &'static str> {
    exact_numeric_mir_type_from_declared_name(Some(declared_type), NumericTarget::host())
        .ok_or("unknown-exact-type")
}

fn classify_conversion_error(error: ExactNumericConversionError) -> &'static str {
    match error {
        ExactNumericConversionError::NegativeToUnsigned { .. } => "negative-to-unsigned",
        ExactNumericConversionError::OutOfRange { .. } => "out-of-range",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_dynamic_and_exact_integer_ranges() {
        assert_eq!(validate_dynamic_integer(255, "u8"), Ok(()));
        assert_eq!(validate_dynamic_integer(256, "u8"), Err("out-of-range"));
        assert_eq!(
            validate_dynamic_integer(-1, "u8"),
            Err("negative-to-unsigned")
        );
        assert_eq!(validate_exact_integer(7, "u8", "u8"), Ok(()));
        assert_eq!(
            validate_exact_integer(7, "i8", "u8"),
            Err("runtime-type-mismatch")
        );
    }
}
