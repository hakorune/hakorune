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

#[test]
fn record_exact_numeric_field_rejects_dynamic_wrong_value_before_publish() {
    let module = compile_src(
        r#"
record ExactFields {
  value: i64
}

static box Main {
  main() {
    local dynamic = "bad"
    local fields = ExactFields { value: dynamic }
    return fields.value
  }
}
"#,
    );

    let mut vm = VM::new();
    let error = vm.execute_module(&module).unwrap_err().to_string();
    assert!(
        error.contains("[type/record_contract_field_runtime_mismatch]"),
        "{error}"
    );
}

#[test]
fn record_with_update_rechecks_unchanged_and_updated_fields() {
    let module = compile_src(
        r#"
record Pair {
  left: i64
  right: i64
}

static box Main {
  main() {
    local dynamic = "bad"
    local pair = Pair { left: 1, right: 2 }
    local next = pair with { right: dynamic }
    return next.left
  }
}
"#,
    );

    let mut vm = VM::new();
    let error = vm.execute_module(&module).unwrap_err().to_string();
    assert!(
        error.contains("[type/record_contract_field_runtime_mismatch]"),
        "{error}"
    );
}

#[test]
fn record_literal_checks_explicit_fields_in_source_order() {
    let module = compile_src(
        r#"
record Pair {
  left: i64
  right: i64
}

static box Main {
  main() {
    local pair = Pair { right: 2, left: 1 }
    return pair.left
  }
}
"#,
    );
    let main = module.functions.get("main").expect("main function");
    let field_order = main
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::RecordFieldContractCheck { field_index, .. } => Some(*field_index),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(field_order, vec![1, 0]);
}

#[test]
fn record_literal_unknown_field_preflights_before_expression_lowering() {
    let source = r#"
record Point {
  x: i64
}

static box Main {
  effect() { return 1 }
  main() {
    local point = Point { missing: Main.effect() }
    return point.x
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).expect("parse unknown field fixture");
    let mut compiler = MirCompiler::with_options(false);
    let error = compiler.compile(ast).unwrap_err();
    assert!(
        error.contains("[type/record_contract_unknown_field]"),
        "{error}"
    );
}

#[test]
fn record_literal_evaluates_explicit_fields_before_declaration_order_defaults() {
    let module = compile_src(
        r#"
record Pair {
  left: i64 = 1
  right: i64
}

static box Main {
  main() {
    local pair = Pair { right: 2 }
    return pair.left
  }
}
"#,
    );
    let main = module.functions.get("main").expect("main function");
    let field_order = main
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::RecordFieldContractCheck { field_index, .. } => Some(*field_index),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(field_order, vec![1, 0]);
}
