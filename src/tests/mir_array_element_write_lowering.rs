use crate::mir::{ArrayElementWriteKind, Callee, MirCompiler, MirInstruction};
use crate::parser::NyashParser;

#[test]
fn all_static_array_write_surfaces_lower_to_explicit_operation() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = r#"
static box Main {
  main() {
    local values = [1]
    values.push(2)
    values.set(0, 3)
    values.insert(0, 4)
    values[0] = 5
    values[0] += 1
    return values.length()
  }
}
"#;
    let ast = NyashParser::parse_from_string(source).unwrap();
    let mut compiler = MirCompiler::with_options(false);
    let module = compiler.compile(ast).unwrap().module;
    let function = module
        .functions
        .values()
        .find(|function| function.signature.name.contains("main"))
        .unwrap();

    let mut kinds = Vec::new();
    let mut residual = Vec::new();
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
    {
        match instruction {
            MirInstruction::ArrayElementWrite { kind, .. } => kinds.push(*kind),
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name, method, ..
                    }),
                ..
            } if box_name == "ArrayBox" && matches!(method.as_str(), "push" | "set" | "insert") => {
                residual.push(method.clone())
            }
            _ => {}
        }
    }
    assert_eq!(
        kinds,
        vec![
            ArrayElementWriteKind::LiteralAppend,
            ArrayElementWriteKind::Push,
            ArrayElementWriteKind::Set,
            ArrayElementWriteKind::Insert,
            ArrayElementWriteKind::Set,
            ArrayElementWriteKind::Set,
        ]
    );
    assert!(
        residual.is_empty(),
        "residual Array write calls: {residual:?}"
    );
}
