//! Test-only target-specific numeric projection.
//!
//! The source receipt remains target-agnostic.  This layer is the only staged
//! projection that receives an explicit `NumericTarget`; progression policy is
//! intentionally a later, separate consumer.

use super::numeric_source_receipt_v4::VerifiedGenericNumericSourceReceiptV1;
use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_mir_type_from_declared_name,
    ExactNumericConstValue, ExactNumericMirType, NumericTarget,
};
use crate::mir::resolved_semantics::BindingRefV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericSubstrateUnresolvedV1 {
    MissingParameterType { index: u32 },
    UnknownParameterType { index: u32 },
    UntypedIntegerLiteral,
    UnknownLiteralType,
    UnsupportedOperand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericSubstrateRejectV1 {
    DuplicateParameterIndex,
    DuplicateParameterBinding,
    MissingParameterBinding,
    TypedLiteralOutOfRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericOperandRoleV1 {
    ConditionRhs,
    StepRhs,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum GenericNumericOperandValueV1 {
    Binding(BindingRefV1),
    TypedInteger(ExactNumericConstValue),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GenericNumericOperandProjectionV1 {
    role: GenericNumericOperandRoleV1,
    value: GenericNumericOperandValueV1,
}

impl GenericNumericOperandProjectionV1 {
    pub(crate) const fn role(&self) -> GenericNumericOperandRoleV1 {
        self.role
    }

    pub(crate) fn value(&self) -> &GenericNumericOperandValueV1 {
        &self.value
    }

    #[cfg(test)]
    pub(crate) fn from_test_parts(
        role: GenericNumericOperandRoleV1,
        value: GenericNumericOperandValueV1,
    ) -> Self {
        Self { role, value }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct GenericNumericParameterProjectionV1 {
    index: u32,
    binding: BindingRefV1,
    numeric_type: ExactNumericMirType,
}

impl GenericNumericParameterProjectionV1 {
    pub(crate) const fn index(&self) -> u32 {
        self.index
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn numeric_type(&self) -> &ExactNumericMirType {
        &self.numeric_type
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct VerifiedGenericNumericSubstrateProjectionV1 {
    receipt: VerifiedGenericNumericSourceReceiptV1,
    target: NumericTarget,
    parameter_types: Box<[GenericNumericParameterProjectionV1]>,
    operands: Box<[GenericNumericOperandProjectionV1]>,
    _seal: GenericNumericSubstrateProjectionSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericNumericSubstrateProjectionSealV1;

impl VerifiedGenericNumericSubstrateProjectionV1 {
    pub(crate) fn receipt(&self) -> &VerifiedGenericNumericSourceReceiptV1 {
        &self.receipt
    }

    pub(crate) const fn target(&self) -> NumericTarget {
        self.target
    }

    pub(crate) fn parameter_types(&self) -> &[GenericNumericParameterProjectionV1] {
        &self.parameter_types
    }

    pub(crate) fn operands(&self) -> &[GenericNumericOperandProjectionV1] {
        &self.operands
    }

    #[cfg(test)]
    pub(crate) fn into_test_parts(
        self,
    ) -> (
        VerifiedGenericNumericSourceReceiptV1,
        NumericTarget,
        Box<[GenericNumericParameterProjectionV1]>,
        Box<[GenericNumericOperandProjectionV1]>,
    ) {
        (
            self.receipt,
            self.target,
            self.parameter_types,
            self.operands,
        )
    }

    #[cfg(test)]
    pub(crate) fn from_test_parts(
        receipt: VerifiedGenericNumericSourceReceiptV1,
        target: NumericTarget,
        parameter_types: Box<[GenericNumericParameterProjectionV1]>,
        operands: Box<[GenericNumericOperandProjectionV1]>,
    ) -> Self {
        Self {
            receipt,
            target,
            parameter_types,
            operands,
            _seal: GenericNumericSubstrateProjectionSealV1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum GenericNumericSubstrateOutcomeV1 {
    Ready(VerifiedGenericNumericSubstrateProjectionV1),
    Unresolved {
        receipt: VerifiedGenericNumericSourceReceiptV1,
        reason: GenericNumericSubstrateUnresolvedV1,
    },
    Rejected {
        receipt: VerifiedGenericNumericSourceReceiptV1,
        reason: GenericNumericSubstrateRejectV1,
    },
}

pub(crate) fn issue_generic_numeric_substrate_projection_v1(
    receipt: VerifiedGenericNumericSourceReceiptV1,
    target: NumericTarget,
) -> GenericNumericSubstrateOutcomeV1 {
    let parameter_types = match project_parameters(&receipt, target) {
        Ok(types) => types,
        Err(SubstrateProjectionFailure::Unresolved(reason)) => {
            return GenericNumericSubstrateOutcomeV1::Unresolved { receipt, reason }
        }
        Err(SubstrateProjectionFailure::Rejected(reason)) => {
            return GenericNumericSubstrateOutcomeV1::Rejected { receipt, reason }
        }
    };
    let mut operands = Vec::new();
    if let Err(failure) = collect_operands(&receipt, &parameter_types, target, &mut operands) {
        return match failure {
            SubstrateProjectionFailure::Unresolved(reason) => {
                GenericNumericSubstrateOutcomeV1::Unresolved { receipt, reason }
            }
            SubstrateProjectionFailure::Rejected(reason) => {
                GenericNumericSubstrateOutcomeV1::Rejected { receipt, reason }
            }
        };
    }

    GenericNumericSubstrateOutcomeV1::Ready(VerifiedGenericNumericSubstrateProjectionV1 {
        receipt,
        target,
        parameter_types: parameter_types.into_boxed_slice(),
        operands: operands.into_boxed_slice(),
        _seal: GenericNumericSubstrateProjectionSealV1,
    })
}

enum SubstrateProjectionFailure {
    Unresolved(GenericNumericSubstrateUnresolvedV1),
    Rejected(GenericNumericSubstrateRejectV1),
}

fn project_parameters(
    receipt: &VerifiedGenericNumericSourceReceiptV1,
    target: NumericTarget,
) -> Result<Vec<GenericNumericParameterProjectionV1>, SubstrateProjectionFailure> {
    let rows = receipt.parameter_types().rows();
    let mut projected = Vec::with_capacity(rows.len());
    for (expected_index, row) in rows.iter().enumerate() {
        let expected_index = u32::try_from(expected_index).map_err(|_| {
            SubstrateProjectionFailure::Rejected(
                GenericNumericSubstrateRejectV1::DuplicateParameterIndex,
            )
        })?;
        if row.index() != expected_index {
            return Err(SubstrateProjectionFailure::Rejected(
                GenericNumericSubstrateRejectV1::DuplicateParameterIndex,
            ));
        }
        if projected
            .iter()
            .any(|item: &GenericNumericParameterProjectionV1| item.binding() == row.binding())
        {
            return Err(SubstrateProjectionFailure::Rejected(
                GenericNumericSubstrateRejectV1::DuplicateParameterBinding,
            ));
        }
        let Some(name) = row.declared_type_name() else {
            return Err(SubstrateProjectionFailure::Unresolved(
                GenericNumericSubstrateUnresolvedV1::MissingParameterType {
                    index: expected_index,
                },
            ));
        };
        let Some(numeric_type) = exact_numeric_mir_type_from_declared_name(Some(name), target)
        else {
            return Err(SubstrateProjectionFailure::Unresolved(
                GenericNumericSubstrateUnresolvedV1::UnknownParameterType {
                    index: expected_index,
                },
            ));
        };
        projected.push(GenericNumericParameterProjectionV1 {
            index: expected_index,
            binding: row.binding(),
            numeric_type,
        });
    }
    Ok(projected)
}

fn collect_operands(
    receipt: &VerifiedGenericNumericSourceReceiptV1,
    parameter_types: &[GenericNumericParameterProjectionV1],
    target: NumericTarget,
    operands: &mut Vec<GenericNumericOperandProjectionV1>,
) -> Result<(), SubstrateProjectionFailure> {
    let facts = receipt.facts();
    for (role, operand) in [
        (
            GenericNumericOperandRoleV1::ConditionRhs,
            facts.condition().rhs(),
        ),
        (GenericNumericOperandRoleV1::StepRhs, facts.step().rhs()),
    ] {
        match operand {
            super::shape_syntax_facts_v3::GenericOperandSyntaxFactV3::Binding(binding) => {
                if !parameter_types.iter().any(|row| row.binding() == *binding) {
                    return Err(SubstrateProjectionFailure::Rejected(
                        GenericNumericSubstrateRejectV1::MissingParameterBinding,
                    ));
                }
                operands.push(GenericNumericOperandProjectionV1 {
                    role,
                    value: GenericNumericOperandValueV1::Binding(*binding),
                });
            }
            super::shape_syntax_facts_v3::GenericOperandSyntaxFactV3::IntegerLiteral(_) => {
                return Err(SubstrateProjectionFailure::Unresolved(
                    GenericNumericSubstrateUnresolvedV1::UntypedIntegerLiteral,
                ));
            }
            super::shape_syntax_facts_v3::GenericOperandSyntaxFactV3::TypedIntegerLiteral {
                value,
                declared_type_name,
            } => {
                let Some(numeric_type) =
                    exact_numeric_mir_type_from_declared_name(Some(declared_type_name), target)
                else {
                    return Err(SubstrateProjectionFailure::Unresolved(
                        GenericNumericSubstrateUnresolvedV1::UnknownLiteralType,
                    ));
                };
                let literal = exact_numeric_const_from_i128(i128::from(*value), &numeric_type)
                    .map_err(|_| {
                        SubstrateProjectionFailure::Rejected(
                            GenericNumericSubstrateRejectV1::TypedLiteralOutOfRange,
                        )
                    })?;
                operands.push(GenericNumericOperandProjectionV1 {
                    role,
                    value: GenericNumericOperandValueV1::TypedInteger(literal),
                });
            }
            super::shape_syntax_facts_v3::GenericOperandSyntaxFactV3::Unsupported(_) => {
                return Err(SubstrateProjectionFailure::Unresolved(
                    GenericNumericSubstrateUnresolvedV1::UnsupportedOperand,
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue};
    use crate::mir::resolved_semantics::explicit_parameter_type_map::{
        issue_explicit_parameter_type_map_v1, ExplicitParameterTypeRowV1,
        VerifiedExplicitParameterSourceReceiptV1, VerifiedExplicitParameterTypeMapV1,
    };
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::{
        carrier_proof_witness::issue_carrier_proof_v1,
        numeric_source_receipt_v4::issue_generic_numeric_source_receipt_v1,
        shape_source_lease_v2::issue_generic_shape_source_lease_v2,
        shape_syntax_facts_v3::issue_condition_step_syntax_facts_v3, tests as lease_tests,
    };
    use crate::mir::resolved_semantics::FunctionSyntaxViewV1;
    use crate::parser::NyashParser;

    const READY_SOURCE: &str = r#"
function generic_ready(i: i64, j: i64) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    const SYMBOLIC_SOURCE: &str = r#"
function generic_symbolic(i, j, bound, delta) {
    loop(i < bound) {
        loop(j < bound) {
            j = j + delta
        }
        i = i + delta
    }
    return j
}
"#;

    const UNKNOWN_SOURCE: &str = r#"
function generic_unknown(i: Mystery, j: Mystery) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    const U8_SOURCE: &str = r#"
function generic_u8(i: u8, j: u8) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    fn function_ast(source: &str) -> ASTNode {
        let root = NyashParser::parse_from_string(source).expect("substrate fixture parses");
        let ASTNode::Program { statements, .. } = root else {
            panic!("substrate fixture must be a Program")
        };
        statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("substrate fixture function")
    }

    fn typed_literals_ast(source: &str, declared_type_name: &str, first_value: i64) -> ASTNode {
        let mut ast = function_ast(source);
        let mut remaining = 4;
        replace_integer_literals(&mut ast, &mut remaining, declared_type_name, first_value);
        assert_eq!(remaining, 0);
        ast
    }

    fn replace_integer_literals(
        node: &mut ASTNode,
        remaining: &mut usize,
        declared_type_name: &str,
        first_value: i64,
    ) {
        match node {
            ASTNode::FunctionDeclaration { body, .. } => {
                for statement in body {
                    replace_integer_literals(statement, remaining, declared_type_name, first_value);
                }
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                replace_integer_literals(condition, remaining, declared_type_name, first_value);
                for statement in body {
                    replace_integer_literals(statement, remaining, declared_type_name, first_value);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                replace_integer_literals(target, remaining, declared_type_name, first_value);
                replace_integer_literals(value, remaining, declared_type_name, first_value);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                replace_integer_literals(left, remaining, declared_type_name, first_value);
                replace_integer_literals(right, remaining, declared_type_name, first_value);
            }
            ASTNode::Return { value, .. } => {
                if let Some(value) = value {
                    replace_integer_literals(value, remaining, declared_type_name, first_value);
                }
            }
            ASTNode::Literal { value, .. } => {
                if *remaining == 0 {
                    return;
                }
                let LiteralValue::Integer(_) = value else {
                    return;
                };
                *value = LiteralValue::TypedInteger {
                    value: first_value,
                    declared_type_name: declared_type_name.to_owned(),
                };
                *remaining -= 1;
            }
            _ => {}
        }
    }

    fn receipt_parts(
        source: &str,
        syntax_ast: ASTNode,
    ) -> (
        super::super::shape_syntax_facts_v3::GenericConditionStepSyntaxFactsV3,
        VerifiedExplicitParameterTypeMapV1,
    ) {
        let syntax = FunctionSyntaxViewV1::from_ast(&syntax_ast).expect("function view");
        let unit = lease_tests::unit(source);
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("function body");
        let root = input.source().body_stmt(&body, 0).expect("outer loop");
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        let v2 = issue_generic_shape_source_lease_v2(function, handoff).expect("shape lease");
        let facts =
            issue_condition_step_syntax_facts_v3(function, syntax, v2).expect("syntax facts");
        let source_receipt = VerifiedExplicitParameterSourceReceiptV1::from_source_unit(&unit)
            .expect("parameter source receipt");
        let map = issue_explicit_parameter_type_map_v1(source_receipt).expect("parameter map");
        (facts, map)
    }

    fn receipt(source: &str, syntax_ast: ASTNode) -> VerifiedGenericNumericSourceReceiptV1 {
        let (facts, map) = receipt_parts(source, syntax_ast);
        issue_generic_numeric_source_receipt_v1(facts, map).expect("numeric source receipt")
    }

    #[test]
    fn typed_i64_source_is_ready_for_explicit_target() {
        let outcome = issue_generic_numeric_substrate_projection_v1(
            receipt(READY_SOURCE, typed_literals_ast(READY_SOURCE, "i64", 3)),
            NumericTarget::host(),
        );
        let GenericNumericSubstrateOutcomeV1::Ready(projection) = outcome else {
            panic!("typed i64 fixture must be ready")
        };
        assert_eq!(projection.parameter_types().len(), 2);
        assert_eq!(projection.operands().len(), 2);
        assert_eq!(
            projection.operands()[0].role(),
            GenericNumericOperandRoleV1::ConditionRhs
        );
        assert_eq!(
            projection.operands()[1].role(),
            GenericNumericOperandRoleV1::StepRhs
        );
        assert_eq!(projection.target(), NumericTarget::host());
    }

    #[test]
    fn untyped_symbolic_parameter_is_unresolved_without_inference() {
        let outcome = issue_generic_numeric_substrate_projection_v1(
            receipt(SYMBOLIC_SOURCE, function_ast(SYMBOLIC_SOURCE)),
            NumericTarget::host(),
        );
        assert!(matches!(
            outcome,
            GenericNumericSubstrateOutcomeV1::Unresolved {
                reason: GenericNumericSubstrateUnresolvedV1::MissingParameterType { .. },
                ..
            }
        ));
    }

    #[test]
    fn unknown_type_name_is_unresolved_not_host_fallback() {
        let outcome = issue_generic_numeric_substrate_projection_v1(
            receipt(UNKNOWN_SOURCE, typed_literals_ast(UNKNOWN_SOURCE, "i64", 3)),
            NumericTarget::host(),
        );
        assert!(matches!(
            outcome,
            GenericNumericSubstrateOutcomeV1::Unresolved {
                reason: GenericNumericSubstrateUnresolvedV1::UnknownParameterType { .. },
                ..
            }
        ));
    }

    #[test]
    fn typed_u8_literal_out_of_range_is_rejected() {
        let outcome = issue_generic_numeric_substrate_projection_v1(
            receipt(U8_SOURCE, typed_literals_ast(U8_SOURCE, "u8", 300)),
            NumericTarget::host(),
        );
        assert!(matches!(
            outcome,
            GenericNumericSubstrateOutcomeV1::Rejected {
                reason: GenericNumericSubstrateRejectV1::TypedLiteralOutOfRange,
                ..
            }
        ));
    }

    #[test]
    fn duplicate_map_row_is_rejected_before_projection() {
        let (facts, map) = receipt_parts(READY_SOURCE, typed_literals_ast(READY_SOURCE, "i64", 3));
        let first = &map.rows()[0];
        let second = &map.rows()[1];
        let forged = VerifiedExplicitParameterTypeMapV1::from_test_parts(
            map.owner(),
            map.function_origin(),
            map.source_kind(),
            vec![
                ExplicitParameterTypeRowV1::from_test_parts(
                    first.index(),
                    first.binding(),
                    first.declared_type_name(),
                ),
                ExplicitParameterTypeRowV1::from_test_parts(
                    second.index(),
                    second.binding(),
                    second.declared_type_name(),
                ),
                ExplicitParameterTypeRowV1::from_test_parts(
                    2,
                    first.binding(),
                    first.declared_type_name(),
                ),
            ]
            .into_boxed_slice(),
        );
        let receipt = issue_generic_numeric_source_receipt_v1(facts, forged)
            .expect("co-seal accepts coverage before duplicate audit");
        assert!(matches!(
            issue_generic_numeric_substrate_projection_v1(receipt, NumericTarget::host()),
            GenericNumericSubstrateOutcomeV1::Rejected {
                reason: GenericNumericSubstrateRejectV1::DuplicateParameterBinding,
                ..
            }
        ));
    }

    #[test]
    fn ready_projection_keeps_receipt_after_source_drop() {
        let outcome = issue_generic_numeric_substrate_projection_v1(
            receipt(READY_SOURCE, typed_literals_ast(READY_SOURCE, "i64", 3)),
            NumericTarget::host(),
        );
        let GenericNumericSubstrateOutcomeV1::Ready(projection) = outcome else {
            panic!("typed i64 fixture must be ready")
        };
        assert_eq!(projection.receipt().parameter_types().rows().len(), 2);
    }
}
