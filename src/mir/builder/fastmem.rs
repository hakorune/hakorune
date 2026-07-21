//! FastMemory source-region lowering.
//!
//! This module is the narrow MIRBuilder owner for `fastmem Contract { ... }`.
//! It records side-table region metadata and emits `MemOp` instructions for
//! the v0 memory dialect. It does not choose page-map strategy, backend route,
//! product activation, or provider/replacement-front policy.

pub(crate) mod branch;
pub(crate) mod calls;
pub(in crate::mir::builder) mod field_load;
pub(crate) mod ops;
mod receipt;

use super::{MirBuilder, ValueId};
use crate::ast::{ASTNode, Span};
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_body_v1, RecursiveChildLoweringPortV1,
};

pub(in crate::mir::builder) fn build_fastmem_region(
    builder: &mut MirBuilder,
    contract: String,
    body: Vec<ASTNode>,
    span: Span,
) -> Result<ValueId, String> {
    let region = builder.register_fastmem_region(contract, span, body.len())?;
    builder.push_fastmem_region(region);
    let result = builder.build_block(body);
    let _ = builder.pop_fastmem_region();
    result
}

/// Port-aware fastmem body driver.  Region registration remains the sole
/// fastmem owner while recursive statements retain the caller's port.
pub(in crate::mir::builder) fn build_fastmem_region_with_port_v1<Port>(
    builder: &mut MirBuilder,
    child: &mut Port,
    contract: String,
    body: Vec<ASTNode>,
    span: Span,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<BodyInput = Vec<ASTNode>, StatementInput = ASTNode>,
{
    let region = builder.register_fastmem_region(contract, span, body.len())?;
    builder.push_fastmem_region(region);
    let result = drive_legacy_body_v1(builder, child, body);
    let _ = builder.pop_fastmem_region();
    result
}

#[cfg(test)]
mod tests;
