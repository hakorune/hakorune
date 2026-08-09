//! Private semantic-role to Recipe-key claims for the later atomic co-seal.

use std::collections::BTreeSet;

use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};

use super::super::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DynamicFullLoopClaimTargetV2 {
    Loop(LoopNodeKeyV1),
    Binding(LoopBindingKeyV1),
    Value(LoopValueKeyV1),
    Item(LoopItemKeyV1),
    Exit(LoopExitKeyV1),
    PreludeInduction {
        binding: LoopBindingKeyV1,
        carrier: LoopCarrierKeyV1,
        entry: LoopValueKeyV1,
    },
    IterationLocal {
        value: LoopValueKeyV1,
    },
    CallableTail {
        binding: LoopBindingKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicFullLoopSourceClaimV2 {
    pub(super) role: DynamicFullBodySourceRoleV1,
    pub(super) target: DynamicFullLoopClaimTargetV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicFullLoopBindingClaimV2 {
    pub(super) role: DynamicFullBodyBindingRoleV1,
    pub(super) target: DynamicFullLoopClaimTargetV2,
}

#[derive(Debug)]
pub(super) struct DynamicFullLoopRecipeClaimsV2 {
    bindings: Box<[DynamicFullLoopBindingClaimV2]>,
    sources: Box<[DynamicFullLoopSourceClaimV2]>,
}

impl DynamicFullLoopRecipeClaimsV2 {
    pub(super) fn exact() -> Self {
        use DynamicFullBodyBindingRoleV1 as B;
        use DynamicFullBodySourceRoleV1 as S;
        use DynamicFullLoopClaimTargetV2 as T;

        let bindings = vec![
            binding(B::Src, T::Value(value(0))),
            binding(B::Pos, T::Value(value(1))),
            binding(B::End, T::Value(value(2))),
            binding(B::PredChars, T::Value(value(3))),
            binding(B::Induction, T::Binding(LoopBindingKeyV1::new(0))),
            binding(B::IterationLocalCh, T::IterationLocal { value: value(10) }),
        ];
        let sources = vec![
            source(
                S::PreludeLocalI,
                T::PreludeInduction {
                    binding: LoopBindingKeyV1::new(0),
                    carrier: LoopCarrierKeyV1::new(0),
                    entry: value(1),
                },
            ),
            source(S::PreludeInitializerPos, T::Value(value(1))),
            source(S::Loop, T::Loop(LoopNodeKeyV1::new(0))),
            source(S::LoopCondition, T::Item(item(1))),
            source(S::LoopConditionI, T::Item(item(0))),
            source(S::LoopConditionEnd, T::Value(value(2))),
            source(S::ChLocal, T::IterationLocal { value: value(10) }),
            source(S::SubstringCall, T::Item(item(6))),
            source(S::SubstringReceiverSrc, T::Value(value(0))),
            source(S::SubstringStartI, T::Item(item(2))),
            source(S::SubstringEndAdd, T::Item(item(5))),
            source(S::SubstringEndI, T::Item(item(3))),
            source(S::SubstringEndDelta, T::Item(item(4))),
            source(S::InnerIf, T::Item(item(10))),
            source(S::InnerIfCondition, T::Item(item(9))),
            source(S::IndexOfCall, T::Item(item(7))),
            source(S::IndexOfReceiverPredChars, T::Value(value(3))),
            source(S::IndexOfArgumentCh, T::Value(value(10))),
            source(S::InnerIfZero, T::Item(item(8))),
            source(S::InnerReturn, T::Exit(LoopExitKeyV1::new(0))),
            source(S::InnerReturnI, T::Item(item(11))),
            source(S::StepAssignment, T::Item(item(16))),
            source(S::StepTargetI, T::Item(item(16))),
            source(S::StepAdd, T::Item(item(15))),
            source(S::StepReadI, T::Item(item(13))),
            source(S::StepDelta, T::Item(item(14))),
            source(
                S::OuterReturn,
                T::CallableTail {
                    binding: LoopBindingKeyV1::new(0),
                },
            ),
            source(
                S::OuterReturnI,
                T::CallableTail {
                    binding: LoopBindingKeyV1::new(0),
                },
            ),
        ];
        debug_assert_eq!(bindings.len(), 6);
        debug_assert_eq!(sources.len(), 28);
        debug_assert_eq!(
            bindings
                .iter()
                .map(|row| row.role)
                .collect::<BTreeSet<_>>()
                .len(),
            bindings.len()
        );
        debug_assert_eq!(
            sources
                .iter()
                .map(|row| row.role)
                .collect::<BTreeSet<_>>()
                .len(),
            sources.len()
        );
        Self {
            bindings: bindings.into_boxed_slice(),
            sources: sources.into_boxed_slice(),
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Box<[DynamicFullLoopBindingClaimV2]>,
        Box<[DynamicFullLoopSourceClaimV2]>,
    ) {
        (self.bindings, self.sources)
    }

    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        bindings: Box<[DynamicFullLoopBindingClaimV2]>,
        sources: Box<[DynamicFullLoopSourceClaimV2]>,
    ) -> Self {
        Self { bindings, sources }
    }

    #[cfg(test)]
    pub(super) fn binding_rows(&self) -> &[DynamicFullLoopBindingClaimV2] {
        &self.bindings
    }

    #[cfg(test)]
    pub(super) fn source_rows(&self) -> &[DynamicFullLoopSourceClaimV2] {
        &self.sources
    }
}

const fn binding(
    role: DynamicFullBodyBindingRoleV1,
    target: DynamicFullLoopClaimTargetV2,
) -> DynamicFullLoopBindingClaimV2 {
    DynamicFullLoopBindingClaimV2 { role, target }
}

const fn source(
    role: DynamicFullBodySourceRoleV1,
    target: DynamicFullLoopClaimTargetV2,
) -> DynamicFullLoopSourceClaimV2 {
    DynamicFullLoopSourceClaimV2 { role, target }
}

const fn item(raw: u32) -> LoopItemKeyV1 {
    LoopItemKeyV1::new(raw)
}

const fn value(raw: u32) -> LoopValueKeyV1 {
    LoopValueKeyV1::new(raw)
}
