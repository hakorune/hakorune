use crate::mir::{MirCompiler, MirInstruction, MirModule};
use crate::parser::NyashParser;

fn compile_src(src: &str) -> MirModule {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let mut compiler = MirCompiler::with_options(false);
    compiler.compile(ast).expect("compile ok").module
}

fn count_newbox(module: &MirModule, box_type: &str) -> usize {
    module
        .functions
        .values()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
        .filter(|inst| {
            matches!(
                inst,
                MirInstruction::NewBox {
                    box_type: inst_box,
                    ..
                } if inst_box == box_type
            )
        })
        .count()
}

fn count_field_sets(module: &MirModule, field_name: &str) -> usize {
    module
        .functions
        .values()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
        .filter(|inst| {
            matches!(
                inst,
                MirInstruction::FieldSet { field, .. } if field == field_name
            )
        })
        .count()
}

#[test]
fn box_new_field_initializer_lowers_to_newbox_then_field_sets() {
    let module = compile_src(
        r#"
box Report {
  accepted: i64
  reason: i64
}

static box Main {
  main() {
    local report = new Report { accepted: 1, reason: 2 }
    return report
  }
}
"#,
    );

    assert_eq!(count_newbox(&module, "Report"), 1);
    assert_eq!(count_field_sets(&module, "accepted"), 1);
    assert_eq!(count_field_sets(&module, "reason"), 1);
}

#[test]
fn box_new_field_initializer_rejects_duplicate_field() {
    let ast = NyashParser::parse_from_string(
        r#"
box Report {
  accepted: i64
}

static box Main {
  main() {
    local report = new Report { accepted: 1, accepted: 2 }
    return report
  }
}
"#,
    )
    .expect("parse ok");
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut compiler = MirCompiler::with_options(false);
    let err = compiler
        .compile(ast)
        .expect_err("duplicate field should fail");
    assert!(err.contains("[box-init/duplicate-field]"), "{err}");
}
