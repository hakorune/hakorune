use crate::backend::VM;
use crate::mir::{MirCompiler, MirInstruction};
use crate::parser::NyashParser;

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| {
        init_global_ring0(default_ring0());
    });
}

fn compile_static_table_source(source: &str) -> crate::mir::MirModule {
    ensure_ring0_initialized();
    let ast = NyashParser::parse_from_string(source).expect("parse static table source");
    let mut compiler = MirCompiler::with_options(false);
    compiler
        .compile(ast)
        .expect("compile static table source")
        .module
}

#[test]
fn static_const_table_load_lowers_to_mir_json_and_vm_value() {
    let source = r#"
static const SIZE_CLASS: u16[] = [8, 16, 24, 32]
static box Main {
  main() {
    return SIZE_CLASS[2]
  }
}
"#;

    let module = compile_static_table_source(source);
    assert_eq!(module.metadata.static_data_plans.len(), 1);

    let main = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("main"))
        .expect("main function");
    let has_static_load = main.blocks.values().any(|block| {
        block.instructions.iter().any(|inst| {
            matches!(
                inst,
                MirInstruction::StaticDataLoad {
                    source_name,
                    element,
                    len,
                    ..
                } if source_name == "SIZE_CLASS" && element == "u16" && *len == 4
            )
        })
    });
    assert!(has_static_load, "expected StaticDataLoad instruction");

    let json = crate::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(&module)
        .expect("emit mir json");
    assert!(json.contains("\"op\":\"static_data_load\""), "{json}");
    assert!(
        json.contains("\"symbol\":\".hako.static.SIZE_CLASS\""),
        "{json}"
    );
    assert!(json.contains("\"static_data_plans\""), "{json}");

    let mut vm = VM::new();
    let out = vm.execute_module(&module).expect("vm exec");
    assert_eq!(out.to_string_box().value, "24");
}

#[test]
fn static_const_table_const_exprs_flow_to_load_value() {
    let source = r#"
static const SIZE_CLASS: u16[] = [8 + 8, 3 * 8, 1 << 5, (40 - 8) | 1]
static box Main {
  main() {
    return SIZE_CLASS[3]
  }
}
"#;

    let module = compile_static_table_source(source);
    let plan = module
        .metadata
        .static_data_plans
        .first()
        .expect("static data plan");
    assert_eq!(plan.values, vec![16, 24, 32, 33]);

    let mut vm = VM::new();
    let out = vm.execute_module(&module).expect("vm exec");
    assert_eq!(out.to_string_box().value, "33");
}

#[test]
fn static_const_table_load_vm_rejects_out_of_range_index() {
    let source = r#"
static const SIZE_CLASS: u16[] = [8, 16, 24, 32]
static box Main {
  main() {
    return SIZE_CLASS[4]
  }
}
"#;

    let module = compile_static_table_source(source);
    let mut vm = VM::new();
    let err = vm
        .execute_module(&module)
        .expect_err("out-of-range static table load should fail");
    assert!(
        err.to_string()
            .contains("[static-const/load-index-out-of-range]"),
        "{err}"
    );
}
