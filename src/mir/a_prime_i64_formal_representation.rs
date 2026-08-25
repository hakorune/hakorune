//! Opaque storage carrier for the selected A-prime formal representation rows.
//!
//! The selected capability issues the rows and the same-session formal opener
//! attaches ValueIds.  Provider admission and wire consumers only carry this
//! product; they do not construct or inspect its physical meaning.

use crate::mir::loop_recipe_contract::LoopValueKeyV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct APrimeI64FormalPhysicalRepresentationProjectionV1 {
    owner: FunctionOwnerIdV1,
    pos: APrimeI64FormalPhysicalRepresentationRowV1,
    end: APrimeI64FormalPhysicalRepresentationRowV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct APrimeI64FormalPhysicalRepresentationRowV1 {
    ordinal: u32,
    binding: BindingRefV1,
    recipe_value: LoopValueKeyV1,
    value: ValueId,
}

impl APrimeI64FormalPhysicalRepresentationProjectionV1 {
    pub(in crate::mir) fn from_adopted_exact_i64(
        owner: FunctionOwnerIdV1,
        pos: (u32, BindingRefV1, LoopValueKeyV1, ValueId),
        end: (u32, BindingRefV1, LoopValueKeyV1, ValueId),
    ) -> Self {
        Self {
            owner,
            pos: APrimeI64FormalPhysicalRepresentationRowV1 {
                ordinal: pos.0,
                binding: pos.1,
                recipe_value: pos.2,
                value: pos.3,
            },
            end: APrimeI64FormalPhysicalRepresentationRowV1 {
                ordinal: end.0,
                binding: end.1,
                recipe_value: end.2,
                value: end.3,
            },
        }
    }

    pub(in crate::mir) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    pub(crate) fn for_test() -> Self {
        let mut issuer =
            crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
                .expect("formal representation test owner issuer");
        let owner = issuer.issue().expect("formal representation test owner");
        Self::from_adopted_exact_i64(
            owner,
            (
                1,
                BindingRefV1::new(owner, hakorune_mir_core::BindingId::new(1)),
                LoopValueKeyV1::new(1),
                ValueId::new(1),
            ),
            (
                2,
                BindingRefV1::new(owner, hakorune_mir_core::BindingId::new(2)),
                LoopValueKeyV1::new(2),
                ValueId::new(2),
            ),
        )
    }
}
