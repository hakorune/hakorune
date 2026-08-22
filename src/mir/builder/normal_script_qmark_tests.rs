use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1,
    ScriptRootQMarkPropagationAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn root_qmark(operand: ASTNode) -> ASTNode {
    ASTNode::QMarkPropagate {
        expression: Box::new(operand),
        span: Span::unknown(),
    }
}

fn root_qmark_entry() -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::QMarkPropagation(
            ScriptRootQMarkPropagationAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn root_qmark_await_seals_the_exact_operand_and_matches_legacy() {
    let program = ASTNode::Program {
        statements: vec![root_qmark(ASTNode::AwaitExpression {
            expression: Box::new(integer(42)),
            span: Span::unknown(),
        })],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![root_qmark_entry()], 1).expect("QMark window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let outcome = FunctionSemanticResolverSessionV1::new(0)
        .expect("resolver")
        .resolve_script(view, &window)
        .expect("QMark resolve");
    let ResolveScriptOutcomeV1::Complete(product) = outcome else {
        panic!("root QMark must complete");
    };
    let sealed = super::VerifiedScriptSemanticSourceV1::seal(&source, product, &window)
        .expect("QMark source seal");
    let receipts = sealed.qmark_propagations().collect::<Vec<_>>();
    assert_eq!(receipts.len(), 1);
    assert_eq!(
        receipts[0].0.node().segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(0),
        ]
    );
    assert_eq!(
        receipts[0].1.node().segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(0),
            SourcePathSegmentV1::QMarkOperand,
        ]
    );
    assert!(matches!(
        receipts[0].2,
        super::ScriptQMarkPropagationTargetV1::CurrentScriptOwner
    ));

    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some("script-root-qmark.hako"))
        .expect("legacy QMark");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-root-qmark.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected QMark");
    let printer = MirPrinter::new();
    let rendered = printer.print_module(&normal.module);
    assert!(rendered.contains("RuntimeDataBox.isOk"), "{rendered}");
    assert!(rendered.contains("RuntimeDataBox.getValue"), "{rendered}");
    assert_eq!(rendered, printer.print_module(&legacy.module));
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn root_qmark_with_an_unsafe_operand_defers_before_child_semantics() {
    let program = ASTNode::Program {
        statements: vec![root_qmark(ASTNode::FunctionCall {
            name: "missing".to_owned(),
            arguments: Vec::new(),
            span: Span::unknown(),
        })],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window =
        VerifiedScriptRootDemandWindowV1::seal(vec![root_qmark_entry()], 1).expect("QMark window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("deferred QMark resolution"),
        ResolveScriptOutcomeV1::Deferred(_)
    ));
}

#[test]
fn root_qmark_missing_operand_keeps_root_lower_diagnostic_and_fresh_reuse() {
    let failing = ASTNode::Program {
        statements: vec![root_qmark(ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        })],
        span: Span::unknown(),
    };
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                failing,
                Some("script-root-qmark-missing.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect_err("missing QMark operand must remain RootLower-owned");
    assert!(error.contains("Undefined variable: missing"), "{error}");

    compiler
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                ASTNode::Program {
                    statements: vec![root_qmark(ASTNode::AwaitExpression {
                        expression: Box::new(integer(42)),
                        span: Span::unknown(),
                    })],
                    span: Span::unknown(),
                },
                Some("script-root-qmark-reuse.hako"),
                std::collections::HashMap::new(),
            )
            .expect("fresh normal request"),
        )
        .expect("fresh QMark request must not reuse rejected semantic state");
}
