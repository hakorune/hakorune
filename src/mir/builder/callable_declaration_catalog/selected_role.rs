//! Sealed role of one selected callable source row.
//!
//! The role is a source-backed disposition, not a Dynamic eligibility guess.
//! In particular, an App Main static child is ordinary-lowerable here but is
//! never admitted to the Dynamic candidate route by this role alone.

use crate::ast::BoxMethodInventoryOrdinalV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectedCallableConsumptionKindV1 {
    Ordinary,
    AppMainStaticChild {
        statement: u32,
        method: BoxMethodInventoryOrdinalV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedCallableConsumptionRoleV1 {
    kind: SelectedCallableConsumptionKindV1,
}

impl SelectedCallableConsumptionRoleV1 {
    pub(super) const fn ordinary() -> Self {
        Self {
            kind: SelectedCallableConsumptionKindV1::Ordinary,
        }
    }

    pub(super) const fn app_main_static_child(
        statement: u32,
        method: BoxMethodInventoryOrdinalV1,
    ) -> Self {
        Self {
            kind: SelectedCallableConsumptionKindV1::AppMainStaticChild { statement, method },
        }
    }

    pub(crate) const fn admits_dynamic(self) -> bool {
        matches!(self.kind, SelectedCallableConsumptionKindV1::Ordinary)
    }

    pub(crate) const fn is_main_static_child(self) -> bool {
        matches!(
            self.kind,
            SelectedCallableConsumptionKindV1::AppMainStaticChild { .. }
        )
    }

    pub(crate) const fn main_static_child_slot(self) -> Option<(u32, BoxMethodInventoryOrdinalV1)> {
        match self.kind {
            SelectedCallableConsumptionKindV1::AppMainStaticChild { statement, method } => {
                Some((statement, method))
            }
            SelectedCallableConsumptionKindV1::Ordinary => None,
        }
    }
}
