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
use crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1;
use crate::mir::resolved_semantics::{
    detect_field_residence_claim, resolve_birth_provider, seal_field_residence_requirement,
    verify_field_residence_relations, CallableSemanticSourceLedgerView,
    CoreMethodInstanceTargetIssuerV1, NamedArrayConstructionRequirementV1,
    NamedArrayFieldResidenceIssueV1, NamedArrayFieldResidenceObjectV1,
    NamedArrayRequirementIssueV1, NamedArrayRequirementV1, ResolvedLoopPlacementV1,
    ResolverCoreMethodCallableContractIssuerV1, SourceExprSiteV1,
};
use crate::parser::{ConstructorSourceIdV1, ParserOrdinaryBoxSourceCoverageV1};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalSameModuleCallableKeyV1};
use std::collections::BTreeMap;

/// One birth-side provider binding issued beside a residence requirement.
/// `constructor` identifies the owning `birth` for the package loan while
/// `provider` is the published caller identity recorded on the physical
/// marker; `site`/`field` bind the exact `new ArrayBox()` store.
#[derive(Debug, Clone)]
pub(crate) struct NamedArrayFieldProviderRowV1 {
    pub(crate) constructor: ConstructorSourceIdV1,
    pub(crate) provider: CanonicalSameModuleCallableKeyV1,
    pub(crate) site: SourceExprSiteV1,
    pub(crate) field: CanonicalFieldRefV1,
}

/// Issued source rows plus the provider bindings collected while sealing
/// field-resident requirements. Both products belong to the same issuance.
#[derive(Debug)]
pub(crate) struct NamedArrayIssuedCallsV1 {
    pub(crate) rows: Box<[(SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1)]>,
    pub(crate) field_providers: Vec<NamedArrayFieldProviderRowV1>,
}

pub(crate) fn issue_source_bound_core_method_calls_with_named_arrays_v1(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    body_shape: &crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1,
    ordinary: &ParserOrdinaryBoxSourceCoverageV1,
    brands: &VerifiedBrandProgramDeclarationCatalogV1,
    constructors: Option<&VerifiedInstanceConstructorSemanticBatchV1>,
    caller_box: &str,
) -> Result<NamedArrayIssuedCallsV1, E> {
    // Existing Text producers are sealed first; lexical source ordering must
    // not determine whether a nested substring can satisfy its parent's demand.
    let mut rows = Vec::from(issue_source_bound_core_method_calls_v1(ledger)?)
        .into_iter()
        .collect::<BTreeMap<_, _>>();
    let mut field_providers = Vec::new();
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
                candidates.push((loop_site, ()));
            }
        }
        let Some((loop_site, ())) = nearest_loop(candidates)? else {
            continue;
        };
        let nested_source = call
            .arguments()
            .first()
            .and_then(|argument| rows.get(argument.site()))
            .map(|row| row.contract());
        let requirement = match NamedArrayConstructionRequirementV1::issue(
            ledger,
            call,
            ordinary,
            brands,
            nested_source,
        ) {
            Ok(requirement) => NamedArrayRequirementV1::Construction(requirement),
            Err(
                NamedArrayRequirementIssueV1::NotNamedArray
                | NamedArrayRequirementIssueV1::DeclarationCollision
                | NamedArrayRequirementIssueV1::InitializerMissing
                | NamedArrayRequirementIssueV1::ConstructionMissing
                | NamedArrayRequirementIssueV1::UnsupportedReceiver,
            ) => match issue_field_residence_requirement(
                ledger,
                body_shape,
                call,
                ordinary,
                brands,
                constructors,
                caller_box,
                nested_source,
                &mut field_providers,
            )? {
                Some(requirement) => requirement,
                None => continue,
            },
            Err(error) => return Err(E::NamedArray(error)),
        };
        let membership = ledger
            .resolved_loop_source(loop_site)
            .map_err(E::LoopMembership)?;
        let row = issue_core_method_manifest_row_ref_v2(CoreMethodOp::ArrayPush, 1)
            .ok_or(E::NamedArray(NamedArrayRequirementIssueV1::ArgumentShape))?;
        let target = match &requirement {
            NamedArrayRequirementV1::Construction(_) => {
                CoreMethodInstanceTargetIssuerV1::array_text_append(
                    CORE_METHOD_MANIFEST_BRAND_V2,
                )
            }
            NamedArrayRequirementV1::FieldResidence(_) => {
                CoreMethodInstanceTargetIssuerV1::array_integer_append(
                    CORE_METHOD_MANIFEST_BRAND_V2,
                )
            }
        }
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
    Ok(NamedArrayIssuedCallsV1 {
        rows: rows.into_iter().collect(),
        field_providers,
    })
}

/// Seals one field-resident Array/I64 append contract. `Ok(None)` keeps the
/// call on the existing generic boundary; every typed rejection is a
/// `NamedArrayResidence` issue and never a silent omission.
#[allow(clippy::too_many_arguments)]
fn issue_field_residence_requirement(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    body_shape: &crate::mir::resolved_semantics::VerifiedResolvedBodyShapeInventoryV1,
    call: &crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    ordinary: &ParserOrdinaryBoxSourceCoverageV1,
    brands: &VerifiedBrandProgramDeclarationCatalogV1,
    constructors: Option<&VerifiedInstanceConstructorSemanticBatchV1>,
    caller_box: &str,
    integer_source: Option<&crate::mir::resolved_semantics::VerifiedResolverCoreMethodCallableContractV1>,
    field_providers: &mut Vec<NamedArrayFieldProviderRowV1>,
) -> Result<Option<NamedArrayRequirementV1>, E> {
    use NamedArrayFieldResidenceIssueV1 as I;
    let Some(claim) = detect_field_residence_claim(ledger, body_shape, call)
        .map_err(E::NamedArrayResidence)?
    else {
        return Ok(None);
    };
    let Some(constructors) = constructors else {
        return Err(E::NamedArrayResidence(I::ProviderMissing));
    };
    let (box_name, receiver_owned) = match &claim.object {
        NamedArrayFieldResidenceObjectV1::Receiver => (caller_box, true),
        NamedArrayFieldResidenceObjectV1::ForeignDeclared(name) => (name.as_ref(), false),
    };
    let Some(box_source) = ordinary
        .row_for(box_name)
        .map_err(|_| E::NamedArrayResidence(I::DeclarationCollision))?
    else {
        return Ok(None);
    };
    let Some((field, declared_type, weak)) = constructors
        .field_declaration(box_source, &claim.field_name)
        .map_err(|_| E::NamedArrayResidence(I::ProviderMissing))?
    else {
        return Ok(None);
    };
    // A declared field type must be exactly ArrayBox. An untyped `init` field
    // carries no declared type; its ArrayBox residence is admitted only when
    // the exact provider below proves a bare `new ArrayBox()` store.
    if declared_type.as_deref().is_some_and(|ty| ty != "ArrayBox") {
        return Ok(None);
    }
    if !receiver_owned {
        return Err(E::NamedArrayResidence(I::ForeignFieldOwner));
    }
    if weak {
        return Err(E::NamedArrayResidence(I::WeakFieldResidence));
    }
    if ordinary
        .row_for("ArrayBox")
        .map_err(|_| E::NamedArrayResidence(I::DeclarationCollision))?
        .is_some()
        || brands.contains_name("ArrayBox")
    {
        return Err(E::NamedArrayResidence(I::DeclarationCollision));
    }
    verify_field_residence_relations(ledger, body_shape, call, &claim, integer_source)
        .map_err(E::NamedArrayResidence)?;
    let Some(birth) = constructors
        .birth_row_for(box_source)
        .map_err(|_| E::NamedArrayResidence(I::ProviderMissing))?
    else {
        return Err(E::NamedArrayResidence(I::ProviderMissing));
    };
    let provider_caller = birth
        .published_birth_key()
        .ok_or(E::NamedArrayResidence(I::ProviderMissing))?
        .clone();
    let [birth_owner] = birth.forest().roots() else {
        return Err(E::NamedArrayResidence(I::ProviderShape));
    };
    let birth_ledger = birth
        .forest()
        .callable_source_ledger(*birth_owner)
        .map_err(|_| E::NamedArrayResidence(I::ProviderMissing))?;
    let birth_body_shape = birth
        .body_shape()
        .ok_or(E::NamedArrayResidence(I::BodyShapeMissing))?;
    let provider_site =
        resolve_birth_provider(&birth_ledger, birth_body_shape, &claim.field_name)
            .map_err(E::NamedArrayResidence)?;
    let requirement = seal_field_residence_requirement(
        ledger.owner(),
        field,
        provider_caller,
        provider_site.clone(),
        &claim,
    );
    field_providers.push(NamedArrayFieldProviderRowV1 {
        constructor: birth.source_id().clone(),
        provider: requirement.provider_caller().clone(),
        site: provider_site,
        field,
    });
    Ok(Some(NamedArrayRequirementV1::FieldResidence(
        requirement,
    )))
}
