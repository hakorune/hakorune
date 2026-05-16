use super::target::{
    classify_numeric_kind_for_target, NumericKind, NumericSignedness, NumericTarget,
};

/// Exact MIR-side numeric type metadata.
///
/// This is deliberately distinct from `MirType::Integer`: it records the
/// resolved signedness/width and the source spelling, but it does not change
/// runtime values or existing lowerers by itself.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) struct ExactNumericMirType {
    pub source_name: String,
    pub kind: NumericKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) struct ExactNumericMirSignature {
    pub params: Vec<Option<ExactNumericMirType>>,
    pub return_type: Option<ExactNumericMirType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-05 model; consumed by later verifier/runtime rows.
pub(crate) struct ExactNumericConstValue {
    pub ty: ExactNumericMirType,
    pub value: i128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-05 model; consumed by later verifier/runtime rows.
pub(crate) enum ExactNumericConversionError {
    NegativeToUnsigned {
        source_name: String,
        value: i128,
    },
    OutOfRange {
        source_name: String,
        value: i128,
        min: i128,
        max: i128,
    },
}

#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) fn exact_numeric_mir_type_from_declared_name(
    declared_type_name: Option<&str>,
    target: NumericTarget,
) -> Option<ExactNumericMirType> {
    let source_name = declared_type_name?.to_string();
    let kind = classify_numeric_kind_for_target(&source_name, target)?;
    Some(ExactNumericMirType { source_name, kind })
}

#[allow(dead_code)] // 294x-04 model; consumed by later MIR fact/lowering rows.
pub(crate) fn exact_numeric_mir_signature_from_declared_names<'a>(
    param_type_names: impl IntoIterator<Item = Option<&'a str>>,
    return_type_name: Option<&'a str>,
    target: NumericTarget,
) -> ExactNumericMirSignature {
    ExactNumericMirSignature {
        params: param_type_names
            .into_iter()
            .map(|name| exact_numeric_mir_type_from_declared_name(name, target))
            .collect(),
        return_type: exact_numeric_mir_type_from_declared_name(return_type_name, target),
    }
}

#[allow(dead_code)] // 294x-05 model; consumed by later verifier/runtime rows.
pub(crate) fn exact_numeric_const_from_i128(
    value: i128,
    ty: &ExactNumericMirType,
) -> Result<ExactNumericConstValue, ExactNumericConversionError> {
    let range = ty.kind.value_range();
    if ty.kind.signedness == NumericSignedness::Unsigned && value < 0 {
        return Err(ExactNumericConversionError::NegativeToUnsigned {
            source_name: ty.source_name.clone(),
            value,
        });
    }
    if value < range.min || value > range.max {
        return Err(ExactNumericConversionError::OutOfRange {
            source_name: ty.source_name.clone(),
            value,
            min: range.min,
            max: range.max,
        });
    }
    Ok(ExactNumericConstValue {
        ty: ty.clone(),
        value,
    })
}

#[allow(dead_code)] // 294x-05 model; consumed by later verifier/runtime rows.
pub(crate) fn exact_numeric_value_from_dynamic_integer(
    value: i64,
    ty: &ExactNumericMirType,
) -> Result<ExactNumericConstValue, ExactNumericConversionError> {
    exact_numeric_const_from_i128(i128::from(value), ty)
}

pub(crate) fn exact_numeric_type_requires_dynamic_integer_range_check(
    ty: &ExactNumericMirType,
) -> bool {
    let range = ty.kind.value_range();
    range.min > i128::from(i64::MIN) || range.max < i128::from(i64::MAX)
}

#[allow(dead_code)] // 294x-05 model; consumed by later verifier/runtime rows.
pub(crate) fn exact_numeric_value_from_dynamic_integer_for_declared_name(
    value: i64,
    declared_type_name: Option<&str>,
    target: NumericTarget,
) -> Result<Option<ExactNumericConstValue>, ExactNumericConversionError> {
    let Some(ty) = exact_numeric_mir_type_from_declared_name(declared_type_name, target) else {
        return Ok(None);
    };
    exact_numeric_value_from_dynamic_integer(value, &ty).map(Some)
}
