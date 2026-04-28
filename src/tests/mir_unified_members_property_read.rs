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

#[test]
fn property_read_on_newbox_reuses_lowered_receiver() {
    let module = compile_src(
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
    );

    assert_eq!(count_newbox(&module, "PropBox"), 1);
    assert_eq!(count_getter_calls(&module, "__get_value"), 1);
    assert_eq!(count_field_gets(&module, "value"), 0);
}
