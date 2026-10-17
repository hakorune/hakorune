//! Shared LIVE/probe terminal field consult. Each existing authority lends its
//! own proof; source classification never substitutes executable receiver Home.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use super::super::lexical_instance_call::{
    lend_integer_field_v1, IntegerFieldSourceLoanV1,
    PreparedBorrowedFormalIngressV1,
};
use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

pub(super) fn consult_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    results: &BorrowedResultCohortV1,
    entry: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    site: &OwnedExprSiteV1,
    receiver_site: &SourceExprSiteV1,
    receiver: BindingRefV1,
    home: BindingRefV1,
    name: &str,
) -> Result<
    Option<(
        hakorune_mir_defs::CanonicalFieldRefV1,
        Option<IntegerFieldSourceLoanV1>,
    )>,
    OrdinaryNewCoSealIssueV1,
> {
    if let Some(field) =
        terminal_home::initialized_integer_field(constructors, candidates, home, name)?
    {
        return Ok(Some((field, None)));
    }
    let class = lexical::nullable_received_result_class(
        selected, batch, classes, candidates, input, entry, home,
    )
    .or_else(|| {
        source
            .as_ref()
            .ok()
            .and_then(|source| source.formal_object_view(home))
            .map(|view| view.class().into())
    });
    if let Some(class) = class {
        if let Some(field) = terminal_home::nullable_result_integer_field(
            constructors,
            batch.ordinary_box_coverage(),
            &class,
            site,
            name,
        )? {
            return Ok(Some((field, None)));
        }
    }
    lend_integer_field_v1(
        results,
        source,
        input,
        constructors,
        entry,
        site,
        receiver_site,
        receiver,
        home,
        name,
    )
    .map(|loan| loan.map(|loan| (loan.field(), Some(loan))))
    .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
        site: site.clone(),
        issue,
    })
}

/// One staging owner for declaration/numeric and exact LOCAL-ready terminal reads.
/// The callback result is acknowledgement of retained evidence, never the evidence itself.
pub(super) struct TerminalFieldConsultV1<'a, F> {
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &'a [OrdinaryNewCandidate],
    coverage: &'a crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    staged: &'a mut std::collections::BTreeMap<OwnedExprSiteV1, field_reads::FieldRead>,
    prove: F,
}
pub(super) fn terminal_field_consult_v1<'a, F>(
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &'a [OrdinaryNewCandidate],
    coverage: &'a crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    staged: &'a mut std::collections::BTreeMap<OwnedExprSiteV1, field_reads::FieldRead>,
    prove: F,
) -> TerminalFieldConsultV1<'a, F>
where F: FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str)
    -> Result<Option<(hakorune_mir_defs::CanonicalFieldRefV1, Option<IntegerFieldSourceLoanV1>)>, OrdinaryNewCoSealIssueV1> {
    TerminalFieldConsultV1 { constructors, candidates, coverage, staged, prove }
}
impl<F> crate::mir::resolved_semantics::home_new_prefix::HomeTerminalFieldSourceConsultV1<OrdinaryNewCoSealIssueV1>
    for TerminalFieldConsultV1<'_, F>
where F: FnMut(&OwnedExprSiteV1, &SourceExprSiteV1, BindingRefV1, BindingRefV1, &str)
    -> Result<Option<(hakorune_mir_defs::CanonicalFieldRefV1, Option<IntegerFieldSourceLoanV1>)>, OrdinaryNewCoSealIssueV1> {
    fn integer_field(&mut self, request: &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1)
        -> Result<bool, OrdinaryNewCoSealIssueV1> {
        use field_reads::TerminalFieldValueSourceV1 as Value;
        let (field, value_source) = if request.owned_ready.is_some() {
            let Some((field, declared)) = terminal_home::local_read_field_declaration(self.constructors,
                self.candidates, self.coverage, None,
                &request.site, request.home, None, &request.field)? else { return Ok(false) };
            let source = field_reads::OwnedReadyFieldReadSourceV1::issue(self.candidates, request, field, declared.as_deref())
                .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: request.site.clone(), issue })?;
            (field, Value::OwnedLocal(source))
        } else {
            let Some((field, numeric)) = (self.prove)(&request.site, &request.receiver_site,
                request.receiver, request.home, &request.field)? else { return Ok(false) };
            (field, numeric.map(Value::Numeric).unwrap_or(Value::Declared))
        };
        if self.staged.insert(request.site.clone(), field_reads::FieldRead {
            receiver_site: request.receiver_site.clone(), receiver: request.receiver, home: request.home,
            field, value_source, progress: field_reads::Progress::Pending,
        }).is_some() { return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: request.site.clone() }); }
        Ok(true)
    }
}
