//! Retain the SAME source dependency and its original finished formal read.
//! This projection adds neither a Home nor an incoming-call permission.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::completion_index::ObjectReturnFieldSupportV1;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::mir) struct FinalizedObjectReturnFieldSupportV1 {
    sources: Box<[ObjectReturnFieldSupportV1]>,
    entry: ValueId,
    copies: Vec<(BasicBlockId, MirInstruction)>,
    call: [(BasicBlockId, MirInstruction); 2],
    callee_ordinal: u32,
    construction: Option<(BasicBlockId, MirInstruction)>,
}
impl FinalizedObjectReturnFieldSupportV1 {
    pub(super) fn new(sources: Box<[ObjectReturnFieldSupportV1]>, entry: ValueId,
        copies: Vec<(BasicBlockId, MirInstruction)>,
        call: [(BasicBlockId, MirInstruction); 2], callee_ordinal: u32,
        construction: Option<(BasicBlockId, MirInstruction)>) -> Self {
        Self { sources, entry, copies, call, callee_ordinal, construction }
    }

    pub(super) fn sources(&self) -> &[ObjectReturnFieldSupportV1] { &self.sources }

    /// Borrow only the already-checked group's physical identity. The source
    /// witness census stays here and is never replaced by wire witness IDs.
    pub(in crate::mir) fn field_dependency(&self) -> Result<(
        hakorune_mir_defs::CanonicalFieldRefV1, hakorune_mir_defs::CanonicalObjectIdV1, u32, u32,
    ), String> {
        let source = self.sources.first().ok_or_else(|| freeze("object-return/field-support-empty"))?;
        Ok((source.relation.field(), source.relation.child(),
            source.relation.formal_ordinal(), source.ordinal))
    }
    pub(in crate::mir) fn entry(&self) -> ValueId { self.entry }
    pub(in crate::mir) fn copies(&self) -> &[(BasicBlockId, MirInstruction)] { &self.copies }
    pub(in crate::mir) fn call(&self) -> &[(BasicBlockId, MirInstruction); 2] { &self.call }
    pub(in crate::mir) fn callee_ordinal(&self) -> u32 { self.callee_ordinal }
    pub(in crate::mir) fn construction(&self) -> Option<&(BasicBlockId, MirInstruction)> { self.construction.as_ref() }

    // Only adjacent occurrences with the SAME issuer and complete physical
    // dependency may share storage. Every original witness remains ordered.
    pub(super) fn push_group(rows: &mut Vec<Self>, next: Self) {
        if let Some(previous) = rows.last_mut() {
            if previous.same_dependency(&next) {
                let mut sources = previous.sources.to_vec();
                sources.extend_from_slice(&next.sources);
                previous.sources = sources.into_boxed_slice();
                return;
            }
        }
        rows.push(next);
    }

    fn same_source_dependency(a: &ObjectReturnFieldSupportV1, b: &ObjectReturnFieldSupportV1) -> bool {
        std::rc::Rc::ptr_eq(&a.relation, &b.relation)
            && a.formal == b.formal && a.ordinal == b.ordinal
    }

    fn same_dependency(&self, other: &Self) -> bool {
        matches!((self.sources.first(), other.sources.first()), (Some(a), Some(b))
            if Self::same_source_dependency(a, b))
            && self.entry == other.entry && self.copies == other.copies
            && self.call == other.call && self.callee_ordinal == other.callee_ordinal
            && self.construction == other.construction
    }

    pub(super) fn validate_function(&self, function: &MirFunction, has_receiver: bool) -> Result<(), String> {
        let source = self.sources.first().ok_or_else(|| freeze("object-return/field-support-empty"))?;
        if self.sources.iter().any(|row| !Self::same_source_dependency(source, row)) {
            return Err(freeze("object-return/field-support-group"));
        }
        let lane = source.ordinal as usize + usize::from(has_receiver);
        if function.params.get(lane) != Some(&self.entry)
            || function.metadata.physical_param_carriers.as_ref().and_then(|rows| rows.get(lane))
                != Some(&crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue)
        { return Err(freeze("object-return/field-support-parameter")); }
        for binding in self.call.iter().chain(self.construction.iter()) {
            if !super::physical_boundary::check_binding(function, None, binding.0, &binding.1)? {
                return Err(freeze("object-return/field-support-call"));
            }
        }
        if !matches!(self.call[0].1, MirInstruction::Invoke { .. })
            || !matches!(self.call[1].1, MirInstruction::InvokeNormalResult { .. })
        { return Err(freeze("object-return/field-support-call-shape")); }
        for binding in &self.copies {
            if !super::physical_boundary::check_binding(function, None, binding.0, &binding.1)?
                || !matches!(binding.1, MirInstruction::Copy { .. })
            { return Err(freeze("object-return/field-support-copy")); }
        }
        Ok(())
    }
}
