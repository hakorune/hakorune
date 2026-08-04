//! Logical JoinSig composition for the depth-one Nested If shell.

use super::nested_schema::{NestedIfNodeKeyV1, NestedIfRecipeProfileV1, NestedIfValueClassV1};
use super::nested_verify::VerifiedNestedIfRecipeArtifactV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedIfJoinCompositionRoleV1 {
    InnerMergeToOuterThen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NestedIfJoinNodeSigV1 {
    pub(crate) node: NestedIfNodeKeyV1,
    pub(crate) binding: super::nested_schema::NestedIfBindingKeyV1,
    pub(crate) class: NestedIfValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NestedIfJoinCompositionV1 {
    pub(crate) from: NestedIfNodeKeyV1,
    pub(crate) to: NestedIfNodeKeyV1,
    pub(crate) role: NestedIfJoinCompositionRoleV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NestedIfJoinSigV1 {
    pub(crate) outer: NestedIfJoinNodeSigV1,
    pub(crate) inner: NestedIfJoinNodeSigV1,
    pub(crate) composition: NestedIfJoinCompositionV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedNestedIfJoinSigV1(NestedIfJoinSigV1);

impl VerifiedNestedIfJoinSigV1 {
    pub(crate) fn as_sig(&self) -> &NestedIfJoinSigV1 {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedIfJoinSigRejectReasonV1 {
    ProfileMismatch,
    NodeMismatch,
    BindingMismatch,
    CompositionMismatch,
}

pub(crate) struct NestedIfJoinSigComposerV1;

impl NestedIfJoinSigComposerV1 {
    pub(crate) fn compose(
        artifact: &VerifiedNestedIfRecipeArtifactV1,
    ) -> Result<VerifiedNestedIfJoinSigV1, NestedIfJoinSigRejectReasonV1> {
        if artifact.artifact().provenance.profile
            != NestedIfRecipeProfileV1::ResolvedTrivialExplicitElseDepthOne
        {
            return Err(NestedIfJoinSigRejectReasonV1::ProfileMismatch);
        }
        let recipe = artifact.recipe();
        let [outer, inner] = recipe.nodes.as_slice() else {
            return Err(NestedIfJoinSigRejectReasonV1::NodeMismatch);
        };
        if outer.key.raw() != 0 || inner.key.raw() != 1 {
            return Err(NestedIfJoinSigRejectReasonV1::NodeMismatch);
        }
        if outer.join.binding != inner.join.binding
            || outer.join.class != inner.join.class
            || outer.then_child != Some(inner.key)
        {
            return Err(NestedIfJoinSigRejectReasonV1::BindingMismatch);
        }
        let composition = NestedIfJoinCompositionV1 {
            from: inner.key,
            to: outer.key,
            role: NestedIfJoinCompositionRoleV1::InnerMergeToOuterThen,
        };
        if composition.from == composition.to {
            return Err(NestedIfJoinSigRejectReasonV1::CompositionMismatch);
        }
        Ok(VerifiedNestedIfJoinSigV1(NestedIfJoinSigV1 {
            outer: NestedIfJoinNodeSigV1 {
                node: outer.key,
                binding: outer.join.binding,
                class: outer.join.class,
            },
            inner: NestedIfJoinNodeSigV1 {
                node: inner.key,
                binding: inner.join.binding,
                class: inner.join.class,
            },
            composition,
        }))
    }
}
