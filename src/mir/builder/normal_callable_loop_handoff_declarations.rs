//! Exact local declarations retained across the ready/body-only partition.
//!
//! Unread locals still have lexical scope. They are retained here without
//! adding a binding-use row or manufacturing a loop carrier.

use super::*;

impl CallableLoopSourceProjectionV1<'_> {
    pub(super) fn local_declarations(
        &self,
        loop_site: &SourceNodeSiteV1,
    ) -> Result<BTreeMap<BindingRefV1, SourceBindingSiteV1>, String> {
        let mut declarations = BTreeMap::new();
        for (site, bindings) in self.locals {
            if !is_direct_loop_body_descendant(loop_site, site) {
                continue;
            }
            for (ordinal, &binding) in bindings.iter().enumerate() {
                if binding.owner() != self.owner {
                    return Err(freeze("foreign-local-declaration"));
                }
                let ordinal = u32::try_from(ordinal)
                    .map_err(|_| freeze("local-declaration-ordinal-overflow"))?;
                let declaration = SourceBindingSiteV1::Local {
                    statement: SourceStmtSiteV1::from_node(site.clone()),
                    ordinal,
                };
                if declarations.insert(binding, declaration).is_some() {
                    return Err(freeze("duplicate-local-declaration-binding"));
                }
            }
        }
        Ok(declarations)
    }
}

pub(super) fn observed_iteration_locals(
    owner: FunctionOwnerIdV1,
    loop_site: &SourceNodeSiteV1,
    receipts: &[CallableLoopBindingReceiptV1],
    declarations: &BTreeMap<BindingRefV1, SourceBindingSiteV1>,
) -> Result<BTreeSet<BindingRefV1>, String> {
    let mut sites = BTreeSet::new();
    for (binding, declaration) in declarations {
        if binding.owner() != owner {
            return Err(freeze("foreign-local-declaration"));
        }
        let SourceBindingSiteV1::Local { statement, ordinal } = declaration else {
            return Err(freeze("nonlocal-declaration"));
        };
        if !is_direct_loop_body_descendant(loop_site, statement.node()) {
            return Err(freeze("local-declaration-outside-loop"));
        }
        if !sites.insert((statement.node().clone(), *ordinal)) {
            return Err(freeze("duplicate-local-declaration-site"));
        }
    }
    Ok(receipts
        .iter()
        .map(CallableLoopBindingReceiptV1::binding)
        .filter(|binding| declarations.contains_key(binding))
        .collect())
}

impl CallableSemanticLoopHandoffPreEffectReceiptV1 {
    pub(in crate::mir::builder) fn local_declarations(
        &self,
    ) -> &BTreeMap<BindingRefV1, SourceBindingSiteV1> {
        &self.local_declarations
    }
}
