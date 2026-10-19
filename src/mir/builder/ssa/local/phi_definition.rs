//! Physical observations from the sole final/provisional Phi writer. This is neither
//! a source/type proof nor a claim that the enclosing CFG is complete.
use super::{reject, PhysicalBinding, LocalSsaPhysicalIndexV1};
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::cell::{Cell, OnceCell};
use std::rc::Rc;

#[derive(Debug)]
pub(in crate::mir::builder) struct PhysicalPhiDefinitionV1 {
    original: PhysicalBinding,
    completed: OnceCell<PhysicalBinding>,
    rolled_back: Cell<bool>,
    final_origin: bool,
}

impl PhysicalPhiDefinitionV1 {
    pub(super) fn new(block: BasicBlockId, dst: ValueId) -> Self {
        Self { original: (block, MirInstruction::Phi { dst, inputs: Vec::new(), type_hint: None }),
            completed: OnceCell::new(), rolled_back: Cell::new(false), final_origin: false }
    }
    pub(super) fn final_definition(binding: PhysicalBinding) -> Self {
        Self { original: binding.clone(), completed: OnceCell::from(binding),
            rolled_back: Cell::new(false), final_origin: true }
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
        if self.rolled_back.get() || before != self.current()?
            || if self.final_origin { Rc::strong_count(self) != 1 } else { self.completed.get().is_some() } {
            // Any identity or append loan retains this SAME Rc, so an observed
            // final definition cannot be replaced behind an immutable relation.
            return Err(reject("phi-completion-origin"));
        }
        if before.0 != after.0 || before.1.dst_value() != after.1.dst_value()
            || !matches!(&after.1, MirInstruction::Phi { .. }) {
            return Err(reject("phi-completion-shape"));
        }
        Ok(PhysicalPhiPatchV1 { definition: Rc::clone(self), binding: after, replace_final: self.final_origin })
    }
}

/// Prepared by the existing index, committed only by the successful Phi writer.
pub(in crate::mir::builder) struct PhysicalPhiPatchV1 {
    definition: Rc<PhysicalPhiDefinitionV1>,
    binding: PhysicalBinding,
    replace_final: bool,
}
impl PhysicalPhiPatchV1 {
    pub(in crate::mir::builder) fn commit(self, index: &mut LocalSsaPhysicalIndexV1) {
        let dst = self.binding.1.dst_value().expect("prepared Phi defines dst");
        assert!(index.phis.get(&dst).is_some_and(|original| Rc::ptr_eq(original, &self.definition)),
            "prepared Phi patch belongs to the same writer index");
        if self.replace_final {
            index.phis.insert(dst, Rc::new(PhysicalPhiDefinitionV1::final_definition(self.binding)));
        } else {
            self.definition.completed.set(self.binding)
                .expect("exclusive Phi writer completed one prepared observation twice");
        }
    }
}
