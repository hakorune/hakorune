use super::VerifiedScriptSemanticSourceV1;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::parser::NyashParser;
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn assert_selected_parity(source: &str, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(NyashParser::parse_from_string(source).unwrap(), Some(hint))
        .unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string(source).unwrap(),
                Some(hint),
                std::collections::HashMap::new(),
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(MirPrinter::new().print_module(&normal.module), MirPrinter::new().print_module(&legacy.module));
    assert_eq!(normal.verification_result, legacy.verification_result);
}

fn assert_selected_program_parity(program: ASTNode, hint: &str) {
    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy.compile_with_source(program.clone(), Some(hint)).unwrap();
    let normal = MirCompiler::with_options(false)
        .compile_normal(NormalCompileRequestV1::for_mir_mode(
            program,
            Some(hint),
            std::collections::HashMap::new(),
        ).unwrap())
        .unwrap();
    assert_eq!(MirPrinter::new().print_module(&normal.module), MirPrinter::new().print_module(&legacy.module));
    assert_eq!(normal.verification_result, legacy.verification_result);
}

fn resolved_entry(index: u32) -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(index))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved,
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn literal_program_seals_one_shared_script_owner_and_projection() {
    let ast = NyashParser::parse_from_string("0").expect("literal source");
    let source = PreparedNormalDefaultProgramRootV1::seal(ast).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![resolved_entry(0)], 1)
        .expect("total source window");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let ResolveScriptOutcomeV1::Complete(owner) = resolver
        .resolve_script(view, &window)
        .expect("Script resolve")
    else {
        panic!("literal Script must Complete");
    };
    let product = VerifiedScriptSemanticSourceV1::seal(&source, owner, &window)
        .expect("Script source product");

    assert_eq!(product.forest().owner_count(), 1);
    assert_eq!(product.runtime_source_indices(), &[0]);
    assert!(product
        .projection()
        .owner_root(source.source_ast(), product.forest().roots()[0])
        .is_ok());
}

#[test]
fn sparse_window_preserves_original_program_ordinal() {
    let source = PreparedNormalDefaultProgramRootV1::seal(ASTNode::Program {
        statements: vec![
            ASTNode::Literal { value: LiteralValue::Integer(0), span: Span::unknown() },
            ASTNode::FunctionDeclaration {
                name: "helper".to_owned(), params: Vec::new(), param_decls: Vec::new(),
                return_type_name: None, body: Vec::new(), uses: Vec::new(), contracts: Vec::new(),
                is_static: true, is_override: false, attrs: Default::default(), span: Span::unknown(),
            },
            ASTNode::Literal { value: LiteralValue::Integer(1), span: Span::unknown() },
        ],
        span: Span::unknown(),
    }).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(
        vec![resolved_entry(0), resolved_entry(1), resolved_entry(2)], 3,
    ).expect("window");
    assert_eq!(window.entry_at(2).unwrap().site().node().segments()[1], SourcePathSegmentV1::ProgramBody(2));
    let _ = source;
}

#[test]
fn selected_normal_lexical_local_and_read_use_one_ledger() {
    assert_selected_parity("local x = 1\nprint(x)", "script-local.hako");
}

#[test]
fn selected_normal_print_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = 1\nprint(-x)", "script-unary.hako");
}

#[test]
fn real_print_fixture_uses_the_selected_normal_request() {
    assert_selected_parity("print(1)", "script-print.hako");
}

#[test]
fn selected_normal_binary_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = 1\nprint((x * 2) + 3)", "script-binary.hako");
}

#[test]
fn selected_normal_await_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = 1\nprint(await -(x + 2))", "script-await.hako");
}

#[test]
fn selected_normal_check_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = true\nprint(check { x })", "script-check.hako");
}

#[test]
fn selected_normal_and_or_lexical_closure_matches_legacy() {
    assert_selected_parity("local x = true\nprint(x and x)\nprint(x or x)", "script-andor.hako");
}

#[test]
fn lexical_fastmem_scope_matches_legacy() {
    let local = ASTNode::Local {
        variables: vec!["x".to_owned()],
        initial_values: vec![Some(Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(1), span: Span::unknown(),
        }))],
        declared_type_names: vec![None], span: Span::unknown(),
    };
    let print = ASTNode::Print {
        expression: Box::new(ASTNode::Variable { name: "x".to_owned(), span: Span::unknown() }),
        span: Span::unknown(),
    };
    assert_selected_program_parity(ASTNode::Program {
        statements: vec![ASTNode::FastMemRegion {
            contract: "PageMapV0".to_owned(), body: vec![local, print], span: Span::unknown(),
        }],
        span: Span::unknown(),
    }, "script-fastmem.hako");
}

#[test]
fn fastmem_weak_child_remains_deferred_before_name_resolution() {
    let mut compiler = MirCompiler::with_options(false);
    let weak_missing = ASTNode::UnaryOp {
        operator: crate::ast::UnaryOperator::Weak,
        operand: Box::new(ASTNode::Variable { name: "missing".to_owned(), span: Span::unknown() }),
        span: Span::unknown(),
    };
    let program = ASTNode::Program {
        statements: vec![ASTNode::FastMemRegion {
            contract: "PageMapV0".to_owned(), body: vec![weak_missing], span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let error = compiler.compile_normal(NormalCompileRequestV1::for_mir_mode(
        program, Some("script-fastmem-weak.hako"), std::collections::HashMap::new(),
    ).unwrap()).expect_err("Weak FastMem child must use the existing lower route");
    assert!(error.contains("Undefined variable: missing"), "{error}");
}

#[test]
fn selected_and_or_failure_discards_candidate_and_reuses_compiler() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler.compile_normal(NormalCompileRequestV1::for_mir_mode(
        NyashParser::parse_from_string("local x = true\nprint(x and missing)").unwrap(),
        Some("script-failure.hako"), std::collections::HashMap::new(),
    ).unwrap()).expect_err("undefined name must reject");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    compiler.compile_normal(NormalCompileRequestV1::for_mir_mode(
        NyashParser::parse_from_string("print(1)").unwrap(), Some("script-reuse.hako"),
        std::collections::HashMap::new(),
    ).unwrap()).expect("fresh request succeeds");
}

#[test]
fn script_static_const_u16_completion_matches_legacy_metadata() {
    assert_selected_parity("static const TABLE: u16[] = [1, 2, 3]\nprint(1)", "script-static.hako");
}

#[test]
fn weak_unary_remains_deferred() {
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler.compile_normal(NormalCompileRequestV1::for_mir_mode(
        NyashParser::parse_from_string("weak missing").unwrap(), Some("script-weak.hako"),
        std::collections::HashMap::new(),
    ).unwrap()).expect_err("weak stays on existing lower route");
    assert!(error.contains("Undefined variable: missing"), "{error}");
}
