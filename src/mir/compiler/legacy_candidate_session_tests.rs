use std::collections::HashMap;

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span, UnaryOperator};
use crate::mir::{MirCompiler, MirPrinter, MirType, NormalCompileRequestV1};
use crate::parser::NyashParser;

fn core_cursor(compiler: &MirCompiler) -> (u32, u32, u32, u32, u32) {
    (
        compiler.builder.core_ctx.peek_next_value().as_u32(),
        compiler.builder.core_ctx.peek_next_block().as_u32(),
        compiler.builder.core_ctx.next_binding_id,
        compiler.builder.core_ctx.temp_slot_counter,
        compiler.builder.core_ctx.debug_join_counter,
    )
}

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.to_owned()),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn unary(operator: crate::ast::UnaryOperator, operand: ASTNode) -> ASTNode {
    ASTNode::UnaryOp {
        operator,
        operand: Box::new(operand),
        span: Span::unknown(),
    }
}

fn binary(operator: crate::ast::BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn awaited(expression: ASTNode) -> ASTNode {
    ASTNode::AwaitExpression {
        expression: Box::new(expression),
        span: Span::unknown(),
    }
}

fn checked(expressions: Vec<ASTNode>) -> ASTNode {
    ASTNode::CheckExpr {
        name: Some("normal-root".to_owned()),
        items: expressions
            .into_iter()
            .enumerate()
            .map(|(index, expression)| crate::ast::CheckItem {
                label: Some(format!("item-{index}")),
                expression,
            })
            .collect(),
        span: Span::unknown(),
    }
}

fn printed(expression: ASTNode) -> ASTNode {
    ASTNode::Print {
        expression: Box::new(expression),
        span: Span::unknown(),
    }
}

fn nowait(variable: &str, expression: ASTNode) -> ASTNode {
    ASTNode::Nowait {
        variable: variable.to_owned(),
        expression: Box::new(expression),
        span: Span::unknown(),
    }
}

fn array(elements: Vec<ASTNode>) -> ASTNode {
    ASTNode::ArrayLiteral {
        elements,
        span: Span::unknown(),
    }
}

fn map(entries: Vec<(&str, ASTNode)>) -> ASTNode {
    ASTNode::MapLiteral {
        entries: entries
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
        span: Span::unknown(),
    }
}

fn grouped_assignment(variable_name: &str, rhs: ASTNode) -> ASTNode {
    ASTNode::GroupedAssignmentExpr {
        lhs: variable_name.to_owned(),
        rhs: Box::new(rhs),
        span: Span::unknown(),
    }
}

fn indexed(target: ASTNode, index: ASTNode) -> ASTNode {
    ASTNode::Index {
        target: Box::new(target),
        index: Box::new(index),
        span: Span::unknown(),
    }
}

fn block_expr(tail_expr: ASTNode) -> ASTNode {
    block_expr_with_prelude(Vec::new(), tail_expr)
}

fn block_expr_with_prelude(prelude_stmts: Vec<ASTNode>, tail_expr: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr: Box::new(tail_expr),
        span: Span::unknown(),
    }
}

fn local(name: &str, initializer: Option<ASTNode>) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![initializer.map(Box::new)],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn task_scope(source_keyword: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::TaskScope {
        body,
        source_keyword: source_keyword.to_owned(),
        span: Span::unknown(),
    }
}

fn program(statement: ASTNode) -> ASTNode {
    ASTNode::Program {
        statements: vec![statement],
        span: Span::unknown(),
    }
}

fn source_file(compiler: &MirCompiler) -> Option<String> {
    compiler.builder.current_source_file()
}

fn normal_request(
    ast: ASTNode,
    source_file: Option<&str>,
    imports: HashMap<String, String>,
) -> NormalCompileRequestV1 {
    NormalCompileRequestV1::for_mir_mode(ast, source_file, imports)
        .expect("test normal request must own Program")
}

#[test]
fn real_integer_zero_fixture_uses_the_selected_normal_request() {
    let source =
        include_str!("../../../tools/checks/fixtures/raw_vm_reference_conformance/integer_0.hako");
    let ast = NyashParser::parse_from_string(source).expect("integer_0 fixture parses");
    let mut compiler = MirCompiler::with_options(false);

    let result = compiler
        .compile_normal(normal_request(ast, Some("integer_0.hako"), HashMap::new()))
        .expect("literal-only Script reaches selected normal lifecycle");

    assert!(result.verification_result.is_ok());
}

fn program_v0_import_bundle_request(ast: ASTNode) -> NormalCompileRequestV1 {
    NormalCompileRequestV1::for_program_json_v0_import_bundle(ast)
        .expect("test Program-v0 import bundle must own Program")
}

#[test]
fn repl_program_matches_legacy_config_and_failure_reuse() {
    let configure = |compiler: &mut MirCompiler| {
        compiler.set_repl_mode(true);
        compiler.set_quiet_internal_logs(true);
        compiler
            .builder
            .comp_ctx
            .plugin_method_sigs
            .insert(("PluginBox".into(), "value/0".into()), MirType::Integer);
        compiler
            .builder
            .comp_ctx
            .using_import_boxes
            .insert("Ambient".into(), "MustNotLeak".into());
    };
    let success = program(literal(7));
    let mut legacy = MirCompiler::new();
    configure(&mut legacy);
    let expected = legacy
        .compile_with_source(success.clone(), Some("<repl>"))
        .expect("legacy REPL oracle");
    let mut typed = MirCompiler::new();
    configure(&mut typed);
    let before = (
        typed.builder.repl_mode,
        typed.builder.comp_ctx.quiet_internal_logs,
        typed.builder.comp_ctx.plugin_method_sigs.clone(),
    );
    let error = typed
        .compile_normal(
            NormalCompileRequestV1::for_repl_program(program(variable("missing"))).unwrap(),
        )
        .expect_err("typed REPL failure");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    assert_eq!(
        (
            typed.builder.repl_mode,
            typed.builder.comp_ctx.quiet_internal_logs,
            typed.builder.comp_ctx.plugin_method_sigs.clone(),
        ),
        before
    );
    let actual = typed
        .compile_normal(NormalCompileRequestV1::for_repl_program(success).unwrap())
        .expect("typed REPL reuse");
    assert_eq!(
        MirPrinter::new().print_module(&actual.module),
        MirPrinter::new().print_module(&expected.module)
    );
    assert_eq!(actual.verification_result, expected.verification_result);
    assert!(typed.builder.comp_ctx.using_import_boxes.is_empty());
    assert_eq!(source_file(&typed).as_deref(), Some("<repl>"));
}

#[test]
fn late_normal_lowering_failure_leaves_live_builder_unchanged_and_reusable() {
    let root = NyashParser::parse_from_string(
        r#"
            box Page { birth() { return 6 } staged() { return 1 } }
            print(missing)
        "#,
    )
    .expect("late-failure source");
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.repl_mode = true;
    compiler.builder.comp_ctx.quiet_internal_logs = true;
    compiler
        .builder
        .comp_ctx
        .using_import_boxes
        .insert("Old".into(), "Live".into());
    compiler
        .builder
        .comp_ctx
        .plugin_method_sigs
        .insert(("PluginBox".into(), "value/0".into()), MirType::Integer);
    compiler.builder.set_source_file_hint("live-before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();

    let before = (
        compiler.builder.repl_mode,
        compiler.builder.comp_ctx.quiet_internal_logs,
        compiler.builder.comp_ctx.using_import_boxes.clone(),
        compiler.builder.comp_ctx.plugin_method_sigs.clone(),
        source_file(&compiler),
        core_cursor(&compiler),
    );
    let failed_imports = HashMap::from([("Failed".to_owned(), "Candidate".to_owned())]);
    let error = compiler
        .compile_normal(normal_request(
            root,
            Some("failed-candidate.hako"),
            failed_imports,
        ))
        .expect_err("instance Script box stays on the retired compat boundary");

    assert!(
        error.contains("[freeze:contract][raw-compat/runtime-box-fate-retired/instance]"),
        "{error}"
    );
    assert_eq!(
        (
            compiler.builder.repl_mode,
            compiler.builder.comp_ctx.quiet_internal_logs,
            compiler.builder.comp_ctx.using_import_boxes.clone(),
            compiler.builder.comp_ctx.plugin_method_sigs.clone(),
            source_file(&compiler),
            core_cursor(&compiler),
        ),
        before
    );
    assert!(compiler.builder.current_module.is_none());

    let result = compiler
        .compile_normal(normal_request(
            program(literal(7)),
            Some("reused.hako"),
            HashMap::new(),
        ))
        .expect("fresh candidate after failure");
    assert!(result.module.functions.contains_key("main"));
    assert!(compiler.builder.comp_ctx.using_import_boxes.is_empty());
    assert_eq!(source_file(&compiler).as_deref(), Some("reused.hako"));
}

#[test]
fn typed_root_return_rejection_keeps_the_published_builder() {
    let mut compiler = MirCompiler::with_options(false);
    let compile = |compiler: &mut MirCompiler, source: &str, path| {
        compiler.compile_normal(normal_request(
            NyashParser::parse_from_string(source).unwrap(),
            Some(path),
            HashMap::new(),
        ))
    };
    let original = compile(&mut compiler, "return 7", "published-before.hako").unwrap();
    assert!(original.verification_result.is_ok());
    let before = (source_file(&compiler), core_cursor(&compiler));

    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        "static box Main { main(): i64 { return null } }",
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(rejected) = transformed
    else {
        panic!("typed Main needs the source-backed entry");
    };
    let error = compiler
        .compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
            rejected,
            Some("rejected-return.hako"),
            HashMap::new(),
        ))
        .unwrap_err();
    assert!(error.contains("[freeze:contract][main-arguments/root-completion]"), "{error}");
    assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());

    let reused = compile(&mut compiler, "return 9", "published-after.hako").unwrap();
    assert!(reused.verification_result.is_ok());
    assert_eq!(source_file(&compiler).as_deref(), Some("published-after.hako"));
}

fn compile_loop_source(
    compiler: &mut MirCompiler,
    source: &str,
    path: &str,
) -> Result<crate::mir::MirCompileResult, String> {
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_with_identity_v1,
        NormalCallableMaterializationOutcomeV1,
    };

    let parsed = materialize_normal_callable_program_with_identity_v1(
        source,
        crate::parser::ParserBuildConfig::default(),
        path,
    )
    .expect("loop fixture must materialize");
    let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = parsed else {
        panic!("loop fixture must use the source-backed normal route");
    };
    compiler.compile_normal(NormalCompileRequestV1::for_mir_mode_callable_source(
        source,
        Some(path),
        HashMap::new(),
    ))
}

#[test]
fn cataloged_static_nested_loops_keep_separate_bool_branches() {
    use crate::mir::{MirInstruction, MirType};

    let source = r#"
        static box Counter {
            nested(n: i64): i64 {
                local i = n
                local j = n
                loop(i + 1 < 4) {
                    loop(j + 1 < 4) { j = j + 1 }
                    i = i + 1
                }
                return j
            }
        }
        static box Main { main() { return Counter.nested(0) } }
    "#;
    let mut compiler = MirCompiler::with_options(false);
    let compiled = compile_loop_source(&mut compiler, source, "nested-static.hako")
        .expect("ordinary static nested loop");
    compiled.verification_result.expect("verified nested loop CFG and SSA");

    let function = &compiled.module.functions["Counter.nested/1"];
    let mut predicates = function.blocks.values().filter_map(|block| {
        let Some(MirInstruction::Branch { condition, .. }) = &block.terminator else {
            return None;
        };
        assert_eq!(function.metadata.value_types.get(condition), Some(&MirType::Bool));
        assert!(block.instructions.iter().any(|instruction| matches!(
            instruction, MirInstruction::Compare { dst, .. } if dst == condition
        )));
        Some((block.id, *condition))
    });
    let (first_block, first_value) = predicates.next().expect("one loop predicate");
    let (second_block, second_value) = predicates.next().expect("other loop predicate");
    assert_ne!(first_block, second_block);
    assert_ne!(first_value, second_value);
}

#[test]
fn rejected_loop_body_rolls_back_emitted_phis_and_reuses_compiler() {
    const REJECTED: &str = include_str!("../../../apps/tests/phase29bq_generic_loop_v1_general_if_min.hako");
    const VALID: &str = include_str!("../../../apps/loop-basics/main.hako");
    let mut compiler = MirCompiler::with_options(false);
    let original = compile_loop_source(&mut compiler, VALID, "loop-before.hako").unwrap();
    assert!(original.verification_result.is_ok());
    let live = |compiler: &MirCompiler| {
        (
            source_file(compiler),
            core_cursor(compiler),
            compiler.builder.current_module.as_ref().map(|module| {
                MirPrinter::new().print_module(module)
            }),
            compiler.builder.current_function_name().map(str::to_owned),
            compiler.builder.current_function_entry_block(),
        )
    };
    let before = live(&compiler);
    let error = compile_loop_source(
        &mut compiler,
        REJECTED,
        "apps/tests/phase29bq_generic_loop_v1_general_if_min.hako",
    )
    .expect_err("the corpus input must be rejected");
    assert_eq!(live(&compiler), before);

    let reused = compile_loop_source(&mut compiler, VALID, "loop-after.hako").unwrap();
    assert!(reused.verification_result.is_ok());
    assert_eq!(source_file(&compiler).as_deref(), Some("loop-after.hako"));
    assert_eq!(reused.module.function_names(), original.module.function_names());

    let pending: usize = error
        .split_once("[freeze:contract][phi_lifecycle/txn_abort] tag=source-loop-header-after pending_count=")
        .and_then(|(_, detail)| detail.split_whitespace().next())
        .and_then(|count| count.parse().ok())
        .unwrap_or_else(|| panic!("expected the emitted PHI transaction to abort: {error}"));
    assert!(pending > 0, "no provisional PHIs were emitted: {error}");
    assert!(error.contains("cleanup_failure_count=0"), "{error}");
    assert!(error.contains("[freeze:contract][local-read/loop-point-missing]"), "{error}");
}

#[test]
fn nonmain_static_box_failure_discards_candidate_and_reuses_live_compiler() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let failing = NyashParser::parse_from_string(
        "static box Helpers { value() { return missing } } static box Main { main() { return 0 } }",
    )
    .expect("failing static Box source");
    let succeeding = NyashParser::parse_from_string(
        "static box Helpers { value() { return 1 } } static box Main { main() { return 0 } }",
    )
    .expect("corrected static Box source");
    let mut compiler = MirCompiler::with_options(false);
    compiler
        .builder
        .set_source_file_hint("static-live-before.hako");
    compiler.builder.next_value_id();
    let before = (source_file(&compiler), core_cursor(&compiler));

    let error = compiler
        .compile_normal(normal_request(
            failing,
            Some("static-failure.hako"),
            HashMap::new(),
        ))
        .expect_err("failing deferred static Box must reject its candidate");

    assert!(error.contains("Undefined variable: missing"), "{error}");
    assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
    assert!(compiler.builder.current_module.is_none());

    let result = compiler
        .compile_normal(normal_request(
            succeeding,
            Some("static-reused.hako"),
            HashMap::new(),
        ))
        .expect("corrected deferred static Box after candidate discard");
    assert!(result.module.functions.contains_key("main"));
    assert_eq!(
        source_file(&compiler).as_deref(),
        Some("static-reused.hako")
    );
}

#[test]
fn explicit_imports_commit_only_with_the_finished_normal_candidate() {
    let mut compiler = MirCompiler::with_options(false);
    compiler
        .builder
        .comp_ctx
        .using_import_boxes
        .insert("Old".into(), "Live".into());
    let imports = HashMap::from([
        ("Alias".to_owned(), "Imported".to_owned()),
        ("Other".to_owned(), "Second".to_owned()),
    ]);

    let result = compiler
        .compile_normal(normal_request(
            program(literal(11)),
            Some("explicit-imports.hako"),
            imports.clone(),
        ))
        .expect("finished explicit-import candidate");

    assert!(result.module.functions.contains_key("main"));
    assert_eq!(compiler.builder.comp_ctx.using_import_boxes, imports);
    assert_eq!(
        source_file(&compiler).as_deref(),
        Some("explicit-imports.hako")
    );
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn normal_pipeline_matches_legacy_compatibility_for_general_module() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = r#"
static box Fixture {
  selected() { return 1 }
  selected_peer() { return 2 }
}

box Page {
  capacity: usize = 0
  birth() { return 6 }
  answer() { return 7 }
}

static box Utility {
  answer() { return 7 }
}

static box Main {
  main(x) {
    local p = new Page()
    p.capacity = x
    return 0
  }
}
    "#;
    let legacy_ast = NyashParser::parse_from_string(source).expect("legacy source");
    let candidate_ast = NyashParser::parse_from_string(source).expect("candidate source");
    let mut legacy_compiler = MirCompiler::with_options(false);
    let legacy_error = legacy_compiler
        .compile_with_source(legacy_ast, Some("numeric-parity.hako"))
        .expect_err("legacy compatibility module stays on the retired birth-global boundary");
    assert!(
        legacy_error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{legacy_error}"
    );
    let mut compiler = MirCompiler::with_options(false);
    let candidate_error = compiler
        .compile_normal(normal_request(
            candidate_ast,
            Some("numeric-parity.hako"),
            HashMap::new(),
        ))
        .expect_err("normal candidate stays on the retired birth-global boundary");
    assert!(
        candidate_error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{candidate_error}"
    );

    let optimized_legacy_ast = NyashParser::parse_from_string(source).expect("legacy source");
    let program_v0_ast = NyashParser::parse_from_string(source).expect("Program-v0 source");
    let mut optimized_legacy_compiler = MirCompiler::with_options(true);
    let optimized_legacy_error = optimized_legacy_compiler
        .compile_with_source(optimized_legacy_ast, Some("<json_v0/imports>"))
        .expect_err("optimized legacy module stays on the retired birth-global boundary");
    assert!(
        optimized_legacy_error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{optimized_legacy_error}"
    );
    let mut program_v0_compiler = MirCompiler::with_options(true);
    let program_v0_error = program_v0_compiler
        .compile_normal(program_v0_import_bundle_request(program_v0_ast))
        .expect_err("Program-v0 import bundle stays on the retired birth-global boundary");
    assert!(
        program_v0_error.contains("[freeze:contract][ordinary-new/birth-global-legacy-stopped]"),
        "{program_v0_error}"
    );
}

#[test]
fn program_v0_typed_failure_keeps_live_builder_reusable_without_retry() {
    let root =
        NyashParser::parse_from_string("print(missing)").expect("Program-v0 late-failure source");
    let mut compiler = MirCompiler::with_options(true);
    compiler
        .builder
        .comp_ctx
        .using_import_boxes
        .insert("Old".into(), "Live".into());
    compiler.builder.set_source_file_hint("live-before.hako");
    compiler.builder.next_value_id();
    let before = (
        compiler.builder.comp_ctx.using_import_boxes.clone(),
        source_file(&compiler),
        core_cursor(&compiler),
    );

    let error = compiler
        .compile_normal(program_v0_import_bundle_request(root))
        .expect_err("undefined variable must reject typed Program-v0 candidate");
    assert!(error.contains("Undefined variable: missing"), "{error}");
    assert_eq!(
        (
            compiler.builder.comp_ctx.using_import_boxes.clone(),
            source_file(&compiler),
            core_cursor(&compiler),
        ),
        before
    );
    assert!(compiler.builder.current_module.is_none());

    let result = compiler
        .compile_normal(program_v0_import_bundle_request(program(literal(7))))
        .expect("fresh typed Program-v0 candidate after failure");
    assert!(result.module.functions.contains_key("main"));
    assert!(compiler.builder.comp_ctx.using_import_boxes.is_empty());
    assert_eq!(source_file(&compiler).as_deref(), Some("<json_v0/imports>"));
}

#[test]
fn program_v0_typed_errors_match_legacy_program_stages_exactly() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    for source in [
        r#"
            static box Main { main() {} }
            static box Main { main() {} }
        "#,
        r#"
            box Page {}
            box Page {}
            static box Main { main() {} }
        "#,
        "print(missing)",
    ] {
        let legacy_ast = NyashParser::parse_from_string(source).expect("legacy Program source");
        let typed_ast = NyashParser::parse_from_string(source).expect("typed Program source");
        let mut legacy_compiler = MirCompiler::with_options(true);
        let legacy_error = legacy_compiler
            .compile_with_source(legacy_ast, Some("<json_v0/imports>"))
            .expect_err("legacy Program stage must reject");
        let mut typed_compiler = MirCompiler::with_options(true);
        let typed_error = typed_compiler
            .compile_normal(program_v0_import_bundle_request(typed_ast))
            .expect_err("typed Program stage must reject");

        assert_eq!(typed_error, legacy_error);
    }
}

#[test]
fn public_program_admission_rejects_non_program_roots() {
    let roots = [
        literal(17),
        unary(
            UnaryOperator::Minus,
            binary(BinaryOperator::Add, literal(1), literal(2)),
        ),
        binary(BinaryOperator::And, boolean(true), boolean(false)),
        binary(
            BinaryOperator::Or,
            boolean(false),
            binary(BinaryOperator::And, boolean(true), boolean(true)),
        ),
        awaited(literal(18)),
        awaited(unary(
            UnaryOperator::Minus,
            binary(BinaryOperator::Add, literal(3), literal(4)),
        )),
        unary(UnaryOperator::Minus, awaited(literal(19))),
        binary(BinaryOperator::Add, awaited(literal(20)), literal(5)),
        awaited(awaited(literal(21))),
        checked(Vec::new()),
        checked(vec![boolean(true)]),
        checked(vec![boolean(true), boolean(false), boolean(true)]),
        checked(vec![checked(vec![boolean(true)]), awaited(boolean(false))]),
        unary(
            UnaryOperator::Minus,
            checked(vec![boolean(true), boolean(false)]),
        ),
        awaited(checked(vec![boolean(true), awaited(boolean(false))])),
        printed(literal(22)),
        printed(awaited(checked(vec![
            boolean(true),
            awaited(boolean(false)),
        ]))),
        nowait(
            "pending",
            awaited(checked(vec![boolean(true), boolean(false)])),
        ),
        array(Vec::new()),
        array(vec![literal(24), array(vec![literal(25), literal(26)])]),
        awaited(array(vec![checked(vec![boolean(true), boolean(false)])])),
        map(Vec::new()),
        map(vec![
            ("key", literal(27)),
            ("key", array(vec![literal(28), literal(29)])),
        ]),
        awaited(map(vec![("nested", map(vec![("value", literal(30))]))])),
        indexed(array(vec![literal(31), literal(32)]), literal(1)),
        indexed(map(vec![("key", literal(33))]), string("key")),
        awaited(indexed(array(vec![literal(34)]), literal(0))),
        block_expr(literal(35)),
        awaited(block_expr(binary(
            BinaryOperator::Add,
            literal(36),
            literal(37),
        ))),
        block_expr(block_expr(checked(vec![boolean(true)]))),
        block_expr_with_prelude(
            vec![printed(literal(38)), nowait("pending", literal(39))],
            variable("pending"),
        ),
        task_scope(
            "co",
            vec![
                printed(literal(40)),
                task_scope("task_scope", vec![nowait("task_result", literal(41))]),
                printed(variable("task_result")),
            ],
        ),
    ];

    for root in roots {
        let error = MirCompiler::with_options(false)
            .compile_with_source(root, Some("non-program-admission.hako"))
            .expect_err("public whole-file admission must reject non-Program root");
        assert_eq!(
            error,
            "[mir/normal-program-admission] selected normal/default source must produce Program"
        );
    }
}

#[test]
fn rejected_nonprogram_admission_leaves_live_builder_unchanged_and_reusable() {
    for root in [
        ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        },
        awaited(ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        }),
        checked(vec![
            boolean(true),
            ASTNode::Variable {
                name: "missing".to_owned(),
                span: Span::unknown(),
            },
            boolean(false),
        ]),
        printed(ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        }),
        nowait(
            "pending",
            ASTNode::Variable {
                name: "missing".to_owned(),
                span: Span::unknown(),
            },
        ),
        array(vec![
            literal(24),
            ASTNode::Variable {
                name: "missing".to_owned(),
                span: Span::unknown(),
            },
        ]),
        map(vec![
            ("before", literal(25)),
            (
                "missing",
                ASTNode::Variable {
                    name: "missing".to_owned(),
                    span: Span::unknown(),
                },
            ),
            ("after", literal(26)),
        ]),
        block_expr(ASTNode::Variable {
            name: "missing".to_owned(),
            span: Span::unknown(),
        }),
        block_expr_with_prelude(
            vec![printed(variable("missing")), nowait("pending", literal(40))],
            variable("pending"),
        ),
        task_scope(
            "co",
            vec![
                printed(variable("missing")),
                task_scope("task_scope", vec![printed(literal(42))]),
            ],
        ),
    ] {
        let mut compiler = MirCompiler::with_options(false);
        compiler.builder.set_source_file_hint("live-before.hako");
        let before = (source_file(&compiler), core_cursor(&compiler));
        let error = compiler
            .compile_with_source(root, Some("failed-nonprogram.hako"))
            .expect_err("public whole-file admission must reject before compilation");
        assert_eq!(
            error,
            "[mir/normal-program-admission] selected normal/default source must produce Program"
        );
        assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
        let result = compiler
            .compile_with_source(program(literal(23)), Some("reused-nonprogram.hako"))
            .expect("fresh selected root after failure");
        assert!(result.module.functions.contains_key("main"));
        assert_eq!(
            source_file(&compiler).as_deref(),
            Some("reused-nonprogram.hako")
        );
    }
}

#[test]
fn responsibility_local_nonprogram_roots_share_public_admission_and_reuse() {
    for (root, reuse) in [
        (
            local("x", Some(block_expr(literal(38)))),
            program(literal(39)),
        ),
        (
            grouped_assignment("missing", literal(31)),
            program(literal(32)),
        ),
        (
            indexed(literal(35), literal(0)),
            program(indexed(array(vec![literal(36)]), literal(0))),
        ),
    ] {
        let mut compiler = MirCompiler::with_options(false);
        compiler.builder.set_source_file_hint("live-before.hako");
        let before = (source_file(&compiler), core_cursor(&compiler));
        let error = compiler
            .compile_with_source(root, Some("nonprogram-root-failure.hako"))
            .expect_err("public whole-file admission must reject non-Program root");
        assert_eq!(
            error,
            "[mir/normal-program-admission] selected normal/default source must produce Program"
        );
        assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
        let result = compiler
            .compile_with_source(reuse, Some("nonprogram-root-reuse.hako"))
            .expect("fresh Program must compile after admission rejection");
        assert!(result.module.functions.contains_key("main"));
        assert_eq!(
            source_file(&compiler).as_deref(),
            Some("nonprogram-root-reuse.hako")
        );
    }
}

#[test]
fn finishing_rejects_optimizer_diagnostics_without_committing() {
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{Callee, EffectMask, MirInstruction};

    let mut compiler = MirCompiler::with_options(false);
    let mut module = compiler
        .compile_normal(normal_request(program(literal(7)), Some("already-live.hako"), HashMap::new()))
        .unwrap()
        .module;
    let before = (source_file(&compiler), core_cursor(&compiler));
    let main = module.functions.get_mut("main").unwrap();
    let entry = main.get_block_mut(main.entry_block).unwrap();
    let receiver = entry.instructions.iter().find_map(MirInstruction::dst_value).unwrap();
    entry.add_instruction_before_terminator(MirInstruction::LegacyCallV0 {
        dst: None,
        func: receiver,
        callee: Some(Callee::Method {
            box_name: "IntegerBox".into(),
            method: "is".into(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: Vec::new(),
        effects: EffectMask::IO,
    });
    compiler.optimize = true;
    let error = crate::test_support::with_env_vars(
        &[
            ("NYASH_OPT_DIAG_FAIL", Some("1")),
            ("NYASH_OPT_DIAG_FORBID_LEGACY", None),
            ("NYASH_MIR_DISABLE_OPT", None),
            ("HAKO_MIR_DISABLE_OPT", None),
            ("HAKO_JOINIR_STRICT", None),
            ("NYASH_JOINIR_STRICT", None),
            ("HAKO_JOINIR_PLANNER_REQUIRED", None),
        ],
        || compiler.finish_built_module(module, super::MirFinishScheduleV1::Legacy, &Default::default()),
    )
    .unwrap_err();
    assert!(error.starts_with("Diagnostic failure:"), "{error}");
    assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn finishing_rejects_an_orphan_static_plan_without_committing() {
    let mut compiler = MirCompiler::with_options(false);
    let mut module = compiler
        .compile_normal(normal_request(program(literal(7)), Some("already-live.hako"), HashMap::new()))
        .unwrap()
        .module;
    let before = (source_file(&compiler), core_cursor(&compiler));
    module.metadata.static_data_plans.push(crate::mir::function::StaticDataPlan {
        source_name: "ORPHAN".into(),
        symbol: ".hako.static.ORPHAN".into(),
        element: "u16".into(),
        align: 2,
        linkage: "private".into(),
        unnamed_addr: true,
        values: vec![1],
    });
    let error = compiler
        .finish_built_module(module, super::MirFinishScheduleV1::Legacy, &Default::default())
        .unwrap_err();
    assert!(error.contains("[type/static_table_contract_spec_missing]"), "{error}");
    assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
    assert!(compiler.builder.current_module.is_none());
}

#[test]
fn canonical_finish_rejects_an_invalid_target_before_publication() {
    let mut compiler = MirCompiler::with_options(false);
    let mut module = compiler
        .compile_normal(normal_request(program(literal(7)), Some("already-live.hako"), HashMap::new()))
        .unwrap()
        .module;
    let before = (source_file(&compiler), core_cursor(&compiler));
    let main = module.functions.get_mut("main").unwrap();
    main.get_block_mut(main.entry_block)
        .unwrap()
        .set_jump_with_edge_args(crate::mir::BasicBlockId::new(9999), None);
    let error = compiler
        .finish_built_canonical_module(module, super::CanonicalFinishScheduleV1::TrivialBindingSsa)
        .unwrap_err();
    assert!(matches!(error, super::CanonicalLoweringErrorV1::MirVerificationFailed { .. }), "{error}");
    assert_eq!((source_file(&compiler), core_cursor(&compiler)), before);
    assert!(compiler.builder.current_module.is_none());
}
