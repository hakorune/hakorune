//! Borrow the original guarded receiver/value pair of one retained write frame.
//! Source correspondence only; physical pins, cleanup and use counts remain owed.
use super::*;
use super::super::borrowed_formal_uses::{BorrowedFormalUsesDraftV1, BorrowedFormalUseDraftKindV1 as Use};
use super::super::borrowed_formal_uses::CheckedIntegerOperandReachV1;
use super::super::super::field_write_claim::OriginalFieldWriteSourceLoanV1;
use crate::mir::resolved_semantics::{SourceNodeSiteV1, SourcePathV1, SourcePathSegmentV1};
use crate::mir::resolved_semantics::home_new_prefix::CheckedFieldWriteFrameV1;
use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedFieldWritePairRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) original: &'a OriginalFieldWriteSourceLoanV1,
    pub(in crate::mir::normal_callable_semantic_package) receiver: (BindingRefV1, BindingRefV1, &'a OwnedExprSiteV1, &'a SourceNodeSiteV1),
    pub(in crate::mir::normal_callable_semantic_package) value: (BindingRefV1, BindingRefV1, &'a OwnedExprSiteV1, &'a CheckedIntegerOperandReachV1),
}

pub(in crate::mir::normal_callable_semantic_package) fn checked_field_write_pair_v1<'a>(
    owner: FunctionOwnerIdV1, formals: &[(u32, BindingRefV1)],
    definition: &'a BorrowedFormalUsesDraftV1, frame: &CheckedFieldWriteFrameV1,
) -> Result<BorrowedFieldWritePairRefV1<'a>, String> {
    let fail = || freeze("borrowed-field-write/source-pair");
    let ExpressionPlanRootV1::Assignment { assignment } = frame.source().root() else { return Err(fail()) };
    if !frame.is_borrowed() || frame.source().owner() != owner { return Err(fail()); }
    let mut receivers = definition.uses.iter().filter_map(|row| match &row.kind {
        Use::FieldWriteReceiver { source, guard }
            if source.write().assignment() == assignment => Some((row, source, guard)),
        _ => None,
    });
    let (receiver, original, guard) = receivers.next().ok_or_else(fail)?;
    if receivers.next().is_some() { return Err(fail()); }
    let mut values = definition.uses.iter().filter_map(|row| match &row.kind {
        Use::FieldWriteValue { source, reach } if source == original => Some((row, reach)),
        _ => None,
    });
    let (value, reach) = values.next().ok_or_else(fail)?;
    if values.next().is_some() || original.write().owner() != owner
        || !Rc::ptr_eq(original.write().source().map_err(|_| fail())?, frame.source())
        || original.write().receiver_site() != receiver.site.site()
        || assignment.value_site() != value.site.site()
        || reach.operand() != (value.binding, value.formal, &value.site)
        || [receiver, value].iter().any(|row| row.site.owner() != owner
            || definition.origins.get(&row.binding) != Some(&row.formal)
            || !formals.iter().any(|(_, binding)| *binding == row.formal))
        || !definition.uses.iter().any(|row| row.formal == receiver.formal
            && matches!(&row.kind, Use::NullCompareOperand { binary }
                if SourcePathV1::from_node(guard).child(SourcePathSegmentV1::IfCondition).expr() == *binary.site()))
        || !definition.uses.iter().any(|row| row.formal == value.formal
            && matches!(&row.kind, Use::CompareOperand { binary, source }
                if reach.guard().matches_compare(value.formal, binary, source)))
    { return Err(fail()); }
    Ok(BorrowedFieldWritePairRefV1 { original,
        receiver: (receiver.binding, receiver.formal, &receiver.site, guard),
        value: (value.binding, value.formal, &value.site, reach) })
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct BorrowedFieldWriteSourceRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) pair: BorrowedFieldWritePairRefV1<'a>,
    pub(in crate::mir::normal_callable_semantic_package) field: &'a Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
}
impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_field_write_use_for_frame_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1, frame: &Rc<CheckedFieldWriteFrameV1>,
    ) -> Result<BorrowedFieldWriteSourceRefV1<'_>, String> {
        let pair = checked_field_write_pair_v1(self.owner, &self.formals, self.definition, frame)?;
        ledger.recheck_borrowed_field_write_frame_v1(frame, pair.original)?;
        let view = self.formal_object_view(pair.receiver.1)?
            .ok_or_else(|| freeze("borrowed-field-write/object-view-missing"))?;
        let field = view.scalar_field_source_v1(pair.original.write().field())
            .ok_or_else(|| freeze("borrowed-field-write/scalar-declaration-missing"))?;
        if field.field().object() != view.object() {
            return Err(freeze("borrowed-field-write/object-drift"));
        }
        Ok(BorrowedFieldWriteSourceRefV1 { pair, field })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_borrowed_field_write_frame_v1(
        &self, frame: &Rc<CheckedFieldWriteFrameV1>, original: &OriginalFieldWriteSourceLoanV1,
    ) -> Result<(), String> {
        self.field_write_claims.recheck_original_write_source_v1(original)
            .map_err(|_| freeze("borrowed-field-write/source-inventory"))?;
        let completion = self.completion_for_owner(frame.source().owner())
            .ok_or_else(|| freeze("borrowed-field-write/completion-missing"))?;
        let flow = completion.cleanup().root_flow()
            .ok_or_else(|| freeze("borrowed-field-write/home-flow-missing"))?;
        let mut matches = flow.expression_plans().iter().filter_map(|plan| plan.borrowed_field_write_frame())
            .filter(|original| Rc::ptr_eq(original, frame));
        if matches.next().is_none() || matches.next().is_some()
            || original.write().owner() != frame.source().owner()
            || !Rc::ptr_eq(original.write().source().map_err(|_| freeze("borrowed-field-write/source-plan"))?, frame.source())
        { return Err(freeze("borrowed-field-write/original-frame")); }
        Ok(())
    }
}

/// Same source objects retained for the sole writer and independent consumers.
/// Construction does not authorize any physical instruction or runtime payload.
#[derive(Debug, Clone)]
pub(crate) struct BorrowedFieldWriteSourceLoanV1 {
    frame: Rc<CheckedFieldWriteFrameV1>,
    completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
    original: OriginalFieldWriteSourceLoanV1,
    receiver: (BindingRefV1, BindingRefV1, OwnedExprSiteV1, SourceNodeSiteV1),
    value: (BindingRefV1, BindingRefV1, OwnedExprSiteV1, CheckedIntegerOperandReachV1),
    field: Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
}
impl BorrowedFieldWriteSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn null_guard_v1(&self) -> &SourceNodeSiteV1 { &self.receiver.3 }
    pub(in crate::mir) fn integer_guard_site_v1(&self) -> &OwnedExprSiteV1 { self.value.3.guard().binary_site_v1() }
    pub(crate) fn field_name(&self) -> &str { self.original.write().field() }
    pub(crate) fn frame(&self) -> &Rc<CheckedFieldWriteFrameV1> { &self.frame }
    pub(crate) fn field_source(&self) -> &Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1> { &self.field }
    pub(crate) fn receiver(&self) -> (BindingRefV1, BindingRefV1, &OwnedExprSiteV1) {
        (self.receiver.0, self.receiver.1, &self.receiver.2)
    }
    pub(crate) fn value(&self) -> (BindingRefV1, BindingRefV1, &OwnedExprSiteV1) {
        (self.value.0, self.value.1, &self.value.2)
    }
    pub(crate) fn validate_source(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        ledger.recheck_borrowed_field_write_frame_v1(&self.frame, &self.original)?;
        let owner = self.frame.source().owner();
        if ledger.completion_source_for_owner_v1(owner)
            .is_none_or(|completion| !Rc::ptr_eq(completion, &self.completion))
        { return Err(freeze("borrowed-field-write/completion-identity")); }
        let entry = ledger.finalized_borrowed_ordinary_entry_source_v1(owner)?;
        let source = entry.borrowed_field_write_use_for_frame_v1(ledger, &self.frame)?;
        if source.pair.original != &self.original || !Rc::ptr_eq(source.field, &self.field)
            || source.pair.receiver != (self.receiver.0, self.receiver.1, &self.receiver.2, &self.receiver.3)
            || source.pair.value != (self.value.0, self.value.1, &self.value.2, &self.value.3)
            || !Rc::ptr_eq(source.pair.value.3.guard(), self.value.3.guard())
        { return Err(freeze("borrowed-field-write/original-pair")); }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    /// The canonical Completion selects this responsibility at its original
    /// statement. No MIR, class-name permission or pending registry selects it.
    pub(crate) fn borrowed_field_write_source_at_v1(
        &self, owner: FunctionOwnerIdV1, statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<Option<BorrowedFieldWriteSourceLoanV1>, String> {
        let Some(completion) = self.completion_source_for_owner_v1(owner) else { return Ok(None) };
        let Some(flow) = completion.cleanup().root_flow() else { return Ok(None) };
        let mut frames = flow.expression_plans().iter().filter_map(|plan| plan.borrowed_field_write_frame())
            .filter(|frame| frame.source().root().statement() == statement);
        let Some(frame) = frames.next() else { return Ok(None) };
        if frames.next().is_some() { return Err(freeze("borrowed-field-write/duplicate-frame")); }
        let entry = self.finalized_borrowed_ordinary_entry_source_v1(owner)?;
        let source = entry.borrowed_field_write_use_for_frame_v1(self, frame)?;
        let loan = BorrowedFieldWriteSourceLoanV1 {
            frame: Rc::clone(frame), completion: Rc::clone(completion), original: source.pair.original.clone(),
            receiver: (source.pair.receiver.0, source.pair.receiver.1,
                source.pair.receiver.2.clone(), source.pair.receiver.3.clone()),
            value: (source.pair.value.0, source.pair.value.1,
                source.pair.value.2.clone(), source.pair.value.3.clone()),
            field: Rc::clone(source.field),
        };
        loan.validate_source(self)?;
        Ok(Some(loan))
    }
}

#[path = "ordinary_new_common_field_write_source.rs"]
mod common_source;
pub(crate) use common_source::FieldWriteSourceLoanV1;
