use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::mir::join_ir::lowering::common::body_local_derived_slot_emitter::BodyLocalDerivedSlotRecipe;
use crate::mir::join_ir::lowering::common::body_local_slot::ReadOnlyBodyLocalSlot;

/// Explicit routing policy for LoopBodyLocal variables used in loop-break conditions.
///
/// This is a "route" decision (not a fallback): we choose exactly one of the supported
/// strategies and reject otherwise.
pub enum BodyLocalRoute {
    Promotion {
        promoted_carrier: CarrierInfo,
        promoted_var: String,
        carrier_name: String,
    },
    ReadOnlySlot(ReadOnlyBodyLocalSlot),
    DerivedSlot(BodyLocalDerivedSlotRecipe),
}
