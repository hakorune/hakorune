//! AST-free source products for the bounded NestedPredicate observer.
//!
//! The compiler projector owns syntax observation and constructs these
//! products.  Route policy consumes them as opaque, resolver-branded facts;
//! this module must not import AST, Builder, Recipe, or MIR authority.

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, ScopeId,
    SourceBindingSiteV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::VerifiedLoopSourceForestBindingV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedObservedRecurrenceOwnerV1 {
    Root,
    Child,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NestedBindingEvidenceV1 {
    pub(crate) binding: BindingRefV1,
    pub(crate) lexical_scope: ScopeId,
    pub(crate) recurrence_owner: NestedObservedRecurrenceOwnerV1,
    pub(crate) parent_visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedPredicateConditionEvidenceV1 {
    pub(crate) site: SourceExprSiteV1,
    pub(crate) lhs_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) bound: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedPredicateUpdateEvidenceV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) target_site: SourceExprSiteV1,
    pub(crate) value_site: SourceExprSiteV1,
    pub(crate) lhs_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) delta: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedRootInitializerEvidenceV1 {
    pub(crate) statement_site: SourceStmtSiteV1,
    pub(crate) value_site: SourceExprSiteV1,
    pub(crate) binding: BindingRefV1,
    pub(crate) value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedRootBodyRoleV1 {
    LocalJ,
    InitializeJ,
    ChildLoop,
    IncrementRoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedChildBodyRoleV1 {
    IncrementAncestor,
    IncrementChild,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedLoopSourceShapeV1 {
    pub(crate) function_origin: FunctionOriginV1,
    pub(crate) root_site: SourceStmtSiteV1,
    pub(crate) child_site: SourceStmtSiteV1,
    pub(crate) child_declaration_site: SourceBindingSiteV1,
    pub(crate) root_condition: NestedPredicateConditionEvidenceV1,
    pub(crate) child_condition: NestedPredicateConditionEvidenceV1,
    pub(crate) root_initializers: [NestedRootInitializerEvidenceV1; 2],
    pub(crate) initialize_child: NestedPredicateUpdateEvidenceV1,
    pub(crate) increment_root: NestedPredicateUpdateEvidenceV1,
    pub(crate) increment_ancestor: NestedPredicateUpdateEvidenceV1,
    pub(crate) increment_child: NestedPredicateUpdateEvidenceV1,
    pub(crate) bindings: [NestedBindingEvidenceV1; 3],
    pub(crate) root_body_roles: [NestedRootBodyRoleV1; 4],
    pub(crate) child_body_roles: [NestedChildBodyRoleV1; 2],
}

/// AST-free, non-Clone projection of one exact nested source forest.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedLoopSourceProjectionV1 {
    pub(crate) forest_binding: VerifiedLoopSourceForestBindingV1,
    pub(crate) shape: VerifiedNestedLoopSourceShapeV1,
    pub(crate) root_frame_key: LoopExecutionFrameKeyV1,
}

impl VerifiedNestedLoopSourceProjectionV1 {
    pub(crate) fn forest_binding(&self) -> &VerifiedLoopSourceForestBindingV1 {
        &self.forest_binding
    }

    pub(crate) fn shape(&self) -> &VerifiedNestedLoopSourceShapeV1 {
        &self.shape
    }

    pub(crate) const fn root_frame_key(&self) -> &LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.shape.bindings[0].binding.owner()
    }

    pub(crate) fn matches_source_identity(
        &self,
        function_origin: FunctionOriginV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.shape.function_origin == function_origin && &self.shape.root_site == site
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopSourceForestBindingV1,
        VerifiedNestedLoopSourceShapeV1,
        LoopExecutionFrameKeyV1,
    ) {
        (self.forest_binding, self.shape, self.root_frame_key)
    }
}
