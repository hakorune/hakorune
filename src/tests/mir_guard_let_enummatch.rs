use crate::ast::ASTNode;
use crate::mir::instruction::MirInstruction;
use crate::mir::MirCompiler;
use crate::parser::NyashParser;

fn compile_source(source: &str) -> crate::mir::MirModule {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast: ASTNode = NyashParser::parse_from_string(source).expect("parse guard-let source");
    let mut compiler = MirCompiler::with_options(false);
    compiler
        .compile(ast)
        .expect("compile guard-let enum match source")
        .module
}

#[test]
fn direct_mir_guard_let_result_ok_lowers_enum_match_shape() {
    let source = r#"
static box Main {
  make(value): Result<i64, i64> {
    if value > 0 {
      return Result::Ok(value)
    }
    return Result::Err(7)
  }

  main(args) {
    local result: Result<i64, i64> = Main.make(3)
    guard let Result::Ok(value) = result else {
      return 1
    }
    return value
  }
}
"#;

    let module = compile_source(source);
    assert!(
        module.metadata.enum_decls.contains_key("Result"),
        "direct MIR builder should publish prelude Result enum metadata"
    );

    let has_variant_make = module.functions.values().any(|function| {
        function.blocks.values().any(|block| {
            block
                .all_instructions()
                .any(|inst| matches!(inst, MirInstruction::VariantMake { enum_name, .. } if enum_name == "Result"))
        })
    });
    assert!(
        has_variant_make,
        "Result::Ok/Err should lower to VariantMake"
    );

    let main = module.functions.get("main").expect("main function");
    let has_variant_tag = main.blocks.values().any(|block| {
        block
            .all_instructions()
            .any(|inst| matches!(inst, MirInstruction::VariantTag { enum_name, .. } if enum_name == "Result"))
    });
    assert!(
        has_variant_tag,
        "guard-let failure test should lower through VariantTag"
    );

    let has_variant_project = main.blocks.values().any(|block| {
        block
            .all_instructions()
            .any(|inst| matches!(inst, MirInstruction::VariantProject { enum_name, variant, .. } if enum_name == "Result" && variant == "Ok"))
    });
    assert!(
        has_variant_project,
        "guard-let binding should lower through VariantProject"
    );
}
