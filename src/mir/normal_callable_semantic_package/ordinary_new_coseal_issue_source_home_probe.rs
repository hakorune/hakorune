//! Source-only home-prefix readiness probe for the ordinary-New cohort.
//! One responsibility: run the readiness/actual probe with the verified
//! walk's same source predicates. Moved out of the issue source unchanged.
use super::*;

/// Run the source-only readiness/actual probe with the verified walk's same
/// source predicates. Readiness and Completion selection remain in the issuer.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn probe_source_home_prefixes_v1(
    brands: Option<&crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1>,
    source_core_calls: Option<&super::super::super::super::core_method_source::SourceCoreMethodCallRowsV1>,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    new_sites: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    explicit_sites: &[SourceStmtSiteV1],
    pending_actuals: &mut super::super::super::lexical_instance_call::PendingBorrowedFormalActualsV1,
    dynamic_source_calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    batch_slot: u32,
    selected: &VerifiedSelectedCallableBatchMapV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameter_contracts: &[super::super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    callable_result_classes: &result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &[OrdinaryNewCandidate], result_constructions: &[OrdinaryNewSiteResolutionV1],
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    array_i64_field_sets: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    borrowed_formal_source: &Result<
        super::super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
        String,
    >,
    static_source_sites: &Result<BTreeSet<OwnedExprSiteV1>, String>,
    lexical_source_targets: &super::super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    borrowed_i64_results: &BorrowedResultCohortV1,
    local_static_call: &mut impl FnMut(
        &OwnedExprSiteV1,
    ) -> Result<
        Option<crate::mir::resolved_semantics::home_new_prefix::StaticScalarCallClaimV1>,
        OrdinaryNewCoSealIssueV1,
    >,
    receiver_rows: &BTreeMap<OwnedExprSiteV1, super::super::super::ReceiverCallClassObservationV1>,
    field_write_claims: &field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    completed: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractBuilderV1,
) -> Result<
    BTreeMap<OwnedExprSiteV1, Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>>,
    OrdinaryNewCoSealIssueV1,
> {

    let route_sources = super::super::local_route::LocalRouteSourcesV1 {
        owner_loan: None, selected, batch, parameter_contracts, callable_result_classes,
        candidates, receiver_call_observations: receiver_rows, input,
    };
    crate::mir::resolved_semantics::home_new_prefix::issue_new_home_prefixes_probing_fields_v1(
        input,
        &new_sites,
        entry_home,
        explicit_sites,
        parameter_contracts
            .iter()
            .filter(|row| row.batch_slot == batch_slot)
            .flat_map(|row| row.parameters.iter())
            .map(|row| (row.ordinal, row.binding, row.kind.clone(), row.demand)),
        // The probe asks the same route question as the verified lane,
        // narrowed to the static and lexical routes it has always covered.
        &mut |site| {
            Ok(route_sources
                .route(site, local_static_call)?
                .filter(|route| route.in_readiness_probe()))
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            terminal_home::argument_integer_field(
                instance_constructors,
                &candidates,
                site,
                receiver_proof,
                home,
                name,
            )
            .map(|field| field.is_some())
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name: &str| {
            terminal_home::receiver_scalar_field(
                instance_constructors,
                receiver_proof,
                site,
                home,
                name,
            )
            .map(|field| field.is_some())
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            terminal_home::receiver_container_field(
                instance_constructors,
                receiver_proof,
                site,
                home,
                name,
            )
            .map(|field| field.is_some())
        },
        &mut |site: &OwnedExprSiteV1, _: &SourceExprSiteV1, _: BindingRefV1, home, name| {
            // The probe shares the verified lane's sealed census — the
            // declared `ArrayBox` type alone admits nothing.
            terminal_home::receiver_array_i64_field_from_census_v1(
                instance_constructors,
                receiver_proof,
                array_i64_field_sets,
                site,
                home,
                name,
            )
        },
        // The probe shares the verified lane's `formal.<field>` index
        // proof — same consult factory, same sealed authority.
        &mut formal_i64_index_consult_v1(
            input,
            instance_constructors,
            batch.ordinary_box_coverage(),
        ),
        // The probe shares the verified lane's local
        // field-read membership — no staging here; the
        // verified walk owns the ledger rows.
        &mut |requests, demand| {
            prove_local_field_read_batch(
                instance_constructors,
                &candidates, result_constructions,
                batch.ordinary_box_coverage(),
                receiver_proof,
                entry_home,
                source_core_calls,
                requests,
                demand,
                borrowed_i64_results,
                borrowed_formal_source,
                &mut |home| {
                    super::super::lexical::nullable_received_result_class(
                        selected,
                        batch,
                        callable_result_classes,
                        &candidates,
                        input,
                        receiver_proof,
                        home,
                    )
                },
                &mut |home| source_formal_class_v1(borrowed_formal_source, home),
                Some((
                    batch,
                    selected,
                    callable_result_classes,
                    lexical_source_targets,
                    field_write_claims,
                )),
            )
            .map(|rows| {
                rows.map(|rows| {
                    rows.into_iter()
                        .map(|(_, row)| row.source.result().clone())
                        .collect()
                })
            })
        },
        &mut |site, actuals| source_call_arguments_v1(
            input, brands, batch, selected, instance_constructors, receiver_proof, entry_home,
            callable_result_classes, field_write_claims, lexical_source_targets, dynamic_source_calls,
            parameter_contracts, candidates, borrowed_formal_source, static_source_sites,
            pending_actuals, borrowed_i64_results, receiver_rows, completed, site, actuals, false,
        ),
        &mut dominated_view_use_consult_v1(
            borrowed_formal_source,
            input,
            borrowed_i64_results,
            array_i64_field_sets,
            instance_constructors,
            batch.ordinary_box_coverage(),
        ),
        &mut |site| Ok(callable_result_classes.object_return_qualification(site)),
    )
}
