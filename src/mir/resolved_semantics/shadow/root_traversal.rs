//! Private dense-root input for the shared shadow traversal.
//!
//! FunctionSyntaxViewV1 remains the public Function/Lambda seam.  This input
//! owns only the private traversal shape so Script can later add a sparse
//! ProgramBody(original ordinal) adapter without widening that public view.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::function_view::{FunctionBodyOriginV1, ReceiverPolicyV1};
use crate::mir::resolved_semantics::{FunctionSyntaxViewV1, SemanticOwnerRootProfileV1};
use crate::mir::resolved_semantics::{ScriptSyntaxViewV1, SourcePathSegmentV1};

use super::path::ShadowSourcePathV0;
use super::script_root_window::{
    ScriptRootSemanticDispositionV1, VerifiedScriptRootDemandWindowV1,
};
use super::traversal_profile::ShadowTraversalProfileV1;

pub(super) struct ShadowRootTraversalInputV1<'ast> {
    params: &'ast [String],
    items: ShadowRootItemsV1<'ast>,
    receiver_policy: ReceiverPolicyV1,
    root_profile: SemanticOwnerRootProfileV1,
    traversal_profile: ShadowTraversalProfileV1,
}

enum ShadowRootItemsV1<'ast> {
    Dense {
        body: &'ast [ASTNode],
        origin: FunctionBodyOriginV1,
    },
    SparseScript {
        view: ScriptSyntaxViewV1<'ast>,
        window: &'ast VerifiedScriptRootDemandWindowV1,
    },
}

impl<'ast> ShadowRootTraversalInputV1<'ast> {
    pub(super) fn dense(view: FunctionSyntaxViewV1<'ast>) -> Self {
        Self {
            params: view.params(),
            items: ShadowRootItemsV1::Dense {
                body: view.body(),
                origin: view.body_origin(),
            },
            receiver_policy: view.receiver_policy(),
            root_profile: view.root_profile(),
            traversal_profile: ShadowTraversalProfileV1::FullFunctionV1,
        }
    }

    pub(super) fn sparse_script(
        view: ScriptSyntaxViewV1<'ast>,
        window: &'ast VerifiedScriptRootDemandWindowV1,
    ) -> Self {
        Self {
            params: &[],
            items: ShadowRootItemsV1::SparseScript { view, window },
            receiver_policy: ReceiverPolicyV1::Absent,
            root_profile: view.root_profile(),
            traversal_profile: ShadowTraversalProfileV1::ScriptLexicalCoreV1,
        }
    }

    pub(super) const fn params(&self) -> &'ast [String] {
        self.params
    }

    pub(super) const fn receiver_policy(&self) -> ReceiverPolicyV1 {
        self.receiver_policy
    }

    pub(super) const fn root_profile(&self) -> SemanticOwnerRootProfileV1 {
        self.root_profile
    }

    pub(super) const fn traversal_profile(&self) -> ShadowTraversalProfileV1 {
        self.traversal_profile
    }

    pub(super) fn body_path(&self) -> ShadowSourcePathV0 {
        match self.items {
            ShadowRootItemsV1::Dense {
                origin: FunctionBodyOriginV1::Function,
                ..
            } => ShadowSourcePathV0::function_body(),
            ShadowRootItemsV1::Dense {
                origin: FunctionBodyOriginV1::Lambda,
                ..
            } => ShadowSourcePathV0::lambda_body(),
            ShadowRootItemsV1::SparseScript { .. } => ShadowSourcePathV0::program_body(),
        }
    }

    pub(super) fn resolve_body(
        &self,
        resolver: &mut super::resolver::ShadowResolverV0<'ast>,
    ) -> Result<(), super::product::ShadowResolveErrorV0> {
        match self.items {
            ShadowRootItemsV1::Dense {
                body,
                origin: FunctionBodyOriginV1::Function,
            } => resolver.resolve_body(body, ShadowSourcePathV0::root_body),
            ShadowRootItemsV1::Dense {
                body,
                origin: FunctionBodyOriginV1::Lambda,
            } => resolver.resolve_body(body, ShadowSourcePathV0::lambda_body_item),
            ShadowRootItemsV1::SparseScript { view, window } => {
                for entry in window.entries() {
                    match entry.semantic() {
                        ScriptRootSemanticDispositionV1::Resolved(demand) => {
                            let [SourcePathSegmentV1::ProgramBodyRoot, SourcePathSegmentV1::ProgramBody(index)] =
                                entry.site().node().segments()
                            else {
                                return Err(
                                    super::product::ShadowResolveErrorV0::UnsupportedStatement {
                                        kind: "invalid Script demand site",
                                        site: entry.site().clone(),
                                    },
                                );
                            };
                            let Some(statement) = view.body().get(*index as usize) else {
                                return Err(
                                    super::product::ShadowResolveErrorV0::UnsupportedStatement {
                                        kind: "missing Script demand statement",
                                        site: entry.site().clone(),
                                    },
                                );
                            };
                            resolver.resolve_root_statement(
                                statement,
                                &ShadowSourcePathV0::program_body()
                                    .child(SourcePathSegmentV1::ProgramBody(*index)),
                                demand,
                            )?;
                        }
                        ScriptRootSemanticDispositionV1::Deferred(_) => {
                            return Err(
                                super::product::ShadowResolveErrorV0::UnsupportedStatement {
                                    kind: "deferred Script responsibility",
                                    site: entry.site().clone(),
                                },
                            );
                        }
                        ScriptRootSemanticDispositionV1::Transparent(_)
                        | ScriptRootSemanticDispositionV1::Transferred(_)
                        | ScriptRootSemanticDispositionV1::Diagnostic(_) => continue,
                    }
                }
                Ok(())
            }
        }
    }
}
