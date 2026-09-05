//! Physical completion of source-issued ordinary-New destinations.
//!
//! Target take, whole-expression completion and local installation are distinct.
//! This retains their exact relation; it does not issue Home availability or
//! claim that Fault cleanup is implemented.

use super::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceNodeSiteV1,
};
use crate::mir::ValueId;

#[derive(Debug)]
pub(super) struct NewLocalCommitV1 {
    binding: BindingRefV1,
    declaration: SourceBindingSiteV1,
    initializer: Option<ValueId>,
    local: Option<ValueId>,
}

impl NewLocalCommitV1 {
    pub(super) fn pending(binding: BindingRefV1, declaration: SourceBindingSiteV1) -> Self {
        Self {
            binding,
            declaration,
            initializer: None,
            local: None,
        }
    }

    pub(super) fn is_complete(&self) -> bool {
        self.initializer.is_some() && self.local.is_some()
    }

    fn at_statement(&self, owner: FunctionOwnerIdV1, site: &SourceNodeSiteV1) -> bool {
        self.binding.owner() == owner
            && matches!(&self.declaration,
            SourceBindingSiteV1::Local { statement, .. } if statement.node() == site)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Called after all New overrides, never merely after Birth returns.
    pub(crate) fn complete_new_expression(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        value: ValueId,
    ) -> Result<(), String> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(());
        }
        let mut rows = self.local_commits.borrow_mut();
        let row = rows
            .get_mut(site)
            .ok_or_else(|| freeze("expression-without-target-take"))?;
        if row.initializer.is_some() || row.local.is_some() {
            return Err(freeze("duplicate-expression-completion"));
        }
        row.initializer = Some(value);
        Ok(())
    }

    /// The caller supplies exact BindingRefs from the existing callable state
    /// and values from the sole completed-local terminal. Validate the entire
    /// statement before committing any row, including ordinal and RHS identity.
    pub(crate) fn complete_local_installation(
        &self,
        owner: FunctionOwnerIdV1,
        statement: &SourceNodeSiteV1,
        completed: &[(BindingRefV1, u32, ValueId, ValueId)],
    ) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        for (binding, ordinal, _, _) in completed {
            if binding.owner() != owner || !seen.insert(*ordinal) {
                return Err(freeze("foreign-or-duplicate-local"));
            }
        }
        if self.claims.borrow().values().any(|claim| {
            claim.destination.owner() == owner
                && matches!(&claim.declaration,
                SourceBindingSiteV1::Local { statement: expected, .. }
                    if expected.node() == statement)
        }) {
            return Err(freeze("local-before-target-take"));
        }
        let mut rows = self.local_commits.borrow_mut();
        let mut commits = Vec::new();
        for (site, row) in rows
            .iter()
            .filter(|(_, row)| row.at_statement(owner, statement))
        {
            let SourceBindingSiteV1::Local { ordinal, .. } = &row.declaration else {
                unreachable!("at_statement requires Local");
            };
            let (_, _, initializer, local) = completed
                .iter()
                .find(|(binding, index, _, _)| *binding == row.binding && index == ordinal)
                .ok_or_else(|| freeze("local-binding-or-ordinal-mismatch"))?;
            if row.local.is_some() {
                return Err(freeze("duplicate-local-installation"));
            }
            if row.initializer != Some(*initializer) || initializer == local {
                return Err(freeze("local-initializer-mismatch"));
            }
            commits.push((site.clone(), *local));
        }
        for (site, local) in commits {
            rows.get_mut(&site)
                .expect("validated row remains present")
                .local = Some(local);
        }
        Ok(())
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/local-commit/{reason}]")
}
