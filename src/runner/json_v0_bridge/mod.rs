mod ast;
mod core;
mod lexer;
mod lowering;

pub use core::{
    maybe_dump_mir, parse_json_v0_to_module, parse_json_v0_to_module_with_imports,
    parse_source_v0_to_module,
};

#[cfg(test)]
mod tests;
