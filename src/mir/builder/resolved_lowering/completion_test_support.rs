use std::sync::Arc;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::RegionId;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, SourceNodeSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1,
};
use crate::mir::{BasicBlockId, MirBuilder, MirCompiler, MirInstruction, MirType, ValueId};

use super::completion_consumption::ResolvedFunctionCompletionConsumptionV1;
use super::draft_seal::{
    DetachedFunctionExitClaimSetV1, FunctionDraftSealPreparationErrorV1,
    MultiSiteExitPreparationErrorV1, PreparedFunctionExitV1, ReadyFunctionDraftSealV1,
};

pub(super) fn resolved_product(
    name: &str,
) -> Arc<crate::mir::resolved_semantics::VerifiedResolvedFunctionV1> {
    let function = function(name, Vec::new());
    let view = FunctionSyntaxViewV1::from_ast(&function).unwrap();
    FunctionSemanticResolverSessionV1::new(0)
        .unwrap()
        .resolve(view)
        .unwrap()
}

pub(super) fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

pub(super) fn return_stmt(value: Option<ASTNode>) -> ASTNode {
    ASTNode::Return {
        value: value.map(Box::new),
        span: Span::unknown(),
    }
}

pub(super) fn if_return(value: i64) -> ASTNode {
    ASTNode::If {
        condition: Box::new(literal(1)),
        then_body: vec![return_stmt(Some(literal(value)))],
        else_body: None,
        span: Span::unknown(),
    }
}

pub(super) fn if_return_unit() -> ASTNode {
    ASTNode::If {
        condition: Box::new(literal(1)),
        then_body: vec![return_stmt(None)],
        else_body: None,
        span: Span::unknown(),
    }
}

pub(super) fn function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

pub(super) fn compile(name: &str, body: Vec<ASTNode>) -> crate::mir::MirFunction {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(name, body)).unwrap();
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("completion_fixture.hako"))
        .unwrap();
    assert!(result.verification_result.is_ok());
    result.module.functions[&format!("{name}/0")].clone()
}

pub(super) fn return_count(function: &crate::mir::MirFunction) -> usize {
    function
        .blocks
        .values()
        .filter(|block| {
            matches!(
                block.terminator.as_ref(),
                Some(MirInstruction::Return { .. })
            )
        })
        .count()
}
