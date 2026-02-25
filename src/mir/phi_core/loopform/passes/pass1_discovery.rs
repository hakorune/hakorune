//! Pass 1: Variable Discovery & ValueId Allocation
//!
//! This is the critical innovation of the LoopForm system: we allocate ALL
//! ValueIds (preheader copies and header PHIs) BEFORE emitting any instructions.
//! This guarantees definition-before-use in SSA form and eliminates circular
//! dependency issues.
//!
//! # Responsibilities
//!
//! 1. **Variable Classification**: Separate variables into:
//!    - **Carriers**: Modified in loop body, need header/exit PHI nodes
//!    - **Pinned**: Loop-invariant parameters, need PHI for SSA form
//!
//! 2. **ValueId Pre-allocation**: Allocate all ValueIds upfront:
//!    - `preheader_copy`: Copy instruction in preheader
//!    - `header_phi`: PHI node in loop header
//!
//! 3. **Counter Management**: Ensure MirFunction counter is ahead of all
//!    existing ValueIds to prevent collisions
//!
//! 4. **GUARD Protection**: Skip loop construction if invalid ValueIds detected
//!
//! # Example
//!
//! ```ignore
//! // Input variables: {x: ValueId(1), y: ValueId(2)}
//! builder.prepare_structure(&mut ops, &current_vars)?;
//!
//! // After pass 1:
//! // - carriers: [CarrierVariable { name: "x", init: 1, copy: 3, phi: 4 }]
//! // - pinned: [PinnedVariable { name: "y", param: 2, copy: 5, phi: 6 }]
//! ```

use crate::mir::ValueId;
use std::collections::BTreeMap;

use crate::mir::phi_core::loopform::builder_core::{LoopFormBuilder, LoopFormOps};
use crate::mir::phi_core::loopform::variable_models::{CarrierVariable, PinnedVariable};

/// Pass 1: Allocate all ValueIds for loop structure
///
/// This is the critical innovation: we allocate ALL ValueIds
/// (preheader copies and header PHIs) BEFORE emitting any instructions.
/// This guarantees definition-before-use in SSA form.
pub fn prepare_structure<O: LoopFormOps>(
    builder: &mut LoopFormBuilder,
    ops: &mut O,
    current_vars: &BTreeMap<String, ValueId>,
) -> Result<(), String> {
    let debug_enabled = std::env::var("NYASH_LOOPFORM_DEBUG").is_ok();

    // Step 5-2: Save preheader snapshot for ValueId comparison in seal_phis()
    builder.preheader_vars = current_vars.clone();

    if debug_enabled {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[loopform/prepare] === START prepare_structure() === {} variables",
            current_vars.len()
        ));
        crate::runtime::get_global_ring0()
            .log
            .debug("[loopform/prepare] Full variable list:");
        let mut sorted_vars: Vec<_> = current_vars.iter().collect();
        sorted_vars.sort_by_key(|(name, _)| name.as_str());
        for (name, value) in &sorted_vars {
            crate::runtime::get_global_ring0()
                .log
                .debug(&format!("[loopform/prepare]   - {} = {:?}", name, value));
        }
    }

    // GUARD: Detect invalid ValueId in variable map
    // ValueId::INVALID (u32::MAX) indicates uninitialized variables
    // Skip this loop construction attempt if detected (likely a premature build)
    for (name, &value) in current_vars.iter() {
        if value == ValueId::INVALID {
            if debug_enabled {
                crate::runtime::get_global_ring0().log.debug(&format!("[loopform/prepare] ⚠️ GUARD: Skipping loop preparation due to invalid ValueId for variable '{}'", name));
                crate::runtime::get_global_ring0().log.debug("[loopform/prepare] This indicates the loop is being built prematurely before variables are defined");
                crate::runtime::get_global_ring0().log.debug("[loopform/prepare] Returning Ok(()) to allow retry with properly initialized variables");
            }
            // Return Ok to skip this attempt without failing the entire compilation
            return Ok(());
        }
    }

    // CRITICAL FIX: Ensure MirFunction counter is ahead of all existing ValueIds
    // Without this, new_value() can return ValueIds that are already in use
    let max_existing_id = current_vars.values().map(|v| v.0).max().unwrap_or(0);

    if debug_enabled {
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[loopform/prepare] Calling ensure_counter_after(max_existing_id={})",
            max_existing_id
        ));
    }

    ops.ensure_counter_after(max_existing_id)?;

    // Count variables by classification for summary
    let mut param_count = 0;
    let mut carrier_count = 0;

    // Separate variables into carriers and pinned based on parameter status
    for (name, &value) in current_vars.iter() {
        // Step 5-3: Skip __pin$ temporary variables (BodyLocalInternal)
        // These are compiler-generated temporaries that should not have PHI nodes
        if name.starts_with("__pin$") && name.contains("$@") {
            if debug_enabled {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/prepare] SKIP __pin$ variable: {}",
                    name
                ));
            }
            continue;
        }

        // Phase 26-A-4: ValueIdベース判定に変更（名前ベース → 型安全）
        if ops.is_parameter(value) {
            param_count += 1;
            // Pinned variable (parameter, not modified in loop)
            let pinned = PinnedVariable {
                name: name.clone(),
                param_value: value,
                preheader_copy: ops.new_value(), // Allocate NOW
                header_phi: ops.new_value(),     // Allocate NOW
            };
            if debug_enabled {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/prepare] PINNED: {} -> init={:?}, copy={:?}, phi={:?}",
                    name, value, pinned.preheader_copy, pinned.header_phi
                ));
            }
            builder.pinned.push(pinned);
        } else {
            carrier_count += 1;
            // Carrier variable (local, modified in loop)
            let carrier = CarrierVariable {
                name: name.clone(),
                init_value: value,
                preheader_copy: ops.new_value(), // Allocate NOW
                header_phi: ops.new_value(),     // Allocate NOW
                latch_value: ValueId(0),         // Will be set during seal (placeholder)
            };
            if debug_enabled {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[loopform/prepare] CARRIER: {} -> init={:?}, copy={:?}, phi={:?}",
                    name, value, carrier.preheader_copy, carrier.header_phi
                ));
            }
            builder.carriers.push(carrier);
        }
    }

    if debug_enabled {
        crate::runtime::get_global_ring0()
            .log
            .debug("[loopform/prepare] === SUMMARY ===");
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[loopform/prepare] Total vars: {}",
            current_vars.len()
        ));
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[loopform/prepare] Pinned (params): {}",
            param_count
        ));
        crate::runtime::get_global_ring0().log.debug(&format!(
            "[loopform/prepare] Carriers (locals): {}",
            carrier_count
        ));
    }

    Ok(())
}
