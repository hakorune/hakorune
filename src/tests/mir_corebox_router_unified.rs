use crate::ast::ASTNode;
use crate::mir::{Callee, MirCompiler, MirInstruction, MirModule, MirType};
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
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let ast: ASTNode = NyashParser::parse_from_string(src).expect("parse ok");
    let mut compiler = MirCompiler::with_options(false);
    compiler.compile(ast).expect("compile ok").module
}

fn method_call_arg_lens(module: &MirModule, box_name: &str, method: &str) -> Vec<usize> {
    let mut arg_lens = Vec::new();
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    callee:
                        Some(Callee::Method {
                            box_name: call_box,
                            method: call_method,
                            ..
                        }),
                    args,
                    ..
                } = inst
                else {
                    continue;
                };
                if call_box == box_name && call_method == method {
                    arg_lens.push(args.len());
                }
            }
        }
    }
    arg_lens
}

fn method_call_result_types(
    module: &MirModule,
    box_name: &str,
    method: &str,
) -> Vec<Option<MirType>> {
    let mut result_types = Vec::new();
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let MirInstruction::Call {
                    dst,
                    callee:
                        Some(Callee::Method {
                            box_name: call_box,
                            method: call_method,
                            ..
                        }),
                    ..
                } = inst
                else {
                    continue;
                };
                if call_box == box_name && call_method == method {
                    result_types
                        .push(dst.and_then(|dst| function.metadata.value_types.get(&dst).cloned()));
                }
            }
        }
    }
    result_types
}

#[path = "mir_corebox_router_unified/array.rs"]
mod array;
#[path = "mir_corebox_router_unified/map.rs"]
mod map;
#[path = "mir_corebox_router_unified/string.rs"]
mod string;
