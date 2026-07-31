//! Private dense-root input for the shared shadow traversal.
//!
//! FunctionSyntaxViewV1 remains the public Function/Lambda seam.  This input
//! owns only the private traversal shape so Script can later add a sparse
//! ProgramBody(original ordinal) adapter without widening that public view.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::function_view::{FunctionBodyOriginV1, ReceiverPolicyV1};
use crate::mir::resolved_semantics::{FunctionSyntaxViewV1, SemanticOwnerRootProfileV1};

use super::path::ShadowSourcePathV0;

pub(super) struct ShadowRootTraversalInputV1<'ast> {
    params: &'ast [String],
    body: &'ast [ASTNode],
    receiver_policy: ReceiverPolicyV1,
    root_profile: SemanticOwnerRootProfileV1,
    body_origin: FunctionBodyOriginV1,
}

impl<'ast> ShadowRootTraversalInputV1<'ast> {
    pub(super) fn dense(view: FunctionSyntaxViewV1<'ast>) -> Self {
        Self {
            params: view.params(),
            body: view.body(),
            receiver_policy: view.receiver_policy(),
            root_profile: view.root_profile(),
            body_origin: view.body_origin(),
        }
    }

    pub(super) const fn params(&self) -> &'ast [String] {
        self.params
    }

    pub(super) const fn body(&self) -> &'ast [ASTNode] {
        self.body
    }

    pub(super) const fn receiver_policy(&self) -> ReceiverPolicyV1 {
        self.receiver_policy
    }

    pub(super) const fn root_profile(&self) -> SemanticOwnerRootProfileV1 {
        self.root_profile
    }

    pub(super) fn body_path(&self) -> ShadowSourcePathV0 {
        match self.body_origin {
            FunctionBodyOriginV1::Function => ShadowSourcePathV0::function_body(),
            FunctionBodyOriginV1::Lambda => ShadowSourcePathV0::lambda_body(),
        }
    }

    pub(super) fn resolve_body(
        &self,
        resolver: &mut super::resolver::ShadowResolverV0<'ast>,
    ) -> Result<(), super::product::ShadowResolveErrorV0> {
        match self.body_origin {
            FunctionBodyOriginV1::Function => {
                resolver.resolve_body(self.body, ShadowSourcePathV0::root_body)
            }
            FunctionBodyOriginV1::Lambda => {
                resolver.resolve_body(self.body, ShadowSourcePathV0::lambda_body_item)
            }
        }
    }
}
