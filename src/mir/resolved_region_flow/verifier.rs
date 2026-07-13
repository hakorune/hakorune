//! Seal-time validation for owned resolved flow products.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ScopeId, SourceExprSiteV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

use super::if_flow::{
    ResolvedFunctionFlowDraftV1, ResolvedIfFlowDraftV1, VerifiedResolvedFunctionFlowV1,
    VerifiedResolvedIfFlowV1,
};
use super::ports::{
    ResolvedElseFallthroughV1, ResolvedFallthroughPortV1, ResolvedIfConditionEffectsV1,
    ResolvedIfJoinBindingV1, ResolvedIfJoinContractV1, ResolvedIfPortValueSourceV1,
    ResolvedIfWholeEffectsV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedRegionFlowVerificationErrorV1 {
    OwnerMismatch {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    MissingIfBundle(SourceStmtSiteV1),
    OptionalElseMismatch {
        site: SourceStmtSiteV1,
        syntax_has_else: bool,
        bundle_has_else: bool,
    },
    MissingScope(ScopeId),
    MissingBinding(BindingRefV1),
    ForeignBinding(BindingRefV1),
    UnrelatedBindingScope {
        binding: BindingRefV1,
        entry_scope: ScopeId,
        owner_scope: ScopeId,
    },
    ScopeParentCycle(ScopeId),
    BranchParentMismatch(SourceStmtSiteV1),
    InvalidIfRowSlot {
        slot: usize,
    },
    DuplicateIfRowSlot {
        slot: usize,
    },
    IfRowSiteMismatch {
        expected: SourceStmtSiteV1,
        actual: SourceStmtSiteV1,
    },
    MissingIfRow(SourceStmtSiteV1),
    DuplicateIfSite(SourceStmtSiteV1),
    IfBundleCardinalityMismatch {
        expected: usize,
        actual: usize,
    },
    DuplicateAssignmentCoverage(SourceExprSiteV1),
    MissingAssignmentCoverage(SourceExprSiteV1),
    UnexpectedAssignmentCoverage(SourceExprSiteV1),
}

pub(super) fn verify_if_flow_draft(
    function: &VerifiedResolvedFunctionV1,
    draft: ResolvedIfFlowDraftV1,
) -> Result<VerifiedResolvedIfFlowV1, ResolvedRegionFlowVerificationErrorV1> {
    let regions = *function
        .if_region_bundle(&draft.site)
        .map_err(|_| ResolvedRegionFlowVerificationErrorV1::MissingIfBundle(draft.site.clone()))?;
    let bundle_has_else = regions.else_pair().is_some();
    if draft.syntax_has_else != bundle_has_else
        || draft.else_effects.is_some() != draft.syntax_has_else
    {
        return Err(
            ResolvedRegionFlowVerificationErrorV1::OptionalElseMismatch {
                site: draft.site,
                syntax_has_else: draft.syntax_has_else,
                bundle_has_else,
            },
        );
    }

    let then_scope = regions.then_pair().scope();
    let surrounding_scope = function
        .scope(then_scope)
        .ok_or(ResolvedRegionFlowVerificationErrorV1::MissingScope(
            then_scope,
        ))?
        .parent()
        .ok_or_else(|| {
            ResolvedRegionFlowVerificationErrorV1::BranchParentMismatch(draft.site.clone())
        })?;
    if let Some(else_pair) = regions.else_pair() {
        let else_scope = else_pair.scope();
        let else_parent = function
            .scope(else_scope)
            .ok_or(ResolvedRegionFlowVerificationErrorV1::MissingScope(
                else_scope,
            ))?
            .parent();
        if else_parent != Some(surrounding_scope) {
            return Err(ResolvedRegionFlowVerificationErrorV1::BranchParentMismatch(
                draft.site,
            ));
        }
    }

    let condition = outer_effects(function, surrounding_scope, draft.condition_effects)?;
    let then_bindings = outer_effects(function, surrounding_scope, draft.then_effects)?;
    let else_bindings = match draft.else_effects {
        Some(effects) => Some(outer_effects(function, surrounding_scope, effects)?),
        None => None,
    };
    let join_rows = join_rows(&then_bindings, else_bindings.as_deref());
    let mut whole_bindings = condition.clone();
    extend_unique(
        &mut whole_bindings,
        join_rows.iter().map(|row| row.binding()),
    );

    let condition_effects = ResolvedIfConditionEffectsV1::from_verified(condition);
    let then_port = ResolvedFallthroughPortV1::from_verified(then_bindings);
    let else_port = match else_bindings {
        Some(bindings) => {
            ResolvedElseFallthroughV1::Explicit(ResolvedFallthroughPortV1::from_verified(bindings))
        }
        None => ResolvedElseFallthroughV1::ImplicitIdentity,
    };
    Ok(VerifiedResolvedIfFlowV1::from_verified(
        draft.site,
        regions,
        condition_effects,
        then_port,
        else_port,
        ResolvedIfJoinContractV1::from_verified(join_rows),
        ResolvedIfWholeEffectsV1::from_verified(whole_bindings),
        draft.coverage.into_verified(),
    ))
}

pub(super) fn seal_resolved_function_flow_v1(
    draft: ResolvedFunctionFlowDraftV1,
    function: &VerifiedResolvedFunctionV1,
) -> Result<VerifiedResolvedFunctionFlowV1, ResolvedRegionFlowVerificationErrorV1> {
    let (owner, expected_sites, row_slots, coverage_draft) = draft.into_parts();
    if owner != function.owner() {
        return Err(ResolvedRegionFlowVerificationErrorV1::OwnerMismatch {
            expected: function.owner(),
            actual: owner,
        });
    }
    if expected_sites.len() != function.if_region_bundle_count() {
        return Err(
            ResolvedRegionFlowVerificationErrorV1::IfBundleCardinalityMismatch {
                expected: function.if_region_bundle_count(),
                actual: expected_sites.len(),
            },
        );
    }

    let mut seen_if_sites = BTreeSet::new();
    let mut rows = Vec::with_capacity(row_slots.len());
    for (expected, row) in expected_sites.into_iter().zip(row_slots) {
        if !seen_if_sites.insert(expected.clone()) {
            return Err(ResolvedRegionFlowVerificationErrorV1::DuplicateIfSite(
                expected,
            ));
        }
        let row = row
            .ok_or_else(|| ResolvedRegionFlowVerificationErrorV1::MissingIfRow(expected.clone()))?;
        let row_owner = row.regions().control().owner();
        if row_owner != owner {
            return Err(ResolvedRegionFlowVerificationErrorV1::OwnerMismatch {
                expected: owner,
                actual: row_owner,
            });
        }
        if row.site() != &expected {
            return Err(ResolvedRegionFlowVerificationErrorV1::IfRowSiteMismatch {
                expected,
                actual: row.site().clone(),
            });
        }
        rows.push(row);
    }

    verify_assignment_coverage(function, &coverage_draft, &rows)?;
    Ok(VerifiedResolvedFunctionFlowV1::from_verified(
        owner,
        rows,
        coverage_draft.into_verified(),
    ))
}

fn outer_effects(
    function: &VerifiedResolvedFunctionV1,
    entry_scope: ScopeId,
    effects: Vec<BindingRefV1>,
) -> Result<Vec<BindingRefV1>, ResolvedRegionFlowVerificationErrorV1> {
    let mut outer = Vec::new();
    for binding in effects {
        if binding.owner() != function.owner() {
            return Err(ResolvedRegionFlowVerificationErrorV1::ForeignBinding(
                binding,
            ));
        }
        let owner_scope = function
            .binding(binding)
            .ok_or(ResolvedRegionFlowVerificationErrorV1::MissingBinding(
                binding,
            ))?
            .owner_scope();
        if scope_is_ancestor(function, owner_scope, entry_scope)? {
            push_unique(&mut outer, binding);
        } else if !scope_is_ancestor(function, entry_scope, owner_scope)? {
            return Err(
                ResolvedRegionFlowVerificationErrorV1::UnrelatedBindingScope {
                    binding,
                    entry_scope,
                    owner_scope,
                },
            );
        }
    }
    Ok(outer)
}

fn scope_is_ancestor(
    function: &VerifiedResolvedFunctionV1,
    ancestor: ScopeId,
    mut scope: ScopeId,
) -> Result<bool, ResolvedRegionFlowVerificationErrorV1> {
    for _ in 0..=function.scope_count() {
        if scope == ancestor {
            return Ok(true);
        }
        let record = function
            .scope(scope)
            .ok_or(ResolvedRegionFlowVerificationErrorV1::MissingScope(scope))?;
        let Some(parent) = record.parent() else {
            return Ok(false);
        };
        scope = parent;
    }
    Err(ResolvedRegionFlowVerificationErrorV1::ScopeParentCycle(
        scope,
    ))
}

fn join_rows(
    then_bindings: &[BindingRefV1],
    else_bindings: Option<&[BindingRefV1]>,
) -> Vec<ResolvedIfJoinBindingV1> {
    let mut bindings = then_bindings.to_vec();
    if let Some(else_bindings) = else_bindings {
        extend_unique(&mut bindings, else_bindings.iter().copied());
    }
    bindings
        .into_iter()
        .map(|binding| {
            ResolvedIfJoinBindingV1::from_verified(
                binding,
                if then_bindings.contains(&binding) {
                    ResolvedIfPortValueSourceV1::BranchExit
                } else {
                    ResolvedIfPortValueSourceV1::PostConditionEntry
                },
                if else_bindings.is_some_and(|bindings| bindings.contains(&binding)) {
                    ResolvedIfPortValueSourceV1::BranchExit
                } else {
                    ResolvedIfPortValueSourceV1::PostConditionEntry
                },
            )
        })
        .collect()
}

fn verify_assignment_coverage(
    function: &VerifiedResolvedFunctionV1,
    function_coverage: &super::coverage::FunctionFlowCoverageDraftV1,
    rows: &[VerifiedResolvedIfFlowV1],
) -> Result<(), ResolvedRegionFlowVerificationErrorV1> {
    let expected = function
        .assignment_targets()
        .map(|(site, _)| site.clone())
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    for site in function_coverage
        .direct_sites()
        .chain(rows.iter().flat_map(|row| row.coverage().direct_sites()))
    {
        if !actual.insert(site.clone()) {
            return Err(
                ResolvedRegionFlowVerificationErrorV1::DuplicateAssignmentCoverage(site.clone()),
            );
        }
    }
    if let Some(site) = expected.difference(&actual).next() {
        return Err(ResolvedRegionFlowVerificationErrorV1::MissingAssignmentCoverage(site.clone()));
    }
    if let Some(site) = actual.difference(&expected).next() {
        return Err(
            ResolvedRegionFlowVerificationErrorV1::UnexpectedAssignmentCoverage(site.clone()),
        );
    }
    Ok(())
}

fn push_unique(bindings: &mut Vec<BindingRefV1>, binding: BindingRefV1) {
    if !bindings.contains(&binding) {
        bindings.push(binding);
    }
}

fn extend_unique(
    bindings: &mut Vec<BindingRefV1>,
    additional: impl IntoIterator<Item = BindingRefV1>,
) {
    for binding in additional {
        push_unique(bindings, binding);
    }
}
