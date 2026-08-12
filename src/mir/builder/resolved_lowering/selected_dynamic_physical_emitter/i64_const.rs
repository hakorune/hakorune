use super::targets::DynamicV2OpaquePhysicalTargetV1;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::emission::constant;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::DynamicV2I8EvidenceV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::ValueId;

/// Session-branded result of the I8 producer.  It is intentionally opaque and
/// non-Clone; later leaves must consume the session rather than re-pairing this
/// value with another target or logical row.
#[derive(Debug)]
pub(in crate::mir) struct DynamicV2I64ProducerReceiptV1<'session> {
    brand: &'session DynamicV2PhysicalSessionBrandV1,
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    block: crate::mir::BasicBlockId,
    value: ValueId,
}

impl<'session> DynamicV2I64ProducerReceiptV1<'session> {
    #[cfg(test)]
    pub(super) fn with_value<R>(&self, callback: impl FnOnce(ValueId) -> R) -> R {
        callback(self.value)
    }
}

pub(super) fn emit<'session>(
    builder: &mut MirBuilder,
    target: &DynamicV2OpaquePhysicalTargetV1,
    evidence: DynamicV2I8EvidenceV1,
    brand: &'session DynamicV2PhysicalSessionBrandV1,
) -> Result<DynamicV2I64ProducerReceiptV1<'session>, DynamicV2I8EmitterRejectV1> {
    if !target.matches(brand) {
        return Err(DynamicV2I8EmitterRejectV1::TargetMismatch);
    }
    let value = constant::emit_integer_at(builder, target.block(), evidence.literal())
        .map_err(DynamicV2I8EmitterRejectV1::ConstantEmission)?;
    Ok(DynamicV2I64ProducerReceiptV1 {
        brand,
        producer: evidence.item(),
        result: evidence.result(),
        block: target.block(),
        value,
    })
}
