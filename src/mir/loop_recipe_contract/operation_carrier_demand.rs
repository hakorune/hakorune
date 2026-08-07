//! Full-program projection for a source-bound derived carrier entry.
//!
//! A derived carrier is anchored at a statement site, not an expression site.
//! Keeping this row separate prevents the operation demand from fabricating a
//! source expression merely to reuse the ordinary read projection.

use super::ids::{LoopCarrierKeyV1, LoopItemKeyV1, LoopNodeKeyV1};
use super::operation_physical_demand::PreparedLoopOperationScheduleRowV1;
use super::schema::LoopValueClassV1;
use crate::mir::resolved_semantics::{BindingRefV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedLoopDerivedCarrierSeedRowV1 {
    pub(super) schedule: PreparedLoopOperationScheduleRowV1,
    pub(super) binding: super::ids::LoopBindingKeyV1,
    pub(super) result: super::ids::LoopValueKeyV1,
    pub(super) source_binding: BindingRefV1,
    pub(super) source_loop: SourceStmtSiteV1,
    pub(super) carrier: LoopCarrierKeyV1,
    pub(super) class: LoopValueClassV1,
}

impl PreparedLoopDerivedCarrierSeedRowV1 {
    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.schedule.item()
    }

    pub(crate) const fn block(&self) -> super::ids::LoopBlockKeyV1 {
        self.schedule.block()
    }

    pub(crate) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.schedule.owner_loop()
    }

    pub(crate) const fn binding(&self) -> super::ids::LoopBindingKeyV1 {
        self.binding
    }

    pub(crate) const fn result(&self) -> super::ids::LoopValueKeyV1 {
        self.result
    }

    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }

    pub(crate) fn source_loop(&self) -> &SourceStmtSiteV1 {
        &self.source_loop
    }

    pub(crate) const fn carrier(&self) -> LoopCarrierKeyV1 {
        self.carrier
    }

    pub(crate) const fn class(&self) -> LoopValueClassV1 {
        self.class
    }
}
