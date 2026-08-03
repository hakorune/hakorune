//! Neutral physical-input capabilities for the first Loop physicalizer.
//!
//! These products carry identity and sealed physical topology only. They do
//! not resolve source names, allocate MIR values, create blocks, or write PHI.

use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopValueKeyV1, VerifiedLoopPhysicalInputV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, BindingId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum LoopPhysicalRoleV1 {
    Preheader,
    Header,
    Body,
    Step,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopPhysicalInputRejectV1 {
    DuplicateBindingKey(LoopBindingKeyV1),
    ForeignBindingOwner {
        expected: FunctionOwnerIdV1,
        actual: FunctionOwnerIdV1,
    },
    MissingBindingKey(LoopBindingKeyV1),
    DuplicateInputValue(LoopValueKeyV1),
    DuplicatePhysicalRole(LoopPhysicalRoleV1),
    DuplicatePhysicalBlock(BasicBlockId),
    MissingPhysicalRole(LoopPhysicalRoleV1),
}

/// Canonical-owner-issued lexical identity projection.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedLoopBindingProjectionV1 {
    owner: FunctionOwnerIdV1,
    rows: Box<[(LoopBindingKeyV1, BindingRefV1)]>,
}

impl VerifiedLoopBindingProjectionV1 {
    pub(in crate::mir::builder) fn try_new(
        owner: FunctionOwnerIdV1,
        mut rows: Vec<(LoopBindingKeyV1, BindingRefV1)>,
    ) -> Result<Self, LoopPhysicalInputRejectV1> {
        rows.sort_by_key(|(key, _)| *key);
        for pair in rows.windows(2) {
            if pair[0].0 == pair[1].0 {
                return Err(LoopPhysicalInputRejectV1::DuplicateBindingKey(pair[0].0));
            }
        }
        if let Some((_, binding)) = rows.iter().find(|(_, binding)| binding.owner() != owner) {
            return Err(LoopPhysicalInputRejectV1::ForeignBindingOwner {
                expected: owner,
                actual: binding.owner(),
            });
        }
        Ok(Self {
            owner,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(in crate::mir::builder) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) fn resolve(
        &self,
        key: LoopBindingKeyV1,
    ) -> Result<BindingRefV1, LoopPhysicalInputRejectV1> {
        self.rows
            .iter()
            .find(|(candidate, _)| *candidate == key)
            .map(|(_, binding)| *binding)
            .ok_or(LoopPhysicalInputRejectV1::MissingBindingKey(key))
    }
}

/// Maps recipe `inputs` to existing lexical bindings in the current preheader.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedLoopInputProjectionV1 {
    preheader: BasicBlockId,
    rows: Box<[(LoopValueKeyV1, LoopBindingKeyV1, crate::mir::ValueId)]>,
}

impl VerifiedLoopInputProjectionV1 {
    pub(in crate::mir::builder) fn try_new(
        preheader: BasicBlockId,
        mut rows: Vec<(LoopValueKeyV1, LoopBindingKeyV1, crate::mir::ValueId)>,
    ) -> Result<Self, LoopPhysicalInputRejectV1> {
        rows.sort_by_key(|(value, _, _)| *value);
        for pair in rows.windows(2) {
            if pair[0].0 == pair[1].0 {
                return Err(LoopPhysicalInputRejectV1::DuplicateInputValue(pair[0].0));
            }
            if pair[0].1 == pair[1].1 {
                return Err(LoopPhysicalInputRejectV1::DuplicateBindingKey(pair[0].1));
            }
        }
        Ok(Self {
            preheader,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(in crate::mir::builder) fn preheader(&self) -> BasicBlockId {
        self.preheader
    }

    pub(in crate::mir::builder) fn binding_for(
        &self,
        value: LoopValueKeyV1,
    ) -> Option<(LoopBindingKeyV1, crate::mir::ValueId)> {
        self.rows
            .iter()
            .find(|(candidate, _, _)| *candidate == value)
            .map(|(_, binding, value)| (*binding, *value))
    }
}

/// Explicit physical P/H/B/S/A role mapping. Logical backedges are not enough
/// to derive this because DirectAccum uses Body -> Step -> Header physically.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedLoopPhysicalRolePlanV1 {
    rows: Box<[(LoopPhysicalRoleV1, BasicBlockId)]>,
}

impl VerifiedLoopPhysicalRolePlanV1 {
    pub(in crate::mir::builder) fn try_new(
        rows: Vec<(LoopPhysicalRoleV1, BasicBlockId)>,
    ) -> Result<Self, LoopPhysicalInputRejectV1> {
        let mut by_role = BTreeMap::new();
        let mut by_block = BTreeMap::new();
        for (role, block) in rows {
            if by_role.insert(role, block).is_some() {
                return Err(LoopPhysicalInputRejectV1::DuplicatePhysicalRole(role));
            }
            if by_block.insert(block, role).is_some() {
                return Err(LoopPhysicalInputRejectV1::DuplicatePhysicalBlock(block));
            }
        }
        for role in [
            LoopPhysicalRoleV1::Preheader,
            LoopPhysicalRoleV1::Header,
            LoopPhysicalRoleV1::Body,
            LoopPhysicalRoleV1::Step,
            LoopPhysicalRoleV1::After,
        ] {
            if !by_role.contains_key(&role) {
                return Err(LoopPhysicalInputRejectV1::MissingPhysicalRole(role));
            }
        }
        Ok(Self {
            rows: by_role.into_iter().collect(),
        })
    }

    pub(in crate::mir::builder) fn block(&self, role: LoopPhysicalRoleV1) -> BasicBlockId {
        self.rows
            .iter()
            .find(|(candidate, _)| *candidate == role)
            .map(|(_, block)| *block)
            .expect("verified physical role")
    }

    pub(in crate::mir::builder) fn rows(&self) -> &[(LoopPhysicalRoleV1, BasicBlockId)] {
        &self.rows
    }
}

pub(in crate::mir::builder) fn direct_accum_physical_input(
    product: crate::mir::loop_recipe_contract::VerifiedDirectAccumRecipeProductV1,
) -> VerifiedLoopPhysicalInputV1 {
    VerifiedLoopPhysicalInputV1::from_direct_accum(product)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn owner() -> FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("issuer")
            .issue()
            .expect("owner")
    }

    #[test]
    fn binding_projection_is_owner_checked_and_keyed() {
        let owner = owner();
        let projection = VerifiedLoopBindingProjectionV1::try_new(
            owner,
            vec![(
                LoopBindingKeyV1::new(0),
                BindingRefV1::new(owner, BindingId::new(7)),
            )],
        )
        .expect("projection");
        assert_eq!(projection.owner(), owner);
        assert_eq!(projection.resolve(LoopBindingKeyV1::new(0)).unwrap().binding(), BindingId::new(7));
        assert!(matches!(
            projection.resolve(LoopBindingKeyV1::new(1)),
            Err(LoopPhysicalInputRejectV1::MissingBindingKey(_))
        ));
    }

    #[test]
    fn physical_role_plan_requires_explicit_standard5_path() {
        let plan = VerifiedLoopPhysicalRolePlanV1::try_new(vec![
            (LoopPhysicalRoleV1::Preheader, BasicBlockId::new(0)),
            (LoopPhysicalRoleV1::Header, BasicBlockId::new(1)),
            (LoopPhysicalRoleV1::Body, BasicBlockId::new(2)),
            (LoopPhysicalRoleV1::Step, BasicBlockId::new(3)),
            (LoopPhysicalRoleV1::After, BasicBlockId::new(4)),
        ])
        .expect("standard5");
        assert_eq!(plan.block(LoopPhysicalRoleV1::Step), BasicBlockId::new(3));
        assert_eq!(plan.rows().len(), 5);
    }

    #[test]
    fn input_projection_rejects_duplicate_value_keys() {
        let error = VerifiedLoopInputProjectionV1::try_new(
            BasicBlockId::new(0),
            vec![
                (
                    LoopValueKeyV1::new(0),
                    LoopBindingKeyV1::new(0),
                    crate::mir::ValueId::new(0),
                ),
                (
                    LoopValueKeyV1::new(0),
                    LoopBindingKeyV1::new(1),
                    crate::mir::ValueId::new(1),
                ),
            ],
        )
        .unwrap_err();
        assert!(matches!(
            error,
            LoopPhysicalInputRejectV1::DuplicateInputValue(_)
        ));
    }
}
