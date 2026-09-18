//! Precommit use of existing root and Local products; no semantic issuance.
use super::super::map_lifecycle_undertaking::{
    verify_map_lifecycle_undertaking, MapCallEdgeContractV1, MapCallEdgeKindV1,
    MapLifecycleUndertakingV1,
};
use super::*;
use crate::mir::builder::BuilderInstallConsumerV1;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::resolved_semantics::home_new_prefix::MapDestinationV1;
use crate::mir::resolved_semantics::{BindingOriginV1, ResolvedLexicalRefV1, SourceBindingSiteV1};

impl VerifiedNormalCallableSemanticPackageV1 {
    /// Describe every sealed member's Map obligations and verify the
    /// selected lowering consumer's declared capability covers all of
    /// them, returning the sealed undertaking for the install product.
    /// The undertaking — never AppMain reachability — is the deep-callee
    /// coverage criterion: obligations enumerate from sealed batch
    /// membership, and every declared MapLiteral site must carry a
    /// Complete flow row in its own owner's completion.
    pub(super) fn preflight_map_install(
        &self,
    ) -> Result<Option<MapLifecycleUndertakingV1>, NormalCallableSemanticPackageInstallIssueV1>
    {
        use NormalCallableSemanticPackageInstallIssueV1 as Issue;
        let obligations = self
            .describe_map_lifecycle_obligations()
            .map_err(Issue::MapObligationDescribe)?;
        if obligations.is_empty() {
            return Ok(None);
        }
        let undertaking = verify_map_lifecycle_undertaking(
            &obligations,
            BuilderInstallConsumerV1::map_lifecycle_capability(),
        )
        .map_err(Issue::MapLifecycleUndertaking)?;
        // Direct-call evidence stays scoped to the loans that carry it:
        // the affine rows must be unspent at install, every map-carrying
        // loan target must have described obligations, and a loan never
        // targets its own owner. Owner-common checks live in `describe`
        // above; nothing here inspects owners outside an actual loan.
        if let Some(loans) = self.direct_call_loans.as_ref() {
            for loan in loans.iter() {
                if loan.has_taken_slot() {
                    return Err(Issue::MapLifecycleConsumerMissing);
                }
                let targets = loan.map_target_owners(&self.batch).unwrap_or_default();
                if targets.iter().any(|target| *target == loan.owner())
                    || targets
                        .iter()
                        .any(|target| !obligations.iter().any(|row| row.owner() == *target))
                {
                    return Err(Issue::MapLifecycleConsumerMissing);
                }
            }
        }
        // Call-edge co-seal: every lifecycle-admitted loan edge that
        // carries a borrowed-Map formal must pair with the caller's
        // described `CallArgument` obligation at the same site/ordinal,
        // and the callee's own parameter contract must declare `Map`
        // there — signature ABI, contract kind, and caller actual agree
        // or the edge is refused. The reverse sweep then refuses any
        // described `CallArgument` obligation no sealed edge covers
        // (for example a map argument riding a non-lifecycle call).
        let mut call_edges = Vec::new();
        if let Some(loans) = self.direct_call_loans.as_ref() {
            for loan in loans.iter() {
                for (site, ordinal, callee) in loan.map_argument_edges() {
                    let contract_map = self.parameter_contracts.iter().any(|row| {
                        row.owner == callee
                            && row.parameters.iter().any(|parameter| {
                                parameter.ordinal == ordinal
                                    && parameter.kind == CallableParameterContractKindV1::Map
                            })
                    });
                    let obligation_match = obligations.iter().any(|owner_rows| {
                        owner_rows.sites().iter().any(|row| {
                            matches!(
                                row.destination(),
                                MapDestinationV1::CallArgument { call, ordinal: expected }
                                    if call == &site && *expected == ordinal
                            )
                        })
                    });
                    if !contract_map || !obligation_match {
                        return Err(Issue::MapLifecycleConsumerMissing);
                    }
                    call_edges.push(MapCallEdgeContractV1::new(
                        site,
                        MapCallEdgeKindV1::Argument { ordinal },
                    ));
                }
            }
        }
        for owner_rows in obligations.iter() {
            for row in owner_rows.sites() {
                let MapDestinationV1::CallArgument { call, ordinal } = row.destination() else {
                    continue;
                };
                let covered = call_edges.iter().any(|edge| {
                    edge.call_site() == call
                        && matches!(
                            edge.kind(),
                            MapCallEdgeKindV1::Argument { ordinal: expected }
                                if *expected == *ordinal
                        )
                });
                if !covered {
                    return Err(Issue::MapLifecycleConsumerMissing);
                }
            }
        }
        let undertaking = undertaking.with_call_edges(call_edges);
        for owner in obligations.iter().map(|row| row.owner()) {
            let mut declarations = self.batch.declarations().filter(|d| d.owner() == owner);
            let declaration = declarations
                .next()
                .ok_or(Issue::MapLifecycleConsumerMissing)?;
            if declarations.next().is_some() {
                return Err(Issue::MapLifecycleConsumerMissing);
            }
            self.batch
                .with_lowering_input(declaration.batch_slot(), |input| {
                    let function = input.function();
                    for original in function.expression_source().initializers() {
                        if !matches!(
                            original.declaration_site(),
                            SourceBindingSiteV1::Local { .. }
                        ) {
                            continue;
                        }
                        let mut current = original;
                        let mut seen = BTreeSet::new();
                        loop {
                            if !seen.insert(current.binding()) {
                                return Err(Issue::MapLifecycleConsumerMissing);
                            }
                            let Some(site) = current.initializer_site() else {
                                break;
                            };
                            let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                                owner,
                                site.clone(),
                            );
                            if let Some(destination) = self
                                .ordinary_new_claim_ledger
                                .map_call_source_binding(&owned)
                            {
                                // A call site is a map source without a
                                // literal flow row: the sealed local-call
                                // relation's destination is the binding.
                                if destination != current.binding() {
                                    return Err(Issue::MapLifecycleConsumerMissing);
                                }
                                crate::mir::builder::validate_map_local_annotation(
                                    original.declared_type_name(),
                                )
                                .map_err(|error| Issue::MapLocalAnnotation(error.into()))?;
                                break;
                            }
                            if self.ordinary_new_claim_ledger.has_map_source(&owned) {
                                let map = self
                                    .ordinary_new_claim_ledger
                                    .map_flow(&owned)
                                    .map_err(|_| Issue::MapLifecycleConsumerMissing)?;
                                if map.local_binding() != Some(current.binding()) {
                                    return Err(Issue::MapLifecycleConsumerMissing);
                                }
                                crate::mir::builder::validate_map_local_annotation(
                                    original.declared_type_name(),
                                )
                                .map_err(|error| Issue::MapLocalAnnotation(error.into()))?;
                                break;
                            }
                            let Some(ResolvedLexicalRefV1::Local(binding)) =
                                function.variable_ref(site)
                            else {
                                break;
                            };
                            let Some(record) = function.binding(binding) else {
                                return Err(Issue::MapLifecycleConsumerMissing);
                            };
                            let BindingOriginV1::Source(declaration) = record.origin() else {
                                break;
                            };
                            let Some(next) = function.expression_source().initializer(declaration)
                            else {
                                break;
                            };
                            if next.binding() != binding {
                                return Err(Issue::MapLifecycleConsumerMissing);
                            }
                            current = next;
                        }
                    }
                    Ok(())
                })
                .map_err(|_| Issue::BatchLoan)??;
        }
        Ok(Some(undertaking))
    }
}
