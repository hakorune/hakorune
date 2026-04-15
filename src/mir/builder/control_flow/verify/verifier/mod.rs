//! Phase 29bq+ (Phase 273 P3): PlanVerifier - CorePlan 不変条件検証 (fail-fast)
//!
//! # Overview
//!
//! 7-module architecture for systematic CorePlan validation before lowering to MIR.
//! Each module enforces specific invariants to prevent silent miscompilation.
//!
//! # Module Architecture
//!
//! ```text
//! verifier/
//! ├── mod.rs                    - This orchestrator (tests + documentation)
//! ├── core.rs                   - Public API & dispatcher
//! ├── primitives.rs             - Error formatting, ValueId checks, EdgeArgs validation
//! ├── position_validators.rs    - Exit position enforcement (V11)
//! ├── plan_validators.rs        - Seq/If/BranchN/Exit validation (V3-V6, V11)
//! ├── effect_validators.rs      - Effect validation (V2, V6, V12 leaf checks)
//! ├── loop_validators.rs        - Loop structure validation (V2, V7-V10, V10b, V14)
//! └── loop_body_validators.rs   - Loop body tree validation (V12)
//! ```
//!
//! # Invariants (V2-V14)
//!
//! ## Structural Invariants
//! - **V2**: Condition validity (valid ValueId for conditions)
//! - **V3**: Exit validity (Return in function, Break/Continue in loop)
//! - **V4**: Seq may be empty (no-op allowed)
//! - **V5**: If/BranchN completeness (branches non-empty or joins present)
//! - **V6**: ValueId validity (all ValueIds pre-generated, non-empty names)
//!
//! ## Loop-Specific Invariants
//! - **V7**: PHI non-empty (loops require at least one carrier)
//! - **V8**: Frag entry matches header_bb (loop entry SSOT)
//! - **V9**: block_effects contains header_bb
//! - **V10**: body_bb effects go in loop_plan.body (block_effects[body_bb] must be empty)
//! - **V10b**: InlineInBody requires empty step_bb effects
//!
//! ## Control Flow Invariants
//! - **V11**: Exit must be last in Seq/If/BranchN branches (ExitMap alignment)
//! - **V12**: Loop.body must be Effect-only (no If/BranchN/Exit plans)
//!   - Exception: ExitIf within IfEffect is allowed (leaf-level exit)
//!
//! ## Edge Argument Invariants
//! - **V13**: EdgeArgs layout validation
//!   - ExprResultPlusCarriers requires at least one value
//!   - OnlyCarriers allows empty values
//!
//! ## Pipeline Invariants
//! - **V14**: Continue target must be in frag wiring (loop pipeline correctness)
//!
//! # Error Format
//!
//! All errors follow the format: `[Vx][reason=...] description`
//! - `Vx`: Invariant number (V2-V14)
//! - `reason`: Stable error code (e.g., `loop_phi_empty`, `exit_not_last`)
//! - `description`: Human-readable details
//!
//! # Design Rationale
//!
//! The 7-module split follows single-responsibility principle:
//! 1. **primitives** - Shared utilities (DRY principle)
//! 2. **core** - Entry point & routing only
//! 3. **position_validators** - Exit position rules (V11)
//! 4. **plan_validators** - Plan-level rules (Seq/If/BranchN/Exit)
//! 5. **effect_validators** - Effect-level rules (IfEffect, ExitIf)
//! 6. **loop_validators** - Loop structure rules (header/body/step/pipeline)
//! 7. **loop_body_validators** - Loop body tree rules (Effect-only enforcement)
//!
//! Each module has a focused responsibility and can be tested independently.
//!
//! # Historical Notes
//!
//! - Phase 273 P3: V1 (Carrier completeness) removed with CoreCarrierInfo
//! - Phase 29bq+: Modularization from 1,416 line monolith → 7 focused modules
//! - Original verifier.rs: Steps 1-7 extraction (2025-01-12)

pub(super) mod core;
pub(super) mod effect_validators;
pub(super) mod loop_body_validators;
pub(super) mod loop_validators;
pub(super) mod plan_validators;
pub(super) mod position_validators;
pub(super) mod primitives;

mod cond_profile;
mod debug_helpers;
#[cfg(test)]
mod tests;

pub(in crate::mir::builder) use core::PlanVerifier;
pub(in crate::mir::builder) use cond_profile::*;
pub(in crate::mir::builder) use debug_helpers::*;
