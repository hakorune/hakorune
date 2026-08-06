//! Pure, consuming adapter from sealed Loop source identity to portable source.

use crate::mir::loop_recipe_contract::{
    LoopNodeKeyV1, LoopNodeSourceBindingV1, LoopRecipeSourceBindingV1, LoopRecipeSourceOwnerV1,
    LoopSourcePathStepV1, LoopSourcePathV1, VerifiedLoopRecipeV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SemanticOwnerSourceKindV1, SourcePathSegmentV1,
    VerifiedResolvedLoopSourceForestV1, VerifiedResolvedLoopSourceV1,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopSourceForestBindingRejectV1 {
    Source {
        member_index: u32,
        reason: LoopRootSourceBindingRejectV1,
    },
    SourceForestEmpty,
    SourceForestOwnerMismatch {
        member_index: u32,
    },
    ParentIndexOutOfRange {
        member_index: u32,
        parent_index: u32,
    },
    RootParentMismatch,
    RecipeLoopCoverageMismatch {
        expected: usize,
        found: usize,
    },
    RecipeParentMismatch {
        member_index: u32,
        expected: Option<LoopNodeKeyV1>,
        found: Option<LoopNodeKeyV1>,
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

/// Non-`Clone` portable projection that retains the forest's local parent
/// indices until a verified Recipe is borrowed for the final wire conversion.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopSourceForestBindingV1 {
    owner: LoopRecipeSourceOwnerV1,
    members: Box<[VerifiedLoopSourceForestBindingMemberV1]>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopSourceForestBindingMemberV1 {
    path: LoopSourcePathV1,
    parent_index: Option<u32>,
}

impl VerifiedLoopSourceForestBindingMemberV1 {
    pub(crate) fn path(&self) -> &LoopSourcePathV1 {
        &self.path
    }

    pub(crate) const fn parent_index(&self) -> Option<u32> {
        self.parent_index
    }
}

impl VerifiedLoopSourceForestBindingV1 {
    pub(crate) fn members(&self) -> &[VerifiedLoopSourceForestBindingMemberV1] {
        &self.members
    }

    pub(crate) fn owner(&self) -> LoopRecipeSourceOwnerV1 {
        self.owner
    }

    pub(crate) fn into_source_binding(
        self,
        recipe: &VerifiedLoopRecipeV1,
    ) -> Result<LoopRecipeSourceBindingV1, LoopSourceForestBindingRejectV1> {
        let expected = recipe.as_recipe().loops.len();
        let found = self.members.len();
        if expected != found {
            return Err(
                LoopSourceForestBindingRejectV1::RecipeLoopCoverageMismatch { expected, found },
            );
        }

        for (index, member) in self.members.iter().enumerate() {
            let member_index = index as u32;
            let found_parent = member.parent_index.map(LoopNodeKeyV1::new);
            if let Some(parent_index) = member.parent_index {
                if parent_index as usize >= found {
                    return Err(LoopSourceForestBindingRejectV1::ParentIndexOutOfRange {
                        member_index,
                        parent_index,
                    });
                }
            }
            if index == 0 && found_parent.is_some() {
                return Err(LoopSourceForestBindingRejectV1::RootParentMismatch);
            }
            let expected_parent = recipe.as_recipe().loops[index].parent;
            if expected_parent != found_parent {
                return Err(LoopSourceForestBindingRejectV1::RecipeParentMismatch {
                    member_index,
                    expected: expected_parent,
                    found: found_parent,
                });
            }
        }

        let loops = self
            .members
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(index, member)| {
                LoopNodeSourceBindingV1::new(LoopNodeKeyV1::new(index as u32), member.path)
            })
            .collect();
        Ok(LoopRecipeSourceBindingV1::new(self.owner, loops))
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

/// Consumes a resolver-owned source forest without selecting a Recipe or
/// recreating any source path from syntax.
pub(crate) fn bind_resolved_loop_source_forest_v1(
    forest: VerifiedResolvedLoopSourceForestV1,
) -> Result<VerifiedLoopSourceForestBindingV1, LoopSourceForestBindingRejectV1> {
    let members = forest.into_members().into_vec();
    let mut members_iter = members.into_iter();
    let Some(first) = members_iter.next() else {
        return Err(LoopSourceForestBindingRejectV1::SourceForestEmpty);
    };
    let first_parent_index = first.parent_index();
    let (origin, source_kind, site) = first.into_source().into_parts();
    let owner = portable_function_owner_v1(origin, source_kind).map_err(|reason| {
        LoopSourceForestBindingRejectV1::Source {
            member_index: 0,
            reason,
        }
    })?;
    let first_path = portable_path_v1(site.node().segments()).map_err(|reason| {
        LoopSourceForestBindingRejectV1::Source {
            member_index: 0,
            reason,
        }
    })?;
    let mut projected = vec![VerifiedLoopSourceForestBindingMemberV1 {
        path: first_path,
        parent_index: first_parent_index,
    }];

    for (index, member) in members_iter.enumerate() {
        let member_index = index as u32 + 1;
        let parent_index = member.parent_index();
        let (origin, source_kind, site) = member.into_source().into_parts();
        let member_owner = portable_function_owner_v1(origin, source_kind).map_err(|reason| {
            LoopSourceForestBindingRejectV1::Source {
                member_index,
                reason,
            }
        })?;
        if member_owner != owner {
            return Err(LoopSourceForestBindingRejectV1::SourceForestOwnerMismatch {
                member_index,
            });
        }
        let path = portable_path_v1(site.node().segments()).map_err(|reason| {
            LoopSourceForestBindingRejectV1::Source {
                member_index,
                reason,
            }
        })?;
        projected.push(VerifiedLoopSourceForestBindingMemberV1 { path, parent_index });
    }

    Ok(VerifiedLoopSourceForestBindingV1 {
        owner,
        members: projected.into_boxed_slice(),
    })
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
pub(crate) fn projection_for_test(parents: &[Option<u32>]) -> VerifiedLoopSourceForestBindingV1 {
    VerifiedLoopSourceForestBindingV1 {
        owner: LoopRecipeSourceOwnerV1::function_body(0, 0),
        members: parents
            .iter()
            .map(|parent_index| VerifiedLoopSourceForestBindingMemberV1 {
                path: LoopSourcePathV1::new(vec![LoopSourcePathStepV1::BodyItem { index: 0 }]),
                parent_index: *parent_index,
            })
            .collect(),
    }
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
