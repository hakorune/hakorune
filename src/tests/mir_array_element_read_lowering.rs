use crate::mir::definitions::MirCall;
use crate::mir::{Callee, MirCompiler, MirInstruction};
use crate::parser::NyashParser;

/// `ArrayBox.get/1` on a proven array receiver is the sole physical read
/// owner — no residual `call_method` may remain for the same surface.
#[test]
fn array_get_lowers_to_explicit_read_operation() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = r#"
static box Main {
  main() {
    local values = [1]
    local first = values.get(0)
    return first
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

    let mut reads = 0usize;
    let mut residual = Vec::new();
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
    {
        match instruction {
            MirInstruction::ArrayElementRead { .. } => reads += 1,
            MirInstruction::LegacyCallV0 {
                callee:
                    Some(Callee::Method {
                        box_name, method, ..
                    }),
                ..
            }
            | MirInstruction::Call(MirCall {
                callee:
                    Callee::Method {
                        box_name, method, ..
                    },
                ..
            }) if box_name == "ArrayBox" && method == "get" => {
                residual.push(method.clone())
            }
            _ => {}
        }
    }
    assert_eq!(reads, 1, "expected exactly one array.read: {function:?}");
    assert!(residual.is_empty(), "residual Array get calls: {residual:?}");
}
