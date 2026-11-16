//! Mandatory post-CFG origin validation over the existing finished groups.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::RootHomeExitKeyV1;

impl OrdinaryNewClaimLedgerV1 {
    /// All ordinary, document and artifact completions cross this module gate.
    /// It lends existing groups and finished projections, not a second ledger.
    pub(in crate::mir) fn validate_finished_i64_origins_v1(
        &self,
        module: &crate::mir::MirModule,
    ) -> Result<(), String> {
        let groups = self.root_local_call_bindings.borrow();
        for (owner, roots) in groups.iter().filter(|(_, roots)| !roots.is_empty()) {
            let _source_scope = self.branch_source_scope_v1(*owner);
            let validate = |symbol: &str, projection: &physical_boundary::FinishedBindings| {
                let function = module
                    .functions
                    .get(symbol)
                    .ok_or_else(|| freeze("child-definition-missing"))?;
                let _physical_scope =
                    self.physical_validation_scope_v1(function, Some(projection), false);
                self.with_function_geometry_v1(function, |geometry| {
                    let analysis =
                        crate::mir::verification::scalar_origin::FinishedI64OriginsV1::new(
                            geometry,
                            Some(module),
                        );
                    for group in roots {
                        group.visit_i64_origins_v1(&mut |read| {
                            read.validate_finished_i64_origin_v1(&analysis, projection)
                        })?;
                    }
                    Ok(())
                })
            };
            let root = self.root_validation.borrow();
            match &*root {
                RootNewValidation::FinishingChecked {
                    owner: root_owner,
                    symbol,
                    projection,
                }
                | RootNewValidation::ArtifactFinalized {
                    owner: root_owner,
                    symbol,
                    projection,
                } if root_owner == owner => validate(symbol, projection)?,
                _ => {
                    drop(root);
                    self.with_finished_child_projection(*owner, validate)?;
                }
            }
        }
        Ok(())
    }
}

impl FinalizedRootSourceHandoffV1 {
    /// Preserve the existing explicit cleanup/publication coverage. Implicit
    /// exits and plain non-lifecycle child exits never carried a module query.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn checks_explicit_exit_origin_v1(
        &self,
        owner: FunctionOwnerIdV1,
        entry: &RootHomeExitEntry,
        lifecycle_owners: &std::collections::BTreeSet<FunctionOwnerIdV1>,
    ) -> bool {
        owner == self.owner
            || matches!(entry, RootHomeExitEntry::Call { .. })
            || lifecycle_owners.contains(&owner)
    }
    /// The public verifier already corroborated source, operands and lifetimes.
    /// Borrow the SAME records, and the verifier's existing immutable indices.
    /// New arguments keep their historical publication boundary, not the
    /// earlier builder group gate.
    pub(in crate::mir) fn validate_finished_i64_origins_v1<'module>(
        &self,
        module: &'module crate::mir::MirModule,
        geometries: &std::collections::HashMap<
            *const MirFunction,
            crate::mir::verification::geometry::FunctionVerificationGeometry<'module>,
        >,
    ) -> Result<(), String> {
        let commits = self.ledger.local_commits.borrow();
        let exits = self.ledger.root_exits.borrow();
        let lifecycle_owners = self.initializer_lifecycle_owners_v1();
        let checked_exit =
            |owner: FunctionOwnerIdV1, key: &RootHomeExitKeyV1, progress: &RootHomeExitProgress| {
                key.explicit_site().is_some()
                    && match progress {
                        RootHomeExitProgress::Finalized(_) => owner == self.owner,
                        RootHomeExitProgress::Emitted { entry, .. } => {
                            self.checks_explicit_exit_origin_v1(owner, entry, &lifecycle_owners)
                        }
                        _ => false,
                    }
            };
        let order_has_records = |order: &root_home::RootHomeCleanupOrderV1| {
            order.return_expression_v1().is_some() || order.installed_map_return().is_some()
        };
        let entry_has_records = |entry: &RootHomeExitEntry| {
            matches!(
                entry,
                RootHomeExitEntry::Call { .. }
                    | RootHomeExitEntry::MapGet {
                        stored: Some(_),
                        ..
                    }
            )
        };
        let owners: std::collections::BTreeSet<_> = self
            .local_calls
            .iter()
            .filter(|(_, roots)| !roots.is_empty())
            .map(|(&owner, _)| owner)
            .chain(commits.iter().filter_map(|(site, row)| {
                match row.new_emission() {
                    Some(NewEmissionProgress::Emitted { arguments, .. })
                        if arguments
                            .iter()
                            .any(|argument| argument.expression_v1().is_some()) =>
                    {
                        Some(site.owner())
                    }
                    _ => None,
                }
            }))
            .chain(
                exits
                    .iter()
                    .filter(|((owner, key), progress)| checked_exit(*owner, key, progress))
                    .filter_map(|((owner, _), progress)| match progress {
                        RootHomeExitProgress::Emitted { order, entry, .. }
                            if order_has_records(order) || entry_has_records(entry) =>
                        {
                            Some(*owner)
                        }
                        RootHomeExitProgress::Finalized(order) if order_has_records(order) => {
                            Some(*owner)
                        }
                        _ => None,
                    }),
            )
            .chain(
                self.call_entries
                    .values()
                    .filter(|(entry, _)| entry_has_records(entry))
                    .map(|_| self.owner),
            )
            .collect();
        for owner in owners {
            self.ledger.with_finished_projection(owner, |symbol, projection| {
                let function = module.functions.get(symbol).filter(|function| function.signature.name == symbol)
                    .ok_or_else(|| freeze("final-call-visit/function-missing"))?;
                let geometry = geometries.get(&(function as *const MirFunction))
                    .ok_or_else(|| freeze("final-call-visit/function-missing"))?;
                let analysis = crate::mir::verification::scalar_origin::FinishedI64OriginsV1::new(geometry, Some(module));
                let mut check = |read: &crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1|
                    read.validate_finished_i64_origin_v1(&analysis, projection);
                if let Some(groups) = self.local_calls.get(&owner) {
                    for group in groups { group.visit_i64_origins_v1(&mut check)?; }
                }
                for (_, row) in commits.iter().filter(|(site, _)| site.owner() == owner) {
                    let Some(NewEmissionProgress::Emitted { arguments, .. }) = row.new_emission() else { continue; };
                    for argument in arguments {
                        if let Some(expression) = argument.expression_v1() { expression.visit_i64_origins_v1(&mut check)?; }
                    }
                }
                for (_, progress) in exits.iter().filter(|((original, key), progress)| *original == owner && checked_exit(*original, key, progress)) {
                    match progress {
                        RootHomeExitProgress::Emitted { order, entry, .. } => {
                            order.visit_i64_origins_v1(&mut check)?;
                            entry.visit_i64_origins_v1(&mut check)?;
                        }
                        RootHomeExitProgress::Finalized(order) => order.visit_i64_origins_v1(&mut check)?,
                        RootHomeExitProgress::Unprepared | RootHomeExitProgress::Unavailable
                        | RootHomeExitProgress::Prepared(_) | RootHomeExitProgress::Emitting(_) => {},
                    }
                }
                if owner == self.owner {
                    for (entry, _) in self.call_entries.values() { entry.visit_i64_origins_v1(&mut check)?; }
                }
                Ok(())
            })?;
        }
        Ok(())
    }
}
