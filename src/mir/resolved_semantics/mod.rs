//! Passive owner-scoped semantic arena schema.
//!
//! See `README.md` before adding resolver or consumer connections.

// SA0 is intentionally disconnected. Remove these scoped allowances as SA1
// gives the schema its first shadow-only producer/consumer.
#![allow(dead_code, unused_imports)]

mod callable_catalog;
mod callable_catalog_candidate;
mod callable_catalog_resolution_source;
mod callable_header_source_unit;
mod callable_header_view;
mod callable_index;
mod callable_module_header_view;
mod callable_symbol;
mod direct_call;
mod direct_call_verifier;
mod function_root;
mod function_view;
mod ids;
mod if_region;
mod loop_region;
mod normalized;
mod normalized_callable_catalog;
mod owner_forest;
mod owner_forest_payload;
mod owner_resolver;
mod owner_root_profile;
mod owner_source_kind;
mod product;
mod records;
mod resolver;
mod script_view;
mod shadow;
mod source_path_policy;
mod source_projection;
mod source_site;
mod verifier;

pub(crate) use callable_catalog::{
    CallableCatalogOwnerSealErrorV1, CallableCatalogSealOutcomeV1,
    CatalogSealedResolverContinuationV1, PreparedCallableCatalogSealV1,
    VerifiedCallableCatalogSourceUnitV1, VerifiedCallableCatalogV1, VerifiedCallableDeclarationV1,
};
pub(crate) use callable_catalog_candidate::{
    CallableCatalogCandidateSealErrorV1, PreparedOwnerFreeCallableCatalogV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};
pub(in crate::mir) use callable_catalog_resolution_source::locate_catalog_function_v1;
pub(crate) use callable_header_source_unit::{
    EmbeddedCallableFunctionSyntaxViewV1, VerifiedCallableHeaderSourceUnitV1,
};
pub(crate) use callable_header_view::{CallableFunctionSyntaxViewV1, CallableHeaderSyntaxViewV1};
pub(crate) use callable_index::{
    CallableIndexSealErrorV1, CallableLookupErrorV1, CallableNamespaceV1, CanonicalCallableKeyV1,
    ExactTrivialCallableSignatureV1, ResolvedCallableRefV1, VerifiedCallableHeaderV1,
    VerifiedCallableIndexV1, VerifiedOwnerFreeCallableHeaderV1,
};
pub(crate) use callable_module_header_view::{
    CallableModuleHeaderSyntaxErrorV1, CallableModuleHeaderSyntaxViewV1,
    LocatedCallableHeaderSyntaxViewV1, SourceCallableDeclarationSiteV1,
};
pub(crate) use callable_symbol::CanonicalCallableSymbolV1;
pub(crate) use direct_call::{ResolvedDirectCallTargetV1, ResolvedDirectCallVerificationErrorV1};
pub(crate) use function_root::{
    ResolvedFunctionLoweringRootsV1, ResolvedFunctionRootVerificationErrorV1,
    ResolvedOwnerLoweringRootsV1,
};
pub(crate) use function_view::FunctionSyntaxViewV1;
pub(in crate::mir) use function_view::ReceiverPolicyV1;
pub(crate) use ids::FunctionOwnerIssuerV1;
pub use ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId, UpvarRefV1};
pub use if_region::ResolvedIfRegionVerificationErrorV1;
pub(crate) use if_region::{ResolvedIfRegionBundleV1, ResolvedIfRegionLookupErrorV1};
pub use loop_region::ResolvedLoopRegionVerificationErrorV1;
pub(crate) use loop_region::{ResolvedLoopRegionBundleV1, ResolvedLoopRegionLookupErrorV1};
pub use normalized::{
    NormalizedAssignmentTargetV1, NormalizedAssignmentV1, NormalizedBindingKeyV1,
    NormalizedBindingRecordV1, NormalizedControlTransferV1, NormalizedDeclarationV1,
    NormalizedExitV1, NormalizedRegionKeyV1, NormalizedRegionRecordV1,
    NormalizedResolvedFunctionGraphV1, NormalizedScopeKeyV1, NormalizedScopeRecordV1,
    NormalizedVariableUseV1,
};
pub(crate) use normalized_callable_catalog::{
    NormalizedCallableCatalogRowV1, NormalizedCallableCatalogV1,
};
pub(crate) use owner_forest::SemanticOwnerForestDraftV1;
pub use owner_forest::{
    NormalizedOwnerKeyV1, NormalizedOwnerRecordV1, NormalizedSemanticOwnerForestGraphV1,
    NormalizedUpvarEdgeV1, NormalizedUpvarObservationV1, OwnerParentEdgeV1,
    SemanticOwnerForestVerificationErrorV1, UpvarAccessKindV1, UpvarObservationV1,
    VerifiedSemanticOwnerForestV1,
};
pub(crate) use owner_forest_payload::VerifiedSemanticOwnerProductV1;
pub(crate) use owner_resolver::ResolveOwnerForestErrorV1;
pub(crate) use owner_root_profile::SemanticOwnerRootProfileV1;
pub use owner_source_kind::SemanticOwnerSourceKindV1;
pub use product::VerifiedResolvedFunctionV1;
pub(crate) use product::{
    ResolvedScopeRegionLookupErrorV1, ResolvedScopeRegionPairV1, VerifiedResolvedOwnerCoreV1,
    VerifiedResolvedScriptV1,
};
pub use records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitRecordV1,
    ResolvedLexicalRefV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1, SyntheticBindingKindV1,
};
pub(crate) use resolver::{
    FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1, ResolveScriptOutcomeV1,
};
pub(crate) use script_view::ScriptSyntaxViewV1;
pub(in crate::mir) use shadow::{
    observe_method_calls_shadow_view_v0, observe_qualified_receiver_shadow_view_v0,
    ShadowMethodCallObservationV0, ShadowMethodCallReceiverV0,
    ShadowQualifiedReceiverDispositionV0, ShadowResolveErrorV0,
};
pub(crate) use shadow::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootDemandWindowSealErrorV1,
    ScriptRootIfControlAdmissionV1, ScriptRootResolvedDemandV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
pub(crate) use source_path_policy::{
    is_statement_expression_surface_v1, BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1,
    ResolvedBodyChildV1, ResolvedExprChildV1, SourceBodyKindV1,
};
pub(in crate::mir) use source_projection::{
    project_source_body_node_v1, project_source_node_v1, ProjectedSourceNodeV1,
};
pub(crate) use source_site::SourcePathV1;
pub use source_site::{
    FunctionOriginV1, OwnedExprSiteV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};
pub use verifier::ResolvedFunctionVerificationErrorV1;

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod callable_catalog_candidate_tests;
#[cfg(test)]
mod callable_catalog_tests;
#[cfg(test)]
mod callable_header_source_unit_tests;
#[cfg(test)]
mod function_root_tests;
#[cfg(test)]
mod if_region_tests;
#[cfg(test)]
mod loop_region_tests;
#[cfg(test)]
mod owner_forest_tests;
#[cfg(test)]
mod resolver_tests;
#[cfg(test)]
mod source_projection_tests;
#[cfg(test)]
mod tests;
