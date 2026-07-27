use std::collections::HashMap;

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::MirCompiler;
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

#[test]
fn late_legacy_lowering_failure_leaves_live_builder_unchanged_and_reusable() {
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
    compiler.builder.set_source_file_hint("live-before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();

    let before = (
        compiler.builder.repl_mode,
        compiler.builder.comp_ctx.quiet_internal_logs,
        compiler.builder.comp_ctx.using_import_boxes.clone(),
        source_file(&compiler),
        core_cursor(&compiler),
    );
    let error = compiler
        .compile_with_source(root, Some("failed-candidate.hako"))
        .expect_err("undefined runtime variable must reject the candidate");

    assert!(error.contains("Undefined variable: missing"), "{error}");
    assert_eq!(
        (
            compiler.builder.repl_mode,
            compiler.builder.comp_ctx.quiet_internal_logs,
            compiler.builder.comp_ctx.using_import_boxes.clone(),
            source_file(&compiler),
            core_cursor(&compiler),
        ),
        before
    );
    assert!(compiler.builder.current_module.is_none());

    let result = compiler
        .compile_with_source(literal(7), Some("reused.hako"))
        .expect("fresh candidate after failure");
    assert!(result.module.functions.contains_key("main"));
    assert!(compiler.builder.comp_ctx.using_import_boxes.is_empty());
    assert_eq!(source_file(&compiler).as_deref(), Some("reused.hako"));
}

#[test]
fn explicit_imports_commit_only_with_the_finished_legacy_candidate() {
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
        .compile_with_source_and_imports(
            literal(11),
            Some("explicit-imports.hako"),
            imports.clone(),
        )
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
fn isolated_candidate_keeps_direct_legacy_numeric_contract_inputs() {
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
    let direct_ast = NyashParser::parse_from_string(source).expect("direct source");
    let candidate_ast = NyashParser::parse_from_string(source).expect("candidate source");
    let mut direct_compiler = MirCompiler::with_options(false);
    direct_compiler
        .builder
        .set_source_file_hint("numeric-direct.hako");
    let direct = direct_compiler
        .builder
        .build_module(direct_ast)
        .expect("direct module");
    let direct = direct_compiler
        .finish_built_module(direct, super::MirFinishScheduleV1::Legacy)
        .expect("direct finish")
        .module;
    let mut compiler = MirCompiler::with_options(false);
    let candidate = compiler
        .compile_with_source(candidate_ast, Some("numeric-candidate.hako"))
        .expect("candidate module")
        .module;

    assert_eq!(
        candidate.metadata.user_box_field_decls,
        direct.metadata.user_box_field_decls
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
    let main_instructions = direct.functions["main"]
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect::<Vec<_>>();
    assert_eq!(
        contract_count(&direct),
        1,
        "main instructions: {main_instructions:#?}"
    );
    assert_eq!(contract_count(&candidate), contract_count(&direct));
}
