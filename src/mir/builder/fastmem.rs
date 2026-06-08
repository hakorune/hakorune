//! FastMemory source-region lowering.
//!
//! This module is the narrow MIRBuilder owner for `fastmem Contract { ... }`.
//! It records side-table region metadata and emits `MemOp` instructions for
//! the v0 memory dialect. It does not choose page-map strategy, backend route,
//! product activation, or provider/replacement-front policy.

pub(crate) mod branch;
pub(crate) mod calls;
pub(crate) mod ops;

use super::{MirBuilder, ValueId};
use crate::ast::{ASTNode, Span};
use crate::mir::instruction::FastMemRegionId;

pub(in crate::mir::builder) fn build_fastmem_region(
    builder: &mut MirBuilder,
    contract: String,
    body: Vec<ASTNode>,
    span: Span,
) -> Result<ValueId, String> {
    let region = builder.register_fastmem_region(contract, span, body.len())?;
    builder.push_fastmem_region(region);
    let result = crate::mir::builder::stmts::block_stmt::build_block(builder, body);
    let _ = builder.pop_fastmem_region();
    result
}

#[cfg(test)]
mod tests;
