//! Test-only co-seal of syntax facts and explicit parameter type rows.
//!
//! This receipt transports resolver/source provenance without classifying a
//! numeric type or choosing a loop route.  The later numeric and progression
//! policy owners remain the only places that may interpret those facts.

use super::shape_syntax_facts_v3::{GenericConditionStepSyntaxFactsV3, GenericOperandSyntaxFactV3};
use crate::mir::resolved_semantics::explicit_parameter_type_map::{
    ExplicitParameterTypeRowV1, VerifiedExplicitParameterTypeMapV1,
};
use crate::mir::resolved_semantics::BindingRefV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericNumericSourceReceiptRejectV1 {
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    BindingOutsideParameterMap,
}

/// Move-only, AST-free transport for one exact source unit.
#[derive(Debug, PartialEq)]
pub(crate) struct VerifiedGenericNumericSourceReceiptV1 {
    facts: GenericConditionStepSyntaxFactsV3,
    parameter_types: VerifiedExplicitParameterTypeMapV1,
    _seal: GenericNumericSourceReceiptSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericNumericSourceReceiptSealV1;

impl VerifiedGenericNumericSourceReceiptV1 {
    pub(crate) fn facts(&self) -> &GenericConditionStepSyntaxFactsV3 {
        &self.facts
    }

    pub(crate) fn parameter_types(&self) -> &VerifiedExplicitParameterTypeMapV1 {
        &self.parameter_types
    }
}

#[cfg(test)]
pub(crate) fn test_function_ast(source: &str) -> crate::ast::ASTNode {
    use crate::ast::ASTNode;
    use crate::parser::NyashParser;

    let root = NyashParser::parse_from_string(source).expect("receipt fixture parses");
    let ASTNode::Program { statements, .. } = root else {
        panic!("receipt fixture must be a Program")
    };
    statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("receipt fixture function")
}

#[cfg(test)]
pub(crate) fn test_receipt_from_ast(
    source: &str,
    syntax_ast: crate::ast::ASTNode,
) -> VerifiedGenericNumericSourceReceiptV1 {
    use crate::mir::resolved_semantics::explicit_parameter_type_map::{
        issue_explicit_parameter_type_map_v1, VerifiedExplicitParameterSourceReceiptV1,
    };
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::{
        carrier_proof_witness::issue_carrier_proof_v1,
        shape_source_lease_v2::issue_generic_shape_source_lease_v2,
        shape_syntax_facts_v3::issue_condition_step_syntax_facts_v3, tests as lease_tests,
    };
    use crate::mir::resolved_semantics::FunctionSyntaxViewV1;

    let syntax = FunctionSyntaxViewV1::from_ast(&syntax_ast).expect("function view");
    let unit = lease_tests::unit(source);
    let input = unit.root_function_input().expect("root input");
    let body = input.source().root_body().expect("function body");
    let root = input.source().body_stmt(&body, 0).expect("outer loop");
    let function = input.function();
    let lease = lease_tests::positive_lease(input, &root);
    let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
    let v2 = issue_generic_shape_source_lease_v2(function, handoff).expect("shape lease");
    let facts = issue_condition_step_syntax_facts_v3(function, syntax, v2)
        .expect("condition/step syntax facts");
    let source_receipt = VerifiedExplicitParameterSourceReceiptV1::from_source_unit(&unit)
        .expect("parameter source receipt");
    let parameter_types =
        issue_explicit_parameter_type_map_v1(source_receipt).expect("parameter type map");
    issue_generic_numeric_source_receipt_v1(facts, parameter_types).expect("numeric receipt")
}

pub(crate) fn issue_generic_numeric_source_receipt_v1(
    facts: GenericConditionStepSyntaxFactsV3,
    parameter_types: VerifiedExplicitParameterTypeMapV1,
) -> Result<VerifiedGenericNumericSourceReceiptV1, GenericNumericSourceReceiptRejectV1> {
    let proof = facts.carrier().carrier().proof();
    if parameter_types.owner() != proof.owner() {
        return Err(GenericNumericSourceReceiptRejectV1::OwnerMismatch);
    }
    if parameter_types.function_origin() != proof.function_origin() {
        return Err(GenericNumericSourceReceiptRejectV1::OriginMismatch);
    }
    if parameter_types.source_kind() != proof.source_kind() {
        return Err(GenericNumericSourceReceiptRejectV1::SourceKindMismatch);
    }
    if !shape_bindings_are_explicit_parameters(&facts, parameter_types.rows()) {
        return Err(GenericNumericSourceReceiptRejectV1::BindingOutsideParameterMap);
    }
    Ok(VerifiedGenericNumericSourceReceiptV1 {
        facts,
        parameter_types,
        _seal: GenericNumericSourceReceiptSealV1,
    })
}

fn shape_bindings_are_explicit_parameters(
    facts: &GenericConditionStepSyntaxFactsV3,
    rows: &[ExplicitParameterTypeRowV1],
) -> bool {
    let bindings = [
        Some(facts.condition().lhs()),
        binding_operand(facts.condition().rhs()),
        Some(facts.step().lhs()),
        binding_operand(facts.step().rhs()),
    ];
    bindings
        .into_iter()
        .flatten()
        .all(|binding| rows.iter().any(|row| row.binding() == binding))
}

fn binding_operand(operand: &GenericOperandSyntaxFactV3) -> Option<BindingRefV1> {
    match operand {
        GenericOperandSyntaxFactV3::Binding(binding) => Some(*binding),
        GenericOperandSyntaxFactV3::IntegerLiteral(_)
        | GenericOperandSyntaxFactV3::TypedIntegerLiteral { .. }
        | GenericOperandSyntaxFactV3::Unsupported(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
    use crate::mir::resolved_semantics::explicit_parameter_type_map::{
        issue_explicit_parameter_type_map_v1, VerifiedExplicitParameterSourceReceiptV1,
    };
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::{
        carrier_proof_witness::issue_carrier_proof_v1,
        shape_source_lease_v2::issue_generic_shape_source_lease_v2,
        shape_syntax_facts_v3::issue_condition_step_syntax_facts_v3, tests as lease_tests,
    };
    use crate::mir::resolved_semantics::FunctionSyntaxViewV1;
    use crate::parser::NyashParser;

    const TYPED_SOURCE: &str = r#"
function generic_typed(i: i64, j) {
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

    fn function_ast(source: &str) -> ASTNode {
        let root = NyashParser::parse_from_string(source).expect("receipt fixture parses");
        let ASTNode::Program { statements, .. } = root else {
            panic!("receipt fixture must be a Program")
        };
        statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("receipt fixture function")
    }

    fn typed_literal_ast(source: &str) -> ASTNode {
        let mut ast = function_ast(source);
        let mut remaining = 4;
        replace_integer_literals_with_typed(&mut ast, &mut remaining);
        assert_eq!(remaining, 0);
        ast
    }

    fn replace_integer_literals_with_typed(node: &mut ASTNode, remaining: &mut usize) {
        match node {
            ASTNode::FunctionDeclaration { body, .. } => {
                for statement in body {
                    replace_integer_literals_with_typed(statement, remaining);
                }
            }
            ASTNode::Loop {
                condition, body, ..
            } => {
                replace_integer_literals_with_typed(condition, remaining);
                for statement in body {
                    replace_integer_literals_with_typed(statement, remaining);
                }
            }
            ASTNode::Assignment { target, value, .. } => {
                replace_integer_literals_with_typed(target, remaining);
                replace_integer_literals_with_typed(value, remaining);
            }
            ASTNode::BinaryOp { left, right, .. } => {
                replace_integer_literals_with_typed(left, remaining);
                replace_integer_literals_with_typed(right, remaining);
            }
            ASTNode::Return { value, .. } => {
                if let Some(value) = value {
                    replace_integer_literals_with_typed(value, remaining);
                }
            }
            ASTNode::Literal { value, .. } => {
                if *remaining == 0 {
                    return;
                }
                let LiteralValue::Integer(actual) = value else {
                    return;
                };
                *value = LiteralValue::TypedInteger {
                    value: *actual,
                    declared_type_name: "i64".to_owned(),
                };
                *remaining -= 1;
            }
            _ => {}
        }
    }

    fn products(
        source: &str,
        syntax_ast: ASTNode,
    ) -> (
        GenericConditionStepSyntaxFactsV3,
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
        let parameter_types =
            issue_explicit_parameter_type_map_v1(source_receipt).expect("parameter type map");
        (facts, parameter_types)
    }

    fn facts_only(source: &str) -> GenericConditionStepSyntaxFactsV3 {
        products(source, function_ast(source)).0
    }

    fn map_only(source: &str) -> VerifiedExplicitParameterTypeMapV1 {
        products(source, function_ast(source)).1
    }

    #[test]
    fn co_seals_same_owner_facts_and_parameter_map() {
        let (facts, map) = products(TYPED_SOURCE, typed_literal_ast(TYPED_SOURCE));
        let receipt = issue_generic_numeric_source_receipt_v1(facts, map).expect("receipt");
        assert_eq!(
            receipt.facts().condition().operator(),
            &BinaryOperator::Less
        );
        assert_eq!(receipt.parameter_types().rows().len(), 2);
    }

    #[test]
    fn retains_typed_literal_spelling_without_classification() {
        let (facts, map) = products(TYPED_SOURCE, typed_literal_ast(TYPED_SOURCE));
        let receipt = issue_generic_numeric_source_receipt_v1(facts, map).expect("receipt");
        assert!(matches!(
            receipt.facts().step().rhs(),
            GenericOperandSyntaxFactV3::TypedIntegerLiteral {
                value: 1,
                declared_type_name,
            } if declared_type_name.as_ref() == "i64"
        ));
        assert_eq!(
            receipt.parameter_types().rows()[1].declared_type_name(),
            None
        );
    }

    #[test]
    fn carries_untyped_symbolic_operands_for_later_policy() {
        let (facts, map) = products(SYMBOLIC_SOURCE, function_ast(SYMBOLIC_SOURCE));
        let receipt = issue_generic_numeric_source_receipt_v1(facts, map).expect("receipt");
        assert!(matches!(
            receipt.facts().condition().rhs(),
            GenericOperandSyntaxFactV3::Binding(_)
        ));
        assert!(receipt
            .parameter_types()
            .rows()
            .iter()
            .all(|row| { row.declared_type_name().is_none() }));
    }

    #[test]
    fn rejects_foreign_owner_before_binding_coverage() {
        let facts = facts_only(lease_tests::SOURCE);
        let map = map_only(lease_tests::SOURCE);
        let result = issue_generic_numeric_source_receipt_v1(facts, map);
        assert!(matches!(
            result,
            Err(GenericNumericSourceReceiptRejectV1::OwnerMismatch)
                | Err(GenericNumericSourceReceiptRejectV1::OriginMismatch)
        ));
    }

    #[test]
    fn rejects_shape_binding_outside_parameter_map() {
        let (facts, map) = products(lease_tests::SOURCE, function_ast(lease_tests::SOURCE));
        let first = &map.rows()[0];
        let truncated = VerifiedExplicitParameterTypeMapV1::from_test_parts(
            map.owner(),
            map.function_origin(),
            map.source_kind(),
            vec![ExplicitParameterTypeRowV1::from_test_parts(
                first.index(),
                first.binding(),
                first.declared_type_name(),
            )]
            .into_boxed_slice(),
        );
        assert_eq!(
            issue_generic_numeric_source_receipt_v1(facts, truncated),
            Err(GenericNumericSourceReceiptRejectV1::BindingOutsideParameterMap)
        );
    }

    fn receipt_after_source_drop() -> VerifiedGenericNumericSourceReceiptV1 {
        let (facts, map) = products(TYPED_SOURCE, typed_literal_ast(TYPED_SOURCE));
        issue_generic_numeric_source_receipt_v1(facts, map).expect("receipt")
    }

    #[test]
    fn receipt_is_usable_after_source_unit_and_ast_drop() {
        let receipt = receipt_after_source_drop();
        assert_eq!(
            receipt.parameter_types().rows()[0].declared_type_name(),
            Some("i64")
        );
        assert!(matches!(
            receipt.facts().condition().rhs(),
            GenericOperandSyntaxFactV3::TypedIntegerLiteral { .. }
        ));
    }
}
