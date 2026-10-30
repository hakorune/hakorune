//! D2 candidate-abort proof for the selected If recipe shape.
//!
//! The production path already owns a whole unpublished compile candidate.
//! This test covers the existing no-call candidate-abort proof; it does not
//! add a fault toggle or transaction owner.

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::{MirCompiler, VerifiedResolvedSourceUnitV1};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn bool_literal(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(literal(value)),
        span: Span::unknown(),
    }
}

fn if_stmt(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn function(else_body: Option<Vec<ASTNode>>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "if_abort_d2".to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            local("x", literal(0)),
            if_stmt(bool_literal(true), vec![assignment("x", 1)], else_body),
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn assert_candidate_abort_reuses_compiler(unit: VerifiedResolvedSourceUnitV1, source_file: &str, source: &str) {
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();

    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).expect("trivial If plan")
    else {
        panic!("If fixture must select TrivialBindingSsa");
    };
    let mut candidate =
        super::module_session::CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    let error = candidate
        .builder_mut()
        .lower_resolved_trivial_function_draft_with_seal_failure_for_test(plan)
        .expect_err("late draft-seal failure must reject after selected If lowering");
    assert!(matches!(
        error,
        super::CanonicalResolvedBuildErrorV1::BuilderContract(detail)
            if detail.contains("DraftSeal") || detail.contains("draft_seal")
    ));
    drop(candidate);

    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());
    assert!(compiler.builder.current_function_entry_block().is_none());

    let result = compile_reuse_source(
        &mut compiler,
        source,
        source_file,
    )
        .expect("same compiler must accept a fresh If request");
    assert!(result.verification_result.is_ok());
    assert!(result.module.functions.contains_key("if_abort_d2/0"));
    let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::free_function("if_abort_d2", 0);
    assert_eq!(
        result.module.canonical_callable_definition_symbol(&key),
        Some("if_abort_d2/0")
    );
}

pub(super) fn compile_reuse_source(
    compiler: &mut MirCompiler,
    text: &str,
    source_file: &str,
) -> Result<crate::mir::compiler::MirCompileResult, String> {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("reuse source parses");
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("reuse source transform")
    else {
        panic!("reuse source must remain source-backed");
    };
    compiler.compile_normal(super::NormalCompileRequestV1::for_mir_mode_callable_source(
        source,
        Some(source_file),
        std::collections::HashMap::new(),
    ))
}

#[test]
fn implicit_if_candidate_discards_after_late_draft_seal_failure() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(None))
        .expect("implicit If fixture resolves");
    assert_candidate_abort_reuses_compiler(unit, "implicit-if-reused.hako", "function if_abort_d2() { local x = 0\n if true { x = 1 }\n return x }\n static box Main { main() { return 0 } }");
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(Some(vec![assignment("x", 2)])))
        .expect("explicit If fixture resolves");
    assert_candidate_abort_reuses_compiler(unit, "explicit-if-reused.hako", "function if_abort_d2() { local x = 0\n if true { x = 1 } else { x = 2 }\n return x }\n static box Main { main() { return 0 } }");
}
