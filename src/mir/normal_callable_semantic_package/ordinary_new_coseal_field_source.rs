//! Borrow original field sources through the one Home consult.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call;

/// Select the Home walk from original non-Birth writer syntax. Selection is
/// not scalar permission: the walk still owes receiver, RHS and field proofs.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn requires_field_scalar_walk_v1(
    writes: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let mut required = false;
    for write in writes.writes().iter().filter(|write| write.owner() == owner && !write.in_birth()) {
        let source = write.source().map_err(|_| OrdinaryNewCoSealIssueV1::SourceNavigation {
            site: OwnedExprSiteV1::new(owner, write.assignment().target_site().clone()),
        })?;
        required |= matches!(source.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment }
            if assignment.form() == crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain);
    }
    Ok(required)
}
/// Same original writer inventory as the borrowed receiver/value source uses.
/// This consult lends syntax; Home qualification still proves each use.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct ScalarFieldsWithOriginalWritesV1<'a, F> {
    reads: F,
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &'a crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    entry: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    results: &'a BorrowedResultCohortV1,
    ingress: &'a Result<lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    candidates: &'a [OrdinaryNewCandidate],
    writes: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn scalar_fields_with_original_writes<'a, F>(
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &'a crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    entry: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    results: &'a BorrowedResultCohortV1,
    ingress: &'a Result<lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    candidates: &'a [OrdinaryNewCandidate],
    writes: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    reads: F,
) -> ScalarFieldsWithOriginalWritesV1<'a, F>
where F: FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str)
    -> Result<bool, OrdinaryNewCoSealIssueV1> {
    ScalarFieldsWithOriginalWritesV1 { reads, constructors, receiver, entry, results, ingress, candidates, writes }
}

impl<F> crate::mir::resolved_semantics::home_new_prefix::HomeFieldSourceConsultV1<OrdinaryNewCoSealIssueV1>
    for ScalarFieldsWithOriginalWritesV1<'_, F>
where F: FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str)
    -> Result<bool, OrdinaryNewCoSealIssueV1> {
    fn construction_source(&mut self, site: &OwnedExprSiteV1, destination: BindingRefV1)
        -> Result<Option<Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>>, OrdinaryNewCoSealIssueV1> {
        let mut rows = self.candidates.iter().filter(|row| &row.site == site && row.destination == destination);
        let Some(row) = rows.next() else { return Ok(None) };
        if rows.next().is_some() {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
        Ok(row.construction_source.as_ref().ok().map(Rc::clone))
    }
    fn scalar_field(&mut self, site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1, home: BindingRefV1, field: &str) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        (self.reads)(site, receiver_site, receiver, home, field)
    }
    fn owned_local_write(&mut self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        acquisition: &OwnedExprSiteV1, root: BindingRefV1,
        construction: &Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>,
        target: &SourceExprSiteV1)
        -> Result<Option<crate::mir::normal_callable_semantic_package::OwnedLocalFieldWriteSourceV1>, OrdinaryNewCoSealIssueV1> {
        let mut candidates = self.candidates.iter().filter(|row| &row.site == acquisition && row.destination == root);
        let Some(candidate) = candidates.next() else { return Ok(None) };
        if candidates.next().is_some() { return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: acquisition.clone() }); }
        if !candidate.construction_source.as_ref().is_ok_and(|source| Rc::ptr_eq(source, construction))
            || candidate.object != construction.object() { return Ok(None); }
        let Some(original) = self.original_writer(input.owner(), target)? else { return Ok(None) };
        Ok(original.owned_local_v1(input, acquisition, root, Rc::clone(construction)))
    }
    fn entry_write(&mut self, input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        target: &SourceExprSiteV1) -> Result<Option<crate::mir::normal_callable_semantic_package::EntryFieldWriteSourceV1>, OrdinaryNewCoSealIssueV1> {
        let (Some(receiver), Some(entry)) = (self.receiver, self.entry) else { return Ok(None) };
        let Some(original) = self.original_writer(input.owner(), target)? else { return Ok(None) };
        if original.write().in_birth() { return Ok(None); }
        let site = OwnedExprSiteV1::new(input.owner(), target.clone());
        let Some((field, declared)) = terminal_home::source_declared_field(self.constructors,
            receiver.1, &site, original.write().field())? else { return Ok(None) };
        if !matches!(declared.as_deref(), None | Some("i64")) { return Ok(None); }
        let Ok(ingress) = self.ingress else { return Ok(None) };
        // A neutral source-only write is not an executable entry frame. Require
        // the complete value basis even for a declared slot before selecting it.
        let Ok(value) = lexical_instance_call::ClosedNumericFieldSourceLoanV1::from_declaration(
            self.results, ingress, field, declared.as_deref(), original.write().field()) else { return Ok(None) };
        let root = terminal_home::ExpressionFieldRootSourceV1::issue_entry_v1(self.constructors,
            receiver, entry, input.owner(), receiver.0, field).map_err(|issue|
                OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site, issue })?;
        Ok(crate::mir::normal_callable_semantic_package::EntryFieldWriteSourceV1::issue(input, original, root, field, declared, Some(value)))
    }
    fn scalar_read_sum(&mut self, read: &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarReadV1)
        -> Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>> {
        self.ingress.as_ref().ok()?.lend_current_scalar_sum_v1(read)
    }
    fn original_writer(&mut self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        target: &SourceExprSiteV1) -> Result<Option<crate::mir::normal_callable_semantic_package::OriginalFieldWriteSourceLoanV1>, OrdinaryNewCoSealIssueV1> {
        let issue = || OrdinaryNewCoSealIssueV1::SourceNavigation { site: OwnedExprSiteV1::new(owner, target.clone()) };
        let loan = self.writes.original_write_source_v1(owner, target).map_err(|_| issue())?;
        if let Some(loan) = &loan { self.writes.recheck_original_write_source_v1(loan).map_err(|_| issue())?; }
        Ok(loan)
    }
    fn assignment_source(&mut self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        target: &SourceExprSiteV1) -> Result<Option<Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>>, OrdinaryNewCoSealIssueV1> {
        let issue = || OrdinaryNewCoSealIssueV1::SourceNavigation { site: OwnedExprSiteV1::new(owner, target.clone()) };
        let Some(loan) = self.original_writer(owner, target)? else { return Ok(None) };
        loan.write().source().map(Rc::clone).map(Some).map_err(|_| issue())
    }
}
