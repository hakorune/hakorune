//! Structural verifier for an artifact-level Loop-to-source wire claim.

use std::collections::BTreeMap;

use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::LoopNodeKeyV1;
use super::schema::{
    LoopRecipeSourceBindingV1, LoopRecipeV1, LoopSourcePathStepV1, LoopSourcePathV1,
};

/// Capability proving only that a wire claim is structurally compatible with
/// the semantic Loop arena.
///
/// It does not prove that the named compilation unit, function, or AST sites
/// exist, nor that they identify the source that produced the recipe. The wire
/// DTO remains `Clone`; only this validation capability is intentionally
/// non-`Clone`.
#[derive(Debug)]
pub(super) struct StructurallyVerifiedLoopRecipeSourceClaimV1(LoopRecipeSourceBindingV1);

impl StructurallyVerifiedLoopRecipeSourceClaimV1 {
    pub(super) fn as_source_binding(&self) -> &LoopRecipeSourceBindingV1 {
        &self.0
    }
}

pub(super) struct LoopRecipeSourceClaimVerifierV1;

impl LoopRecipeSourceClaimVerifierV1 {
    pub(super) fn verify(
        recipe: &LoopRecipeV1,
        binding: LoopRecipeSourceBindingV1,
    ) -> Result<StructurallyVerifiedLoopRecipeSourceClaimV1, Reject> {
        let expected = recipe.loops.len();
        let found = binding.loops.len();
        if found != expected {
            return Err(Reject::SourceBindingCoverageMismatch { expected, found });
        }

        for (index, row) in binding.loops.iter().enumerate() {
            let expected = LoopNodeKeyV1::new(index as u32);
            if row.loop_key != expected {
                return Err(Reject::NonCanonicalSourceBindingOrder {
                    expected,
                    found: row.loop_key,
                });
            }
        }

        let mut path_owners: BTreeMap<&LoopSourcePathV1, LoopNodeKeyV1> = BTreeMap::new();
        for row in &binding.loops {
            if let Some(first) = path_owners.insert(&row.path, row.loop_key) {
                return Err(Reject::DuplicateLoopSourcePath {
                    first,
                    second: row.loop_key,
                });
            }
        }

        for (loop_node, row) in recipe.loops.iter().zip(&binding.loops) {
            let Some(parent_loop) = loop_node.parent else {
                if !matches!(
                    row.path.steps.first(),
                    Some(LoopSourcePathStepV1::BodyItem { .. })
                ) {
                    return Err(Reject::RootSourcePathMustStartWithBodyItem {
                        loop_key: row.loop_key,
                    });
                }
                check_no_body_item_after_root(row.loop_key, &row.path.steps)?;
                continue;
            };

            // The semantic verifier has already proven parent keys canonical,
            // in-range, and ordered before children.
            let parent_path = &binding.loops[parent_loop.raw() as usize].path.steps;
            let child_path = &row.path.steps;
            if child_path.len() <= parent_path.len()
                || !child_path.as_slice().starts_with(parent_path.as_slice())
            {
                return Err(Reject::NestedSourcePathNotDescendant {
                    loop_key: row.loop_key,
                    parent_loop,
                });
            }
            if !matches!(
                child_path.get(parent_path.len()),
                Some(LoopSourcePathStepV1::LoopBodyItem { .. })
            ) {
                return Err(Reject::NestedSourcePathMustEnterLoopBody {
                    loop_key: row.loop_key,
                    parent_loop,
                });
            }
            for (step_index, step) in child_path.iter().enumerate().skip(parent_path.len() + 1) {
                match step {
                    LoopSourcePathStepV1::ScopeBodyItem { .. } => {}
                    LoopSourcePathStepV1::BodyItem { .. } => {
                        return Err(Reject::SourcePathBodyItemAfterRoot {
                            loop_key: row.loop_key,
                            step_index,
                        });
                    }
                    LoopSourcePathStepV1::LoopBodyItem { .. } => {
                        return Err(Reject::NestedSourcePathSkipsIntermediateLoop {
                            loop_key: row.loop_key,
                            parent_loop,
                            step_index,
                        });
                    }
                }
            }
        }

        Ok(StructurallyVerifiedLoopRecipeSourceClaimV1(binding))
    }
}

fn check_no_body_item_after_root(
    loop_key: LoopNodeKeyV1,
    steps: &[LoopSourcePathStepV1],
) -> Result<(), Reject> {
    for (step_index, step) in steps.iter().enumerate().skip(1) {
        if matches!(step, LoopSourcePathStepV1::BodyItem { .. }) {
            return Err(Reject::SourcePathBodyItemAfterRoot {
                loop_key,
                step_index,
            });
        }
    }
    Ok(())
}
