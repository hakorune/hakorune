//! Deterministic semantic graph containing provenance keys, never raw IDs.

use std::collections::BTreeMap;

use super::ids::{BindingRefV1, ScopeId};
use super::product::ResolvedFunctionDataV1;
use super::records::{
    BindingKindV1, BindingOriginV1, RegionKindV1, RegionOriginV1, ResolvedAssignmentTargetV1,
    ResolvedControlTransferV1, ResolvedExitOriginV1, ResolvedLexicalRefV1, ScopeKindV1,
    ScopeOriginV1,
};
use super::source_site::{
    FunctionOriginV1, ResolvedExitSiteV1, SourceBindingSiteV1, SourceExprSiteV1,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedBindingKeyV1(pub BindingOriginV1);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedScopeKeyV1 {
    pub kind: ScopeKindV1,
    pub origin: ScopeOriginV1,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedRegionKeyV1 {
    pub kind: RegionKindV1,
    pub origin: RegionOriginV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedBindingRecordV1 {
    pub key: NormalizedBindingKeyV1,
    pub diagnostic_name: Box<str>,
    pub kind: BindingKindV1,
    pub owner_scope: NormalizedScopeKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedScopeRecordV1 {
    pub key: NormalizedScopeKeyV1,
    pub parent: Option<NormalizedScopeKeyV1>,
    pub owner_region: NormalizedRegionKeyV1,
    pub declarations: Box<[NormalizedBindingKeyV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedRegionRecordV1 {
    pub key: NormalizedRegionKeyV1,
    pub parent: Option<NormalizedRegionKeyV1>,
    pub lexical_scope: Option<NormalizedScopeKeyV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedDeclarationV1 {
    pub site: SourceBindingSiteV1,
    pub binding: NormalizedBindingKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedVariableUseV1 {
    pub site: SourceExprSiteV1,
    pub binding: NormalizedBindingKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedAssignmentTargetV1 {
    BindingRebind(NormalizedBindingKeyV1),
    FieldWrite { receiver: SourceExprSiteV1 },
    IndexWrite { receiver: SourceExprSiteV1 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedAssignmentV1 {
    pub site: SourceExprSiteV1,
    pub target: NormalizedAssignmentTargetV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedControlTransferV1 {
    Continue {
        target_loop: NormalizedRegionKeyV1,
    },
    Break {
        target_loop: NormalizedRegionKeyV1,
    },
    Return {
        target_function: NormalizedRegionKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedExitV1 {
    pub site: ResolvedExitSiteV1,
    pub source_region: NormalizedRegionKeyV1,
    pub origin: ResolvedExitOriginV1,
    pub transfer: NormalizedControlTransferV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedResolvedFunctionGraphV1 {
    function_origin: FunctionOriginV1,
    source_kind: super::SemanticOwnerSourceKindV1,
    bindings: Box<[NormalizedBindingRecordV1]>,
    scopes: Box<[NormalizedScopeRecordV1]>,
    regions: Box<[NormalizedRegionRecordV1]>,
    declarations: Box<[NormalizedDeclarationV1]>,
    variable_uses: Box<[NormalizedVariableUseV1]>,
    assignments: Box<[NormalizedAssignmentV1]>,
    exits: Box<[NormalizedExitV1]>,
}

impl NormalizedResolvedFunctionGraphV1 {
    pub const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub const fn source_kind(&self) -> super::SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub fn bindings(&self) -> &[NormalizedBindingRecordV1] {
        &self.bindings
    }

    pub fn scopes(&self) -> &[NormalizedScopeRecordV1] {
        &self.scopes
    }

    pub fn regions(&self) -> &[NormalizedRegionRecordV1] {
        &self.regions
    }

    pub fn declarations(&self) -> &[NormalizedDeclarationV1] {
        &self.declarations
    }

    pub fn variable_uses(&self) -> &[NormalizedVariableUseV1] {
        &self.variable_uses
    }

    pub fn assignments(&self) -> &[NormalizedAssignmentV1] {
        &self.assignments
    }

    pub fn exits(&self) -> &[NormalizedExitV1] {
        &self.exits
    }
}

pub(super) fn build_normalized_graph(
    data: &ResolvedFunctionDataV1,
) -> NormalizedResolvedFunctionGraphV1 {
    let binding_keys = data
        .bindings
        .iter()
        .map(|(id, record)| (*id, NormalizedBindingKeyV1(record.origin().clone())))
        .collect::<BTreeMap<_, _>>();
    let scope_keys = data
        .scopes
        .iter()
        .map(|(id, record)| {
            (
                *id,
                NormalizedScopeKeyV1 {
                    kind: record.kind(),
                    origin: record.origin().clone(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    let region_keys = data
        .regions
        .iter()
        .map(|(id, record)| {
            (
                *id,
                NormalizedRegionKeyV1 {
                    kind: record.kind(),
                    origin: record.origin().clone(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut bindings = data
        .bindings
        .iter()
        .map(|(id, record)| NormalizedBindingRecordV1 {
            key: binding_keys[id].clone(),
            diagnostic_name: record.diagnostic_name().into(),
            kind: record.kind(),
            owner_scope: scope_keys[&record.owner_scope()].clone(),
        })
        .collect::<Vec<_>>();
    bindings.sort_by(|left, right| left.key.cmp(&right.key));

    let mut scopes = data
        .scopes
        .iter()
        .map(|(id, record)| {
            let mut declarations = record
                .declarations()
                .iter()
                .map(|binding| binding_key(binding, &binding_keys))
                .collect::<Vec<_>>();
            declarations.sort();
            NormalizedScopeRecordV1 {
                key: scope_keys[id].clone(),
                parent: record.parent().map(|parent| scope_keys[&parent].clone()),
                owner_region: region_keys[&record.owner_region()].clone(),
                declarations: declarations.into_boxed_slice(),
            }
        })
        .collect::<Vec<_>>();
    scopes.sort_by(|left, right| left.key.cmp(&right.key));

    let mut regions = data
        .regions
        .iter()
        .map(|(id, record)| NormalizedRegionRecordV1 {
            key: region_keys[id].clone(),
            parent: record.parent().map(|parent| region_keys[&parent].clone()),
            lexical_scope: record.scope_key(&scope_keys),
        })
        .collect::<Vec<_>>();
    regions.sort_by(|left, right| left.key.cmp(&right.key));

    NormalizedResolvedFunctionGraphV1 {
        function_origin: data.function_origin,
        source_kind: data.source_kind,
        bindings: bindings.into_boxed_slice(),
        scopes: scopes.into_boxed_slice(),
        regions: regions.into_boxed_slice(),
        declarations: normalize_declarations(data, &binding_keys),
        variable_uses: normalize_uses(data, &binding_keys),
        assignments: normalize_assignments(data, &binding_keys),
        exits: normalize_exits(data, &region_keys),
    }
}

fn binding_key(
    binding: &BindingRefV1,
    keys: &BTreeMap<hakorune_mir_core::BindingId, NormalizedBindingKeyV1>,
) -> NormalizedBindingKeyV1 {
    keys[&binding.binding()].clone()
}

trait RegionScopeKeyV1 {
    fn scope_key(
        &self,
        keys: &BTreeMap<ScopeId, NormalizedScopeKeyV1>,
    ) -> Option<NormalizedScopeKeyV1>;
}

impl RegionScopeKeyV1 for super::records::ResolvedRegionRecordV1 {
    fn scope_key(
        &self,
        keys: &BTreeMap<ScopeId, NormalizedScopeKeyV1>,
    ) -> Option<NormalizedScopeKeyV1> {
        self.lexical_scope().map(|scope| keys[&scope].clone())
    }
}

fn normalize_declarations(
    data: &ResolvedFunctionDataV1,
    binding_keys: &BTreeMap<hakorune_mir_core::BindingId, NormalizedBindingKeyV1>,
) -> Box<[NormalizedDeclarationV1]> {
    data.declarations
        .iter()
        .map(|(site, binding)| NormalizedDeclarationV1 {
            site: site.clone(),
            binding: binding_key(binding, binding_keys),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn normalize_uses(
    data: &ResolvedFunctionDataV1,
    binding_keys: &BTreeMap<hakorune_mir_core::BindingId, NormalizedBindingKeyV1>,
) -> Box<[NormalizedVariableUseV1]> {
    data.variable_uses
        .iter()
        .filter_map(|(site, lexical_ref)| {
            let ResolvedLexicalRefV1::Local(binding) = lexical_ref else {
                return None;
            };
            Some(NormalizedVariableUseV1 {
                site: site.clone(),
                binding: binding_key(binding, binding_keys),
            })
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn normalize_assignments(
    data: &ResolvedFunctionDataV1,
    binding_keys: &BTreeMap<hakorune_mir_core::BindingId, NormalizedBindingKeyV1>,
) -> Box<[NormalizedAssignmentV1]> {
    data.assignment_targets
        .iter()
        .filter_map(|(site, target)| {
            let target = match target {
                ResolvedAssignmentTargetV1::UpvarRebind(_) => return None,
                ResolvedAssignmentTargetV1::BindingRebind(binding) => {
                    NormalizedAssignmentTargetV1::BindingRebind(binding_key(binding, binding_keys))
                }
                ResolvedAssignmentTargetV1::FieldWrite { receiver } => {
                    NormalizedAssignmentTargetV1::FieldWrite {
                        receiver: receiver.clone(),
                    }
                }
                ResolvedAssignmentTargetV1::IndexWrite { receiver } => {
                    NormalizedAssignmentTargetV1::IndexWrite {
                        receiver: receiver.clone(),
                    }
                }
            };
            Some(NormalizedAssignmentV1 {
                site: site.clone(),
                target,
            })
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn normalize_exits(
    data: &ResolvedFunctionDataV1,
    region_keys: &BTreeMap<super::ids::RegionId, NormalizedRegionKeyV1>,
) -> Box<[NormalizedExitV1]> {
    data.resolved_exits
        .iter()
        .map(|(site, exit)| NormalizedExitV1 {
            site: site.clone(),
            source_region: region_keys[&exit.source_region()].clone(),
            origin: exit.origin(),
            transfer: match exit.transfer() {
                ResolvedControlTransferV1::Continue { target_loop } => {
                    NormalizedControlTransferV1::Continue {
                        target_loop: region_keys[&target_loop].clone(),
                    }
                }
                ResolvedControlTransferV1::Break { target_loop } => {
                    NormalizedControlTransferV1::Break {
                        target_loop: region_keys[&target_loop].clone(),
                    }
                }
                ResolvedControlTransferV1::Return { target_function } => {
                    NormalizedControlTransferV1::Return {
                        target_function: region_keys[&target_function].clone(),
                    }
                }
            },
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
