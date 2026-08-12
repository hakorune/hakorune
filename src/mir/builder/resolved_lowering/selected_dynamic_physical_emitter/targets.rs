//! Session-private physical targets for the selected Dynamic canary.
//!
//! The canonical SSA session owns block allocation. This value groups the
//! function-entry `Enter` block plus the five role-to-block projections needed
//! by the bounded physical layout; it does not own edges, operation order,
//! Completion, or PHI meaning.

use std::sync::Arc;

use super::DynamicV2PhysicalSessionBrandV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::{
    DynamicV2PhysicalBlockTargetV1, DynamicV2PhysicalScheduleRowV1,
    DynamicV2PhysicalScheduleSegmentV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::BasicBlockId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicV2PhysicalTargetRoleV1 {
    Header,
    BodyPrelude,
    ThenTerminal,
    Continuation,
    After,
}

impl DynamicV2PhysicalTargetRoleV1 {
    fn from_target(target: DynamicV2PhysicalBlockTargetV1) -> Self {
        match target {
            DynamicV2PhysicalBlockTargetV1::Header => Self::Header,
            DynamicV2PhysicalBlockTargetV1::BodyPrelude => Self::BodyPrelude,
            DynamicV2PhysicalBlockTargetV1::ThenTerminal => Self::ThenTerminal,
            DynamicV2PhysicalBlockTargetV1::Continuation => Self::Continuation,
            DynamicV2PhysicalBlockTargetV1::After => Self::After,
        }
    }
}

#[derive(Debug)]
pub(super) struct DynamicV2PhysicalTargetSetV1 {
    brand: Arc<()>,
    enter: BasicBlockId,
    header: BasicBlockId,
    body_prelude: BasicBlockId,
    then_terminal: BasicBlockId,
    continuation: BasicBlockId,
    after: BasicBlockId,
}

#[derive(Debug)]
pub(super) struct DynamicV2OpaquePhysicalTargetV1 {
    brand: Arc<()>,
    block: BasicBlockId,
}

impl DynamicV2OpaquePhysicalTargetV1 {
    pub(super) fn matches(&self, brand: &DynamicV2PhysicalSessionBrandV1) -> bool {
        Arc::ptr_eq(&self.brand, &brand.0)
    }

    pub(super) const fn block(&self) -> BasicBlockId {
        self.block
    }
}

impl DynamicV2PhysicalTargetSetV1 {
    fn validate_role_coverage(
        schedule: &[DynamicV2PhysicalScheduleRowV1],
        outer_tail_target: DynamicV2PhysicalBlockTargetV1,
    ) -> Result<(), String> {
        let mut seen = [false; 5];
        for row in schedule {
            let index = match DynamicV2PhysicalTargetRoleV1::from_target(row.target()) {
                DynamicV2PhysicalTargetRoleV1::Header => 0,
                DynamicV2PhysicalTargetRoleV1::BodyPrelude => 1,
                DynamicV2PhysicalTargetRoleV1::ThenTerminal => 2,
                DynamicV2PhysicalTargetRoleV1::Continuation => 3,
                DynamicV2PhysicalTargetRoleV1::After => 4,
            };
            if seen[index] {
                return Err("selected physical target role is duplicated".to_owned());
            }
            seen[index] = true;
        }
        if !seen[..4].iter().all(|present| *present)
            || outer_tail_target != DynamicV2PhysicalBlockTargetV1::After
        {
            return Err("selected physical target roles are incomplete".to_owned());
        }
        Ok(())
    }

    pub(super) fn issue(
        canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
        builder: &mut MirBuilder,
        brand: &DynamicV2PhysicalSessionBrandV1,
        schedule: &[DynamicV2PhysicalScheduleRowV1],
        outer_tail_target: DynamicV2PhysicalBlockTargetV1,
    ) -> Result<Self, String> {
        Self::validate_role_coverage(schedule, outer_tail_target)?;

        // The function entry is the logical loop Enter. The loop Header is a
        // distinct unpublished block; its PHI/edge meaning belongs to the
        // canonical session and is not issued by this BoxShape.
        let enter = canonical.entry_block(builder)?;
        let header = canonical.create_unpublished_block(builder)?;
        let body_prelude = canonical.create_unpublished_block(builder)?;
        let then_terminal = canonical.create_unpublished_block(builder)?;
        let continuation = canonical.create_unpublished_block(builder)?;
        let after = canonical.create_unpublished_block(builder)?;
        let blocks = [
            enter,
            header,
            body_prelude,
            then_terminal,
            continuation,
            after,
        ];
        for (index, block) in blocks.iter().enumerate() {
            if blocks[..index].contains(block) {
                return Err("selected physical target blocks are not distinct".to_owned());
            }
        }
        Ok(Self {
            brand: Arc::clone(&brand.0),
            enter,
            header,
            body_prelude,
            then_terminal,
            continuation,
            after,
        })
    }

    pub(super) fn with_role<R>(
        &self,
        role: DynamicV2PhysicalTargetRoleV1,
        callback: impl FnOnce(DynamicV2OpaquePhysicalTargetV1) -> R,
    ) -> R {
        let block = match role {
            DynamicV2PhysicalTargetRoleV1::Header => self.header,
            DynamicV2PhysicalTargetRoleV1::BodyPrelude => self.body_prelude,
            DynamicV2PhysicalTargetRoleV1::ThenTerminal => self.then_terminal,
            DynamicV2PhysicalTargetRoleV1::Continuation => self.continuation,
            DynamicV2PhysicalTargetRoleV1::After => self.after,
        };
        callback(DynamicV2OpaquePhysicalTargetV1 {
            brand: Arc::clone(&self.brand),
            block,
        })
    }

    pub(super) fn with_enter_header<R>(
        &self,
        callback: impl FnOnce(DynamicV2OpaquePhysicalTargetV1, DynamicV2OpaquePhysicalTargetV1) -> R,
    ) -> R {
        callback(
            DynamicV2OpaquePhysicalTargetV1 {
                brand: Arc::clone(&self.brand),
                block: self.enter,
            },
            DynamicV2OpaquePhysicalTargetV1 {
                brand: Arc::clone(&self.brand),
                block: self.header,
            },
        )
    }

    #[cfg(test)]
    pub(super) fn blocks_for_test(&self) -> [BasicBlockId; 6] {
        [
            self.enter,
            self.header,
            self.body_prelude,
            self.then_terminal,
            self.continuation,
            self.after,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(item: u32, target: DynamicV2PhysicalBlockTargetV1) -> DynamicV2PhysicalScheduleRowV1 {
        DynamicV2PhysicalScheduleRowV1::from_test_parts(
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(item),
            DynamicV2PhysicalScheduleSegmentV1::Prelude,
            target,
        )
    }

    fn complete_roles() -> Vec<DynamicV2PhysicalScheduleRowV1> {
        vec![
            row(0, DynamicV2PhysicalBlockTargetV1::Header),
            row(1, DynamicV2PhysicalBlockTargetV1::BodyPrelude),
            row(2, DynamicV2PhysicalBlockTargetV1::ThenTerminal),
            row(3, DynamicV2PhysicalBlockTargetV1::Continuation),
        ]
    }

    #[test]
    fn duplicate_role_is_rejected_before_block_allocation() {
        let mut rows = complete_roles();
        rows.push(row(4, DynamicV2PhysicalBlockTargetV1::Header));
        assert_eq!(
            DynamicV2PhysicalTargetSetV1::validate_role_coverage(
                &rows,
                DynamicV2PhysicalBlockTargetV1::After,
            ),
            Err("selected physical target role is duplicated".to_owned())
        );
    }

    #[test]
    fn missing_role_is_rejected_before_block_allocation() {
        let mut rows = complete_roles();
        rows.pop();
        assert_eq!(
            DynamicV2PhysicalTargetSetV1::validate_role_coverage(
                &rows,
                DynamicV2PhysicalBlockTargetV1::After,
            ),
            Err("selected physical target roles are incomplete".to_owned())
        );
    }
}
