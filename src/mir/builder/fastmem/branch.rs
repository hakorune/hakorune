//! FastMemory branch lowering.
//!
//! This module owns the narrow CFG shape allowed inside `fastmem` regions:
//! an `if/else` split whose condition is a region-local `OwnerEq` MemOp.

use crate::mir::builder::MirBuilder;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::ValueId;
use std::collections::HashSet;

pub(crate) fn ensure_fastmem_owner_eq_condition(
    builder: &MirBuilder,
    region: FastMemRegionId,
    condition_value: ValueId,
) -> Result<(), String> {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
    let mut visited = HashSet::new();
    let is_owner_eq = is_owner_eq_value(function, region, condition_value, &mut visited);
    if is_owner_eq {
        Ok(())
    } else {
        Err("[freeze:contract][fastmem/branch_cfg_requires_owner_eq_condition]".to_string())
    }
}

fn is_owner_eq_value(
    function: &crate::mir::MirFunction,
    region: FastMemRegionId,
    value: ValueId,
    visited: &mut HashSet<ValueId>,
) -> bool {
    if !visited.insert(value) {
        return false;
    }

    for block in function.blocks.values() {
        for instruction in &block.instructions {
            match instruction {
                crate::mir::MirInstruction::MemOp {
                    region: actual_region,
                    kind: MemOpKind::OwnerEq,
                    dst: Some(dst),
                    ..
                } if *actual_region == region && *dst == value => {
                    return true;
                }
                crate::mir::MirInstruction::Copy { dst, src } if *dst == value => {
                    if is_owner_eq_value(function, region, *src, visited) {
                        return true;
                    }
                }
                _ => {}
            }
        }
    }

    false
}
