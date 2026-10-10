//! Transport of the canonical LoopCond after role into
//! the existing child physical boundary. No source or PHI fact is inferred.
use super::*;
use crate::mir::builder::VerifiedLoopCondAfterPhysicalRoleV1;
use std::collections::{BTreeMap, BTreeSet};

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn record_loop_cond_after_role(
        &self,
        role: VerifiedLoopCondAfterPhysicalRoleV1,
    ) -> Result<(), String> {
        let owner = role.owner();
        if self.completion_for_owner(owner).is_none() {
            return Err(freeze("loop-cond-after-owner-missing"));
        }
        let key = (owner, role.site().clone());
        let mut roles = self.loop_cond_after_roles.borrow_mut();
        if roles.contains_key(&key) {
            return Err(freeze("duplicate-loop-cond-after-role"));
        }
        roles.insert(key, role);
        Ok(())
    }

    pub(super) fn recorded_loop_cond_after_blocks(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
        bindings: &[(BasicBlockId, MirInstruction)],
    ) -> Result<BTreeMap<BasicBlockId, Box<[MirInstruction]>>, String> {
        let recorded: BTreeSet<_> = bindings.iter().map(|(block, _)| *block).collect();
        let mut allowed = BTreeMap::new();
        for ((role_owner, site), role) in self.loop_cond_after_roles.borrow().iter() {
            if *role_owner != owner {
                continue;
            }
            if role.owner() != owner || role.site() != site {
                return Err(freeze("loop-cond-after-role-source-drift"));
            }
            let block = role.block();
            if !function.blocks.contains_key(&block) {
                return Err(freeze("loop-cond-after-block-missing"));
            }
            if recorded.contains(&block)
                && allowed
                    .insert(block, role.phi_prefix().to_vec().into_boxed_slice())
                    .is_some()
            {
                return Err(freeze("loop-cond-after-block-duplicate"));
            }
        }
        Ok(allowed)
    }
}
