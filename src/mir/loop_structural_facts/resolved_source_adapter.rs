//! Pure, consuming adapter from sealed Loop source identity to portable source.

use crate::mir::loop_recipe_contract::{
    LoopNodeSourceBindingV1, LoopRecipeSourceBindingV1, LoopRecipeSourceOwnerV1,
    LoopSourcePathStepV1, LoopSourcePathV1, VerifiedLoopRecipeV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SemanticOwnerSourceKindV1, SourcePathSegmentV1, VerifiedResolvedLoopSourceV1,
};

/// Fail-closed boundary for source shapes not represented by the portable DTO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopRootSourceBindingRejectV1 {
    UnsupportedOwnerRoot(SemanticOwnerSourceKindV1),
    MissingFunctionBodyItem,
    UnsupportedRoot(SourcePathSegmentV1),
    UnsupportedAncestor {
        depth: u32,
        segment: SourcePathSegmentV1,
    },
    OrphanBodyRoot {
        depth: u32,
        segment: SourcePathSegmentV1,
    },
}

/// Non-Clone proof that one resolved Loop root has portable source identity.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopRootSourceV1 {
    owner: LoopRecipeSourceOwnerV1,
    path: LoopSourcePathV1,
}

impl VerifiedLoopRootSourceV1 {
    /// Consumes local authority and binds it to the verified semantic root.
    ///
    /// The caller cannot inject an arbitrary Loop key; the semantic verifier
    /// is the sole authority for canonical root identity.
    pub(crate) fn into_root_claim(
        self,
        recipe: &VerifiedLoopRecipeV1,
    ) -> LoopRecipeSourceBindingV1 {
        LoopRecipeSourceBindingV1::new(
            self.owner,
            vec![LoopNodeSourceBindingV1::new(recipe.root_loop(), self.path)],
        )
    }
}

/// Converts one sealed resolved Loop token without consulting syntax or routes.
///
/// The resolved statement grammar omits body-root marker segments. The
/// portable vocabulary preserves those markers through its owner root and
/// typed `*BodyItem` steps, so this projection is lossless.
pub(crate) fn bind_resolved_loop_root_v1(
    source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedLoopRootSourceV1, LoopRootSourceBindingRejectV1> {
    let (origin, source_kind, site) = source.into_parts();
    let owner = portable_function_owner_v1(origin, source_kind)?;
    let path = portable_path_v1(site.node().segments())?;
    Ok(VerifiedLoopRootSourceV1 { owner, path })
}

fn portable_function_owner_v1(
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
) -> Result<LoopRecipeSourceOwnerV1, LoopRootSourceBindingRejectV1> {
    if source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(LoopRootSourceBindingRejectV1::UnsupportedOwnerRoot(
            source_kind,
        ));
    }
    Ok(LoopRecipeSourceOwnerV1::function_body(
        origin.compilation_unit_ordinal(),
        origin.function_ordinal(),
    ))
}

fn portable_path_v1(
    segments: &[SourcePathSegmentV1],
) -> Result<LoopSourcePathV1, LoopRootSourceBindingRejectV1> {
    let Some((root, ancestors)) = segments.split_first() else {
        return Err(LoopRootSourceBindingRejectV1::MissingFunctionBodyItem);
    };
    let SourcePathSegmentV1::Body(index) = root else {
        return Err(if is_body_root(root) {
            LoopRootSourceBindingRejectV1::OrphanBodyRoot {
                depth: 0,
                segment: root.clone(),
            }
        } else {
            LoopRootSourceBindingRejectV1::UnsupportedRoot(root.clone())
        });
    };

    let mut steps = Vec::with_capacity(segments.len());
    steps.push(LoopSourcePathStepV1::BodyItem { index: *index });
    for (offset, segment) in ancestors.iter().enumerate() {
        let depth = offset as u32 + 1;
        let step = match segment {
            SourcePathSegmentV1::ScopeBody(index) => {
                LoopSourcePathStepV1::ScopeBodyItem { index: *index }
            }
            SourcePathSegmentV1::LoopBody(index) => {
                LoopSourcePathStepV1::LoopBodyItem { index: *index }
            }
            segment if is_body_root(segment) => {
                return Err(LoopRootSourceBindingRejectV1::OrphanBodyRoot {
                    depth,
                    segment: segment.clone(),
                });
            }
            unsupported => {
                return Err(LoopRootSourceBindingRejectV1::UnsupportedAncestor {
                    depth,
                    segment: unsupported.clone(),
                });
            }
        };
        steps.push(step);
    }
    Ok(LoopSourcePathV1::new(steps))
}

fn is_body_root(segment: &SourcePathSegmentV1) -> bool {
    matches!(
        segment,
        SourcePathSegmentV1::FunctionBody
            | SourcePathSegmentV1::ProgramBodyRoot
            | SourcePathSegmentV1::ScopeBodyRoot
            | SourcePathSegmentV1::TaskScopeBodyRoot
            | SourcePathSegmentV1::FastMemBodyRoot
            | SourcePathSegmentV1::IfThenBody
            | SourcePathSegmentV1::IfElseBody
            | SourcePathSegmentV1::LoopBodyRoot
            | SourcePathSegmentV1::LambdaBodyRoot
            | SourcePathSegmentV1::BlockExprPreludeRoot
            | SourcePathSegmentV1::TryBodyRoot
            | SourcePathSegmentV1::CatchBodyRoot
            | SourcePathSegmentV1::CleanupBodyRoot
    )
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn explicit_body_root_without_a_compact_item_is_rejected() {
        assert_eq!(
            portable_path_v1(&[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::ScopeBodyRoot,
            ]),
            Err(LoopRootSourceBindingRejectV1::OrphanBodyRoot {
                depth: 1,
                segment: SourcePathSegmentV1::ScopeBodyRoot,
            })
        );
    }

    #[test]
    fn match_ancestor_is_outside_the_closed_portable_grammar() {
        assert_eq!(
            portable_path_v1(&[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::MatchArm(1),
            ]),
            Err(LoopRootSourceBindingRejectV1::UnsupportedAncestor {
                depth: 1,
                segment: SourcePathSegmentV1::MatchArm(1),
            })
        );
    }

    #[test]
    fn program_owner_has_no_portable_issuer_in_this_slice() {
        assert_eq!(
            portable_function_owner_v1(
                FunctionOriginV1::new(0, 0),
                SemanticOwnerSourceKindV1::Script,
            ),
            Err(LoopRootSourceBindingRejectV1::UnsupportedOwnerRoot(
                SemanticOwnerSourceKindV1::Script,
            ))
        );
    }
}
