//! Pass 2: Preheader Copy Generation
//!
//! This pass emits copy instructions in the preheader block for all variables
//! that participate in the loop. The copies serve as initial values for the
//! loop header PHI nodes.
//!
//! # Responsibilities
//!
//! 1. **Deterministic Ordering**: Emit copies in consistent order:
//!    - Pinned variables first
//!    - Carrier variables second
//!
//! 2. **Copy Emission**: For each variable:
//!    - Emit: `preheader_copy = init_value`
//!    - This creates a stable definition before the loop header
//!
//! 3. **Jump Emission**: Emit jump to loop header block
//!
//! # Why Preheader Copies?
//!
//! The preheader copies serve multiple purposes:
//! - **SSA Definition**: Ensure every loop variable has a single definition point
//! - **PHI Input**: Provide first input to header PHI nodes
//! - **Determinism**: Consistent ValueId allocation across compilations
//!
//! # Example
//!
//! ```ignore
//! // After Pass 1:
//! // - carriers: [{ name: "x", init: 1, copy: 3, phi: 4 }]
//! // - pinned: [{ name: "y", param: 2, copy: 5, phi: 6 }]
//!
//! builder.emit_preheader(&mut ops)?;
//!
//! // Generated MIR in preheader block:
//! // r5 = r2   // pinned first
//! // r3 = r1   // carriers second
//! // jump @header
//! ```

use crate::mir::phi_core::loopform::builder_core::{LoopFormBuilder, LoopFormOps};

/// Pass 2: Emit preheader block instructions
///
/// Emits copy instructions for ALL variables in deterministic order:
/// 1. Pinned variables first
/// 2. Carrier variables second
///
/// This ordering ensures consistent ValueId allocation across runs.
pub fn emit_preheader<O: LoopFormOps>(
    builder: &LoopFormBuilder,
    ops: &mut O,
) -> Result<(), String> {
    ops.set_current_block(builder.preheader_id)?;

    // Emit copies for pinned variables
    for pinned in &builder.pinned {
        ops.emit_copy(pinned.preheader_copy, pinned.param_value)?;
    }

    // Emit copies for carrier variables
    for carrier in &builder.carriers {
        ops.emit_copy(carrier.preheader_copy, carrier.init_value)?;
    }

    // Jump to header
    ops.emit_jump(builder.header_id)?;

    Ok(())
}
