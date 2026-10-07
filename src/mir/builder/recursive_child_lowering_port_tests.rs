use super::raw_structured_child_scope::RawStructuredChildScopePortV1;
use super::recursive_child_lowering_port::{
    RecursiveChildLoweringPortV1, ScriptDirectStaticClaimIngressV1,
};
use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

struct DefaultPort;

impl RecursiveChildLoweringPortV1 for DefaultPort {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        unreachable!("test port does not lower bodies")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        unreachable!("test port does not lower statements")
    }

    fn lower_expression(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        unreachable!("test port does not lower expressions")
    }
}

#[test]
fn default_claim_ingress_is_non_consuming_and_unavailable() {
    let mut port = DefaultPort;
    let first = RecursiveChildLoweringPortV1::script_direct_static_claim_ingress_v1(
        &mut port, "Helper", "value", 0,
    )
    .expect("default ingress is infallible");
    let second = RecursiveChildLoweringPortV1::script_direct_static_claim_ingress_v1(
        &mut port, "Helper", "value", 0,
    )
    .expect("default ingress remains infallible");
    assert_eq!(first, ScriptDirectStaticClaimIngressV1::Unavailable);
    assert_eq!(second, ScriptDirectStaticClaimIngressV1::Unavailable);
}

#[test]
fn structured_scope_delegates_the_non_consuming_hook() {
    let mut child = DefaultPort;
    let mut scope = RawStructuredChildScopePortV1::new(&mut child, vec![], vec![]);
    let result = RecursiveChildLoweringPortV1::script_direct_static_claim_ingress_v1(
        &mut scope, "Helper", "value", 0,
    )
    .expect("structured delegation is infallible for the default child");
    assert_eq!(result, ScriptDirectStaticClaimIngressV1::Unavailable);
}

struct BinaryCompletionPort {
    children: usize,
    completions: Vec<Option<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>>,
    reject_completion: bool,
    reject_child: bool,
}

impl BinaryCompletionPort {
    fn new() -> Self {
        Self {
            children: 0,
            completions: Vec::new(),
            reject_completion: false,
            reject_child: false,
        }
    }
}

impl RecursiveChildLoweringPortV1 for BinaryCompletionPort {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(&mut self, _: &mut MirBuilder, _: Vec<ASTNode>) -> Result<ValueId, String> {
        unreachable!("binary test does not lower bodies")
    }

    fn lower_statement(&mut self, _: &mut MirBuilder, _: ASTNode) -> Result<ValueId, String> {
        unreachable!("binary test does not lower statements")
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        self.children += 1;
        if self.reject_child {
            return Err("original-child-refusal".into());
        }
        let ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(value),
            ..
        } = input
        else {
            unreachable!("binary children are exact integers")
        };
        super::emission::constant::emit_integer(builder, value)
    }

    fn complete_ordinary_binary_expression_v1(
        &mut self,
        completed: &super::ops::CompletedOrdinaryBinaryV1,
    ) -> Result<(), String> {
        self.completions
            .push(completed.comparison_original().cloned());
        if self.reject_completion {
            return Err("original-completion-refusal".into());
        }
        Ok(())
    }
}

fn binary_completion_input(
    operator: crate::ast::BinaryOperator,
) -> super::ops::RawLegacyBinaryInputV1 {
    let literal = |value| ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(value),
        span: crate::ast::Span::unknown(),
    };
    super::ops::RawLegacyBinaryInputV1::new(literal(7), operator, literal(3))
}

#[test]
fn raw_binary_completion_crosses_nested_structured_scopes_exactly_once() {
    use super::raw_structured_child_scope::PreparedRawChildSourceV1;
    use crate::mir::{CompareOp, MirInstruction};
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_nested_binary_completion/0".into());
    let mut child = BinaryCompletionPort::new();
    let sources = || {
        vec![
            PreparedRawChildSourceV1::Preserve,
            PreparedRawChildSourceV1::Preserve,
        ]
    };
    let mut outer = RawStructuredChildScopePortV1::new(&mut child, sources(), vec![]);
    let mut inner = RawStructuredChildScopePortV1::new(&mut outer, sources(), vec![]);
    let value = super::ops::drive_ordinary_binary_expression_v1(
        &mut builder,
        &mut inner,
        &binary_completion_input(crate::ast::BinaryOperator::LessEqual),
    )
    .unwrap();
    inner.complete_exact_demands_v1().unwrap();
    outer.complete_exact_demands_v1().unwrap();
    assert_eq!(child.children, 2);
    assert_eq!(child.completions.len(), 1);
    let (block, original) = child.completions[0].as_ref().unwrap();
    assert!(
        matches!(original, MirInstruction::Compare { dst, op: CompareOp::Le, .. } if *dst == value)
    );
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert_eq!(
        function.blocks[block]
            .instructions
            .iter()
            .filter(|inst| *inst == original)
            .count(),
        1
    );
    assert_eq!(
        function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter(|inst| matches!(inst, MirInstruction::Compare { .. }))
            .count(),
        1
    );
}

#[test]
fn raw_binary_completion_refusal_is_preserved_without_retry() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_binary_completion_refusal/0".into());
    let mut child = BinaryCompletionPort::new();
    child.reject_completion = true;
    let error = super::ops::drive_ordinary_binary_expression_v1(
        &mut builder,
        &mut child,
        &binary_completion_input(crate::ast::BinaryOperator::Greater),
    )
    .unwrap_err();
    assert_eq!(error, "original-completion-refusal");
    assert_eq!(child.children, 2);
    assert_eq!(child.completions.len(), 1);
}

#[test]
fn raw_binary_child_refusal_never_reaches_completion() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_binary_child_refusal/0".into());
    let mut child = BinaryCompletionPort::new();
    child.reject_child = true;
    let error = super::ops::drive_ordinary_binary_expression_v1(
        &mut builder,
        &mut child,
        &binary_completion_input(crate::ast::BinaryOperator::Greater),
    )
    .unwrap_err();
    assert_eq!(error, "original-child-refusal");
    assert_eq!(child.children, 1);
    assert!(child.completions.is_empty());
}

#[test]
fn raw_arithmetic_completion_does_not_fabricate_comparison() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw_arithmetic_completion/0".into());
    let mut child = BinaryCompletionPort::new();
    super::ops::drive_ordinary_binary_expression_v1(
        &mut builder,
        &mut child,
        &binary_completion_input(crate::ast::BinaryOperator::Add),
    )
    .unwrap();
    assert_eq!(child.children, 2);
    assert_eq!(child.completions, vec![None]);
}
