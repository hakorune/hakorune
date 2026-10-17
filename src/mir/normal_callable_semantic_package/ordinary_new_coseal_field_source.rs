//! Borrow original field sources through the one Home consult.
use super::*;

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
        required |= source.nodes().iter().any(|node| node.binary().is_some_and(|binary|
            matches!(binary.operator(), crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Subtract
                | crate::mir::resolved_semantics::ResolvedBinaryOperatorV1::Divide)));
    }
    Ok(required)
}
/// Same original writer inventory as the borrowed receiver/value source uses.
/// This consult lends syntax; Home qualification still proves each use.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct ScalarFieldsWithOriginalWritesV1<'a, F> {
    reads: F,
    candidates: &'a [OrdinaryNewCandidate],
    writes: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn scalar_fields_with_original_writes<'a, F>(
    candidates: &'a [OrdinaryNewCandidate],
    writes: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    reads: F,
) -> ScalarFieldsWithOriginalWritesV1<'a, F>
where F: FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str)
    -> Result<bool, OrdinaryNewCoSealIssueV1> {
    ScalarFieldsWithOriginalWritesV1 { reads, candidates, writes }
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
    fn assignment_source(&mut self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        target: &SourceExprSiteV1) -> Result<Option<Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>>, OrdinaryNewCoSealIssueV1> {
        let issue = || OrdinaryNewCoSealIssueV1::SourceNavigation { site: OwnedExprSiteV1::new(owner, target.clone()) };
        let Some(loan) = self.writes.original_write_source_v1(owner, target).map_err(|_| issue())? else { return Ok(None) };
        self.writes.recheck_original_write_source_v1(&loan).map_err(|_| issue())?;
        loan.write().source().map(Rc::clone).map(Some).map_err(|_| issue())
    }
}
