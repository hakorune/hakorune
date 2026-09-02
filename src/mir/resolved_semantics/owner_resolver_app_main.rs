//! App Main-only callable-index co-issue.
//!
//! This is kept outside the general owner-forest owner so the Main-specific
//! source/target relation does not inflate the common resolver module or leak
//! a batch-wide index into unrelated lowering inputs.

use super::*;
use crate::mir::resolved_semantics::{
    CallableHeaderSyntaxViewV1, VerifiedOwnerFreeCallableHeaderV1,
};

/// A callable header is either an exact candidate for the bounded App Main
/// FreeStatic index or an explicitly out-of-profile helper.  Keeping the
/// latter as a named state prevents an `Err(..) => continue` branch from
/// looking like an accidental loss of source coverage.
enum AppMainFreeStaticHeaderDispositionV1 {
    Candidate(VerifiedOwnerFreeCallableHeaderV1),
    OutsideExactProfile,
}

fn classify_app_main_free_static_header_v1(
    view: CallableHeaderSyntaxViewV1<'_>,
    top_level: bool,
) -> AppMainFreeStaticHeaderDispositionV1 {
    let result = if top_level {
        VerifiedOwnerFreeCallableHeaderV1::seal_top_level(view)
    } else {
        VerifiedOwnerFreeCallableHeaderV1::seal(view)
    };
    match result {
        Ok(header) => AppMainFreeStaticHeaderDispositionV1::Candidate(header),
        Err(_outside_exact_profile) => AppMainFreeStaticHeaderDispositionV1::OutsideExactProfile,
    }
}

impl FunctionSemanticResolverSessionV1 {
    /// Co-issue the exact App Main root direct-call targets from this same
    /// source traversal and resolver session. Non-Main roots remain
    /// observer-only; children of the Main root are explicitly unindexed so a
    /// root index cannot cross a lambda boundary.
    pub(crate) fn resolve_source_bound_selected_callable_forests_with_main_freestatic_targets(
        &mut self,
        inputs: &[SelectedCallableResolverInputV1<'_>],
        brand_catalog: Option<&VerifiedBrandProgramDeclarationCatalogV1>,
        app_main_identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> Result<
        ResolveSourceBoundSelectedCallableForestsWithAppMainFreeStaticOutcomeV1,
        SourceBoundSelectedCallableResolverRejectV1,
    > {
        let mut trees = Vec::with_capacity(inputs.len());
        let mut first_deferred = None;
        let mut deferred_rest = Vec::new();
        for input in inputs {
            match construct_selected_callable_owner_tree_v1(input.view(), brand_catalog) {
                Ok(tree) => trees.push((input, tree)),
                Err(error) => match selected_callable_source_deferral(error) {
                    Ok(observation) => {
                        let deferred = SelectedCallableResolverDeferredV1::from_parts(
                            input.source().clone(),
                            observation,
                        );
                        if first_deferred.is_none() {
                            first_deferred = Some(deferred);
                        } else {
                            deferred_rest.push(deferred);
                        }
                    }
                    Err(error) => {
                        return Err(SourceBoundSelectedCallableResolverRejectV1::from_parts(
                            input.source().clone(),
                            error,
                        ));
                    }
                },
            }
        }
        if let Some(first) = first_deferred {
            return Ok(
                ResolveSourceBoundSelectedCallableForestsWithAppMainFreeStaticOutcomeV1::Deferred(
                    SelectedCallableResolverDeferredBatchV1::from_non_empty_parts(
                        first,
                        deferred_rest.into_boxed_slice(),
                    ),
                ),
            );
        }

        // Reserve every root owner before sealing any forest. This is the sole
        // owner/identity assignment for the batch and the index below refers
        // to exactly the same resolver session.
        let mut reserved = Vec::with_capacity(trees.len());
        for (input, tree) in trees {
            let (origin, owner) = self.issue_owner().map_err(|error| {
                SourceBoundSelectedCallableResolverRejectV1::from_parts(
                    input.source().clone(),
                    ResolveOwnerForestErrorV1::Function(error),
                )
            })?;
            reserved.push((input, tree, origin, owner));
        }

        let mut headers = Vec::new();
        for (input, _tree, _origin, owner) in &reserved {
            if !input.is_free_static_index_candidate() {
                continue;
            }
            let Some(header) = input.header() else {
                return Err(Self::reject_app_main(
                    input.source().clone(),
                    AppMainFreeStaticResolverIssueV1::HeaderMissing,
                ));
            };
            // Unsupported headers are deliberately not index candidates. A
            // call-free App Main must remain installable even when an
            // unrelated helper lies outside the exact FreeStatic profile;
            // a direct call to that helper is rejected by the root policy
            // below instead of being repaired from its name or arity.
            match classify_app_main_free_static_header_v1(header, input.is_top_level_callable()) {
                AppMainFreeStaticHeaderDispositionV1::Candidate(header) => {
                    headers.push(header.attach_owner(*owner))
                }
                AppMainFreeStaticHeaderDispositionV1::OutsideExactProfile => {}
            }
        }
        let callable_index = if headers.is_empty() {
            None
        } else {
            Some(
                super::super::VerifiedCallableIndexV1::seal_many(headers).map_err(|_| {
                    Self::reject_app_main(
                        inputs
                            .first()
                            .map(|input| input.source().clone())
                            .unwrap_or_else(|| {
                                SelectedCallableResolverSourceIdentityV1::Callable {
                                    identity: app_main_identity.clone(),
                                    diagnostic_owner: None,
                                    diagnostic_name: "Main".into(),
                                }
                            }),
                        AppMainFreeStaticResolverIssueV1::IndexSeal,
                    )
                })?,
            )
        };

        let mut forests = Vec::with_capacity(reserved.len());
        let mut body_shapes = BTreeMap::new();
        for (input, tree, origin, owner) in reserved {
            let is_app_main = input
                .source()
                .callable_identity()
                .is_some_and(|identity| identity.same_as(app_main_identity));
            if is_app_main {
                if let Some(issue) =
                    app_main_direct_call_policy_issue(&tree, callable_index.as_ref())
                {
                    return Err(Self::reject_app_main(input.source().clone(), issue));
                }
            }
            let policy = if is_app_main && callable_index.is_some() {
                DirectCallCanonicalizationPolicyV1::RequireCallableIndexAtRoot
            } else if is_app_main {
                DirectCallCanonicalizationPolicyV1::RejectUnindexed
            } else {
                DirectCallCanonicalizationPolicyV1::ObserveOnly
            };
            let root_callable_index = is_app_main.then_some(callable_index.as_ref()).flatten();
            let mut draft = SemanticOwnerForestDraftV1::new();
            if let Err(error) = self.seal_owner_tree(
                tree,
                &BTreeMap::new(),
                None,
                Some((origin, owner)),
                root_callable_index,
                policy,
                &mut draft,
                Some(&mut body_shapes),
            ) {
                return Err(SourceBoundSelectedCallableResolverRejectV1::from_parts(
                    input.source().clone(),
                    error,
                ));
            }
            let forest = draft.seal().map_err(|error| {
                SourceBoundSelectedCallableResolverRejectV1::from_parts(
                    input.source().clone(),
                    ResolveOwnerForestErrorV1::Verification(error),
                )
            })?;
            forests.push(forest);
        }
        Ok(
            ResolveSourceBoundSelectedCallableForestsWithAppMainFreeStaticOutcomeV1::Complete {
                forests: forests.into_boxed_slice(),
                body_shapes,
                callable_index,
            },
        )
    }

    fn reject_app_main(
        source: SelectedCallableResolverSourceIdentityV1,
        issue: AppMainFreeStaticResolverIssueV1,
    ) -> SourceBoundSelectedCallableResolverRejectV1 {
        SourceBoundSelectedCallableResolverRejectV1::from_parts(
            source,
            ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::AppMainDirectCall(issue)),
        )
    }
}

/// Check the only direct-call policy split before canonicalizing the forest.
/// The root may use the same-session index when one exists; recursive owners
/// never inherit it.  With no exact index, even a root call is an explicit
/// missing-relation terminal rather than a name/arity repair.
fn app_main_direct_call_policy_issue(
    tree: &ShadowOwnerConstructionTreeV1<'_>,
    callable_index: Option<&super::super::VerifiedCallableIndexV1>,
) -> Option<AppMainFreeStaticResolverIssueV1> {
    if !tree.function.direct_calls.is_empty() {
        let Some(index) = callable_index else {
            return Some(AppMainFreeStaticResolverIssueV1::TargetMissing);
        };
        for call in tree.function.direct_calls.values() {
            if index
                .resolve_free_static_source_call(&call.name, call.arity)
                .is_err()
            {
                let same_name_exists = index
                    .headers()
                    .any(|header| header.source_key().name() == call.name.as_ref());
                return Some(if same_name_exists {
                    AppMainFreeStaticResolverIssueV1::ArityMismatch
                } else {
                    AppMainFreeStaticResolverIssueV1::TargetMissing
                });
            }
        }
    }
    if tree
        .children
        .iter()
        .any(|child| tree_has_direct_call(&child.tree))
    {
        return Some(AppMainFreeStaticResolverIssueV1::NestedOwnerObservation);
    }
    None
}

fn tree_has_direct_call(tree: &ShadowOwnerConstructionTreeV1<'_>) -> bool {
    !tree.function.direct_calls.is_empty()
        || tree
            .children
            .iter()
            .any(|child| tree_has_direct_call(&child.tree))
}
