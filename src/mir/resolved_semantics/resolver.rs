//! Canonical function semantic resolver and draft-to-arena conversion.

use std::collections::BTreeMap;
use std::sync::Arc;

use hakorune_mir_core::BindingId;

use super::body_shape::{
    issue_resolved_method_call_sources_v1, seal_shadow_body_shape,
    ResolvedFunctionBodyShapeProductV1, ResolvedMethodCallSourceIssueV1,
    VerifiedResolvedBodyShapeInventoryV1,
};
use super::brand_source_relation::{
    seal_brand_call_source_relations_v1, BrandCallSourceRelationSealErrorV1,
};
use super::callable_index::{CallableLookupErrorV1, VerifiedCallableIndexV1};
use super::direct_call::{ResolvedDirectCallObservationV1, ResolvedDirectCallTargetV1};
use super::expression_source::seal_shadow_expression_source_v1;
use super::function_view::FunctionSyntaxViewV1;
use super::ids::{
    BindingRefV1, FunctionOwnerIssueExhaustedV1, FunctionOwnerIssuerV1, RegionId, ScopeId,
};
use super::ordered_capture::OrderedCaptureDemandV1;
use super::owner_forest::UpvarAccessKindV1;
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitRecordV1,
    ResolvedLexicalRefV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::script_view::ScriptSyntaxViewV1;
use super::shadow::{
    resolve_function_shadow_view_v0, resolve_script_shadow_view_v0, ShadowAncestorCaptureAccessV0,
    ShadowAssignmentTargetV0, ShadowBindingKindV0, ShadowBindingOrdinalV0, ShadowControlExitV0,
    ShadowExitOriginV0, ShadowLexicalRefV0, ShadowRegionIdV0, ShadowRegionKindV0,
    ShadowResolveErrorV0, ShadowResolvedFunctionV0, ShadowScopeIdV0, ShadowScopeKindV0,
};
use super::source_site::{FunctionOriginV1, ResolvedExitSiteV1};
use super::source_site_inventory::ResolvedSourceSiteInventoryDraftV1;
use super::{EnumVariantDemandV1, RecordSchemaDemandV1};
use super::{
    ResolvedFunctionVerificationErrorV1, ScriptResolverDeferredV1, VerifiedResolvedFunctionV1,
    VerifiedResolvedScriptV1,
};

#[path = "resolver_canonicalization.rs"]
mod canonicalization;
use canonicalization::canonicalize_draft;

#[derive(Debug)]
pub(crate) enum ResolveScriptOutcomeV1 {
    Complete(VerifiedResolvedScriptV1),
    Deferred(ScriptResolverDeferredV1),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolveFunctionErrorV1 {
    OwnerIssue(FunctionOwnerIssueExhaustedV1),
    FunctionOrdinalExhausted,
    Syntax(ShadowResolveErrorV0),
    ScriptInvariant(ShadowResolveErrorV0),
    DraftInvariant(&'static str),
    MethodCallSource(ResolvedMethodCallSourceIssueV1),
    Verification(ResolvedFunctionVerificationErrorV1),
    CallableLookup(CallableLookupErrorV1),
    BrandSourceRelation(BrandCallSourceRelationSealErrorV1),
}

/// One resolver session per compilation input.
#[derive(Debug)]
pub(crate) struct FunctionSemanticResolverSessionV1 {
    compilation_unit_ordinal: u32,
    next_function_ordinal: u32,
    owners: FunctionOwnerIssuerV1,
}

pub(super) struct SealedOwnerConstructionV1 {
    pub(super) product: VerifiedResolvedFunctionV1,
    pub(super) binding_refs: BTreeMap<ShadowBindingOrdinalV0, BindingRefV1>,
    pub(super) scope_ids: BTreeMap<ShadowScopeIdV0, ScopeId>,
    pub(super) ordered_capture_demands: Box<[OrderedCaptureDemandV1]>,
    pub(super) body_shape: VerifiedResolvedBodyShapeInventoryV1,
}

pub(super) struct SealedScriptConstructionV1 {
    pub(super) product: VerifiedResolvedScriptV1,
    pub(super) binding_refs: BTreeMap<ShadowBindingOrdinalV0, BindingRefV1>,
    pub(super) scope_ids: BTreeMap<ShadowScopeIdV0, ScopeId>,
    pub(super) ordered_capture_demands: Box<[OrderedCaptureDemandV1]>,
}

#[derive(Debug, Clone)]
pub(super) struct AncestorBindingV1 {
    pub(super) reference: BindingRefV1,
}

/// Explicitly selects how the one shared shadow direct-call row is sealed.
/// Selected observation retains source facts without issuing a target;
/// indexed full-function sealing resolves the existing target; every other
/// unindexed path keeps its typed rejection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectCallCanonicalizationPolicyV1 {
    ObserveOnly,
    RequireCallableIndex,
    RejectUnindexed,
}

struct CanonicalizedDraftV1 {
    data: ResolvedFunctionDataV1,
    source_sites: ResolvedSourceSiteInventoryDraftV1,
    binding_refs: BTreeMap<ShadowBindingOrdinalV0, BindingRefV1>,
    scope_ids: BTreeMap<ShadowScopeIdV0, ScopeId>,
    ordered_capture_demands: Box<[OrderedCaptureDemandV1]>,
    body_shape: VerifiedResolvedBodyShapeInventoryV1,
}

impl FunctionSemanticResolverSessionV1 {
    pub(crate) fn new(compilation_unit_ordinal: u32) -> Result<Self, ResolveFunctionErrorV1> {
        Ok(Self {
            compilation_unit_ordinal,
            next_function_ordinal: 0,
            owners: FunctionOwnerIssuerV1::new_for_compilation()
                .map_err(ResolveFunctionErrorV1::OwnerIssue)?,
        })
    }

    pub(crate) fn resolve(
        &mut self,
        view: FunctionSyntaxViewV1<'_>,
    ) -> Result<Arc<VerifiedResolvedFunctionV1>, ResolveFunctionErrorV1> {
        let (origin, owner) = self.issue_owner()?;
        let draft =
            resolve_function_shadow_view_v0(view).map_err(ResolveFunctionErrorV1::Syntax)?;
        self.seal_owner(owner, origin, draft).map(Arc::new)
    }

    /// Issue the existing resolved function and neutral body shape from the
    /// same shadow traversal.  This is the sole resolver-session entry for
    /// the I0 shape inventory; callers cannot rescan the AST afterward.
    pub(crate) fn resolve_with_body_shape(
        &mut self,
        view: FunctionSyntaxViewV1<'_>,
    ) -> Result<ResolvedFunctionBodyShapeProductV1, ResolveFunctionErrorV1> {
        let (origin, owner) = self.issue_owner()?;
        let draft =
            resolve_function_shadow_view_v0(view).map_err(ResolveFunctionErrorV1::Syntax)?;
        let sealed = self.seal_owner_with_maps(owner, origin, draft)?;
        Ok(ResolvedFunctionBodyShapeProductV1::from_parts(
            Arc::new(sealed.product),
            sealed.body_shape,
        ))
    }

    pub(crate) fn resolve_script(
        &mut self,
        view: ScriptSyntaxViewV1<'_>,
        window: &super::VerifiedScriptRootDemandWindowV1,
    ) -> Result<ResolveScriptOutcomeV1, ResolveFunctionErrorV1> {
        self.resolve_script_with_declaration_views(view, window, &(), &(), &())
    }

    pub(crate) fn resolve_script_with_declaration_views(
        &mut self,
        view: ScriptSyntaxViewV1<'_>,
        window: &super::VerifiedScriptRootDemandWindowV1,
        record_schemas: &dyn RecordSchemaDemandV1,
        enum_variants: &dyn EnumVariantDemandV1,
        enum_matches: &dyn super::EnumMatchDemandV1,
    ) -> Result<ResolveScriptOutcomeV1, ResolveFunctionErrorV1> {
        let draft = match resolve_script_shadow_view_v0(
            view,
            window,
            record_schemas,
            enum_variants,
            enum_matches,
        ) {
            Ok(draft) => draft,
            Err(error) => match error.clone().into_script_resolver_deferred() {
                Some(deferred) => return Ok(ResolveScriptOutcomeV1::Deferred(deferred)),
                None => return Err(ResolveFunctionErrorV1::ScriptInvariant(error)),
            },
        };
        let (origin, owner) = self.issue_owner()?;
        self.seal_script_owner(owner, origin, draft)
            .map(ResolveScriptOutcomeV1::Complete)
    }

    pub(super) fn issue_owner(
        &mut self,
    ) -> Result<(FunctionOriginV1, super::FunctionOwnerIdV1), ResolveFunctionErrorV1> {
        let ordinal = self.next_function_ordinal;
        self.next_function_ordinal = ordinal
            .checked_add(1)
            .ok_or(ResolveFunctionErrorV1::FunctionOrdinalExhausted)?;
        let origin = FunctionOriginV1::new(self.compilation_unit_ordinal, ordinal);
        let owner = self
            .owners
            .issue()
            .map_err(ResolveFunctionErrorV1::OwnerIssue)?;
        Ok((origin, owner))
    }

    pub(crate) fn seal_script_owner(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
    ) -> Result<VerifiedResolvedScriptV1, ResolveFunctionErrorV1> {
        self.seal_script_owner_with_maps(owner, origin, draft)
            .map(|sealed| sealed.product)
    }

    pub(super) fn seal_script_owner_with_maps(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
    ) -> Result<SealedScriptConstructionV1, ResolveFunctionErrorV1> {
        let record_literal_demands = draft.record_literal_demands.clone();
        let enum_variant_demands = draft.enum_variant_demands.clone();
        let enum_match_demands = draft.enum_match_demands.clone();
        let qmark_propagation_sites = draft.qmark_propagation_sites.clone();
        let match_control_sites = draft.match_control_sites.clone();
        let canonical = canonicalize_draft(
            owner,
            origin,
            draft,
            &BTreeMap::new(),
            None,
            DirectCallCanonicalizationPolicyV1::RejectUnindexed,
        )?;
        let product = VerifiedResolvedScriptV1::from_canonical_data(
            canonical.data,
            canonical.source_sites,
            canonical.body_shape,
            record_literal_demands,
            enum_variant_demands,
            enum_match_demands,
            qmark_propagation_sites,
            match_control_sites,
        )
        .map_err(ResolveFunctionErrorV1::Verification)?;
        Ok(SealedScriptConstructionV1 {
            product,
            binding_refs: canonical.binding_refs,
            scope_ids: canonical.scope_ids,
            ordered_capture_demands: canonical.ordered_capture_demands,
        })
    }

    pub(super) fn seal_owner(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
    ) -> Result<VerifiedResolvedFunctionV1, ResolveFunctionErrorV1> {
        self.seal_owner_with_maps(owner, origin, draft)
            .map(|sealed| sealed.product)
    }

    pub(super) fn seal_owner_with_maps(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        self.seal_owner_with_ancestors(owner, origin, draft, &BTreeMap::new())
    }

    pub(super) fn seal_owner_with_ancestors(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
        ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        self.seal_owner_with_ancestors_and_direct_call_policy(
            owner,
            origin,
            draft,
            ancestors,
            DirectCallCanonicalizationPolicyV1::RejectUnindexed,
        )
    }

    pub(super) fn seal_owner_with_ancestors_and_direct_call_policy(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
        ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
        direct_call_policy: DirectCallCanonicalizationPolicyV1,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        let canonical =
            canonicalize_draft(owner, origin, draft, ancestors, None, direct_call_policy)?;
        self.seal_canonical_owner(canonical)
    }

    pub(super) fn seal_owner_with_ancestors_and_callable_index(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
        ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
        callable_index: &VerifiedCallableIndexV1,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        let canonical = canonicalize_draft(
            owner,
            origin,
            draft,
            ancestors,
            Some(callable_index),
            DirectCallCanonicalizationPolicyV1::RequireCallableIndex,
        )?;
        self.seal_canonical_owner(canonical)
    }

    pub(super) fn seal_owner_with_callable_index(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        origin: FunctionOriginV1,
        draft: ShadowResolvedFunctionV0,
        callable_index: &VerifiedCallableIndexV1,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        let canonical = canonicalize_draft(
            owner,
            origin,
            draft,
            &BTreeMap::new(),
            Some(callable_index),
            DirectCallCanonicalizationPolicyV1::RequireCallableIndex,
        )?;
        self.seal_canonical_owner(canonical)
    }

    fn seal_canonical_owner(
        &mut self,
        canonical: CanonicalizedDraftV1,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        let product = ResolvedFunctionDraftV1 {
            data: canonical.data,
        }
        .seal_with_source_sites(canonical.source_sites)
        .map_err(ResolveFunctionErrorV1::Verification)?;
        Ok(SealedOwnerConstructionV1 {
            product,
            binding_refs: canonical.binding_refs,
            scope_ids: canonical.scope_ids,
            ordered_capture_demands: canonical.ordered_capture_demands,
            body_shape: canonical.body_shape,
        })
    }
}
