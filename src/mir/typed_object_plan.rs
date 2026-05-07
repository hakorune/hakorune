/*!
 * Typed object layout plans for EXE lowering.
 *
 * MIR owns the object layout truth. Backends consume these plans instead of
 * rediscovering user-box declarations or cloning VM InstanceBox semantics.
 */

mod storage_inference;

use crate::mir::function::TypedObjectPlan;
use crate::mir::MirModule;

pub const TYPED_OBJECT_LAYOUT_KIND_RUNTIME_SLOT_OBJECT_V0: &str = "runtime_slot_object_v0";

pub fn refresh_module_typed_object_plans(module: &mut MirModule) {
    module.metadata.typed_object_plans = build_typed_object_plans(module);
}

pub fn build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan> {
    storage_inference::build_typed_object_plans(module)
}
