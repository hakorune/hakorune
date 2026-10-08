//! Source readiness for existing cleanup construction, never artifact permission.
use super::super::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    ObjectReturnAcquisitionV1, TerminalRelationV1, TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn object_return_construction_ready_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let retained = self
            .normal_return_dispositions
            .as_ref()
            .is_some_and(|rows| rows.keys().any(|(actual, _)| *actual == owner));
        if !self.has_pending_object_return_v1(owner) && !retained {
            return Ok(true);
        }
        // An Object source cannot borrow a root fallback after losing its index.
        let Some(completion) = self.completion_index.get(&owner) else {
            return Ok(false);
        };
        let completion = completion
            .as_ref()
            .map_err(|issue| format!("{}: {issue:?}", freeze("completion")))?;
        if completion.owner() != owner {
            return Err(freeze("completion-owner"));
        }
        self.validate_object_return_disposition_exits_v1(owner, completion.explicit_sites())?;
        let Some(terminals) = self.terminal_relation_index.get(&owner) else {
            return Ok(false);
        };
        let mut ready = completion
            .cleanup()
            .root_flow()
            .is_some_and(|flow| flow.all_exits_ready())
            && !completion.explicit_sites().is_empty();
        for exit in completion.explicit_sites() {
            let Some(relation) = terminals.get(exit) else {
                ready = false;
                continue;
            };
            if relation.owner() != owner || relation.return_site() != exit {
                return Err(freeze("terminal-identity"));
            }
            // Check every sibling even if an earlier sibling was unavailable.
            let projection = self.normal_exit_projection_v1(owner, exit)?;
            ready &= projection.is_some();
            if let TerminalRelationV1::Value(value) = relation {
                if let TerminalReturnedSourceV1::OwnedCall(call) = value.returned() {
                    match call.acquisition() {
                        ObjectReturnAcquisitionV1::Direct { .. } => {
                            ready &= self
                                .verified_direct_object_return_source_v1(owner, exit)?
                                .is_some_and(|view| view.teardown().is_some());
                        }
                        ObjectReturnAcquisitionV1::Received(_) => {
                            // The same projection corroborates the retained acquisition;
                            // the existing Home planner owns installed full end plans.
                        }
                    }
                }
            }
        }
        Ok(ready)
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/object-return/construction-{reason}]")
}
