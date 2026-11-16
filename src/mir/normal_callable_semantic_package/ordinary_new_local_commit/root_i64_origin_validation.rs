//! Mandatory post-CFG origin validation over the existing finished groups.
use super::*;

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
                    let context = CallValidationContextV1::Analyzed(self, &analysis);
                    for group in RootLifecyclePrefixBindingGroupV1::snapshot_tree_v2(roots) {
                        group.validate_recorded_in_context(context, function, Some(projection))?;
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
