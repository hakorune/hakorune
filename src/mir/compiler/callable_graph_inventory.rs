//! P0c-MR-G0 shared callable topology inventory.
//!
//! Targets are already resolved by the complete catalog. This product projects
//! those identities to canonical keys once, preserves exact call-site
//! multiplicity, and derives unique topology edges. It owns no DAG/SCC proof,
//! ABI, evaluation order, effect, MIR, publication, or backend policy.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    CanonicalCallableKeyV1, ResolvedCallableRefV1, SourceExprSiteV1,
};

use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedCallableGraphSiteV1 {
    caller: CanonicalCallableKeyV1,
    site: SourceExprSiteV1,
    target: CanonicalCallableKeyV1,
}

impl VerifiedCallableGraphSiteV1 {
    pub(crate) const fn caller(&self) -> &CanonicalCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &CanonicalCallableKeyV1 {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct VerifiedCallableGraphEdgeV1 {
    caller: CanonicalCallableKeyV1,
    target: CanonicalCallableKeyV1,
}

impl VerifiedCallableGraphEdgeV1 {
    pub(crate) const fn caller(&self) -> &CanonicalCallableKeyV1 {
        &self.caller
    }

    pub(crate) const fn target(&self) -> &CanonicalCallableKeyV1 {
        &self.target
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallableGraphInventoryV1 {
    nodes: Box<[CanonicalCallableKeyV1]>,
    call_sites: Box<[VerifiedCallableGraphSiteV1]>,
    unique_edges: Box<[VerifiedCallableGraphEdgeV1]>,
}

impl VerifiedCallableGraphInventoryV1 {
    pub(crate) fn verify(
        module: &VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, CallableGraphInventoryErrorV1> {
        let catalog = module.source().catalog();
        let functions = module.functions_by_key();
        if catalog.len() != functions.len()
            || functions
                .keys()
                .any(|key| catalog.index().lookup(key).is_none())
        {
            return Err(CallableGraphInventoryErrorV1::NodeSetMismatch {
                catalog: catalog.len(),
                functions: functions.len(),
            });
        }

        let nodes = functions.keys().cloned().collect::<Vec<_>>();
        let mut call_sites = Vec::new();
        for (caller, unit) in functions {
            let [root] = unit.forest().roots() else {
                return Err(CallableGraphInventoryErrorV1::RootCardinality {
                    caller: caller.clone(),
                    actual: unit.forest().roots().len(),
                });
            };
            let function = unit.forest().owner(*root).ok_or_else(|| {
                CallableGraphInventoryErrorV1::MissingCallerFunction {
                    caller: caller.clone(),
                }
            })?;
            for (site, target) in function.direct_call_targets() {
                let target_key = catalog
                    .index()
                    .header_for_callable(target.callable())
                    .map_err(
                        |_| CallableGraphInventoryErrorV1::TargetIdentityOutsideCatalog {
                            caller: caller.clone(),
                            site: site.clone(),
                            target: target.callable(),
                        },
                    )?
                    .source_key()
                    .clone();
                call_sites.push(VerifiedCallableGraphSiteV1 {
                    caller: caller.clone(),
                    site: site.clone(),
                    target: target_key,
                });
            }
        }
        seal_inventory(nodes, call_sites)
    }

    pub(crate) fn nodes(&self) -> &[CanonicalCallableKeyV1] {
        &self.nodes
    }

    pub(crate) fn call_sites(&self) -> &[VerifiedCallableGraphSiteV1] {
        &self.call_sites
    }

    pub(crate) fn unique_edges(&self) -> &[VerifiedCallableGraphEdgeV1] {
        &self.unique_edges
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CallableGraphInventoryErrorV1 {
    NodeSetMismatch {
        catalog: usize,
        functions: usize,
    },
    RootCardinality {
        caller: CanonicalCallableKeyV1,
        actual: usize,
    },
    MissingCallerFunction {
        caller: CanonicalCallableKeyV1,
    },
    TargetIdentityOutsideCatalog {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
        target: ResolvedCallableRefV1,
    },
    TargetOutsideNodeSet {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
        target: CanonicalCallableKeyV1,
    },
    DuplicateGraphSite {
        caller: CanonicalCallableKeyV1,
        site: SourceExprSiteV1,
    },
}

fn seal_inventory(
    mut nodes: Vec<CanonicalCallableKeyV1>,
    mut call_sites: Vec<VerifiedCallableGraphSiteV1>,
) -> Result<VerifiedCallableGraphInventoryV1, CallableGraphInventoryErrorV1> {
    nodes.sort();
    nodes.dedup();
    call_sites.sort_by(|left, right| {
        (&left.caller, &left.site, &left.target).cmp(&(&right.caller, &right.site, &right.target))
    });

    let node_set = nodes.iter().cloned().collect::<BTreeSet<_>>();
    let mut site_set = BTreeSet::new();
    let mut edge_set = BTreeSet::new();
    for row in &call_sites {
        if !node_set.contains(&row.caller) || !node_set.contains(&row.target) {
            return Err(CallableGraphInventoryErrorV1::TargetOutsideNodeSet {
                caller: row.caller.clone(),
                site: row.site.clone(),
                target: row.target.clone(),
            });
        }
        if !site_set.insert((row.caller.clone(), row.site.clone())) {
            return Err(CallableGraphInventoryErrorV1::DuplicateGraphSite {
                caller: row.caller.clone(),
                site: row.site.clone(),
            });
        }
        edge_set.insert(VerifiedCallableGraphEdgeV1 {
            caller: row.caller.clone(),
            target: row.target.clone(),
        });
    }

    Ok(VerifiedCallableGraphInventoryV1 {
        nodes: nodes.into_boxed_slice(),
        call_sites: call_sites.into_boxed_slice(),
        unique_edges: edge_set.into_iter().collect::<Vec<_>>().into_boxed_slice(),
    })
}

#[cfg(test)]
mod tests;
