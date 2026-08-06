//! Test-only numeric progression policy over one sealed substrate projection.
//!
//! This layer consumes the target-specific substrate by value exactly once. It
//! classifies only syntax-level comparison/update progression; type, width,
//! range, and overflow remain owned by the substrate projection.

use super::numeric_substrate_projection_v5::{
    GenericNumericOperandProjectionV1, GenericNumericOperandRoleV1, GenericNumericOperandValueV1,
    VerifiedGenericNumericSubstrateProjectionV1,
};
use crate::ast::BinaryOperator;
use crate::mir::numeric_substrate::ExactNumericConstValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericPolicyUnresolvedV1 {
    UnsupportedComparison,
    UnsupportedUpdate,
    SymbolicDelta,
    NonProgressingStep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericPolicyRejectV1 {
    MissingOperandRole,
    DuplicateOperandRole,
    ForeignOperandBinding,
    ConditionStepBindingMismatch,
    DirectionMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericComparisonDirectionV1 {
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GenericNumericComparisonPolicyV1 {
    direction: GenericNumericComparisonDirectionV1,
    strict: bool,
}

impl GenericNumericComparisonPolicyV1 {
    pub(crate) const fn direction(&self) -> GenericNumericComparisonDirectionV1 {
        self.direction
    }

    pub(crate) const fn strict(&self) -> bool {
        self.strict
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericProgressionOpV1 {
    Add,
    Subtract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericProgressionDirectionV1 {
    Increasing,
    Decreasing,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GenericNumericProgressionPolicyV1 {
    op: GenericNumericProgressionOpV1,
    direction: GenericNumericProgressionDirectionV1,
    delta: ExactNumericConstValue,
}

impl GenericNumericProgressionPolicyV1 {
    pub(crate) const fn op(&self) -> GenericNumericProgressionOpV1 {
        self.op
    }

    pub(crate) const fn direction(&self) -> GenericNumericProgressionDirectionV1 {
        self.direction
    }

    pub(crate) fn delta(&self) -> &ExactNumericConstValue {
        &self.delta
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct VerifiedGenericNumericPolicyV1 {
    substrate: VerifiedGenericNumericSubstrateProjectionV1,
    comparison: GenericNumericComparisonPolicyV1,
    progression: GenericNumericProgressionPolicyV1,
    _seal: GenericNumericPolicySealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericNumericPolicySealV1;

impl VerifiedGenericNumericPolicyV1 {
    pub(crate) fn substrate(&self) -> &VerifiedGenericNumericSubstrateProjectionV1 {
        &self.substrate
    }

    pub(crate) const fn comparison(&self) -> GenericNumericComparisonPolicyV1 {
        self.comparison
    }

    pub(crate) fn progression(&self) -> &GenericNumericProgressionPolicyV1 {
        &self.progression
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum GenericNumericPolicyOutcomeV1 {
    Ready(VerifiedGenericNumericPolicyV1),
    Unresolved {
        substrate: VerifiedGenericNumericSubstrateProjectionV1,
        reason: GenericNumericPolicyUnresolvedV1,
    },
    Rejected {
        substrate: VerifiedGenericNumericSubstrateProjectionV1,
        reason: GenericNumericPolicyRejectV1,
    },
}

pub(crate) fn issue_generic_numeric_policy_v1(
    substrate: VerifiedGenericNumericSubstrateProjectionV1,
) -> GenericNumericPolicyOutcomeV1 {
    let facts = substrate.receipt().facts();
    let condition_step_binding = facts.condition().lhs() == facts.step().lhs();
    if !condition_step_binding {
        return GenericNumericPolicyOutcomeV1::Rejected {
            substrate,
            reason: GenericNumericPolicyRejectV1::ConditionStepBindingMismatch,
        };
    }

    let condition = match comparison_policy(facts.condition().operator()) {
        Some(policy) => policy,
        None => {
            return GenericNumericPolicyOutcomeV1::Unresolved {
                substrate,
                reason: GenericNumericPolicyUnresolvedV1::UnsupportedComparison,
            }
        }
    };
    let op = match facts.step().operator() {
        BinaryOperator::Add => GenericNumericProgressionOpV1::Add,
        BinaryOperator::Subtract => GenericNumericProgressionOpV1::Subtract,
        _ => {
            return GenericNumericPolicyOutcomeV1::Unresolved {
                substrate,
                reason: GenericNumericPolicyUnresolvedV1::UnsupportedUpdate,
            }
        }
    };

    let condition_rhs = match operand_by_role(&substrate, GenericNumericOperandRoleV1::ConditionRhs)
    {
        Ok(value) => value.value().clone(),
        Err(reason) => return policy_reject(substrate, reason),
    };
    let step_rhs = match operand_by_role(&substrate, GenericNumericOperandRoleV1::StepRhs) {
        Ok(value) => value.value().clone(),
        Err(reason) => return policy_reject(substrate, reason),
    };
    if let Err(reason) = validate_binding(&substrate, &condition_rhs) {
        return policy_reject(substrate, reason);
    }
    if let Err(reason) = validate_binding(&substrate, &step_rhs) {
        return policy_reject(substrate, reason);
    }

    let GenericNumericOperandValueV1::TypedInteger(delta) = step_rhs else {
        return GenericNumericPolicyOutcomeV1::Unresolved {
            substrate,
            reason: GenericNumericPolicyUnresolvedV1::SymbolicDelta,
        };
    };
    if delta.value == 0 {
        return GenericNumericPolicyOutcomeV1::Unresolved {
            substrate,
            reason: GenericNumericPolicyUnresolvedV1::NonProgressingStep,
        };
    }

    let progression_direction = match op {
        GenericNumericProgressionOpV1::Add if delta.value > 0 => {
            GenericNumericProgressionDirectionV1::Increasing
        }
        GenericNumericProgressionOpV1::Subtract if delta.value > 0 => {
            GenericNumericProgressionDirectionV1::Decreasing
        }
        _ => return policy_reject(substrate, GenericNumericPolicyRejectV1::DirectionMismatch),
    };
    let expected_direction = match condition.direction {
        GenericNumericComparisonDirectionV1::Less => {
            GenericNumericProgressionDirectionV1::Increasing
        }
        GenericNumericComparisonDirectionV1::Greater => {
            GenericNumericProgressionDirectionV1::Decreasing
        }
    };
    if progression_direction != expected_direction {
        return policy_reject(substrate, GenericNumericPolicyRejectV1::DirectionMismatch);
    }

    GenericNumericPolicyOutcomeV1::Ready(VerifiedGenericNumericPolicyV1 {
        substrate,
        comparison: condition,
        progression: GenericNumericProgressionPolicyV1 {
            op,
            direction: progression_direction,
            delta: delta.clone(),
        },
        _seal: GenericNumericPolicySealV1,
    })
}

fn comparison_policy(operator: &BinaryOperator) -> Option<GenericNumericComparisonPolicyV1> {
    match operator {
        BinaryOperator::Less => Some(GenericNumericComparisonPolicyV1 {
            direction: GenericNumericComparisonDirectionV1::Less,
            strict: true,
        }),
        BinaryOperator::LessEqual => Some(GenericNumericComparisonPolicyV1 {
            direction: GenericNumericComparisonDirectionV1::Less,
            strict: false,
        }),
        BinaryOperator::Greater => Some(GenericNumericComparisonPolicyV1 {
            direction: GenericNumericComparisonDirectionV1::Greater,
            strict: true,
        }),
        BinaryOperator::GreaterEqual => Some(GenericNumericComparisonPolicyV1 {
            direction: GenericNumericComparisonDirectionV1::Greater,
            strict: false,
        }),
        _ => None,
    }
}

fn operand_by_role(
    substrate: &VerifiedGenericNumericSubstrateProjectionV1,
    role: GenericNumericOperandRoleV1,
) -> Result<&GenericNumericOperandProjectionV1, GenericNumericPolicyRejectV1> {
    let mut matching = substrate
        .operands()
        .iter()
        .filter(|operand| operand.role() == role);
    let Some(first) = matching.next() else {
        return Err(GenericNumericPolicyRejectV1::MissingOperandRole);
    };
    if matching.next().is_some() {
        return Err(GenericNumericPolicyRejectV1::DuplicateOperandRole);
    }
    Ok(first)
}

fn validate_binding(
    substrate: &VerifiedGenericNumericSubstrateProjectionV1,
    value: &GenericNumericOperandValueV1,
) -> Result<(), GenericNumericPolicyRejectV1> {
    let GenericNumericOperandValueV1::Binding(binding) = value else {
        return Ok(());
    };
    if substrate
        .parameter_types()
        .iter()
        .any(|parameter| parameter.binding() == *binding)
    {
        Ok(())
    } else {
        Err(GenericNumericPolicyRejectV1::ForeignOperandBinding)
    }
}

fn policy_reject(
    substrate: VerifiedGenericNumericSubstrateProjectionV1,
    reason: GenericNumericPolicyRejectV1,
) -> GenericNumericPolicyOutcomeV1 {
    GenericNumericPolicyOutcomeV1::Rejected { substrate, reason }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue};
    use crate::mir::numeric_substrate::NumericTarget;
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::{
        numeric_source_receipt_v4::{test_function_ast, test_receipt_from_ast},
        numeric_substrate_projection_v5::issue_generic_numeric_substrate_projection_v1,
    };

    const INCREASING_SOURCE: &str = r#"
function generic(i: i64, bound: i64) {
    loop(i < bound) {
        loop(i < bound) {
            i = i + 1
        }
    }
    return i
}
"#;
    const DECREASING_SOURCE: &str = r#"
function generic(i: i64, bound: i64) {
    loop(i > bound) {
        loop(i > bound) {
            i = i - 1
        }
    }
    return i
}
"#;
    const SYMBOLIC_SOURCE: &str = r#"
function generic(i: i64, bound: i64, delta: i64) {
    loop(i < bound) {
        loop(i < bound) {
            i = i + delta
        }
    }
    return i
}
"#;
    const EQUAL_SOURCE: &str = r#"
function generic(i: i64, bound: i64) {
    loop(i == bound) {
        loop(i == bound) {
            i = i + 1
        }
    }
    return i
}
"#;
    const MULTIPLY_SOURCE: &str = r#"
function generic(i: i64, bound: i64) {
    loop(i < bound) {
        loop(i < bound) {
            i = i * 2
        }
    }
    return i
}
"#;
    const ZERO_SOURCE: &str = r#"
function generic(i: i64, bound: i64) {
    loop(i < bound) {
        loop(i < bound) {
            i = i + 0
        }
    }
    return i
}
"#;
    const MISMATCH_SOURCE: &str = r#"
function generic(i: i64, bound: i64) {
    loop(i < bound) {
        loop(i < bound) {
            i = i - 1
        }
    }
    return i
}
"#;

    fn typed_ast(source: &str) -> ASTNode {
        let mut ast = test_function_ast(source);
        type_all_integer_literals(&mut ast);
        ast
    }

    fn type_all_integer_literals(node: &mut ASTNode) {
        match node {
            ASTNode::FunctionDeclaration { body, .. } => {
                for statement in body {
                    type_all_integer_literals(statement);
                }
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                type_all_integer_literals(condition);
                for statement in body {
                    type_all_integer_literals(statement);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                type_all_integer_literals(target);
                type_all_integer_literals(value);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                type_all_integer_literals(left);
                type_all_integer_literals(right);
            }
            ASTNode::Return { value, .. } => {
                if let Some(value) = value {
                    type_all_integer_literals(value);
                }
            }
            ASTNode::Literal { value, .. } => {
                if let LiteralValue::Integer(actual) = value {
                    let actual = *actual;
                    *value = LiteralValue::TypedInteger {
                        value: actual,
                        declared_type_name: "i64".to_owned(),
                    };
                }
            }
            _ => {}
        }
    }

    fn ready(source: &str) -> VerifiedGenericNumericSubstrateProjectionV1 {
        let receipt = test_receipt_from_ast(source, typed_ast(source));
        match issue_generic_numeric_substrate_projection_v1(receipt, NumericTarget::host()) {
            super::super::numeric_substrate_projection_v5::GenericNumericSubstrateOutcomeV1::Ready(
                projection,
            ) => projection,
            other => panic!("fixture must produce a substrate projection: {other:?}"),
        }
    }

    #[test]
    fn classifies_increasing_strict_progression() {
        let outcome = issue_generic_numeric_policy_v1(ready(INCREASING_SOURCE));
        let GenericNumericPolicyOutcomeV1::Ready(policy) = outcome else {
            panic!("expected ready policy")
        };
        assert_eq!(
            policy.comparison(),
            GenericNumericComparisonPolicyV1 {
                direction: GenericNumericComparisonDirectionV1::Less,
                strict: true,
            }
        );
        assert_eq!(
            policy.progression().direction(),
            GenericNumericProgressionDirectionV1::Increasing
        );
        assert_eq!(policy.progression().delta().value, 1);
    }

    #[test]
    fn preserves_decreasing_and_non_strict_shapes() {
        let outcome = issue_generic_numeric_policy_v1(ready(DECREASING_SOURCE));
        let GenericNumericPolicyOutcomeV1::Ready(policy) = outcome else {
            panic!("expected ready decreasing policy")
        };
        assert_eq!(
            policy.progression().direction(),
            GenericNumericProgressionDirectionV1::Decreasing
        );

        let source = INCREASING_SOURCE.replace("i < bound", "i <= bound");
        let outcome = issue_generic_numeric_policy_v1(ready(&source));
        let GenericNumericPolicyOutcomeV1::Ready(policy) = outcome else {
            panic!("expected ready non-strict policy")
        };
        assert!(!policy.comparison().strict());
    }

    #[test]
    fn symbolic_delta_is_unresolved() {
        let receipt = test_receipt_from_ast(SYMBOLIC_SOURCE, test_function_ast(SYMBOLIC_SOURCE));
        let substrate = match issue_generic_numeric_substrate_projection_v1(
            receipt,
            NumericTarget::host(),
        ) {
            super::super::numeric_substrate_projection_v5::GenericNumericSubstrateOutcomeV1::Ready(
                projection,
            ) => projection,
            other => panic!("fixture must produce a substrate projection: {other:?}"),
        };
        assert!(matches!(
            issue_generic_numeric_policy_v1(substrate),
            GenericNumericPolicyOutcomeV1::Unresolved {
                reason: GenericNumericPolicyUnresolvedV1::SymbolicDelta,
                ..
            }
        ));
    }

    #[test]
    fn unsupported_operator_shapes_are_unresolved() {
        let equality = issue_generic_numeric_policy_v1(ready(EQUAL_SOURCE));
        assert!(matches!(
            equality,
            GenericNumericPolicyOutcomeV1::Unresolved {
                reason: GenericNumericPolicyUnresolvedV1::UnsupportedComparison,
                ..
            }
        ));

        let multiply = issue_generic_numeric_policy_v1(ready(MULTIPLY_SOURCE));
        assert!(matches!(
            multiply,
            GenericNumericPolicyOutcomeV1::Unresolved {
                reason: GenericNumericPolicyUnresolvedV1::UnsupportedUpdate,
                ..
            }
        ));
    }

    #[test]
    fn zero_delta_and_direction_mismatch_are_not_ready() {
        let zero = issue_generic_numeric_policy_v1(ready(ZERO_SOURCE));
        assert!(matches!(
            zero,
            GenericNumericPolicyOutcomeV1::Unresolved {
                reason: GenericNumericPolicyUnresolvedV1::NonProgressingStep,
                ..
            }
        ));

        let mismatch = issue_generic_numeric_policy_v1(ready(MISMATCH_SOURCE));
        assert!(matches!(
            mismatch,
            GenericNumericPolicyOutcomeV1::Rejected {
                reason: GenericNumericPolicyRejectV1::DirectionMismatch,
                ..
            }
        ));
    }

    #[test]
    fn forged_duplicate_and_foreign_roles_reject_before_ready() {
        let projection = ready(INCREASING_SOURCE);
        let foreign = ready(DECREASING_SOURCE);
        let foreign_binding = match foreign.operands()[0].value() {
            GenericNumericOperandValueV1::Binding(binding) => *binding,
            GenericNumericOperandValueV1::TypedInteger(_) => panic!("bound must be a binding"),
        };
        let duplicate_value = projection.operands()[0].value().clone();
        let step_value = projection.operands()[1].value().clone();
        let (receipt, target, parameters, _) = projection.into_test_parts();
        let duplicate = VerifiedGenericNumericSubstrateProjectionV1::from_test_parts(
            receipt,
            target,
            parameters,
            vec![
                GenericNumericOperandProjectionV1::from_test_parts(
                    GenericNumericOperandRoleV1::ConditionRhs,
                    duplicate_value,
                ),
                GenericNumericOperandProjectionV1::from_test_parts(
                    GenericNumericOperandRoleV1::ConditionRhs,
                    GenericNumericOperandValueV1::Binding(foreign_binding),
                ),
            ]
            .into_boxed_slice(),
        );
        assert!(matches!(
            issue_generic_numeric_policy_v1(duplicate),
            GenericNumericPolicyOutcomeV1::Rejected {
                reason: GenericNumericPolicyRejectV1::DuplicateOperandRole,
                ..
            }
        ));

        let projection = ready(INCREASING_SOURCE);
        let (receipt, target, parameters, _) = projection.into_test_parts();
        let foreign_role = VerifiedGenericNumericSubstrateProjectionV1::from_test_parts(
            receipt,
            target,
            parameters,
            vec![
                GenericNumericOperandProjectionV1::from_test_parts(
                    GenericNumericOperandRoleV1::ConditionRhs,
                    GenericNumericOperandValueV1::Binding(foreign_binding),
                ),
                GenericNumericOperandProjectionV1::from_test_parts(
                    GenericNumericOperandRoleV1::StepRhs,
                    step_value,
                ),
            ]
            .into_boxed_slice(),
        );
        assert!(matches!(
            issue_generic_numeric_policy_v1(foreign_role),
            GenericNumericPolicyOutcomeV1::Rejected {
                reason: GenericNumericPolicyRejectV1::ForeignOperandBinding,
                ..
            }
        ));
    }

    #[test]
    fn ready_policy_retains_substrate_provenance_after_fixture_drop() {
        let policy = match issue_generic_numeric_policy_v1(ready(INCREASING_SOURCE)) {
            GenericNumericPolicyOutcomeV1::Ready(policy) => policy,
            other => panic!("expected ready policy: {other:?}"),
        };
        assert_eq!(policy.substrate().parameter_types().len(), 2);
        assert_eq!(policy.substrate().operands().len(), 2);
    }
}
