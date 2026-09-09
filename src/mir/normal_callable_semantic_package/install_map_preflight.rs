//! Precommit use of existing root and Local products; no semantic issuance.
use super::*;
use crate::mir::resolved_semantics::{BindingOriginV1, ResolvedLexicalRefV1, SourceBindingSiteV1};

impl VerifiedNormalCallableSemanticPackageV1 {
    pub(super) fn preflight_map_install(
        &self,
    ) -> Result<(), NormalCallableSemanticPackageInstallIssueV1> {
        use NormalCallableSemanticPackageInstallIssueV1 as Issue;
        // Membership is already sealed per declared root. Never infer owning
        // admission from ledger presence or skip unissued non-AppMain Maps.
        for declaration in self.batch.declarations() {
            for expression in declaration.body_shape().expressions() {
                if let crate::mir::resolved_semantics::BodyExpressionShapeV1::MapLiteral { site, .. } = expression {
                    let owned = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                        declaration.owner(), site.clone(),
                    );
                    self.ordinary_new_claim_ledger.map_flow(&owned)
                        .map_err(|_| Issue::MapLifecycleConsumerMissing)?;
                }
            }
        }
        let owners = self
            .ordinary_new_claim_ledger
            .map_install_owners()
            .map_err(|()| Issue::MapLifecycleConsumerMissing)?;
        if owners.is_empty() {
            return Ok(());
        }
        let root_owner = self.ordinary_new_claim_ledger.root_owner();
        if let Some(loan) = self.app_main_direct_call_loan.as_ref() {
            let targets = loan
                .map_target_owners(&self.batch)
                .ok_or(Issue::MapLifecycleConsumerMissing)?;
            if targets.iter().any(|target| *target == loan.owner())
                || targets.iter().any(|target| !owners.contains(target))
                || owners.iter().any(|owner| {
                    Some(*owner) != root_owner && !targets.iter().any(|target| target == owner)
                })
            {
                return Err(Issue::MapLifecycleConsumerMissing);
            }
        } else if owners.iter().any(|owner| Some(*owner) != root_owner) {
            return Err(Issue::MapLifecycleConsumerMissing);
        }
        for owner in owners.iter().copied() {
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
                            if self.ordinary_new_claim_ledger.has_map_source(&owned) {
                                let map = self
                                    .ordinary_new_claim_ledger
                                    .map_flow(&owned)
                                    .map_err(|_| Issue::MapLifecycleConsumerMissing)?;
                                if map.destination() != current.binding() {
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
        Ok(())
    }
}
