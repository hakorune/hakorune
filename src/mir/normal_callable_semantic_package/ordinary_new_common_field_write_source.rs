//! One source-origin dispatch for the shared FieldSet packet. No physical
//! operand, Home installation or Normal/Fault execution permission is lent here.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::QualifiedFieldWriteRhsV1;

#[derive(Debug, Clone)]
pub(crate) struct FieldWriteSourceLoanV1 {
    origin: FieldWriteSourceOriginV1,
}
#[derive(Debug, Clone)]
enum FieldWriteSourceOriginV1 {
    FormalBorrowed(BorrowedFieldWriteSourceLoanV1),
    OwnedLocal {
        frame: Rc<CheckedFieldWriteFrameV1>,
        completion: Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
        rhs: Rc<QualifiedFieldWriteRhsV1>,
    },
}
impl FieldWriteSourceLoanV1 {
    pub(crate) fn frame(&self) -> &Rc<CheckedFieldWriteFrameV1> {
        match &self.origin { FieldWriteSourceOriginV1::FormalBorrowed(source) => source.frame(),
            FieldWriteSourceOriginV1::OwnedLocal { frame, .. } => frame }
    }
    pub(crate) fn receiver(&self) -> Result<(BindingRefV1, crate::mir::resolved_semantics::OwnedExprSiteV1), String> {
        if let Some(formal) = self.borrowed() {
            let (binding, _, site) = formal.receiver();
            return Ok((binding, site.clone()));
        }
        let local = self.frame().owned_local().ok_or_else(|| freeze("field-write/local-origin"))?;
        Ok((local.root(), crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            self.frame().source().owner(), local.original().write().receiver_site().clone())))
    }
    pub(crate) fn value_site(&self) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        let source = self.frame().source();
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(source.owner(), source.nodes()[0].site().clone())
    }
    pub(crate) fn borrowed(&self) -> Option<&BorrowedFieldWriteSourceLoanV1> {
        match &self.origin { FieldWriteSourceOriginV1::FormalBorrowed(source) => Some(source), _ => None }
    }
    pub(crate) fn owned_rhs(&self) -> Option<&Rc<QualifiedFieldWriteRhsV1>> {
        match &self.origin { FieldWriteSourceOriginV1::OwnedLocal { rhs, .. } => Some(rhs), _ => None }
    }
    pub(crate) fn validate_source(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        let FieldWriteSourceOriginV1::OwnedLocal { frame, completion, rhs } = &self.origin else {
            return self.borrowed().expect("closed formal origin").validate_source(ledger);
        };
        let fail = || freeze("owned-field-write/original-source");
        let local = frame.owned_local().ok_or_else(fail)?;
        if ledger.completion_source_for_owner_v1(frame.source().owner())
            .is_none_or(|current| !Rc::ptr_eq(current, completion))
            || !Rc::ptr_eq(local.source(), frame.source())
            || local.original().write().owner() != frame.source().owner()
            || local.construction().allocation_only_i64_field(local.original().write().field()) != Some(local.field())
        { return Err(fail()); }
        ledger.field_write_claims.recheck_original_write_source_v1(local.original()).map_err(|_| fail())?;
        ledger.with_local_new_source_v1(local.acquisition(), local.root(), |source| {
            if source.object != local.field().object() || !source.construction_source.as_ref()
                .is_ok_and(|original| Rc::ptr_eq(original, local.construction())) { return Err(fail()); }
            Ok(())
        })?.ok_or_else(fail)?;
        let flow = completion.cleanup().root_flow().ok_or_else(fail)?;
        let mut plans = flow.expression_plans().iter().filter(|plan|
            plan.field_write_frame().is_some_and(|original| Rc::ptr_eq(original, frame)));
        let plan = plans.next().ok_or_else(fail)?;
        if plans.next().is_some() || plan.field_write_rhs().is_none_or(|original| !Rc::ptr_eq(original, rhs))
            || !rhs.is_consistent(frame.source()) { return Err(fail()); }
        Ok(())
    }
}
impl OrdinaryNewClaimLedgerV1 {
    /// The original Completion chooses the origin at its exact statement.
    pub(crate) fn field_write_source_at_v1(&self, owner: FunctionOwnerIdV1,
        statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    ) -> Result<Option<FieldWriteSourceLoanV1>, String> {
        let Some(completion) = self.completion_source_for_owner_v1(owner) else { return Ok(None) };
        let Some(flow) = completion.cleanup().root_flow() else { return Ok(None) };
        let mut plans = flow.expression_plans().iter().filter(|plan| plan.field_write_frame()
            .is_some_and(|frame| frame.source().root().statement() == statement));
        let Some(plan) = plans.next() else { return Ok(None) };
        if plans.next().is_some() { return Err(freeze("field-write/duplicate-original-plan")); }
        let frame = plan.field_write_frame().expect("selected original frame");
        let origin = if frame.is_borrowed() {
            let source = self.borrowed_field_write_source_at_v1(owner, statement)?
                .ok_or_else(|| freeze("field-write/formal-source-missing"))?;
            if !Rc::ptr_eq(source.frame(), frame) { return Err(freeze("field-write/formal-frame-drift")); }
            FieldWriteSourceOriginV1::FormalBorrowed(source)
        } else {
            let rhs = plan.field_write_rhs().ok_or_else(|| freeze("field-write/owned-rhs-missing"))?;
            FieldWriteSourceOriginV1::OwnedLocal { frame: Rc::clone(frame),
                completion: Rc::clone(completion), rhs: Rc::clone(rhs) }
        };
        let source = FieldWriteSourceLoanV1 { origin };
        source.validate_source(self)?;
        Ok(Some(source))
    }
}
