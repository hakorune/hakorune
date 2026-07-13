//! Sealed cross-owner topology for one root function and its nested owners.

use std::collections::{BTreeMap, BTreeSet};

use super::normalized::{NormalizedResolvedFunctionGraphV1, NormalizedScopeKeyV1};
use super::records::ResolvedScopeRecordV1;
use super::{
    FunctionOriginV1, FunctionOwnerIdV1, OwnedExprSiteV1, ScopeId, SourceExprSiteV1,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedSemanticOwnerForestGraphV1 {
    root: NormalizedOwnerKeyV1,
    owners: Box<[NormalizedOwnerRecordV1]>,
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
        let normalized = build_normalized_forest(root, &self.owners, &self.parents)?;
        Ok(VerifiedSemanticOwnerForestV1 {
            owners: self.owners,
            parents: self.parents,
            root,
            child_at,
            normalized,
        })
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
) -> Result<NormalizedSemanticOwnerForestGraphV1, SemanticOwnerForestVerificationErrorV1> {
    let root_origin = owners[&root].function_origin();
    let mut keys = BTreeMap::new();
    for owner in owners.keys().copied() {
        normalized_owner_key(owner, root_origin, parents, &mut keys);
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
    Ok(NormalizedSemanticOwnerForestGraphV1 {
        root: keys[&root].clone(),
        owners: records.into_boxed_slice(),
    })
}

fn normalized_owner_key(
    owner: FunctionOwnerIdV1,
    root: FunctionOriginV1,
    parents: &BTreeMap<FunctionOwnerIdV1, OwnerParentEdgeV1>,
    cache: &mut BTreeMap<FunctionOwnerIdV1, NormalizedOwnerKeyV1>,
) -> NormalizedOwnerKeyV1 {
    if let Some(key) = cache.get(&owner) {
        return key.clone();
    }
    let definition_chain = if let Some(edge) = parents.get(&owner) {
        let mut chain = normalized_owner_key(edge.parent_owner, root, parents, cache)
            .definition_chain
            .into_vec();
        chain.push(edge.definition_site.site().clone());
        chain
    } else {
        Vec::new()
    };
    let key = NormalizedOwnerKeyV1 {
        root,
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

    pub fn normalized_graph(&self) -> &NormalizedSemanticOwnerForestGraphV1 {
        &self.normalized
    }
}
