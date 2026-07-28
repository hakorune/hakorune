use crate::ast::ASTNode;
use crate::mir::{Callee, MirCompiler, MirInstruction, MirModule, ValueId};
use crate::parser::NyashParser;

fn compile_src(src: &str) -> MirModule {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::tests::helpers::env::with_env_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1", || {
        let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
        let mut compiler = MirCompiler::with_options(false);
        compiler.compile(ast).expect("compile ok").module
    })
}

fn copy_root(instructions: &[&MirInstruction], mut value: ValueId) -> ValueId {
    for _ in 0..instructions.len() {
        let Some(src) = instructions
            .iter()
            .find_map(|instruction| match instruction {
                MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            })
        else {
            break;
        };
        value = src;
    }
    value
}

fn count_field_gets(module: &MirModule, field_name: &str) -> usize {
    module
        .functions
        .values()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
        .filter(|inst| {
            matches!(
                inst,
                MirInstruction::FieldGet { field, .. } if field == field_name
            )
        })
        .count()
}

fn assert_property_read_uses_getter(
    src: &str,
    box_type: &str,
    property_name: &str,
    getter_name: &str,
) {
    let module = compile_src(src);
    let function = module
        .functions
        .values()
        .find(|function| {
            function.blocks.values().any(|block| {
                block.instructions.iter().any(|instruction| {
                    matches!(
                        instruction,
                        MirInstruction::NewBox { box_type: ty, .. } if ty == box_type
                    )
                })
            })
        })
        .expect("function containing property receiver");
    let instructions = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect::<Vec<_>>();
    let newboxes = instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::NewBox {
                dst, box_type: ty, ..
            } if ty == box_type => Some(*dst),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(newboxes.len(), 1);
    let global_suffix = format!(".{getter_name}/0");
    let getter_receivers = instructions
        .iter()
        .filter_map(|instruction| match instruction {
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } if method == getter_name && args.len() == 1 => Some((*receiver, args[0])),
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                args,
                ..
            } if name.ends_with(&global_suffix) && args.len() == 1 => Some((args[0], args[0])),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(getter_receivers.len(), 1);
    assert_eq!(copy_root(&instructions, getter_receivers[0].0), newboxes[0]);
    assert_eq!(copy_root(&instructions, getter_receivers[0].1), newboxes[0]);
    assert_eq!(count_field_gets(&module, property_name), 0);
}

#[test]
fn property_read_on_newbox_reuses_lowered_receiver() {
    assert_property_read_uses_getter(
        r#"
box PropBox {
  get value: IntegerBox => 42
}

static box Main {
  main() {
    return (new PropBox()).value
  }
}
"#,
        "PropBox",
        "value",
        "__get_value",
    );
}

#[test]
fn once_property_read_uses_once_getter() {
    assert_property_read_uses_getter(
        r#"
box PropBox {
  once cached: IntegerBox => 7
}

static box Main {
  main() {
    return (new PropBox()).cached
  }
}
"#,
        "PropBox",
        "cached",
        "__get_once_cached",
    );
}

#[test]
fn birth_once_property_read_uses_birth_getter() {
    assert_property_read_uses_getter(
        r#"
box PropBox {
  birth_once config: IntegerBox => 9
}

static box Main {
  main() {
    return (new PropBox()).config
  }
}
"#,
        "PropBox",
        "config",
        "__get_birth_config",
    );
}
