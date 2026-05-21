use crate::backend::VM;
use crate::box_trait::IntegerBox;
use crate::mir::{MirCompiler, MirInstruction, MirModule};
use crate::parser::NyashParser;

fn compile_src(src: &str) -> MirModule {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
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

#[test]
fn record_construction_ergonomics_defaults_shorthand_and_with_run_without_newbox() {
    let module = compile_src(
        r#"
record ReportFields {
  accepted: i64 = 0
  reason: i64 = 2
  count: i64 = 0
}

static box Main {
  main() {
    local reason = 3
    local fields = ReportFields { reason }
    local next = fields with { count: 4 }
    return next.accepted + next.reason + next.count
  }
}
"#,
    );

    assert_eq!(
        count_newbox(&module, "ReportFields"),
        0,
        "record construction must remain builder-local"
    );

    let mut vm = VM::new();
    let out = vm.execute_module(&module).expect("vm exec");
    let Some(value) = out.as_any().downcast_ref::<IntegerBox>() else {
        panic!("expected IntegerBox result, got {out:?}");
    };
    assert_eq!(value.value, 7);
}

#[test]
fn record_construction_ergonomics_empty_literal_uses_all_defaults() {
    let module = compile_src(
        r#"
record ReportFields {
  accepted: i64 = 1
  reason: i64 = 2
  count: i64 = 3
}

static box Main {
  main() {
    local fields = ReportFields {}
    return fields.accepted + fields.reason + fields.count
  }
}
"#,
    );

    assert_eq!(count_newbox(&module, "ReportFields"), 0);

    let mut vm = VM::new();
    let out = vm.execute_module(&module).expect("vm exec");
    let Some(value) = out.as_any().downcast_ref::<IntegerBox>() else {
        panic!("expected IntegerBox result, got {out:?}");
    };
    assert_eq!(value.value, 6);
}
