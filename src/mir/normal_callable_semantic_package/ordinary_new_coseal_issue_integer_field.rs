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
