//! Pure original expression-class transfer, shared by source consumers.
//! These are source classes only. They grant no checked operation, range,
//! receiver, Home, result ABI or execution permission. Consumers retain those
//! separate proofs, original operand sites and their complete dependencies.
use crate::mir::resolved_semantics::ResolvedBinaryOperatorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpressionValueClassV1 {
    I64,
    String,
    /// Original tag-preserving runtime carrier, issued only by an explicit
    /// untyped-formal source proof; never inferred from `Opaque`.
    Dynamic,
    Bool,
    Null,
    /// Sealed rows exist but no source class is proven: unannotated
    /// parameters, unproven call results, or non-value bindings.
    Opaque,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExpressionClassIssueV1 {
    Unproven,
    NullableOperandNotString,
}

/// Whether the operator's I64 result exists on the Normal path alone: its value is
/// produced without a Fault exit. Divide/Modulo Fault on a zero divisor and the
/// shifts on an out-of-range count, so a class of I64 for them is not a Normal-only
/// result. Only the operators the checked-result lane emits are listed.
pub(crate) fn binary_operator_cannot_fault_v1(operator: ResolvedBinaryOperatorV1) -> bool {
    use ResolvedBinaryOperatorV1 as Op;
    matches!(operator, Op::Add | Op::Subtract | Op::Multiply)
}

/// R16: the one table from a comparison operator to its operator family; `None` for a
/// non-comparison operator. Every consumer that qualifies a comparison reads it.
pub(crate) fn comparison_operator_family_v1(
    operator: ResolvedBinaryOperatorV1,
) -> Option<crate::mir::dynamic_operator_contract::DynamicOperatorFamilyV1> {
    use crate::mir::dynamic_operator_contract::DynamicOperatorFamilyV1 as Family;
    Some(match operator {
        ResolvedBinaryOperatorV1::Equal => Family::Equal,
        ResolvedBinaryOperatorV1::NotEqual => Family::NotEqual,
        ResolvedBinaryOperatorV1::Less => Family::Less,
        ResolvedBinaryOperatorV1::Greater => Family::Greater,
        ResolvedBinaryOperatorV1::LessEqual => Family::LessEqual,
        ResolvedBinaryOperatorV1::GreaterEqual => Family::GreaterEqual,
        _ => return None,
    })
}

/// R16: the operator-level half of the table below. A comparison or logic operator
/// produces Bool whatever its operands; `binary_expression_class_v1` decides whether
/// the operand classes are admitted. Consumers ask this to name the result kind of a
/// root operator and never list operators themselves.
pub(crate) fn binary_operator_yields_bool_v1(operator: ResolvedBinaryOperatorV1) -> bool {
    use ResolvedBinaryOperatorV1 as Op;
    matches!(
        operator,
        Op::Equal | Op::NotEqual | Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
            | Op::And | Op::Or
    )
}

/// Preserve the source-result issuer's original transfer laws. In particular,
/// String Add's source class alone does not satisfy the stricter checked String
/// envelope; Divide/Modulo's class is not a proof of a normal/non-Fault result.
pub(crate) fn binary_expression_class_v1(
    operator: ResolvedBinaryOperatorV1,
    lhs: ExpressionValueClassV1,
    rhs: ExpressionValueClassV1,
) -> Result<ExpressionValueClassV1, ExpressionClassIssueV1> {
    use ExpressionValueClassV1 as Class;
    use ResolvedBinaryOperatorV1 as Op;
    match operator {
        Op::Add if lhs == Class::String || rhs == Class::String => Ok(Class::String),
        Op::Add if lhs == Class::Dynamic && rhs == Class::I64 => Ok(Class::Dynamic),
        // The Dynamic-left subtraction contract accepts only the exact
        // source I64 RHS; its checked result remains a tagged Dynamic value.
        Op::Subtract if lhs == Class::Dynamic && rhs == Class::I64 => Ok(Class::Dynamic),
        Op::Add | Op::Subtract | Op::Multiply | Op::Divide | Op::Modulo
        | Op::BitAnd | Op::BitOr | Op::BitXor | Op::Shl | Op::Shr
            if lhs == Class::I64 && rhs == Class::I64 => Ok(Class::I64),
        Op::Equal => Ok(Class::Bool),
        Op::NotEqual => {
            if (lhs == Class::Null || rhs == Class::Null)
                && lhs != Class::String && rhs != Class::String
            { return Err(ExpressionClassIssueV1::NullableOperandNotString); }
            Ok(Class::Bool)
        }
        Op::Less | Op::Greater | Op::LessEqual | Op::GreaterEqual
            if (lhs == Class::I64 && rhs == Class::I64)
                || (lhs == Class::String && rhs == Class::String)
                || (lhs == Class::Dynamic && rhs == Class::String)
                || (lhs == Class::String && rhs == Class::Dynamic) => Ok(Class::Bool),
        Op::Less if lhs == Class::Dynamic && rhs == Class::I64 => Ok(Class::Bool),
        Op::Greater if lhs == Class::Dynamic && rhs == Class::I64 => Ok(Class::Bool),
        Op::Less if lhs == Class::I64 && rhs == Class::Dynamic => Ok(Class::Bool),
        Op::And | Op::Or if lhs == Class::Bool && rhs == Class::Bool => Ok(Class::Bool),
        _ => Err(ExpressionClassIssueV1::Unproven),
    }
}
