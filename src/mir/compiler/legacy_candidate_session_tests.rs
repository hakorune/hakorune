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
            function staged() { return 1 }
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
        .expect_err("undefined runtime variable must reject the candidate");

    assert!(error.contains("Undefined variable: missing"), "{error}");
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
box Page {
  capacity: usize = 0
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
    let legacy = legacy_compiler
        .compile_with_source(legacy_ast, Some("numeric-parity.hako"))
        .expect("legacy compatibility module");
    let mut compiler = MirCompiler::with_options(false);
    let candidate = compiler
        .compile_normal(normal_request(
            candidate_ast,
            Some("numeric-parity.hako"),
            HashMap::new(),
        ))
        .expect("normal candidate");

    assert_eq!(
        candidate.module.metadata.user_box_field_decls,
        legacy.module.metadata.user_box_field_decls
    );
    assert_eq!(
        MirPrinter::new().print_module(&candidate.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(
        format!("{:?}", candidate.verification_result),
        format!("{:?}", legacy.verification_result)
    );
    let contract_count = |module: &crate::mir::MirModule| {
        module
            .functions
            .values()
            .map(|function| {
                function
                    .metadata
                    .exact_numeric_runtime_check_contracts
                    .len()
            })
            .sum::<usize>()
    };
    let main_instructions = legacy.module.functions["main"]
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect::<Vec<_>>();
    assert_eq!(
        contract_count(&legacy.module),
        1,
        "main instructions: {main_instructions:#?}"
    );
    assert_eq!(
        contract_count(&candidate.module),
        contract_count(&legacy.module)
    );

    let optimized_legacy_ast = NyashParser::parse_from_string(source).expect("legacy source");
    let program_v0_ast = NyashParser::parse_from_string(source).expect("Program-v0 source");
    let mut optimized_legacy_compiler = MirCompiler::with_options(true);
    let optimized_legacy = optimized_legacy_compiler
        .compile_with_source(optimized_legacy_ast, Some("<json_v0/imports>"))
        .expect("optimized legacy compatibility module");
    let mut program_v0_compiler = MirCompiler::with_options(true);
    let program_v0 = program_v0_compiler
        .compile_normal(program_v0_import_bundle_request(program_v0_ast))
        .expect("typed Program-v0 import bundle");

    assert_eq!(
        MirPrinter::new().print_module(&program_v0.module),
        MirPrinter::new().print_module(&optimized_legacy.module)
    );
    assert_eq!(
        program_v0.module.metadata.user_box_field_decls,
        optimized_legacy.module.metadata.user_box_field_decls
    );
    assert_eq!(
        format!("{:?}", program_v0.verification_result),
        format!("{:?}", optimized_legacy.verification_result)
    );
    assert!(program_v0_compiler
        .builder
        .comp_ctx
        .using_import_boxes
        .is_empty());
    assert_eq!(
        source_file(&program_v0_compiler).as_deref(),
        Some("<json_v0/imports>")
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
