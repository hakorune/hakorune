//! Canonical function semantic resolver and draft-to-arena conversion.

use std::collections::BTreeMap;
use std::sync::Arc;

use hakorune_mir_core::BindingId;

use super::function_view::FunctionSyntaxViewV1;
use super::ids::{
    BindingRefV1, FunctionOwnerIssueExhaustedV1, FunctionOwnerIssuerV1, RegionId, ScopeId,
};
use super::product::{ResolvedFunctionDataV1, ResolvedFunctionDraftV1};
use super::records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedBindingRecordV1, ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedExitRecordV1,
    ResolvedLexicalRefV1, ResolvedRegionRecordV1, ResolvedScopeRecordV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::shadow::{
    resolve_function_shadow_view_v0, ShadowAssignmentTargetV0, ShadowBindingKindV0,
    ShadowBindingOrdinalV0, ShadowControlExitV0, ShadowExitOriginV0, ShadowLexicalRefV0,
    ShadowRegionIdV0, ShadowRegionKindV0, ShadowResolveErrorV0, ShadowResolvedFunctionV0,
    ShadowScopeIdV0, ShadowScopeKindV0,
};
use super::source_site::{FunctionOriginV1, ResolvedExitSiteV1};
use super::{ResolvedFunctionVerificationErrorV1, VerifiedResolvedFunctionV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolveFunctionErrorV1 {
    OwnerIssue(FunctionOwnerIssueExhaustedV1),
    FunctionOrdinalExhausted,
    Syntax(ShadowResolveErrorV0),
    DraftInvariant(&'static str),
    Verification(ResolvedFunctionVerificationErrorV1),
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
}

#[derive(Debug, Clone)]
pub(super) struct AncestorBindingV1 {
    pub(super) reference: BindingRefV1,
}

struct CanonicalizedDraftV1 {
    data: ResolvedFunctionDataV1,
    binding_refs: BTreeMap<ShadowBindingOrdinalV0, BindingRefV1>,
    scope_ids: BTreeMap<ShadowScopeIdV0, ScopeId>,
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
        let draft = resolve_function_shadow_view_v0(origin, view)
            .map_err(ResolveFunctionErrorV1::Syntax)?;
        self.seal_owner(owner, draft).map(Arc::new)
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

    pub(super) fn seal_owner(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        draft: ShadowResolvedFunctionV0,
    ) -> Result<VerifiedResolvedFunctionV1, ResolveFunctionErrorV1> {
        self.seal_owner_with_maps(owner, draft)
            .map(|sealed| sealed.product)
    }

    pub(super) fn seal_owner_with_maps(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        draft: ShadowResolvedFunctionV0,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        self.seal_owner_with_ancestors(owner, draft, &BTreeMap::new())
    }

    pub(super) fn seal_owner_with_ancestors(
        &mut self,
        owner: super::FunctionOwnerIdV1,
        draft: ShadowResolvedFunctionV0,
        ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
    ) -> Result<SealedOwnerConstructionV1, ResolveFunctionErrorV1> {
        let canonical = canonicalize_draft(owner, draft, ancestors)?;
        let product = ResolvedFunctionDraftV1 {
            data: canonical.data,
        }
        .seal()
        .map_err(ResolveFunctionErrorV1::Verification)?;
        Ok(SealedOwnerConstructionV1 {
            product,
            binding_refs: canonical.binding_refs,
            scope_ids: canonical.scope_ids,
        })
    }
}

fn canonicalize_draft(
    owner: super::FunctionOwnerIdV1,
    draft: ShadowResolvedFunctionV0,
    ancestors: &BTreeMap<Box<str>, AncestorBindingV1>,
) -> Result<CanonicalizedDraftV1, ResolveFunctionErrorV1> {
    let binding_ids = draft
        .bindings
        .keys()
        .map(|id| (*id, BindingId::new(id.raw())))
        .collect::<BTreeMap<_, _>>();
    let scope_ids = draft
        .scopes
        .keys()
        .map(|id| (*id, ScopeId::new(owner, id.raw())))
        .collect::<BTreeMap<_, _>>();
    let region_ids = draft
        .regions
        .keys()
        .map(|id| (*id, RegionId::new(owner, id.raw())))
        .collect::<BTreeMap<_, _>>();
    let scope_owner_regions = scope_owner_regions(&draft)?;

    let bindings = draft
        .bindings
        .iter()
        .map(|(id, record)| {
            let binding = binding_ids[id];
            let owner_scope = scope_ids[&record.owner_scope];
            (
                binding,
                ResolvedBindingRecordV1::new(
                    record.diagnostic_name.clone(),
                    binding_kind(record.kind),
                    owner_scope,
                    BindingOriginV1::Source(record.origin.clone()),
                ),
            )
        })
        .collect();

    let scopes = draft
        .scopes
        .iter()
        .map(|(id, record)| {
            let scope = scope_ids[id];
            let origin = if *id == draft.function_scope {
                ScopeOriginV1::Function(draft.function_origin)
            } else {
                ScopeOriginV1::Source(record.origin.clone().ok_or(
                    ResolveFunctionErrorV1::DraftInvariant(
                        "non-function scope lacks source origin",
                    ),
                )?)
            };
            let declarations = record
                .declarations
                .iter()
                .map(|id| BindingRefV1::new(owner, binding_ids[id]))
                .collect();
            Ok((
                scope,
                ResolvedScopeRecordV1::new(
                    scope_kind(record.kind),
                    record.parent.map(|id| scope_ids[&id]),
                    region_ids[&scope_owner_regions[id]],
                    declarations,
                    origin,
                ),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;

    let regions = draft
        .regions
        .iter()
        .map(|(id, record)| {
            let origin = if *id == draft.function_region {
                RegionOriginV1::Function(draft.function_origin)
            } else {
                RegionOriginV1::Source(record.origin.clone().ok_or(
                    ResolveFunctionErrorV1::DraftInvariant(
                        "non-function region lacks source origin",
                    ),
                )?)
            };
            Ok((
                region_ids[id],
                ResolvedRegionRecordV1::new(
                    region_kind(record.kind),
                    record.parent.map(|id| region_ids[&id]),
                    record.lexical_scope.map(|id| scope_ids[&id]),
                    origin,
                ),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;

    let binding_ref = |id: ShadowBindingOrdinalV0| BindingRefV1::new(owner, binding_ids[&id]);
    let declarations = draft
        .declarations
        .iter()
        .map(|(site, id)| (site.clone(), binding_ref(*id)))
        .collect();
    let variable_uses = draft
        .variable_uses
        .iter()
        .map(|(site, lexical_ref)| {
            let lexical_ref = match lexical_ref {
                ShadowLexicalRefV0::Local(id) => ResolvedLexicalRefV1::Local(binding_ref(*id)),
                ShadowLexicalRefV0::Ancestor(name) => {
                    let ancestor =
                        ancestors
                            .get(name)
                            .ok_or(ResolveFunctionErrorV1::DraftInvariant(
                                "shadow ancestor reference lacks canonical source",
                            ))?;
                    let upvar = super::UpvarRefV1::new(owner, ancestor.reference);
                    ResolvedLexicalRefV1::Upvar(upvar)
                }
            };
            Ok((site.clone(), lexical_ref))
        })
        .collect::<Result<BTreeMap<_, _>, ResolveFunctionErrorV1>>()?;
    let assignment_targets = draft
        .assignment_targets
        .iter()
        .map(|(site, target)| {
            let target = match target {
                ShadowAssignmentTargetV0::BindingRebind(id) => {
                    ResolvedAssignmentTargetV1::BindingRebind(binding_ref(*id))
                }
                ShadowAssignmentTargetV0::FieldWrite { receiver } => {
                    ResolvedAssignmentTargetV1::FieldWrite {
                        receiver: receiver.clone(),
                    }
                }
                ShadowAssignmentTargetV0::IndexWrite { receiver } => {
                    ResolvedAssignmentTargetV1::IndexWrite {
                        receiver: receiver.clone(),
                    }
                }
            };
            (site.clone(), target)
        })
        .collect();
    let resolved_exits = draft
        .resolved_exits
        .iter()
        .map(|(site, record)| {
            let origin = match record.origin {
                ShadowExitOriginV0::ExplicitContinue => ResolvedExitOriginV1::ExplicitContinue,
                ShadowExitOriginV0::ExplicitBreak => ResolvedExitOriginV1::ExplicitBreak,
                ShadowExitOriginV0::ExplicitReturn => ResolvedExitOriginV1::ExplicitReturn,
            };
            let transfer = match record.transfer {
                ShadowControlExitV0::Continue { target_loop } => {
                    ResolvedControlTransferV1::Continue {
                        target_loop: region_ids[&target_loop],
                    }
                }
                ShadowControlExitV0::Break { target_loop } => ResolvedControlTransferV1::Break {
                    target_loop: region_ids[&target_loop],
                },
                ShadowControlExitV0::Return { target_function } => {
                    ResolvedControlTransferV1::Return {
                        target_function: region_ids[&target_function],
                    }
                }
            };
            (
                ResolvedExitSiteV1::Statement(site.clone()),
                ResolvedExitRecordV1::new(region_ids[&record.source_region], origin, transfer),
            )
        })
        .collect();

    let data = ResolvedFunctionDataV1 {
        owner,
        function_origin: draft.function_origin,
        function_scope: scope_ids[&draft.function_scope],
        function_region: region_ids[&draft.function_region],
        bindings,
        scopes,
        regions,
        declarations,
        variable_uses,
        assignment_targets,
        resolved_exits,
    };
    let binding_refs = binding_ids
        .into_iter()
        .map(|(shadow, binding)| (shadow, BindingRefV1::new(owner, binding)))
        .collect();
    Ok(CanonicalizedDraftV1 {
        data,
        binding_refs,
        scope_ids,
    })
}

fn scope_owner_regions(
    draft: &ShadowResolvedFunctionV0,
) -> Result<BTreeMap<ShadowScopeIdV0, ShadowRegionIdV0>, ResolveFunctionErrorV1> {
    let mut owners = BTreeMap::new();
    for (region, record) in &draft.regions {
        if let Some(scope) = record.lexical_scope {
            if owners.insert(scope, *region).is_some() {
                return Err(ResolveFunctionErrorV1::DraftInvariant(
                    "scope has multiple owner regions",
                ));
            }
        }
    }
    if draft.scopes.keys().any(|scope| !owners.contains_key(scope)) {
        return Err(ResolveFunctionErrorV1::DraftInvariant(
            "scope lacks owner region",
        ));
    }
    Ok(owners)
}

const fn binding_kind(kind: ShadowBindingKindV0) -> BindingKindV1 {
    match kind {
        ShadowBindingKindV0::Receiver => BindingKindV1::Receiver,
        ShadowBindingKindV0::Parameter { index } => BindingKindV1::Parameter { index },
        ShadowBindingKindV0::Local { ordinal } => BindingKindV1::Local { ordinal },
        ShadowBindingKindV0::Outbox { ordinal } => BindingKindV1::Outbox { ordinal },
        ShadowBindingKindV0::Nowait => BindingKindV1::Nowait,
    }
}

const fn scope_kind(kind: ShadowScopeKindV0) -> ScopeKindV1 {
    match kind {
        ShadowScopeKindV0::Function => ScopeKindV1::Function,
        ShadowScopeKindV0::LexicalBlock => ScopeKindV1::LexicalBlock,
        ShadowScopeKindV0::IfThen => ScopeKindV1::IfThen,
        ShadowScopeKindV0::IfElse => ScopeKindV1::IfElse,
        ShadowScopeKindV0::LoopBody => ScopeKindV1::LoopBody,
    }
}

const fn region_kind(kind: ShadowRegionKindV0) -> RegionKindV1 {
    match kind {
        ShadowRegionKindV0::Function => RegionKindV1::Function,
        ShadowRegionKindV0::Sequence => RegionKindV1::Sequence,
        ShadowRegionKindV0::LexicalScope => RegionKindV1::LexicalScope,
        ShadowRegionKindV0::If => RegionKindV1::If,
        ShadowRegionKindV0::IfThen => RegionKindV1::IfThen,
        ShadowRegionKindV0::IfElse => RegionKindV1::IfElse,
        ShadowRegionKindV0::Loop => RegionKindV1::Loop,
    }
}
