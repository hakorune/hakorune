//! Shared physical null-guard observations for selected field writes and Set indexes.
//! Source selection stays with the retained Completion and original use draft.
use super::*;
use crate::mir::builder::{IfConditionMaterializationV1, LocalSsaPhysicalRelationV1};
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathV1, SourcePathSegmentV1, SourceStmtSiteV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, CompareOp};
type Binding = (BasicBlockId, MirInstruction);

#[derive(Debug)]
pub(super) struct GuardObservationV1 {
    site: OwnedExprSiteV1,
    comparison: Binding,
    operands: [Rc<LocalSsaPhysicalRelationV1>; 2],
    original: Vec<Binding>,
    pending: Option<Rc<IfConditionMaterializationV1>>,
    branch: Option<Binding>,
}
impl OrdinaryNewClaimLedgerV1 {
    /// Return the original guard node and formal operand side, never a new
    /// semantic receipt. Multiple writes may share exactly this source guard.
    pub(in crate::mir) fn borrowed_field_write_guard_for_binary_v1(
        &self, owner: FunctionOwnerIdV1, site: &OwnedExprSiteV1,
    ) -> Result<Option<(SourceNodeSiteV1, BindingRefV1, usize)>, String> {
        let Some(completion) = self.completion_source_for_owner_v1(owner) else { return Ok(None) };
        let Some(flow) = completion.cleanup().root_flow() else { return Ok(None) };
        use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
        let Some(source) = self.borrowed_formal_source.as_ref().and_then(|source| source.as_ref().ok()) else { return Ok(None) };
        let Some(definition) = source.definitions.get(&owner).or_else(|| source.source_only_definitions.get(&owner)) else { return Ok(None) };
        if !definition.uses.iter().any(|row| matches!(&row.kind, Use::NullCompareOperand { binary } if binary == site)) {
            return Ok(None);
        }
        if !definition.uses.iter().any(|row| {
            let guard = match &row.kind {
                Use::FieldWriteReceiver { guard, .. } => guard,
                Use::ArrayElementFieldIndexValue { index, .. } => index.parts_v1().4,
                _ => return false,
            };
            SourcePathV1::from_node(guard).child(SourcePathSegmentV1::IfCondition).expr() == *site.site()
        }) { return Ok(None); }
        let entry = self.finalized_borrowed_ordinary_entry_source_v1(owner)?;
        let mut result = None;
        for row in entry.legacy_uses_v1() {
            let (guard, formal) = match &row.kind {
                Use::FieldWriteReceiver { guard, .. } => (guard, row.formal),
                Use::ArrayElementFieldIndexValue { index, .. } => {
                    let (_, _, binding, formal, guard) = index.parts_v1();
                    if entry.origins().get(&binding) != Some(&formal) {
                        return Err(freeze("borrowed-write-guard/index-origin"));
                    }
                    (guard, formal)
                }
                _ => continue,
            };
            let condition = SourcePathV1::from_node(guard).child(SourcePathSegmentV1::IfCondition).expr();
            if site.owner() != owner || site.site() != &condition { continue; }
            if let Use::FieldWriteReceiver { source: original, .. } = &row.kind {
                let original_source = original.write().source().map_err(|_| freeze("borrowed-write-guard/write-source"))?;
                let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.borrowed_field_write_frame())
                    .filter(|frame| Rc::ptr_eq(frame.source(), original_source));
                let frame = frames.next().ok_or_else(|| freeze("borrowed-write-guard/frame-missing"))?;
                if frames.next().is_some() { return Err(freeze("borrowed-write-guard/frame-duplicate")); }
                let write = entry.borrowed_field_write_use_for_frame_v1(self, frame)?;
                if write.pair.receiver.3 != guard || write.pair.receiver.1 != formal {
                    return Err(freeze("borrowed-write-guard/receiver-source"));
                }
            }
            let mut uses = entry.legacy_uses_v1().filter(|row| row.formal == formal
                && matches!(&row.kind, Use::NullCompareOperand { binary } if binary == site));
            let row = uses.next().ok_or_else(|| freeze("borrowed-write-guard/null-source"))?;
            if uses.next().is_some() { return Err(freeze("borrowed-write-guard/null-source-duplicate")); }
            let side = if row.site.site() == &SourcePathV1::from_node(site.site().node()).child(SourcePathSegmentV1::Lhs).expr() { 0 }
                else if row.site.site() == &SourcePathV1::from_node(site.site().node()).child(SourcePathSegmentV1::Rhs).expr() { 1 }
                else { return Err(freeze("borrowed-write-guard/operand-site")); };
            let selected = (guard.clone(), formal, side);
            if result.as_ref().is_some_and(|prior| prior != &selected) {
                return Err(freeze("borrowed-write-guard/source-drift"));
            }
            result = Some(selected);
        }
        Ok(result)
    }

    pub(in crate::mir) fn record_borrowed_field_write_guard_compare_v1(
        &self, owner: FunctionOwnerIdV1, site: OwnedExprSiteV1,
        comparison: Binding, operands: [Rc<LocalSsaPhysicalRelationV1>; 2],
        function: &MirFunction,
    ) -> Result<(), String> {
        let (guard, _, null_side) = self.borrowed_field_write_guard_for_binary_v1(owner, &site)?
            .ok_or_else(|| freeze("borrowed-write-guard/source-missing"))?;
        if !matches!(comparison.1, MirInstruction::Compare { op: CompareOp::Eq, lhs, rhs, .. }
            if (lhs, rhs) == (operands[0].values().1, operands[1].values().1))
        { return Err(freeze("borrowed-write-guard/compare-shape")); }
        let mut original = vec![comparison.clone()];
        for operand in &operands { operand.verify(function)?; operand.append_observed_bindings_v1(&mut original); }
        // The sibling was a literal in this exact source binary. Its original
        // child producer must be retained by the same LocalSSA relation.
        let null = operands[1 - null_side].values().0;
        if original.iter().filter(|(_, row)| matches!(row,
            MirInstruction::Const { dst, value: crate::mir::ConstValue::Null } if *dst == null)).count() != 1
        { return Err(freeze("borrowed-write-guard/null-producer")); }
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let entry = entries.get_mut(&owner).ok_or_else(|| freeze("borrowed-write-guard/entry"))?;
        if entry.write_guards.contains_key(&guard) { return Err(freeze("borrowed-write-guard/compare-duplicate")); }
        entry.write_guards.insert(guard, GuardObservationV1 { site, comparison, operands, original, pending: None, branch: None });
        Ok(())
    }

    pub(in crate::mir) fn prepare_borrowed_field_write_guard_condition_v1(
        &self, owner: FunctionOwnerIdV1, guard: &SourceNodeSiteV1,
        pending: Rc<IfConditionMaterializationV1>, function: &MirFunction,
    ) -> Result<(), String> {
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let record = entries.get_mut(&owner).and_then(|entry| entry.write_guards.get_mut(guard))
            .ok_or_else(|| freeze("borrowed-write-guard/compare-missing"))?;
        if record.pending.is_some() || record.comparison.1.dst_value() != Some(pending.values().0) {
            return Err(freeze("borrowed-write-guard/condition-identity"));
        }
        pending.relation().verify(function)?;
        record.pending = Some(pending);
        Ok(())
    }

    pub(in crate::mir) fn seal_borrowed_field_write_guard_branch_v1(
        &self, owner: FunctionOwnerIdV1, guard: &SourceNodeSiteV1,
        pending: &Rc<IfConditionMaterializationV1>, branch: Binding,
    ) -> Result<(), String> {
        let mut entries = self.borrowed_entry_values.borrow_mut();
        let record = entries.get_mut(&owner).and_then(|entry| entry.write_guards.get_mut(guard))
            .ok_or_else(|| freeze("borrowed-write-guard/compare-missing"))?;
        if record.pending.as_ref().is_none_or(|original| !Rc::ptr_eq(original, pending)) || record.branch.is_some()
            || branch.0 != pending.block()
            || !matches!(&branch.1, MirInstruction::Branch { condition, then_bb, else_bb,
                then_edge_args: Some(then_args), else_edge_args: Some(else_args) }
                if *condition == pending.values().1 && (*then_bb, *else_bb) == pending.arms()
                    && then_args.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly
                    && else_args.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly
                    && then_args.values.is_empty() && else_args.values.is_empty())
        { return Err(freeze("borrowed-write-guard/branch-identity")); }
        record.branch = Some(branch);
        Ok(())
    }

    pub(in crate::mir) fn original_borrowed_field_write_guard_bindings_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Binding>, String> {
        let Some(completion) = self.completion_source_for_owner_v1(owner) else { return Ok(Vec::new()) };
        let Some(flow) = completion.cleanup().root_flow() else { return Ok(Vec::new()) };
        let mut guards = BTreeMap::new();
        for frame in flow.expression_plans().iter().filter_map(|plan| plan.borrowed_field_write_frame()) {
            let entry = self.finalized_borrowed_ordinary_entry_source_v1(owner)?;
            let source = entry.borrowed_field_write_use_for_frame_v1(self, frame)?;
            let guard = source.pair.receiver.3;
            guards.insert(guard.clone(), OwnedExprSiteV1::new(owner,
                SourcePathV1::from_node(guard).child(SourcePathSegmentV1::IfCondition).expr()));
        }
        if let Some(source) = self.borrowed_formal_source.as_ref().and_then(|row| row.as_ref().ok()) {
            if let Some(definition) = source.definitions.get(&owner).or_else(|| source.source_only_definitions.get(&owner)) {
                use super::super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
                for row in &definition.uses {
                    if let Use::ArrayElementFieldIndexValue { index, .. } = &row.kind {
                        let guard = index.parts_v1().4;
                        guards.insert(guard.clone(), OwnedExprSiteV1::new(owner,
                            SourcePathV1::from_node(guard).child(SourcePathSegmentV1::IfCondition).expr()));
                    }
                }
            }
        }
        let entries = self.borrowed_entry_values.borrow();
        let records = entries.get(&owner).map(|entry| &entry.write_guards);
        if records.map_or(0, BTreeMap::len) != guards.len() { return Err(freeze("borrowed-write-guard/coverage")); }
        let mut result = Vec::new();
        for (guard, site) in guards {
            let record = records.and_then(|rows| rows.get(&guard)).ok_or_else(|| freeze("borrowed-write-guard/missing"))?;
            let pending = record.pending.as_ref().ok_or_else(|| freeze("borrowed-write-guard/no-condition"))?;
            let branch = record.branch.as_ref().ok_or_else(|| freeze("borrowed-write-guard/unsealed"))?;
            if record.site != site { return Err(freeze("borrowed-write-guard/source-site")); }
            for operand in &record.operands { operand.require_completed_v1()?; }
            pending.relation().require_completed_v1()?;
            let mut rows = record.original.clone();
            rows.extend_from_slice(pending.bindings());
            pending.relation().append_observed_bindings_v1(&mut rows);
            rows.push(branch.clone());
            for original in rows {
                let mut resolved = None;
                for relation in record.operands.iter().chain(std::iter::once(pending.relation())) {
                    if let Some(row) = relation.resolve_observed_binding_v1(&original, true)? {
                        if resolved.as_ref().is_some_and(|prior| prior != &row) {
                            return Err(freeze("borrowed-write-guard/ambiguous-completion"));
                        }
                        resolved = Some(row);
                    }
                }
                let row = resolved.unwrap_or(original);
                if !result.contains(&row) { result.push(row); }
            }
        }
        Ok(result)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// The loan selects its original guard; only retained physical observations
    /// are lent. The finishing owner and both final scans still owe projection.
    pub(in crate::mir) fn with_borrowed_field_write_guard_original_v1(
        &self, source: &crate::mir::normal_callable_semantic_package::BorrowedFieldWriteSourceLoanV1,
        visit: impl FnOnce(&Binding, &[Rc<LocalSsaPhysicalRelationV1>; 2],
            &Rc<IfConditionMaterializationV1>, &Binding, &[Binding]) -> Result<(), String>,
    ) -> Result<(), String> {
        source.validate_source(self)?;
        let owner = source.frame().source().owner();
        self.with_borrowed_entry_null_guard_original_v1(owner, source.null_guard_v1(), visit)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn with_borrowed_entry_null_guard_original_v1(
        &self, owner: FunctionOwnerIdV1, guard: &SourceNodeSiteV1,
        visit: impl FnOnce(&Binding, &[Rc<LocalSsaPhysicalRelationV1>; 2],
            &Rc<IfConditionMaterializationV1>, &Binding, &[Binding]) -> Result<(), String>,
    ) -> Result<(), String> {
        self.original_borrowed_field_write_guard_bindings_v1(owner)?;
        let entries = self.borrowed_entry_values.borrow();
        let record = entries.get(&owner).and_then(|entry| entry.write_guards.get(guard))
            .ok_or_else(|| freeze("borrowed-write-guard/original-missing"))?;
        visit(&record.comparison, &record.operands,
            record.pending.as_ref().ok_or_else(|| freeze("borrowed-write-guard/no-condition"))?,
            record.branch.as_ref().ok_or_else(|| freeze("borrowed-write-guard/unsealed"))?, &record.original)
    }
}
