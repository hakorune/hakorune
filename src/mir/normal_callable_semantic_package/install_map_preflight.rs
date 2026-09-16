//! Precommit use of existing root and Local products; no semantic issuance.
use super::super::map_lifecycle_undertaking::{
    verify_map_lifecycle_undertaking, MapLifecycleUndertakingV1,
};
use super::*;
use crate::mir::builder::BuilderInstallConsumerV1;
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
            .map_err(|_| Issue::MapLifecycleConsumerMissing)?;
        if obligations.is_empty() {
            return Ok(None);
        }
        let undertaking = verify_map_lifecycle_undertaking(
            &obligations,
            BuilderInstallConsumerV1::map_lifecycle_capability(),
        )
        .map_err(|_| Issue::MapLifecycleConsumerMissing)?;
        // AppMain execution-product evidence stays scoped to the loan
        // that carries it: the affine rows must be unspent at install,
        // every map-carrying loan target must have described obligations,
        // and the loan never targets its own owner. Per-owner lane
        // admissibility still applies inside this lane; what is gone is
        // the fixed call-count/reachability shape bound, which the
        // undertaking replaces as coverage proof.
        if let Some(loan) = self.direct_call_loan.as_ref() {
            if loan.has_taken_slot() {
                return Err(Issue::MapLifecycleConsumerMissing);
            }
            self.ordinary_new_claim_ledger
                .map_install_owners()
                .map_err(|()| Issue::MapLifecycleConsumerMissing)?;
            let targets = loan.map_target_owners(&self.batch).unwrap_or_default();
            if targets.iter().any(|target| *target == loan.owner())
                || targets
                    .iter()
                    .any(|target| !obligations.iter().any(|row| row.owner() == *target))
            {
                return Err(Issue::MapLifecycleConsumerMissing);
            }
        }
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
