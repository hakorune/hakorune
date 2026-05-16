use super::exact_values::ExactNumericConstValue;
use super::target::NumericSignedness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // 294x-07 policy; consumed by later VM/backend exact-op rows.
pub(crate) enum ExactNumericArithmeticOp {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-07 policy; consumed by later VM/backend exact-op rows.
pub(crate) enum ExactNumericArithmeticError {
    TypeMismatch {
        left_source_name: String,
        right_source_name: String,
    },
    ResultOutOfRange {
        source_name: String,
        op: ExactNumericArithmeticOp,
        lhs: i128,
        rhs: i128,
        result: Option<i128>,
        min: i128,
        max: i128,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // 294x-08 policy; consumed by later VM/backend exact-op rows.
pub(crate) enum ExactNumericCompareOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-08 policy; consumed by later VM/backend exact-op rows.
pub(crate) enum ExactNumericCompareError {
    TypeMismatch {
        left_source_name: String,
        right_source_name: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // 294x-08 policy; consumed by later VM/backend exact-op rows.
pub(crate) enum ExactNumericShiftError {
    SignedLogicalShift {
        source_name: String,
    },
    ShiftCountOutOfRange {
        source_name: String,
        shift: u32,
        width_bits: u32,
    },
}

#[allow(dead_code)] // 294x-07 policy; consumed by later VM/backend exact-op rows.
pub(crate) fn exact_numeric_checked_arithmetic(
    lhs: &ExactNumericConstValue,
    rhs: &ExactNumericConstValue,
    op: ExactNumericArithmeticOp,
) -> Result<ExactNumericConstValue, ExactNumericArithmeticError> {
    if lhs.ty != rhs.ty {
        return Err(ExactNumericArithmeticError::TypeMismatch {
            left_source_name: lhs.ty.source_name.clone(),
            right_source_name: rhs.ty.source_name.clone(),
        });
    }

    let result = match op {
        ExactNumericArithmeticOp::Add => lhs.value.checked_add(rhs.value),
        ExactNumericArithmeticOp::Sub => lhs.value.checked_sub(rhs.value),
        ExactNumericArithmeticOp::Mul => lhs.value.checked_mul(rhs.value),
    };
    let range = lhs.ty.kind.value_range();
    let Some(value) = result else {
        return Err(ExactNumericArithmeticError::ResultOutOfRange {
            source_name: lhs.ty.source_name.clone(),
            op,
            lhs: lhs.value,
            rhs: rhs.value,
            result: None,
            min: range.min,
            max: range.max,
        });
    };
    if value < range.min || value > range.max {
        return Err(ExactNumericArithmeticError::ResultOutOfRange {
            source_name: lhs.ty.source_name.clone(),
            op,
            lhs: lhs.value,
            rhs: rhs.value,
            result: Some(value),
            min: range.min,
            max: range.max,
        });
    }

    Ok(ExactNumericConstValue {
        ty: lhs.ty.clone(),
        value,
    })
}

#[allow(dead_code)] // 294x-08 policy; consumed by later VM/backend exact-op rows.
pub(crate) fn exact_numeric_compare(
    lhs: &ExactNumericConstValue,
    rhs: &ExactNumericConstValue,
    op: ExactNumericCompareOp,
) -> Result<bool, ExactNumericCompareError> {
    if lhs.ty != rhs.ty {
        return Err(ExactNumericCompareError::TypeMismatch {
            left_source_name: lhs.ty.source_name.clone(),
            right_source_name: rhs.ty.source_name.clone(),
        });
    }

    Ok(match op {
        ExactNumericCompareOp::Eq => lhs.value == rhs.value,
        ExactNumericCompareOp::Ne => lhs.value != rhs.value,
        ExactNumericCompareOp::Lt => lhs.value < rhs.value,
        ExactNumericCompareOp::Le => lhs.value <= rhs.value,
        ExactNumericCompareOp::Gt => lhs.value > rhs.value,
        ExactNumericCompareOp::Ge => lhs.value >= rhs.value,
    })
}

#[allow(dead_code)] // 294x-08 policy; consumed by later VM/backend exact-op rows.
pub(crate) fn exact_numeric_logical_shr(
    value: &ExactNumericConstValue,
    shift: u32,
) -> Result<ExactNumericConstValue, ExactNumericShiftError> {
    if value.ty.kind.signedness != NumericSignedness::Unsigned {
        return Err(ExactNumericShiftError::SignedLogicalShift {
            source_name: value.ty.source_name.clone(),
        });
    }

    let width_bits = value.ty.kind.width.bits();
    if shift >= width_bits {
        return Err(ExactNumericShiftError::ShiftCountOutOfRange {
            source_name: value.ty.source_name.clone(),
            shift,
            width_bits,
        });
    }

    Ok(ExactNumericConstValue {
        ty: value.ty.clone(),
        value: value.value >> shift,
    })
}
