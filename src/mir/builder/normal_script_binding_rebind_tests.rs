use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1,
    ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use crate::parser::NyashParser;

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
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

fn grouped_assignment(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::GroupedAssignmentExpr {
        lhs: name.to_owned(),
        rhs: Box::new(value),
        span: Span::unknown(),
    }
}

fn print(expr: ASTNode) -> ASTNode {
    ASTNode::Print {
        expression: Box::new(expr),
        span: Span::unknown(),
    }
}

fn grouped_rebind_program(rhs: ASTNode, include_print: bool) -> ASTNode {
    let mut statements = vec![local("x", integer(1)), grouped_assignment("x", rhs)];
    if include_print {
        statements.push(print(variable("x")));
    }
    ASTNode::Program {
        statements,
        span: Span::unknown(),
    }
}

fn assert_selected_parity(source: &str, hint: &str) {
    let program = NyashParser::parse_from_string(source).expect("assignment source");
    crate::test_support::with_env_vars(&[("NYASH_MIR_UNIFIED_CALL", None)], || {
        let legacy = MirCompiler::with_options(false)
            .compile_with_source(program.clone(), Some(hint))
            .expect("legacy assignment");
        let normal = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    program,
                    Some(hint),
                    std::collections::HashMap::new(),
                )
                .expect("normal request"),
            )
            .expect("selected assignment");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module),
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    });
}

fn resolved_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

fn rebind_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::BindingRebind(
            ScriptRootBindingRebindAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn variable_target_rebind_is_part_of_the_complete_script_owner() {
    let program = NyashParser::parse_from_string("local x = 1\nx = 2\nprint(x)")
        .expect("binding-rebind source");
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![resolved_entry(0), rebind_entry(1), resolved_entry(2)],
        3,
    )
    .expect("binding-rebind demand window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("binding-rebind resolve"),
        ResolveScriptOutcomeV1::Complete(_)
    ));
}

#[test]
fn prior_local_variable_assignment_rebinds_the_script_ledger() {
    assert_selected_parity(
        "local x = 1\nx = 2\nprint(x)",
        "script-binding-rebind-assignment.hako",
    );
}

#[test]
fn prior_local_variable_compound_assignment_rebinds_the_script_ledger() {
    assert_selected_parity(
        "local x = 1\nx += 2\nprint(x)",
        "script-binding-rebind-compound.hako",
    );
}

fn assert_selected_program_parity(program: ASTNode, hint: &str) {
    crate::test_support::with_env_vars(&[("NYASH_MIR_UNIFIED_CALL", None)], || {
        let legacy = MirCompiler::with_options(false)
            .compile_with_source(program.clone(), Some(hint))
            .expect("legacy assignment");
        let normal = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    program,
                    Some(hint),
                    std::collections::HashMap::new(),
                )
                .expect("normal request"),
            )
            .expect("selected assignment");
        assert_eq!(
            MirPrinter::new().print_module(&normal.module),
            MirPrinter::new().print_module(&legacy.module),
        );
        assert_eq!(normal.verification_result, legacy.verification_result);
    });
}

#[test]
fn prior_local_grouped_assignment_rebinds_the_script_ledger() {
    // The grouped `(x = ...)` statement spelling was retired from the
    // parser; build the admitted GroupedAssignmentExpr node directly.
    let program = grouped_rebind_program(
        ASTNode::UnaryOp {
            operator: UnaryOperator::Minus,
            operand: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(variable("x")),
                right: Box::new(integer(1)),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
        true,
    );
    assert_selected_program_parity(program, "script-binding-rebind-grouped.hako");
}

#[test]
fn grouped_assignment_resolves_as_a_complete_script_rebind() {
    let program = grouped_rebind_program(integer(2), true);
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![resolved_entry(0), rebind_entry(1), resolved_entry(2)],
        3,
    )
    .expect("grouped binding-rebind demand window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("grouped binding-rebind resolve"),
        ResolveScriptOutcomeV1::Complete(_)
    ));
}

#[test]
fn failed_rebind_request_discards_its_ledger_before_fresh_reuse() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("local x = 1\nx = missing").expect("failing source"),
                Some("script-binding-rebind-failure.hako"),
                std::collections::HashMap::new(),
            )
            .expect("failing request"),
        )
        .expect_err("missing RHS must retain existing RootLower diagnostic");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("local x = 1\nx = 2\nprint(x)")
                    .expect("fresh source"),
                Some("script-binding-rebind-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh request"),
        )
        .expect("fresh request must not reuse a failed ledger");
}

#[test]
fn grouped_rebind_rhs_failure_discards_its_ledger_before_fresh_reuse() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                grouped_rebind_program(variable("missing"), false),
                Some("script-binding-rebind-grouped-failure.hako"),
                std::collections::HashMap::new(),
            )
            .expect("failing request"),
        )
        .expect_err("missing grouped RHS must retain existing RootLower diagnostic");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                grouped_rebind_program(integer(2), true),
                Some("script-binding-rebind-grouped-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh request"),
        )
        .expect("fresh grouped request must not reuse a failed ledger");
}
