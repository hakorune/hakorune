//! Private root input for the shared shadow traversal.
//!
//! FunctionSyntaxViewV1 remains the public Function/Lambda seam.  Script uses
//! the sparse ProgramBody(original ordinal) adapter here without widening that
//! public view.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::function_view::{FunctionBodyOriginV1, ReceiverPolicyV1};
use crate::mir::resolved_semantics::{
    EnumMatchDemandV1, EnumVariantDemandV1, RecordSchemaDemandV1, ScriptSyntaxViewV1, SourcePathSegmentV1,
};
use crate::mir::resolved_semantics::{FunctionSyntaxViewV1, SemanticOwnerRootProfileV1};

use super::path::ShadowSourcePathV0;
use super::script_root_dispatch::dispatch_resolved_script_root_statement;
use super::script_root_window::{
    ScriptRootResolvedDemandV1, ScriptRootSemanticDispositionV1, VerifiedScriptRootDemandWindowV1,
};
use super::traversal_profile::ShadowTraversalProfileV1;

pub(super) struct ShadowRootTraversalInputV1<'ast, 'schema> {
    params: &'ast [String],
    items: ShadowRootItemsV1<'ast>,
    receiver_policy: ReceiverPolicyV1,
    root_profile: SemanticOwnerRootProfileV1,
    traversal_profile: ShadowTraversalProfileV1,
    enum_variant_demand: Option<&'schema dyn EnumVariantDemandV1>,
    enum_match_demand: Option<&'schema dyn EnumMatchDemandV1>,
    record_schema_demand: Option<&'schema dyn RecordSchemaDemandV1>,
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

impl<'ast, 'schema> ShadowRootTraversalInputV1<'ast, 'schema> {
    pub(super) fn dense(view: FunctionSyntaxViewV1<'ast>) -> Self {
        Self::dense_with_profile(view, ShadowTraversalProfileV1::FullFunctionV1)
    }

    pub(super) fn dense_with_profile(
        view: FunctionSyntaxViewV1<'ast>,
        traversal_profile: ShadowTraversalProfileV1,
    ) -> Self {
        Self {
            params: view.params(),
            items: ShadowRootItemsV1::Dense {
                body: view.body(),
                origin: view.body_origin(),
            },
            receiver_policy: view.receiver_policy(),
            root_profile: view.root_profile(),
            traversal_profile,
            enum_variant_demand: None,
            enum_match_demand: None,
            record_schema_demand: None,
        }
    }

    pub(super) fn sparse_script(
        view: ScriptSyntaxViewV1<'ast>,
        window: &'ast VerifiedScriptRootDemandWindowV1,
        record_schema_demand: &'schema dyn RecordSchemaDemandV1,
        enum_variant_demand: &'schema dyn EnumVariantDemandV1,
        enum_match_demand: &'schema dyn EnumMatchDemandV1,
    ) -> Self {
        Self {
            params: &[],
            items: ShadowRootItemsV1::SparseScript { view, window },
            receiver_policy: ReceiverPolicyV1::Absent,
            root_profile: view.root_profile(),
            traversal_profile: ShadowTraversalProfileV1::ScriptLexicalCoreV1,
            enum_variant_demand: Some(enum_variant_demand),
            enum_match_demand: Some(enum_match_demand),
            record_schema_demand: Some(record_schema_demand),
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

    pub(super) const fn record_schema_demand(&self) -> Option<&'schema dyn RecordSchemaDemandV1> {
        self.record_schema_demand
    }

    pub(super) const fn enum_variant_demand(&self) -> Option<&'schema dyn EnumVariantDemandV1> {
        self.enum_variant_demand
    }

    pub(super) const fn enum_match_demand(&self) -> Option<&'schema dyn EnumMatchDemandV1> {
        self.enum_match_demand
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
        resolver: &mut super::resolver::ShadowResolverV0<'ast, 'schema>,
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
                            if matches!(demand, ScriptRootResolvedDemandV1::ReturnExit(_))
                                && !window.is_final_ordinal(*index as usize)
                            {
                                return Err(
                                    super::product::ShadowResolveErrorV0::UnsupportedStatement {
                                        kind: "non-final Script root Return receipt",
                                        site: entry.site().clone(),
                                    },
                                );
                            }
                            dispatch_resolved_script_root_statement(
                                resolver,
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
