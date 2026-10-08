//! Borrowed whole-Root object result evidence; grants no physical ABI permission.
use super::super::{result_class_claim, OrdinaryNewClaimLedgerV1};
use super::normal_return::NormalReturnDispositionV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1};
use result_class_claim::OrdinaryNewResultClassV1;
use std::collections::BTreeSet;
use std::rc::Rc;

/// Metadata borrows the original Facts and terminals, never a second issuer.
pub(in crate::mir::normal_callable_semantic_package) struct RootObjectResultDescriptorV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) owner: FunctionOwnerIdV1,
    pub(in crate::mir::normal_callable_semantic_package) class: &'a str,
    pub(in crate::mir::normal_callable_semantic_package) kind: InvokeCallResultKind,
    pub(in crate::mir::normal_callable_semantic_package) terminals: Vec<&'a TerminalRelationV1>,
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn checked_root_object_result_v1(
        &self,
    ) -> Result<Option<RootObjectResultDescriptorV1<'_>>, String> {
        let Some(key) = &self.app_main_catalog_key else {
            return Ok(None);
        };
        let (class, kind) = match self.callable_result_classes.get(key) {
            Some(OrdinaryNewResultClassV1::Object(class)) => {
                (class.as_ref(), InvokeCallResultKind::Handle)
            }
            Some(OrdinaryNewResultClassV1::NullableObject(class)) => {
                (class.as_ref(), InvokeCallResultKind::NullableHandle)
            }
            _ => return Ok(None),
        };
        let Some(root) = &self.root_completion else {
            return Ok(None);
        };
        let root = root
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("completion")))?;
        let owner = root.owner();
        self.validate_object_return_disposition_exits_v1(owner, root.explicit_sites())?;
        let Some(indexed) = self.completion_index.get(&owner) else {
            return Ok(None);
        };
        let indexed = indexed
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("indexed-completion")))?;
        if !Rc::ptr_eq(root, indexed) {
            return Err(freeze("completion-identity"));
        }
        let Some(table) = self.terminal_relation_index.get(&owner) else {
            return Ok(None);
        };
        if !Rc::ptr_eq(table, &self.terminal_relation) {
            return Err(freeze("terminal-table-identity"));
        }
        let outcomes = self
            .callable_result_classes
            .outcomes(key)
            .ok_or_else(|| freeze("facts-missing"))?;
        let mut facts = BTreeSet::new();
        for row in outcomes {
            if row.site().owner() != owner
                || !facts.insert(row.site().clone())
                || row.witnesses().is_empty()
                || row.witnesses().iter().any(|w| w.site() != row.site())
            {
                return Err(freeze("facts-identity"));
            }
            let origins: BTreeSet<_> = row.witnesses().iter().map(|w| w.origin().clone()).collect();
            if &origins != row.alternatives() {
                return Err(freeze("facts-alternatives"));
            }
            for origin in &origins {
                match origin {
                    result_class_claim::ResultValueOriginV1::Fresh(name)
                        if name.as_ref() == class => {}
                    result_class_claim::ResultValueOriginV1::Null
                        if kind == InvokeCallResultKind::NullableHandle => {}
                    result_class_claim::ResultValueOriginV1::ForwardFormal { .. } => {
                        return Err(freeze("facts-owned-origin"))
                    }
                    _ => return Err(freeze("facts-class-kind")),
                }
            }
        }
        let mut ready = root.returns_value()
            && root.implicit_body_end().is_none()
            && !root.explicit_sites().is_empty();
        let mut mapped = BTreeSet::new();
        let mut exits = BTreeSet::new();
        let mut terminals = Vec::new();
        for exit in root.explicit_sites() {
            if !exits.insert(exit.clone()) {
                return Err(freeze("exit-duplicate"));
            }
            let Some(relation) = table.get(exit) else {
                ready = false;
                continue;
            };
            if relation.owner() != owner || relation.return_site() != exit {
                return Err(freeze("terminal-identity"));
            }
            // Visit every sibling even when an earlier source is unavailable.
            ready &= self.normal_exit_projection_v1(owner, exit)?.is_some();
            let TerminalRelationV1::Value(value) = relation else {
                ready = false;
                continue;
            };
            let site = OwnedExprSiteV1::new(owner, value.value_site().clone());
            if !facts.contains(&site) || !mapped.insert(site.clone()) {
                return Err(freeze("value-coverage"));
            }
            let row = outcomes
                .iter()
                .find(|row| row.site() == &site)
                .ok_or_else(|| freeze("value-facts"))?;
            match value.returned() {
                TerminalReturnedSourceV1::OwnedCall(obligation) => {
                    let qualification = obligation.qualification();
                    if self
                        .callable_result_classes
                        .object_return_qualification(&site)
                        .as_ref()
                        != Some(qualification)
                        || qualification.class().class() != Some(class)
                        || qualification.witnesses().len() != row.witnesses().len()
                        || !qualification
                            .witnesses()
                            .iter()
                            .zip(row.witnesses())
                            .all(|(a, b)| Rc::ptr_eq(a, b))
                    {
                        return Err(freeze("call-facts-identity"));
                    }
                    match self
                        .normal_return_dispositions
                        .as_ref()
                        .and_then(|rows| rows.get(&(owner, exit.clone())))
                    {
                        Some(NormalReturnDispositionV1::Verified { proof }) => {
                            if proof.acquisition().original() != obligation.as_ref()
                                || proof.alternatives().is_empty()
                            {
                                return Err(freeze("call-proof-identity"));
                            }
                        }
                        _ => ready = false,
                    }
                }
                _ => {
                    for witness in row.witnesses() {
                        ready &= self.checked_object_return_leaf_v1(key, witness)?.is_some();
                    }
                }
            }
            terminals.push(relation);
        }
        if table.keys().any(|exit| !exits.contains(exit)) {
            return Err(freeze("orphan-terminal"));
        }
        if mapped != facts {
            ready = false;
        }
        Ok(ready.then_some(RootObjectResultDescriptorV1 {
            owner,
            class,
            kind,
            terminals,
        }))
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/root-object-result-{reason}]")
}
