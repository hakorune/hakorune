/*!
 * # Frag Composition API - Single Source of Truth (Phase 280)
 *
 * This module is the **Single Source of Truth** for Frag composition.
 *
 * ## Purpose (Phase 280)
 *
 * Pattern numbers (1-9+) are **symptom labels** for regression tests, NOT architectural concepts.
 * The architectural SSOT is **Frag composition rules** (`seq`/`if`/`loop`/`cleanup`).
 *
 * **Upstream (Extractor/Normalizer)**: Finish "shape recognition" and extract route-specific knowledge
 * **Downstream (Composition)**: Use Frag composition rules to build CFG converging to SSOT
 * **Terminator Generation**: `FragEmitSession`（手順SSOT）+ `emit_frag()`（低レベルSSOT）(Phase 29bq+)
 *
 * ## Entry Points (Composition Operations)
 *
 * - `seq(a, b)`: Sequential composition (Normal wiring)
 * - `if_(header, cond, t, e, join_frag)`: Conditional composition (Branch wiring)
 * - `loop_(loop_id, header, after, body)`: Loop composition (Break/Continue wiring)
 * - `cleanup(body, cleanup)`: Cleanup composition (TODO: Phase 280+)
 *
 * ## Composition Contract (Invariants)
 *
 * - **Input**: `Frag` (entry + exits + wires + branches)
 * - **Output**: `Frag` (new entry + merged exits + merged wires + merged branches)
 * - **Guarantee**: Composition preserves invariants (`verify_frag_invariants_strict`)
 * - **No Allocation**: Caller (Normalizer) allocates `BasicBlockId`/`ValueId`
 * - **Pure CFG Transform**: Composition rearranges `exits`/`wires`/`branches` only
 *
 * ## Ownership Model (3-tier)
 *
 * 1. **Normalizer** (Tier 1): Allocates blocks/values, route-specific knowledge
 * 2. **Composition** (Tier 2): Rearranges exits/wires/branches, route-agnostic
 * 3. **Lowerer** (Tier 3): Emits MIR terminators via `FragEmitSession::emit_and_seal()`
 *
 * ## Usage Example
 *
 * ```rust
 * // Tier 1: Normalizer allocates blocks
 * let header_bb = builder.next_block_id();
 * let body_bb = builder.next_block_id();
 * let after_bb = builder.next_block_id();
 *
 * // Build Frags for body
 * let body_frag = Frag { /* body CFG */ };
 *
 * // Tier 2: Composition wires exits (no allocation)
 * let loop_frag = compose::loop_(loop_id, header_bb, after_bb, body_frag);
 *
 * // Tier 3: Lowerer emits terminators
 * session.emit_and_seal(func, &loop_frag)?;
 * ```
 *
 * ## References
 *
 * - **SSOT Documentation**: `docs/development/current/main/design/edgecfg-fragments.md` (Active SSOT)
 * - **Pattern Absorption**: `docs/development/current/main/joinir-architecture-overview.md` (Section 0.2)
 * - **Phase 280 Roadmap**: `docs/development/current/main/phases/phase-280/README.md`
 *
 * ## History
 *
 * - Phase 264: Entry API creation (signatures only)
 * - Phase 265-268: Implementation (seq/if/loop wiring, emit_frag SSOT)
 * - Phase 280: SSOT positioning (composition as legacy numbered-label absorption destination)
 */

use std::collections::BTreeMap;

use crate::config::env;
use crate::mir::builder::control_flow::edgecfg::api::block_params::BlockParams;
use crate::mir::BasicBlockId;

mod cleanup;
mod if_;
mod loop_;
mod seq;

pub(crate) use if_::if_;
#[cfg(test)]
pub(crate) use loop_::loop_;
#[cfg(test)]
pub(crate) use seq::seq;

pub(super) fn merge_block_params(
    target: &mut BTreeMap<BasicBlockId, BlockParams>,
    incoming: BTreeMap<BasicBlockId, BlockParams>,
    context: &str,
) -> Result<(), String> {
    let strict = env::joinir_strict_enabled() || env::joinir_dev_enabled();
    for (block, params) in incoming {
        if target.contains_key(&block) {
            if strict {
                return Err(format!(
                    "[{}] duplicate block_params for {:?}",
                    context, block
                ));
            }
            continue;
        }
        target.insert(block, params);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
