//! Passive owner-scoped semantic arena schema.
//!
//! See `README.md` before adding resolver or consumer connections.

// SA0 is intentionally disconnected. Remove these scoped allowances as SA1
// gives the schema its first shadow-only producer/consumer.
#![allow(dead_code, unused_imports)]

mod body_effect_control_coverage;
mod body_shape;
mod callable_catalog;
mod callable_catalog_candidate;
mod callable_catalog_resolution_source;
mod callable_header_source_unit;
mod callable_header_view;
mod callable_index;
mod callable_module_header_view;
mod callable_source_ledger;
mod callable_symbol;
mod declared_instance_contract;
mod declared_query_body_source;
mod direct_call;
mod direct_call_verifier;
mod enum_match_demand;
mod enum_variant_demand;
mod function_root;
mod function_view;
pub(crate) mod generic_g0;
mod home_abi;
mod home_relation;
mod ids;
mod if_region;
mod instance_method_body_owner;
mod instance_method_body_source;
mod instance_method_declaration;
mod instance_method_function_carrier;
mod loop_family_window;
#[cfg(test)]
mod loop_family_window_tests;
mod loop_region;
mod normalized;
mod normalized_callable_catalog;
mod ordered_capture;
mod owner_construction_tree;
mod owner_forest;
mod owner_forest_payload;
mod owner_resolver;
mod owner_root_profile;
mod owner_source_kind;
mod product;
mod query_behavior;
mod query_body_conformance;
mod query_body_conformance_evidence;
mod query_body_facts;
mod record_schema_demand;
mod records;
mod resolver;
mod script_view;
mod shadow;
mod source_path_policy;
mod source_projection;
mod source_site;
mod source_site_inventory;
mod verifier;

pub(crate) use body_shape::{
    BodyEffectKindV1, BodyEffectShapeV1, BodyExpressionShapeV1, BodyMeReceiverV1,
    BodyShapeRelationV1, BodyStatementShapeV1, ResolvedFunctionBodyShapeProductV1,
    ResolvedMethodCallArgumentSourceV1, ResolvedMethodCallReceiverSourceV1,
    ResolvedMethodCallSourceIssueV1, VerifiedResolvedBodyShapeInventoryV1,
    VerifiedResolvedMethodCallSourceV1,
};
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
pub(crate) use callable_source_ledger::{
    CallableSemanticSourceLedgerView, CallableSourceLedgerRejectV1, CallableSourceRowDispositionV1,
    CallableSourceRowFamilyV1, VerifiedCallableLoopMembershipV1,
};
pub(crate) use callable_symbol::CanonicalCallableSymbolV1;
pub(crate) use declared_instance_contract::{
    DeclaredInstanceMethodContractIssueV1, DeclaredInstanceMethodContractIssuerV1,
    DeclaredInstanceMethodContractRefV1, DeclaredInstanceMethodIdentityV1,
    VerifiedDeclaredInstanceMethodContractCatalogV1,
};
pub(crate) use declared_query_body_source::{
    DeclaredQueryBodySourceIssueV1, DeclaredQueryBodySourceIssuerV1,
    VerifiedDeclaredQueryBodySourceCatalogV1, VerifiedDeclaredQueryBodySourceRowRefV1,
};
pub(crate) use direct_call::{ResolvedDirectCallTargetV1, ResolvedDirectCallVerificationErrorV1};
pub(crate) use enum_match_demand::{
    admit_direct_enum_match_v1, EnumMatchAdmissionV1, EnumMatchDemandV1,
};
pub(crate) use enum_variant_demand::{EnumVariantAdmissionV1, EnumVariantDemandV1};
pub(crate) use function_root::{
    ResolvedFunctionLoweringRootsV1, ResolvedFunctionRootVerificationErrorV1,
    ResolvedOwnerLoweringRootsV1,
};
pub(crate) use function_view::FunctionSyntaxViewV1;
pub(in crate::mir) use function_view::ReceiverPolicyV1;
pub(crate) use home_abi::{
    CallableHomeAbiIssuerV1, HomeAbiIssueV1, ResolverHomeCapabilityEnvironmentV1,
    VerifiedDeclaredInstanceMethodHomeCatalogV1, VerifiedHomeAbiV1,
};
pub(crate) use home_relation::{
    HomeDemandV1, HomeDestinationV1, HomeRelationBrandIssuerV1, HomeRelationBrandV1,
    HomeRelationRejectV1, HomeResultRelationV1, HomeRootRefV1,
};
pub(crate) use ids::FunctionOwnerIssuerV1;
pub use ids::{BindingRefV1, FunctionOwnerIdV1, RegionId, ScopeId, UpvarRefV1};
pub use if_region::ResolvedIfRegionVerificationErrorV1;
pub(crate) use if_region::{ResolvedIfRegionBundleV1, ResolvedIfRegionLookupErrorV1};
pub(in crate::mir) use instance_method_body_owner::{
    InstanceMethodBodyOwnerBindingIssueV1, InstanceMethodBodyOwnerBindingIssuerV1,
    VerifiedInstanceMethodBodyOwnerCatalogV1, VerifiedInstanceMethodBodyOwnerRowV1,
};
pub(crate) use instance_method_body_source::{
    InstanceMethodBodySourceIssueV1, InstanceMethodBodySourceIssuerV1,
    VerifiedInstanceMethodBodySourceCatalogV1, VerifiedInstanceMethodBodySourceRowV1,
};
pub(crate) use instance_method_declaration::{
    InstanceMethodDeclarationIssueV1, ResolverCatalogBrandV1, ResolverNominalBoxDeclarationInputV1,
    ResolverNominalBoxTypeIdV1, ResolverNominalTypeEnvironmentIssueV1,
    ResolverNominalTypeEnvironmentV1, ResolverSemanticValueTypeV1,
    SemanticInstanceDeclarationIssuerV1, VerifiedInstanceMethodDeclarationCatalogV1,
    VerifiedInstanceMethodDeclarationV1, VerifiedSemanticCallableSignatureV1,
};
pub(in crate::mir) use instance_method_function_carrier::{
    InstanceMethodFunctionCarrierIssueV1, InstanceMethodFunctionCarrierIssuerV1,
    VerifiedInstanceMethodFunctionCarrierCatalogV1, VerifiedInstanceMethodFunctionCarrierRowV1,
    VerifiedMethodBodyCoverageV1,
};
pub(crate) use loop_family_window::{
    LoopFamilyWindowLeaseIssueV1, VerifiedLoopFamilyWindowLeaseV1,
};
#[cfg(test)]
pub(crate) use loop_region::loop_execution_frame_key_for_test;
pub use loop_region::ResolvedLoopRegionVerificationErrorV1;
pub(crate) use loop_region::{
    LoopExecutionFrameKeyV1, ResolvedLoopRegionBundleV1, ResolvedLoopRegionLookupErrorV1,
    ResolvedLoopSourceForestRejectV1, VerifiedResolvedLoopSourceForestMemberV1,
    VerifiedResolvedLoopSourceForestV1, VerifiedResolvedLoopSourceV1,
};
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
pub(crate) use owner_resolver::{
    ResolveOwnerForestErrorV1, ResolveScriptForestOutcomeV1,
    ResolveSelectedCallableForestsOutcomeV1,
};
pub(crate) use owner_root_profile::SemanticOwnerRootProfileV1;
pub use owner_source_kind::SemanticOwnerSourceKindV1;
pub use product::VerifiedResolvedFunctionV1;
pub(crate) use product::{
    ResolvedScopeRegionLookupErrorV1, ResolvedScopeRegionPairV1, VerifiedResolvedOwnerCoreV1,
    VerifiedResolvedScriptV1,
};
pub(crate) use query_behavior::{
    DeclaredQueryBehaviorIssuerV1, DeclaredQueryBehaviorV1, QueryBehaviorIssueV1,
    VerifiedDeclaredQueryBehaviorCatalogV1, VerifiedDeclaredQueryBehaviorV1,
};
pub(in crate::mir) use query_body_conformance::{
    QueryBodyConformanceIssueV1, QueryBodyConformanceIssuerV1,
    VerifiedCallableBodyConformanceCatalogV1, VerifiedCallableBodyConformanceV1,
};
pub(in crate::mir) use query_body_conformance_evidence::{
    QueryBodyConformanceEvidenceDeclineV1, QueryBodyConformanceEvidenceIssueV1,
    QueryBodyConformanceEvidenceIssuerV1, QueryBodyConformanceEvidenceNoSafeSliceV1,
    QueryBodyConformanceEvidenceRejectV1, QueryBodyHomeTransferV1,
    VerifiedQueryBodyConformanceEvidenceCatalogV1, VerifiedQueryBodyConformanceEvidenceV1,
    VerifiedQueryBodyHomeFlowEvidenceV1,
};
pub(in crate::mir) use query_body_facts::{
    OrdinaryReturnFactV1, QueryBodyFactsDeclineV1, QueryBodyFactsIssueV1, QueryBodyFactsIssuerV1,
    QueryBodyFactsRejectV1, QueryBodyFactsUnresolvedV1, ReceiverReadFactV1,
    VerifiedCallableQueryBodyFactsCatalogV1, VerifiedCallableQueryBodyFactsRowV1,
};
pub(crate) use record_schema_demand::{
    FullyExplicitRecordLiteralAdmissionV1, RecordSchemaDemandV1,
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
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootDemandWindowSealErrorV1, ScriptRootIfControlAdmissionV1,
    ScriptRootIndexWriteAdmissionV1, ScriptRootMatchControlAdmissionV1,
    ScriptRootQMarkPropagationAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
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
    FunctionOriginV1, OwnedExprSiteV1, OwnedHeaderSiteV1, ResolvedExitSiteV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourceHeaderSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
};
pub use source_site_inventory::{
    ResolvedSourceSiteInventoryVerificationErrorV1, VerifiedResolvedSourceSiteInventoryV1,
};
pub use verifier::ResolvedFunctionVerificationErrorV1;

#[cfg(test)]
mod block_expr_tests;
#[cfg(test)]
mod body_shape_tests;
#[cfg(test)]
mod callable_catalog_candidate_tests;
#[cfg(test)]
mod callable_catalog_tests;
#[cfg(test)]
mod callable_header_source_unit_tests;
#[cfg(test)]
mod callable_source_ledger_tests;
#[cfg(test)]
mod declared_instance_contract_tests;
#[cfg(test)]
mod explicit_parameter_type_map;
#[cfg(test)]
mod function_root_tests;
#[cfg(test)]
pub(crate) mod generic_resolved_carrier_provenance;
#[cfg(test)]
#[path = "generic_resolved_carrier_source_lease_tests.rs"]
pub(crate) mod generic_resolved_carrier_source_lease;
#[cfg(test)]
mod if_region_tests;
#[cfg(test)]
mod instance_method_body_owner_tests;
#[cfg(test)]
mod instance_method_function_carrier_tests;
#[cfg(test)]
mod loop_region_tests;
#[cfg(test)]
mod ordered_capture_tests;
#[cfg(test)]
mod owner_forest_tests;
#[cfg(test)]
mod query_behavior_tests;
#[cfg(test)]
mod resolver_tests;
#[cfg(test)]
mod source_projection_tests;
#[cfg(test)]
mod source_site_inventory_tests;
#[cfg(test)]
mod tests;
