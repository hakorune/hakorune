//! Existing CoreMethod source issuer extended with conditional named Array rows.
//! Production activation waits for the retained artifact consumer, not a flag.
use super::core_method::{nearest_loop, SourceBoundCoreMethodTargetIssueV1 as E};
use super::{issue_source_bound_core_method_calls_v1, VerifiedSourceBoundCoreMethodCallV1};
use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    issue_core_method_manifest_row_ref_v2, lookup_core_method_result_row_v2,
    CORE_METHOD_MANIFEST_BRAND_V2,
};
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, CoreMethodInstanceTargetIssuerV1,
    NamedArrayConstructionRequirementV1, NamedArrayRequirementIssueV1, ResolvedLoopPlacementV1,
    ResolverCoreMethodCallableContractIssuerV1, SourceExprSiteV1,
};
use crate::parser::ParserOrdinaryBoxSourceCoverageV1;
use std::collections::BTreeMap;

pub(crate) fn issue_source_bound_core_method_calls_with_named_arrays_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    ordinary: &ParserOrdinaryBoxSourceCoverageV1,
    brands: &VerifiedBrandProgramDeclarationCatalogV1,
) -> Result<Box<[(SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1)]>, E> {
    // Existing Text producers are sealed first; lexical source ordering must
    // not determine whether a nested substring can satisfy its parent's demand.
    let mut rows = Vec::from(issue_source_bound_core_method_calls_v1(ledger)?)
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    for (site, call) in ledger.method_calls() {
        let Some(manifest) =
            lookup_core_method_result_row_v2("ArrayBox", call.selector(), call.arity())
        else {
            continue;
        };
        if manifest.op != CoreMethodOp::ArrayPush {
            continue;
        }
        let mut candidates = Vec::new();
        for loop_site in ledger.loop_sites() {
            if ledger
                .resolved_loop_placement(loop_site, site)
                .map_err(E::LoopLookup)?
                == Some(ResolvedLoopPlacementV1::Body)
            {
                candidates.push(loop_site);
            }
        }
        let Some(loop_site) = nearest_loop(candidates)? else {
            continue;
        };
        let text_source = call
            .arguments()
            .first()
            .and_then(|argument| rows.get(argument.site()))
            .map(|row| row.contract());
        let requirement = match NamedArrayConstructionRequirementV1::issue(
            ledger,
            call,
            ordinary,
            brands,
            text_source,
        ) {
            Ok(requirement) => requirement,
            Err(
                NamedArrayRequirementIssueV1::NotNamedArray
                | NamedArrayRequirementIssueV1::DeclarationCollision
                | NamedArrayRequirementIssueV1::InitializerMissing
                | NamedArrayRequirementIssueV1::ConstructionMissing
                | NamedArrayRequirementIssueV1::UnsupportedReceiver,
            ) => continue,
            Err(error) => return Err(E::NamedArray(error)),
        };
        let membership = ledger
            .resolved_loop_source(loop_site)
            .map_err(E::LoopMembership)?;
        let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::ArrayPush, 1)
            .ok_or(E::NamedArray(NamedArrayRequirementIssueV1::ArgumentShape))?;
        let target =
            CoreMethodInstanceTargetIssuerV1::array_text_append(CORE_METHOD_MANIFEST_BRAND_V2)
                .map_err(E::Target)?
                .issue(row)
                .map_err(E::Target)?;
        let contract = ResolverCoreMethodCallableContractIssuerV1::issue_named_array(
            ledger,
            call,
            &membership,
            ResolvedLoopPlacementV1::Body,
            target,
            requirement,
        )
        .map_err(E::Contract)?;
        if rows
            .insert(
                site.clone(),
                VerifiedSourceBoundCoreMethodCallV1::new(contract),
            )
            .is_some()
        {
            return Err(E::DuplicateSite(site.clone()));
        }
    }
    Ok(rows.into_iter().collect())
}
