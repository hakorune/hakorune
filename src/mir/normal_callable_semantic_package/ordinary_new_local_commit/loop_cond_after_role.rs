//! Transport of the canonical LoopCond after role into
//! the existing child physical boundary. No source or PHI fact is inferred.
use super::*;
use crate::mir::builder::VerifiedLoopCondAfterPhysicalRoleV1;
use std::collections::{BTreeMap, BTreeSet};

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn recorded_source_loop_roles_v2(&self, owner: FunctionOwnerIdV1)
        -> Vec<std::rc::Rc<VerifiedLoopCondAfterPhysicalRoleV1>> {
        self.loop_cond_after_roles.borrow().iter().filter(|((actual, _), _)| *actual == owner)
            .map(|(_, role)| std::rc::Rc::clone(role)).collect()
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
            for (block, prefix) in role.loop_phis() {
                if recorded.contains(block) && allowed.insert(*block, prefix.clone()).is_some() {
                    return Err(freeze("loop-cond-after-block-duplicate"));
                }
            }
        }
        Ok(allowed)
    }
}
