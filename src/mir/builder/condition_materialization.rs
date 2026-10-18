//! Physical condition correspondence shared by structured control writers.
//! Captures existing normalization only; no source purpose, CFG or permission.
use super::{MirBuilder, ValueId};
use super::ssa::local::LocalSsaPhysicalRelationV1;
use crate::mir::{BasicBlockId, MirInstruction};
use crate::mir::loop_api::LoopBuilderApi;
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir) struct ConditionMaterializationV1 {
    original: ValueId,
    condition: ValueId,
    block: BasicBlockId,
    bindings: Vec<(BasicBlockId, MirInstruction)>,
    relation: Rc<LocalSsaPhysicalRelationV1>,
}
impl ConditionMaterializationV1 {
    pub(in crate::mir) fn values(&self) -> (ValueId, ValueId) { (self.original, self.condition) }
    pub(in crate::mir) fn block(&self) -> BasicBlockId { self.block }
    pub(in crate::mir) fn bindings(&self) -> &[(BasicBlockId, MirInstruction)] { &self.bindings }
    pub(in crate::mir) fn relation(&self) -> &Rc<LocalSsaPhysicalRelationV1> { &self.relation }
}

/// Stack-local observation spanning the existing try_cond/finalize calls.
/// Consuming finish neither emits a Branch nor grants its later affine seal.
pub(in crate::mir::builder) struct ConditionMaterializationCaptureV1 {
    original: ValueId,
    block: BasicBlockId,
    start: usize,
}
impl ConditionMaterializationCaptureV1 {
    pub(in crate::mir::builder) fn begin(builder: &MirBuilder, original: ValueId) -> Result<Self, String> {
        let id = builder.current_block()?;
        let block = builder.function_state.current_function.as_ref().and_then(|function| function.blocks.get(&id))
            .ok_or_else(|| "[freeze:contract][if-form/condition-block]".to_owned())?;
        if block.terminator.is_some() { return Err("[freeze:contract][if-form/closed-condition]".into()); }
        Ok(Self { original, block: id, start: block.instructions.len() })
    }
    pub(in crate::mir::builder) fn finish(self, builder: &mut MirBuilder,
        pre_branch: BasicBlockId, condition: ValueId,
    ) -> Result<Rc<ConditionMaterializationV1>, String> {
        if self.block != pre_branch || builder.function_state.current_block != Some(pre_branch) { return Err("[freeze:contract][if-form/condition-block-drift]".into()); }
        let function = builder.function_state.current_function.as_ref()
            .ok_or_else(|| "[freeze:contract][if-form/function-missing]".to_owned())?;
        let block = function.blocks.get(&self.block)
            .ok_or_else(|| "[freeze:contract][if-form/condition-block]".to_owned())?;
        let bindings = block.instructions.get(self.start..)
            .ok_or_else(|| "[freeze:contract][if-form/condition-append]".to_owned())?
            .iter().cloned().map(|row| (self.block, row)).collect();
        let relation = builder.function_state.local_ssa_correspondence.loan(function, self.original, condition)
            .ok_or_else(|| "[freeze:contract][if-form/condition-correspondence]".to_owned())?;
        relation.verify(function)?;
        Ok(Rc::new(ConditionMaterializationV1 { original: self.original, condition, block: self.block, bindings, relation }))
    }
}
