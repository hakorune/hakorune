//! Pass 3: Header PHI Construction
//!
//! This pass generates incomplete PHI nodes in the loop header block.
//! The PHI nodes initially have only the preheader input; the latch input
//! will be added in Pass 4 after the loop body is lowered.
//!
//! # Responsibilities
//!
//! 1. **Incomplete PHI Generation**: For each variable:
//!    - Emit: `header_phi = phi [preheader_copy, preheader]`
//!    - Update variable binding to header_phi
//!
//! 2. **Receiver Variable Aliasing** (Hotfix 7):
//!    - Handle `me` → `__pin$N$@recv` aliasing
//!    - Update all pin levels to point to same PHI
//!    - Prevents stale ValueId references in nested loops
//!
//! 3. **Deterministic Order**: Process in consistent order:
//!    - Pinned variables first
//!    - Carrier variables second
//!
//! # Why Incomplete PHIs?
//!
//! We cannot complete the PHI nodes at this point because:
//! - The loop body hasn't been lowered yet
//! - We don't know the latch values
//! - Continue statements may create additional predecessors
//!
//! Pass 4 will complete these PHIs after loop body construction.
//!
//! # Example
//!
//! ```ignore
//! // After Pass 2 (preheader):
//! // r3 = r1  (carrier preheader copy)
//! // r5 = r2  (pinned preheader copy)
//! // jump @header
//!
//! builder.emit_header_phis(&mut ops)?;
//!
//! // Generated MIR in header block:
//! // r6 = phi [r5, @preheader]     // pinned (incomplete)
//! // r4 = phi [r3, @preheader]     // carrier (incomplete)
//! // var[y] = r6
//! // var[x] = r4
//! ```

use crate::mir::phi_core::loopform::builder_core::{LoopFormBuilder, LoopFormOps};

/// Pass 3: Emit header block PHI nodes (incomplete)
///
/// Creates incomplete PHI nodes with only preheader input.
/// These will be completed in seal_phis() after loop body is lowered.
pub fn emit_header_phis<O: LoopFormOps>(
    builder: &mut LoopFormBuilder,
    ops: &mut O,
) -> Result<(), String> {
    ops.set_current_block(builder.header_id)?;

    // Emit PHIs for pinned variables
    for pinned in &builder.pinned {
        ops.emit_phi(
            pinned.header_phi,
            vec![(builder.preheader_id, pinned.preheader_copy)],
        )?;
        ops.update_var(pinned.name.clone(), pinned.header_phi);

        // 🔧 Hotfix 7 (Enhanced): Update aliases for pinned receiver variables
        // When a variable like "me" is pinned to "__pin$N$@recv", both names
        // must point to the same PHI value to avoid stale ValueId references.
        // This handles:
        // 1. Direct receiver: "me" → "__pin$1$@recv"
        // 2. Nested loops: "__pin$1$@recv" → "__pin$2$@recv"
        // 3. Multiple aliasing scenarios
        if pinned.name.contains("@recv") {
            // Always update "me" (the canonical receiver name)
            ops.update_var("me".to_string(), pinned.header_phi);

            // Also update all previous pin levels (__pin$1$@recv, __pin$2$@recv, etc.)
            // Extract the pin counter and update all lower levels
            if let Some(idx) = pinned.name.find("$") {
                if let Some(end_idx) = pinned.name[idx + 1..].find("$") {
                    if let Ok(counter) = pinned.name[idx + 1..idx + 1 + end_idx].parse::<u32>() {
                        // Update all previous pin levels (1 through counter-1)
                        for i in 1..counter {
                            let alias = format!("__pin${}$@recv", i);
                            ops.update_var(alias, pinned.header_phi);
                        }
                    }
                }
            }
        }
    }

    // Emit PHIs for carrier variables
    for carrier in &builder.carriers {
        ops.emit_phi(
            carrier.header_phi,
            vec![(builder.preheader_id, carrier.preheader_copy)],
        )?;
        ops.update_var(carrier.name.clone(), carrier.header_phi);
    }

    Ok(())
}
