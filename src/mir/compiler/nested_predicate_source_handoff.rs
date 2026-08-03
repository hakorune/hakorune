//! One-time source-role handoff for the bounded Nested Predicate pipeline.
//!
//! D2-D consumes the resolver projection once. This handoff preserves only the
//! resolver-owned role evidence needed by the later symbolic topology issuer;
//! it carries no AST, source path, recipe authority, or physical identity.

use crate::mir::loop_structural_facts::VerifiedLoopSourceForestBindingV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ScopeId, SourceBindingSiteV1,
    SourceStmtSiteV1,
};

use super::nested_predicate_projection::{
    NestedBindingEvidenceV1, NestedChildBodyRoleV1, NestedPredicateConditionEvidenceV1,
    NestedPredicateUpdateEvidenceV1, NestedRootBodyRoleV1, NestedRootInitializerEvidenceV1,
    VerifiedNestedLoopSourceShapeV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedPhysicalSourceHandoffRejectV1 {
    ForestShape,
    BindingOwnerMismatch,
    ScopeOwnerMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NestedSourceForestParentReceiptV1 {
    member_count: u32,
    parent_indices: Box<[Option<u32>]>,
}

impl NestedSourceForestParentReceiptV1 {
    pub(crate) fn issue(
        forest: &VerifiedLoopSourceForestBindingV1,
    ) -> Result<Self, NestedPhysicalSourceHandoffRejectV1> {
        let parent_indices = forest
            .members()
            .iter()
            .map(|member| member.parent_index())
            .collect::<Box<_>>();
        if parent_indices.as_ref() != [None, Some(0)] {
            return Err(NestedPhysicalSourceHandoffRejectV1::ForestShape);
        }
        Ok(Self {
            member_count: parent_indices.len() as u32,
            parent_indices,
        })
    }

    pub(crate) fn member_count(&self) -> u32 {
        self.member_count
    }

    pub(crate) fn parent_indices(&self) -> &[Option<u32>] {
        &self.parent_indices
    }
}

/// Non-Clone source evidence handed from the semantic producer to D4.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPhysicalSourceHandoffV1 {
    owner: FunctionOwnerIdV1,
    root_frame_key: LoopExecutionFrameKeyV1,
    shape: VerifiedNestedLoopSourceShapeV1,
    forest_parent_receipt: NestedSourceForestParentReceiptV1,
}

impl VerifiedNestedPhysicalSourceHandoffV1 {
    pub(crate) fn issue(
        forest: &VerifiedLoopSourceForestBindingV1,
        shape: VerifiedNestedLoopSourceShapeV1,
        root_frame_key: LoopExecutionFrameKeyV1,
    ) -> Result<Self, NestedPhysicalSourceHandoffRejectV1> {
        let receipt = NestedSourceForestParentReceiptV1::issue(forest)?;
        let owner = shape.bindings[0].binding.owner();
        if shape
            .bindings
            .iter()
            .any(|evidence| evidence.binding.owner() != owner)
        {
            return Err(NestedPhysicalSourceHandoffRejectV1::BindingOwnerMismatch);
        }
        if shape
            .bindings
            .iter()
            .any(|evidence| evidence.lexical_scope.owner() != owner)
        {
            return Err(NestedPhysicalSourceHandoffRejectV1::ScopeOwnerMismatch);
        }
        Ok(Self {
            owner,
            root_frame_key,
            shape,
            forest_parent_receipt: receipt,
        })
    }

    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn shape(&self) -> &VerifiedNestedLoopSourceShapeV1 {
        &self.shape
    }

    pub(crate) fn forest_parent_receipt(&self) -> &NestedSourceForestParentReceiptV1 {
        &self.forest_parent_receipt
    }

    pub(crate) fn root_site(&self) -> &SourceStmtSiteV1 {
        &self.shape.root_site
    }

    pub(crate) fn child_site(&self) -> &SourceStmtSiteV1 {
        &self.shape.child_site
    }

    pub(crate) fn child_declaration_site(&self) -> &SourceBindingSiteV1 {
        &self.shape.child_declaration_site
    }

    pub(crate) fn bindings(&self) -> &[NestedBindingEvidenceV1; 3] {
        &self.shape.bindings
    }

    pub(crate) fn conditions(&self) -> [&NestedPredicateConditionEvidenceV1; 2] {
        [&self.shape.root_condition, &self.shape.child_condition]
    }

    pub(crate) fn updates(&self) -> [&NestedPredicateUpdateEvidenceV1; 4] {
        [
            &self.shape.initialize_child,
            &self.shape.increment_root,
            &self.shape.increment_ancestor,
            &self.shape.increment_child,
        ]
    }

    pub(crate) fn root_initializers(&self) -> &[NestedRootInitializerEvidenceV1; 2] {
        &self.shape.root_initializers
    }

    pub(crate) fn root_body_roles(&self) -> &[NestedRootBodyRoleV1; 4] {
        &self.shape.root_body_roles
    }

    pub(crate) fn child_body_roles(&self) -> &[NestedChildBodyRoleV1; 2] {
        &self.shape.child_body_roles
    }

    pub(crate) fn binding_refs(&self) -> [BindingRefV1; 3] {
        self.shape.bindings.map(|evidence| evidence.binding)
    }

    pub(crate) fn lexical_scopes(&self) -> [ScopeId; 3] {
        self.shape.bindings.map(|evidence| evidence.lexical_scope)
    }
}
