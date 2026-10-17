//! Physical observations of the sole provisional-Phi writer. This is neither
//! a source/type proof nor a claim that the enclosing CFG is complete.
use super::{reject, PhysicalBinding};
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::cell::{Cell, OnceCell};
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::builder) struct PhysicalPhiDefinitionV1 {
    original: PhysicalBinding,
    completed: OnceCell<PhysicalBinding>,
    rolled_back: Cell<bool>,
}

impl PhysicalPhiDefinitionV1 {
    pub(super) fn new(block: BasicBlockId, dst: ValueId) -> Self {
        Self { original: (block, MirInstruction::Phi { dst, inputs: Vec::new(), type_hint: None }),
            completed: OnceCell::new(), rolled_back: Cell::new(false) }
    }
    pub(super) fn current(&self) -> Result<&PhysicalBinding, String> {
        if self.rolled_back.get() { return Err(reject("phi-rolled-back")); }
        Ok(self.completed.get().unwrap_or(&self.original))
    }
    pub(super) fn require_completed(&self) -> Result<(), String> {
        self.current()?;
        if !matches!(self.completed.get(), Some((_, MirInstruction::Phi { inputs, .. })) if !inputs.is_empty()) {
            return Err(reject("phi-incomplete"));
        }
        Ok(())
    }
    pub(super) fn resolves(&self, binding: &PhysicalBinding) -> bool {
        binding == &self.original || self.completed.get() == Some(binding)
    }
    pub(super) fn rollback(&self) { self.rolled_back.set(true); }
    pub(super) fn prepare(self: &Rc<Self>, before: &PhysicalBinding, after: PhysicalBinding)
        -> Result<PhysicalPhiPatchV1, String>
    {
        if self.rolled_back.get() || self.completed.get().is_some() || before != &self.original {
            return Err(reject("phi-completion-origin"));
        }
        if before.0 != after.0 || before.1.dst_value() != after.1.dst_value()
            || !matches!(&after.1, MirInstruction::Phi { .. }) {
            return Err(reject("phi-completion-shape"));
        }
        Ok(PhysicalPhiPatchV1 { definition: Rc::clone(self), binding: after })
    }
}

/// Prepared by the existing index, committed only by the successful Phi writer.
pub(in crate::mir::builder) struct PhysicalPhiPatchV1 {
    definition: Rc<PhysicalPhiDefinitionV1>,
    binding: PhysicalBinding,
}
impl PhysicalPhiPatchV1 {
    pub(in crate::mir::builder) fn commit(self) {
        self.definition.completed.set(self.binding)
            .expect("exclusive Phi writer completed one prepared observation twice");
    }
}
