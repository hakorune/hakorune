//! Test-only syntax facts copied at the resolver/source-view boundary.
//!
//! The product contains no AST or source lifetime.  It deliberately records
//! syntax as written; loop-family policy owns whether those facts are useful.

use super::shape_source_lease_v2::GenericShapeSourceLeaseV2;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::resolved_semantics::source_projection::{
    project_source_body_node_v1, ProjectedSourceNodeV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionSyntaxViewV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericSyntaxFactRejectV3 {
    SourceKindMismatch,
    MissingProjectedNode,
    ConditionNotBinary,
    StepNotPlainAssignment,
    StepValueNotBinary,
    StepTargetNotBinding,
    BindingMismatch,
    OperandUnresolved,
    PostLoopOrderUnproven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericUnsupportedOperandV3 {
    NonIntegerLiteral,
    UpvarOrCapture,
    CallOrEffect,
    OtherExpression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericOperandSyntaxFactV3 {
    Binding(BindingRefV1),
    IntegerLiteral(i64),
    Unsupported(GenericUnsupportedOperandV3),
}

#[derive(Debug, PartialEq)]
pub(crate) struct GenericConditionSyntaxFactV3 {
    site: SourceExprSiteV1,
    operator: BinaryOperator,
    lhs: BindingRefV1,
    rhs: GenericOperandSyntaxFactV3,
}

impl GenericConditionSyntaxFactV3 {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn operator(&self) -> &BinaryOperator {
        &self.operator
    }

    pub(crate) const fn lhs(&self) -> BindingRefV1 {
        self.lhs
    }

    pub(crate) const fn rhs(&self) -> GenericOperandSyntaxFactV3 {
        self.rhs
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct GenericStepSyntaxFactV3 {
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    value_site: SourceExprSiteV1,
    operator: BinaryOperator,
    lhs: BindingRefV1,
    rhs: GenericOperandSyntaxFactV3,
}

impl GenericStepSyntaxFactV3 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) fn operator(&self) -> &BinaryOperator {
        &self.operator
    }

    pub(crate) const fn lhs(&self) -> BindingRefV1 {
        self.lhs
    }

    pub(crate) const fn rhs(&self) -> GenericOperandSyntaxFactV3 {
        self.rhs
    }
}

/// Move-only syntax projection over the immutable V2 role handoff.
#[derive(Debug, PartialEq)]
pub(crate) struct GenericConditionStepSyntaxFactsV3 {
    carrier: GenericShapeSourceLeaseV2,
    condition: GenericConditionSyntaxFactV3,
    step: GenericStepSyntaxFactV3,
    _seal: GenericConditionStepSyntaxFactsSealV3,
}

#[derive(Debug, PartialEq)]
struct GenericConditionStepSyntaxFactsSealV3;

impl GenericConditionStepSyntaxFactsV3 {
    pub(crate) fn carrier(&self) -> &GenericShapeSourceLeaseV2 {
        &self.carrier
    }

    pub(crate) fn condition(&self) -> &GenericConditionSyntaxFactV3 {
        &self.condition
    }

    pub(crate) fn step(&self) -> &GenericStepSyntaxFactV3 {
        &self.step
    }
}

pub(crate) fn issue_condition_step_syntax_facts_v3<'source>(
    function: &VerifiedResolvedFunctionV1,
    source: FunctionSyntaxViewV1<'source>,
    carrier: GenericShapeSourceLeaseV2,
) -> Result<GenericConditionStepSyntaxFactsV3, GenericSyntaxFactRejectV3> {
    if source.source_kind() != function.source_kind() {
        return Err(GenericSyntaxFactRejectV3::SourceKindMismatch);
    }
    ensure_post_loop_order(
        carrier.carrier().proof().root_site(),
        carrier.carrier().proof().post_loop_read_site(),
    )?;

    let condition_site = carrier.condition().condition_site().clone();
    let condition = project_node(source, &condition_site)?;
    let (condition_operator, condition_lhs_site, condition_rhs_site) = match condition {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => (operator.clone(), left, right),
        _ => return Err(GenericSyntaxFactRejectV3::ConditionNotBinary),
    };
    let condition_lhs = operand_fact(
        function,
        &carrier.condition().induction().site(),
        condition_lhs_site,
    )?;
    let expected_binding = carrier.condition().induction().binding();
    let GenericOperandSyntaxFactV3::Binding(condition_lhs) = condition_lhs else {
        return Err(GenericSyntaxFactRejectV3::OperandUnresolved);
    };
    if condition_lhs != expected_binding {
        return Err(GenericSyntaxFactRejectV3::BindingMismatch);
    }
    let condition_rhs = operand_fact(
        function,
        carrier.condition().bound_site(),
        condition_rhs_site,
    )?;

    let step_statement_site = carrier.step().statement_site().clone();
    let step_statement = project_node(source, &step_statement_site)?;
    let (step_target, step_value) = match step_statement {
        ASTNode::Assignment { target, value, .. } => (target, value),
        _ => return Err(GenericSyntaxFactRejectV3::StepNotPlainAssignment),
    };
    let step_value_site = carrier.step().value_site().clone();
    let (step_operator, step_lhs_site, step_rhs_site) = match step_value.as_ref() {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => (operator.clone(), left, right),
        _ => return Err(GenericSyntaxFactRejectV3::StepValueNotBinary),
    };
    if !matches!(step_target.as_ref(), ASTNode::Variable { .. }) {
        return Err(GenericSyntaxFactRejectV3::StepTargetNotBinding);
    }
    let target_binding = match function.assignment_target(carrier.step().target_site()) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => *binding,
        _ => return Err(GenericSyntaxFactRejectV3::StepTargetNotBinding),
    };
    let step_lhs = operand_fact(
        function,
        &carrier.step().operand_read().site(),
        step_lhs_site,
    )?;
    let GenericOperandSyntaxFactV3::Binding(step_lhs) = step_lhs else {
        return Err(GenericSyntaxFactRejectV3::OperandUnresolved);
    };
    if step_lhs != carrier.step().operand_read().binding() || step_lhs != target_binding {
        return Err(GenericSyntaxFactRejectV3::BindingMismatch);
    }
    let step_rhs = operand_fact(function, carrier.step().delta_site(), step_rhs_site)?;
    let target_site = carrier.step().target_site().clone();

    Ok(GenericConditionStepSyntaxFactsV3 {
        carrier,
        condition: GenericConditionSyntaxFactV3 {
            site: condition_site,
            operator: condition_operator,
            lhs: condition_lhs,
            rhs: condition_rhs,
        },
        step: GenericStepSyntaxFactV3 {
            statement_site: step_statement_site,
            target_site,
            value_site: step_value_site,
            operator: step_operator,
            lhs: step_lhs,
            rhs: step_rhs,
        },
        _seal: GenericConditionStepSyntaxFactsSealV3,
    })
}

fn project_node<'source>(
    source: FunctionSyntaxViewV1<'source>,
    site: &impl SourceSiteSegments,
) -> Result<&'source ASTNode, GenericSyntaxFactRejectV3> {
    match project_source_body_node_v1(source.body(), site.node()) {
        Some(ProjectedSourceNodeV1::Node(node)) => Ok(node),
        _ => Err(GenericSyntaxFactRejectV3::MissingProjectedNode),
    }
}

trait SourceSiteSegments {
    fn node(&self) -> &crate::mir::resolved_semantics::SourceNodeSiteV1;
}

impl SourceSiteSegments for SourceExprSiteV1 {
    fn node(&self) -> &crate::mir::resolved_semantics::SourceNodeSiteV1 {
        self.node()
    }
}

impl SourceSiteSegments for SourceStmtSiteV1 {
    fn node(&self) -> &crate::mir::resolved_semantics::SourceNodeSiteV1 {
        self.node()
    }
}

fn operand_fact(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
    node: &ASTNode,
) -> Result<GenericOperandSyntaxFactV3, GenericSyntaxFactRejectV3> {
    match node {
        ASTNode::Variable { .. } => match function.variable_ref(site) {
            Some(ResolvedLexicalRefV1::Local(binding)) => {
                Ok(GenericOperandSyntaxFactV3::Binding(binding))
            }
            Some(ResolvedLexicalRefV1::Upvar(_)) => Ok(GenericOperandSyntaxFactV3::Unsupported(
                GenericUnsupportedOperandV3::UpvarOrCapture,
            )),
            None => Err(GenericSyntaxFactRejectV3::OperandUnresolved),
        },
        ASTNode::Literal { value, .. } => match value {
            LiteralValue::Integer(value) => Ok(GenericOperandSyntaxFactV3::IntegerLiteral(*value)),
            _ => Ok(GenericOperandSyntaxFactV3::Unsupported(
                GenericUnsupportedOperandV3::NonIntegerLiteral,
            )),
        },
        ASTNode::Call { .. }
        | ASTNode::FunctionCall { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FromCall { .. }
        | ASTNode::New { .. } => Ok(GenericOperandSyntaxFactV3::Unsupported(
            GenericUnsupportedOperandV3::CallOrEffect,
        )),
        _ => Ok(GenericOperandSyntaxFactV3::Unsupported(
            GenericUnsupportedOperandV3::OtherExpression,
        )),
    }
}

fn ensure_post_loop_order(
    root: &SourceStmtSiteV1,
    read: &SourceExprSiteV1,
) -> Result<(), GenericSyntaxFactRejectV3> {
    let Some(SourcePathSegmentV1::Body(root_index)) = root.node().segments().first() else {
        return Err(GenericSyntaxFactRejectV3::PostLoopOrderUnproven);
    };
    let Some(SourcePathSegmentV1::Body(read_index)) = read.node().segments().first() else {
        return Err(GenericSyntaxFactRejectV3::PostLoopOrderUnproven);
    };
    (read_index > root_index)
        .then_some(())
        .ok_or(GenericSyntaxFactRejectV3::PostLoopOrderUnproven)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ASTNode;
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::{
        carrier_proof_witness::issue_carrier_proof_v1,
        shape_source_lease_v2::issue_generic_shape_source_lease_v2, tests as lease_tests,
    };
    use crate::parser::NyashParser;

    fn function_ast(source: &str) -> ASTNode {
        let root = NyashParser::parse_from_string(source).expect("syntax fixture parses");
        let ASTNode::Program { statements, .. } = root else {
            panic!("syntax fixture must be a Program")
        };
        statements
            .into_iter()
            .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
            .expect("syntax fixture function")
    }

    fn replace_first_assignment_with_grouped(node: &mut ASTNode) -> bool {
        match node {
            ASTNode::FunctionDeclaration { body, .. } => {
                body.iter_mut().any(replace_first_assignment_with_grouped)
            }
            ASTNode::Loop { body, .. } => {
                for statement in body {
                    if replace_first_assignment_with_grouped(statement) {
                        return true;
                    }
                }
                false
            }
            ASTNode::Assignment {
                target,
                value,
                span,
            } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return false;
                };
                *node = ASTNode::GroupedAssignmentExpr {
                    lhs: name.clone(),
                    rhs: value.clone(),
                    span: *span,
                };
                true
            }
            _ => false,
        }
    }

    fn issue(source: &str) -> GenericConditionStepSyntaxFactsV3 {
        let syntax_ast = function_ast(source);
        let syntax = FunctionSyntaxViewV1::from_ast(&syntax_ast).expect("function view");
        let unit = lease_tests::unit(source);
        let (input, root) = lease_tests::input_and_root(&unit);
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        let v2 = issue_generic_shape_source_lease_v2(function, handoff).expect("v2 roles");
        issue_condition_step_syntax_facts_v3(function, syntax, v2).expect("syntax facts")
    }

    const CANONICAL: &str = r#"
function generic_both(i, j) {
    loop(i < 3) {
        loop(j < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    const NON_CANONICAL: &str = r#"
function generic_noncanonical(i, j) {
    loop(i <= 7) {
        loop(j <= 7) {
            j = j * 2
        }
        i = i * 2
    }
    return j
}
"#;

    const SYMBOLIC_OPERANDS: &str = r#"
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

    #[test]
    fn copies_as_written_operator_and_integer_facts_without_policy() {
        let facts = issue(CANONICAL);
        assert_eq!(facts.condition().operator(), &BinaryOperator::Less);
        assert_eq!(
            facts.condition().rhs(),
            GenericOperandSyntaxFactV3::IntegerLiteral(3)
        );
        assert_eq!(facts.step().operator(), &BinaryOperator::Add);
        assert_eq!(
            facts.step().rhs(),
            GenericOperandSyntaxFactV3::IntegerLiteral(1)
        );
        assert_eq!(facts.condition().lhs(), facts.step().lhs());
    }

    #[test]
    fn copies_noncanonical_operators_for_later_policy() {
        let facts = issue(NON_CANONICAL);
        assert_eq!(facts.condition().operator(), &BinaryOperator::LessEqual);
        assert_eq!(facts.step().operator(), &BinaryOperator::Multiply);
    }

    #[test]
    fn preserves_symbolic_operands_as_binding_facts_for_later_policy() {
        let facts = issue(SYMBOLIC_OPERANDS);
        assert!(matches!(
            facts.condition().rhs(),
            GenericOperandSyntaxFactV3::Binding(_)
        ));
        assert!(matches!(
            facts.step().rhs(),
            GenericOperandSyntaxFactV3::Binding(_)
        ));
    }

    #[test]
    fn rejects_grouped_assignment_step_before_publication() {
        let mut syntax_ast = function_ast(CANONICAL);
        assert!(replace_first_assignment_with_grouped(&mut syntax_ast));
        let syntax = FunctionSyntaxViewV1::from_ast(&syntax_ast).expect("function view");
        let unit = lease_tests::unit(CANONICAL);
        let (input, root) = lease_tests::input_and_root(&unit);
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        let v2 = issue_generic_shape_source_lease_v2(function, handoff).expect("v2 roles");
        assert_eq!(
            issue_condition_step_syntax_facts_v3(function, syntax, v2),
            Err(GenericSyntaxFactRejectV3::StepNotPlainAssignment)
        );
    }

    #[test]
    fn rejects_unproven_pre_loop_read_order() {
        let root = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(1),
        ]));
        let read = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
        ]));
        assert_eq!(
            ensure_post_loop_order(&root, &read),
            Err(GenericSyntaxFactRejectV3::PostLoopOrderUnproven)
        );
    }

    #[test]
    fn output_is_source_lifetime_free_and_retains_v2() {
        let facts = issue(CANONICAL);
        assert_eq!(
            facts.carrier().condition().condition_site(),
            facts.condition().site()
        );
    }
}
