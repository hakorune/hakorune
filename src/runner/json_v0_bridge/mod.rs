mod ast;
mod core;
mod lexer;
mod lowering;
mod source_anchor;

pub(crate) use source_anchor::Stage1ProgramJsonCallAnchorReceiptV1;

pub(crate) use core::parse_json_v0_to_module_with_source_anchors;
pub use core::{
    maybe_dump_mir, parse_json_v0_to_module, parse_json_v0_to_module_with_imports,
    parse_source_v0_to_module,
};

#[cfg(test)]
mod tests;
