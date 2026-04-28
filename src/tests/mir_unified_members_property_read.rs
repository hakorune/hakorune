use crate::ast::ASTNode;
use crate::mir::{Callee, MirCompiler, MirInstruction, MirModule};
use crate::parser::NyashParser;

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        std::env::set_var(key, value);
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(value) => std::env::set_var(self.key, value),
            None => std::env::remove_var(self.key),
        }
    }
}

fn compile_src(src: &str) -> MirModule {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    let _unified_members = EnvGuard::set("NYASH_ENABLE_UNIFIED_MEMBERS", "1");
    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
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

fn count_getter_calls(module: &MirModule, method_name: &str) -> usize {
    let global_suffix = format!(".{}/0", method_name);
    module
        .functions
        .values()
        .flat_map(|function| function.blocks.values())
        .flat_map(|block| block.instructions.iter())
        .filter(|inst| {
            let MirInstruction::Call { callee, .. } = inst else {
                return false;
            };
            match callee {
                Some(Callee::Method { method, .. }) => method == method_name,
                Some(Callee::Global(name)) => name.ends_with(&global_suffix),
                _ => false,
            }
        })
        .count()
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

    assert_eq!(count_newbox(&module, box_type), 1);
    assert_eq!(count_getter_calls(&module, getter_name), 1);
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
