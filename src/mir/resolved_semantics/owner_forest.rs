//! Sealed cross-owner topology for one root function and its nested owners.

use std::collections::{BTreeMap, BTreeSet};

use super::normalized::{
    NormalizedBindingKeyV1, NormalizedResolvedFunctionGraphV1, NormalizedScopeKeyV1,
};
use super::records::{BindingOriginV1, ResolvedLexicalRefV1, ResolvedScopeRecordV1, ScopeOriginV1};
use super::{
    FunctionOriginV1, FunctionOwnerIdV1, OwnedExprSiteV1, ScopeId, SourceBindingSiteV1,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1, UpvarRefV1,
    VerifiedResolvedFunctionV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerParentEdgeV1 {
    parent_owner: FunctionOwnerIdV1,
    definition_site: OwnedExprSiteV1,
    parent_scope: ScopeId,
}

impl OwnerParentEdgeV1 {
    pub(crate) const fn new(
        parent_owner: FunctionOwnerIdV1,
        definition_site: OwnedExprSiteV1,
        parent_scope: ScopeId,
    ) -> Self {
        Self {
            parent_owner,
            definition_site,
            parent_scope,
        }
    }

    pub const fn parent_owner(&self) -> FunctionOwnerIdV1 {
        self.parent_owner
    }

    pub const fn definition_site(&self) -> &OwnedExprSiteV1 {
        &self.definition_site
    }

    pub const fn parent_scope(&self) -> ScopeId {
        self.parent_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedOwnerKeyV1 {
    root: FunctionOriginV1,
    root_source_kind: super::SemanticOwnerSourceKindV1,
    definition_chain: Box<[SourceExprSiteV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedOwnerRecordV1 {
    key: NormalizedOwnerKeyV1,
    parent: Option<NormalizedOwnerKeyV1>,
    definition_site: Option<SourceExprSiteV1>,
    parent_scope: Option<NormalizedScopeKeyV1>,
    product: NormalizedResolvedFunctionGraphV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpvarAccessKindV1 {
    Read,
    Rebind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpvarObservationV1 {
    site: OwnedExprSiteV1,
    upvar: UpvarRefV1,
    access: UpvarAccessKindV1,
}

impl UpvarObservationV1 {
    pub const fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }

    pub const fn upvar(&self) -> UpvarRefV1 {
        self.upvar
    }

    pub const fn access(&self) -> UpvarAccessKindV1 {
        self.access
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedUpvarObservationV1 {
    owner: NormalizedOwnerKeyV1,
    site: SourceExprSiteV1,
    access: UpvarAccessKindV1,
    source_owner: NormalizedOwnerKeyV1,
    source_binding: NormalizedBindingKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedUpvarEdgeV1 {
    capturing_owner: NormalizedOwnerKeyV1,
    source_owner: NormalizedOwnerKeyV1,
    source_binding: NormalizedBindingKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedSemanticOwnerForestGraphV1 {
    root: NormalizedOwnerKeyV1,
    owners: Box<[NormalizedOwnerRecordV1]>,
    upvar_observations: Box<[NormalizedUpvarObservationV1]>,
    upvars: Box<[NormalizedUpvarEdgeV1]>,
}

#[derive(Debug, Default)]
pub(crate) struct SemanticOwnerForestDraftV1 {
    owners: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
}

#[derive(Debug)]
pub struct VerifiedSemanticOwnerForestV1 {
    owners: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
    root: FunctionOwnerIdV1,
    child_at: BTreeMap<OwnedExprSiteV1, FunctionOwnerIdV1>,
    upvar_observations: Box<[UpvarObservationV1]>,
    upvars: Box<[UpvarRefV1]>,
    normalized: NormalizedSemanticOwnerForestGraphV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticOwnerForestVerificationErrorV1 {
    EmptyForest,
    DuplicateOwner(FunctionOwnerIdV1),
    DuplicateParent(FunctionOwnerIdV1),
    OwnerKeyMismatch(FunctionOwnerIdV1),
    MixedCompilation(FunctionOwnerIdV1),
    MissingChildOwner(FunctionOwnerIdV1),
    MissingParentOwner(FunctionOwnerIdV1),
    SelfParent(FunctionOwnerIdV1),
    ParentCycle(FunctionOwnerIdV1),
    MultipleRoots,
    DefinitionSiteOwnerMismatch(FunctionOwnerIdV1),
    ForeignParentScope(FunctionOwnerIdV1),
    MissingParentScope(FunctionOwnerIdV1),
    ParentScopeMismatch(FunctionOwnerIdV1),
    DuplicateDefinitionSite(OwnedExprSiteV1),
    NormalizedOwnerCollision,
    MissingUpvarSource(UpvarRefV1),
    NonAncestorUpvarSource(UpvarRefV1),
    InvisibleUpvarSource(UpvarRefV1),
    ShadowedUpvarSource(UpvarRefV1),
}

impl SemanticOwnerForestDraftV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert_owner(
        &mut self,
        owner: FunctionOwnerIdV1,
        product: VerifiedResolvedFunctionV1,
    ) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
        if self.owners.insert(owner, product).is_some() {
            return Err(SemanticOwnerForestVerificationErrorV1::DuplicateOwner(
                owner,
            ));
        }
        Ok(())
    }

    pub(crate) fn insert_parent(
        &mut self,
        child: FunctionOwnerIdV1,
        edge: OwnerParentEdgeV1,
    ) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
        if self.parents.insert(child, edge).is_some() {
            return Err(SemanticOwnerForestVerificationErrorV1::DuplicateParent(
                child,
            ));
        }
        Ok(())
    }

    pub(crate) fn seal(
        self,
    ) -> Result<VerifiedSemanticOwnerForestV1, SemanticOwnerForestVerificationErrorV1> {
        if self.owners.is_empty() {
            return Err(SemanticOwnerForestVerificationErrorV1::EmptyForest);
        }
        let compilation = self
            .owners
            .first_key_value()
            .expect("non-empty forest checked above")
            .0
            .compilation_brand();
        for (owner, product) in &self.owners {
            if *owner != product.owner() {
                return Err(SemanticOwnerForestVerificationErrorV1::OwnerKeyMismatch(
                    *owner,
                ));
            }
            if owner.compilation_brand() != compilation {
                return Err(SemanticOwnerForestVerificationErrorV1::MixedCompilation(
                    *owner,
                ));
            }
        }

        let mut child_at = BTreeMap::new();
        for (child, edge) in &self.parents {
            verify_parent_edge(&self.owners, *child, edge)?;
            if child_at
                .insert(edge.definition_site.clone(), *child)
                .is_some()
            {
                return Err(
                    SemanticOwnerForestVerificationErrorV1::DuplicateDefinitionSite(
                        edge.definition_site.clone(),
                    ),
                );
            }
        }
        verify_acyclic(&self.owners, &self.parents)?;

        let roots = self
            .owners
            .keys()
            .filter(|owner| !self.parents.contains_key(owner))
            .copied()
            .collect::<Vec<_>>();
        if roots.len() != 1 {
            return Err(SemanticOwnerForestVerificationErrorV1::MultipleRoots);
        }
        let root = roots[0];
        let (upvar_observations, upvars) = derive_and_verify_upvars(&self.owners, &self.parents)?;
        let normalized = build_normalized_forest(
            root,
            &self.owners,
            &self.parents,
            &upvar_observations,
            &upvars,
        )?;
        Ok(VerifiedSemanticOwnerForestV1 {
            owners: self.owners,
            parents: self.parents,
            root,
            child_at,
            upvar_observations,
            upvars,
            normalized,
        })
    }
}

fn derive_and_verify_upvars(
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
) -> Result<(Box<[UpvarObservationV1]>, Box<[UpvarRefV1]>), SemanticOwnerForestVerificationErrorV1>
{
    let mut observations = Vec::new();
    let mut upvars = BTreeSet::new();
    for (owner, product) in owners {
        for (site, lexical_ref) in product.variable_refs() {
            let ResolvedLexicalRefV1::Upvar(upvar) = lexical_ref else {
                continue;
            };
            verify_upvar_relation(*owner, *upvar, owners, parents)?;
            observations.push(UpvarObservationV1 {
                site: OwnedExprSiteV1::new(*owner, site.clone()),
                upvar: *upvar,
                access: UpvarAccessKindV1::Read,
            });
            upvars.insert(*upvar);
        }
        for (site, target) in product.assignment_targets() {
            let super::ResolvedAssignmentTargetV1::UpvarRebind(upvar) = target else {
                continue;
            };
            verify_upvar_relation(*owner, *upvar, owners, parents)?;
            observations.push(UpvarObservationV1 {
                site: OwnedExprSiteV1::new(*owner, site.clone()),
                upvar: *upvar,
                access: UpvarAccessKindV1::Rebind,
            });
            upvars.insert(*upvar);
        }
    }
    observations.sort();
    Ok((
        observations.into_boxed_slice(),
        upvars.into_iter().collect::<Vec<_>>().into_boxed_slice(),
    ))
}

fn verify_upvar_relation(
    owner: FunctionOwnerIdV1,
    upvar: UpvarRefV1,
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
    let source_owner = upvar.source().owner();
    let Some(source_product) = owners.get(&source_owner) else {
        return Err(SemanticOwnerForestVerificationErrorV1::MissingUpvarSource(
            upvar,
        ));
    };
    let Some(source_record) = source_product.binding(upvar.source()) else {
        return Err(SemanticOwnerForestVerificationErrorV1::MissingUpvarSource(
            upvar,
        ));
    };
    let Some(definition_edge) = edge_below_ancestor(owner, source_owner, parents) else {
        return Err(SemanticOwnerForestVerificationErrorV1::NonAncestorUpvarSource(upvar));
    };
    if !binding_visible_at_definition(source_product, source_record.origin(), definition_edge) {
        return Err(SemanticOwnerForestVerificationErrorV1::InvisibleUpvarSource(upvar));
    }
    verify_nearest_visible_source(
        owner,
        upvar,
        source_record.diagnostic_name(),
        owners,
        parents,
    )
}

fn edge_below_ancestor<'a>(
    mut descendant: FunctionOwnerIdV1,
    ancestor: FunctionOwnerIdV1,
    parents: &'a BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
) -> Option<&'a OwnerParentEdgeV1> {
    loop {
        let edge = parents.get(&descendant)?;
        if edge.parent_owner == ancestor {
            return Some(edge);
        }
        descendant = edge.parent_owner;
    }
}

fn verify_nearest_visible_source(
    capturing_owner: FunctionOwnerIdV1,
    upvar: UpvarRefV1,
    diagnostic_name: &str,
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
    let mut child = capturing_owner;
    loop {
        let Some(edge) = parents.get(&child) else {
            return Err(SemanticOwnerForestVerificationErrorV1::NonAncestorUpvarSource(upvar));
        };
        let parent_owner = edge.parent_owner;
        let parent = &owners[&parent_owner];
        if let Some(nearest) = nearest_visible_binding(parent, edge, diagnostic_name) {
            if nearest != upvar.source() {
                return Err(SemanticOwnerForestVerificationErrorV1::ShadowedUpvarSource(
                    upvar,
                ));
            }
            return Ok(());
        }
        if parent_owner == upvar.source().owner() {
            return Err(SemanticOwnerForestVerificationErrorV1::InvisibleUpvarSource(upvar));
        }
        child = parent_owner;
    }
}

fn nearest_visible_binding(
    owner: &VerifiedResolvedFunctionV1,
    edge: &OwnerParentEdgeV1,
    diagnostic_name: &str,
) -> Option<super::BindingRefV1> {
    let mut nearest = None;
    for (binding, record) in owner.bindings() {
        if record.diagnostic_name() != diagnostic_name
            || !binding_visible_at_definition(owner, record.origin(), edge)
        {
            continue;
        }
        nearest = match nearest {
            None => Some(binding),
            Some(current) => {
                let current_scope = owner.binding(current)?.owner_scope();
                let candidate_scope = record.owner_scope();
                if current_scope != candidate_scope
                    && scope_is_ancestor(owner, current_scope, candidate_scope)
                {
                    Some(binding)
                } else {
                    Some(current)
                }
            }
        };
    }
    nearest
}

fn binding_visible_at_definition(
    owner: &VerifiedResolvedFunctionV1,
    origin: &BindingOriginV1,
    edge: &OwnerParentEdgeV1,
) -> bool {
    let binding_scope = match origin {
        BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
        | BindingOriginV1::Source(SourceBindingSiteV1::Parameter { .. }) => {
            Some(owner.function_scope())
        }
        BindingOriginV1::Source(site) => owner
            .declaration_binding(site)
            .and_then(|binding| owner.binding(binding))
            .map(|record| record.owner_scope()),
        BindingOriginV1::Synthetic { .. } => None,
    };
    let Some(binding_scope) = binding_scope else {
        return false;
    };
    if !scope_is_ancestor(owner, binding_scope, edge.parent_scope) {
        return false;
    }
    let Some(declaration_site) = binding_statement(origin) else {
        return matches!(
            origin,
            BindingOriginV1::Source(SourceBindingSiteV1::Receiver)
                | BindingOriginV1::Source(SourceBindingSiteV1::Parameter { .. })
        );
    };
    let Some(scope) = owner.scope(binding_scope) else {
        return false;
    };
    let Some(declaration_index) = direct_member_index(scope.origin(), declaration_site.node())
    else {
        return false;
    };
    let Some(definition_index) =
        direct_member_index(scope.origin(), edge.definition_site.site().node())
    else {
        return false;
    };
    declaration_index < definition_index
}

fn scope_is_ancestor(
    owner: &VerifiedResolvedFunctionV1,
    ancestor: ScopeId,
    mut descendant: ScopeId,
) -> bool {
    loop {
        if descendant == ancestor {
            return true;
        }
        let Some(parent) = owner.scope(descendant).and_then(|record| record.parent()) else {
            return false;
        };
        descendant = parent;
    }
}

fn binding_statement(origin: &BindingOriginV1) -> Option<&SourceStmtSiteV1> {
    match origin {
        BindingOriginV1::Source(SourceBindingSiteV1::Local { statement, .. })
        | BindingOriginV1::Source(SourceBindingSiteV1::Outbox { statement, .. })
        | BindingOriginV1::Source(SourceBindingSiteV1::Nowait { statement }) => Some(statement),
        _ => None,
    }
}

fn direct_member_index(origin: &ScopeOriginV1, site: &SourceNodeSiteV1) -> Option<u32> {
    let ScopeOriginV1::Source(origin) = origin else {
        return None;
    };
    let (root, prefix) = origin.segments().split_last()?;
    let member = site.segments().strip_prefix(prefix)?.first()?;
    match (root, member) {
        (SourcePathSegmentV1::FunctionBody, SourcePathSegmentV1::Body(index))
        | (SourcePathSegmentV1::LambdaBodyRoot, SourcePathSegmentV1::LambdaBody(index))
        | (SourcePathSegmentV1::ScopeBodyRoot, SourcePathSegmentV1::ScopeBody(index))
        | (SourcePathSegmentV1::TaskScopeBodyRoot, SourcePathSegmentV1::TaskScopeBody(index))
        | (SourcePathSegmentV1::FastMemBodyRoot, SourcePathSegmentV1::FastMemBody(index))
        | (
            SourcePathSegmentV1::BlockExprPreludeRoot,
            SourcePathSegmentV1::BlockExprPrelude(index),
        )
        | (SourcePathSegmentV1::IfThenBody, SourcePathSegmentV1::IfThen(index))
        | (SourcePathSegmentV1::IfElseBody, SourcePathSegmentV1::IfElse(index))
        | (SourcePathSegmentV1::LoopBodyRoot, SourcePathSegmentV1::LoopBody(index)) => Some(*index),
        (SourcePathSegmentV1::BlockExprPreludeRoot, SourcePathSegmentV1::BlockExprTail) => {
            Some(u32::MAX)
        }
        _ => None,
    }
}

fn verify_parent_edge(
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    child: FunctionOwnerIdV1,
    edge: &OwnerParentEdgeV1,
) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
    if !owners.contains_key(&child) {
        return Err(SemanticOwnerForestVerificationErrorV1::MissingChildOwner(
            child,
        ));
    }
    let Some(parent) = owners.get(&edge.parent_owner) else {
        return Err(SemanticOwnerForestVerificationErrorV1::MissingParentOwner(
            edge.parent_owner,
        ));
    };
    if child == edge.parent_owner {
        return Err(SemanticOwnerForestVerificationErrorV1::SelfParent(child));
    }
    if edge.definition_site.owner() != edge.parent_owner {
        return Err(SemanticOwnerForestVerificationErrorV1::DefinitionSiteOwnerMismatch(child));
    }
    if edge.parent_scope.owner() != edge.parent_owner {
        return Err(SemanticOwnerForestVerificationErrorV1::ForeignParentScope(
            child,
        ));
    }
    if parent.scope(edge.parent_scope).is_none() {
        return Err(SemanticOwnerForestVerificationErrorV1::MissingParentScope(
            child,
        ));
    }
    if parent.exact_scope_containing(edge.definition_site.site().node()) != Some(edge.parent_scope)
    {
        return Err(SemanticOwnerForestVerificationErrorV1::ParentScopeMismatch(
            child,
        ));
    }
    Ok(())
}

fn verify_acyclic(
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
) -> Result<(), SemanticOwnerForestVerificationErrorV1> {
    for owner in owners.keys().copied() {
        let mut seen = BTreeSet::new();
        let mut cursor = owner;
        while let Some(edge) = parents.get(&cursor) {
            if !seen.insert(cursor) {
                return Err(SemanticOwnerForestVerificationErrorV1::ParentCycle(owner));
            }
            cursor = edge.parent_owner;
        }
    }
    Ok(())
}

fn build_normalized_forest(
    root: FunctionOwnerIdV1,
    owners: &BTreeMap<FunctionOwnerIdV1, VerifiedResolvedFunctionV1>,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
    observations: &[UpvarObservationV1],
    upvars: &[UpvarRefV1],
) -> Result<NormalizedSemanticOwnerForestGraphV1, SemanticOwnerForestVerificationErrorV1> {
    let root_origin = owners[&root].function_origin();
    let root_source_kind = owners[&root].source_kind();
    let mut keys = BTreeMap::new();
    for owner in owners.keys().copied() {
        normalized_owner_key(owner, root_origin, root_source_kind, parents, &mut keys);
    }
    let mut unique = BTreeSet::new();
    if keys.values().any(|key| !unique.insert(key.clone())) {
        return Err(SemanticOwnerForestVerificationErrorV1::NormalizedOwnerCollision);
    }
    let mut records = owners
        .iter()
        .map(|(owner, product)| {
            let edge = parents.get(owner);
            NormalizedOwnerRecordV1 {
                key: keys[owner].clone(),
                parent: edge.map(|edge| keys[&edge.parent_owner].clone()),
                definition_site: edge.map(|edge| edge.definition_site.site().clone()),
                parent_scope: edge.map(|edge| normalized_scope(&owners[&edge.parent_owner], edge)),
                product: product.normalized_graph().clone(),
            }
        })
        .collect::<Vec<_>>();
    records.sort_by(|left, right| left.key.cmp(&right.key));
    let mut upvar_observations = observations
        .iter()
        .map(|observation| {
            let source = observation.upvar.source();
            let source_record = owners[&source.owner()].binding(source).unwrap();
            NormalizedUpvarObservationV1 {
                owner: keys[&observation.upvar.capturing_owner()].clone(),
                site: observation.site.site().clone(),
                access: observation.access,
                source_owner: keys[&source.owner()].clone(),
                source_binding: NormalizedBindingKeyV1(source_record.origin().clone()),
            }
        })
        .collect::<Vec<_>>();
    upvar_observations.sort();
    let mut normalized_upvars = upvars
        .iter()
        .map(|upvar| {
            let source = upvar.source();
            let source_record = owners[&source.owner()].binding(source).unwrap();
            NormalizedUpvarEdgeV1 {
                capturing_owner: keys[&upvar.capturing_owner()].clone(),
                source_owner: keys[&source.owner()].clone(),
                source_binding: NormalizedBindingKeyV1(source_record.origin().clone()),
            }
        })
        .collect::<Vec<_>>();
    normalized_upvars.sort();
    Ok(NormalizedSemanticOwnerForestGraphV1 {
        root: keys[&root].clone(),
        owners: records.into_boxed_slice(),
        upvar_observations: upvar_observations.into_boxed_slice(),
        upvars: normalized_upvars.into_boxed_slice(),
    })
}

fn normalized_owner_key(
    owner: FunctionOwnerIdV1,
    root: FunctionOriginV1,
    root_source_kind: super::SemanticOwnerSourceKindV1,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
    cache: &mut BTreeMap<FunctionOwnerIdV1, NormalizedOwnerKeyV1>,
) -> NormalizedOwnerKeyV1 {
    if let Some(key) = cache.get(&owner) {
        return key.clone();
    }
    let definition_chain = if let Some(edge) = parents.get(&owner) {
        let mut chain =
            normalized_owner_key(edge.parent_owner, root, root_source_kind, parents, cache)
                .definition_chain
                .into_vec();
        chain.push(edge.definition_site.site().clone());
        chain
    } else {
        Vec::new()
    };
    let key = NormalizedOwnerKeyV1 {
        root,
        root_source_kind,
        definition_chain: definition_chain.into_boxed_slice(),
    };
    cache.insert(owner, key.clone());
    key
}

fn normalized_scope(
    owner: &VerifiedResolvedFunctionV1,
    edge: &OwnerParentEdgeV1,
) -> NormalizedScopeKeyV1 {
    let record: &ResolvedScopeRecordV1 = owner.scope(edge.parent_scope).unwrap();
    NormalizedScopeKeyV1 {
        kind: record.kind(),
        origin: record.origin().clone(),
    }
}

impl VerifiedSemanticOwnerForestV1 {
    pub(crate) fn owners(
        &self,
    ) -> impl Iterator<Item = (FunctionOwnerIdV1, &VerifiedResolvedFunctionV1)> {
        self.owners.iter().map(|(owner, product)| (*owner, product))
    }

    pub fn owner(&self, owner: FunctionOwnerIdV1) -> Option<&VerifiedResolvedFunctionV1> {
        self.owners.get(&owner)
    }

    pub fn parent(&self, child: FunctionOwnerIdV1) -> Option<&OwnerParentEdgeV1> {
        self.parents.get(&child)
    }

    pub fn roots(&self) -> &[FunctionOwnerIdV1] {
        std::slice::from_ref(&self.root)
    }

    pub fn child_at(&self, site: &OwnedExprSiteV1) -> Option<FunctionOwnerIdV1> {
        self.child_at.get(site).copied()
    }

    pub fn owner_count(&self) -> usize {
        self.owners.len()
    }

    pub fn upvars(&self) -> &[UpvarRefV1] {
        &self.upvars
    }

    pub fn upvar_observations(&self) -> &[UpvarObservationV1] {
        &self.upvar_observations
    }

    pub fn normalized_graph(&self) -> &NormalizedSemanticOwnerForestGraphV1 {
        &self.normalized
    }
}
