use std::sync::Arc;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::NormalCompileRequestV1;
use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};
use crate::mir::{MirCompiler, MirInstruction};

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

pub(super) fn compile(name: &str, body: &str) -> crate::mir::MirFunction {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let text =
        format!("function {name}() {{ {body} }}\nstatic box Main {{ main() {{ return 0 }} }}");
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        &text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    else {
        panic!("completion fixture must remain source-backed");
    };
    let result = MirCompiler::with_options(false)
        .compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
            source,
            Some("completion_fixture.hako"),
            std::collections::HashMap::new(),
        ))
        .unwrap();
    assert!(result.verification_result.is_ok());
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::free_function(name, 0);
    assert_eq!(
        result.module.canonical_callable_definition_symbol(&key),
        Some(format!("{name}/0").as_str())
    );
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
