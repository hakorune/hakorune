use std::collections::HashMap;

use crate::ast::{ASTNode, LiteralValue, Span};
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

fn source_file(compiler: &MirCompiler) -> Option<String> {
    compiler.builder.current_source_file()
}

fn normal_request(
    ast: ASTNode,
    source_file: Option<&str>,
    imports: HashMap<String, String>,
) -> NormalCompileRequestV1 {
    NormalCompileRequestV1::for_mir_mode(ast, source_file, imports)
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
            literal(7),
            Some("reused.hako"),
            HashMap::new(),
        ))
        .expect("fresh candidate after failure");
    assert!(result.module.functions.contains_key("main"));
    assert!(compiler.builder.comp_ctx.using_import_boxes.is_empty());
    assert_eq!(source_file(&compiler).as_deref(), Some("reused.hako"));
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
            literal(11),
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
}

#[test]
fn normal_pipeline_matches_legacy_compatibility_for_non_program_root() {
    let mut legacy_compiler = MirCompiler::with_options(false);
    let legacy = legacy_compiler
        .compile_with_source(literal(17), Some("non-program-parity.hako"))
        .expect("legacy non-Program root");
    let mut compiler = MirCompiler::with_options(false);
    let candidate = compiler
        .compile_normal(normal_request(
            literal(17),
            Some("non-program-parity.hako"),
            HashMap::new(),
        ))
        .expect("normal non-Program root");

    assert_eq!(
        MirPrinter::new().print_module(&candidate.module),
        MirPrinter::new().print_module(&legacy.module)
    );
    assert_eq!(
        format!("{:?}", candidate.verification_result),
        format!("{:?}", legacy.verification_result)
    );
    assert_eq!(
        candidate.module.metadata.source_file,
        legacy.module.metadata.source_file
    );
}
