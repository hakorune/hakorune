//! Prepared full-program row types for the Loop operation demand.

use super::super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::super::schema::{LoopOperationV1, LoopValueClassV1};
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLoopOperationScheduleRowV1 {
    pub(super) item: LoopItemKeyV1,
    pub(super) block: LoopBlockKeyV1,
    pub(super) owner_loop: LoopNodeKeyV1,
}

impl PreparedLoopOperationScheduleRowV1 {
    pub(crate) const fn new(
        item: LoopItemKeyV1,
        block: LoopBlockKeyV1,
        owner_loop: LoopNodeKeyV1,
    ) -> Self {
        Self {
            item,
            block,
            owner_loop,
        }
    }
}

/// Full Recipe-order operation view derived from a prepared program.
///
/// This is intentionally a complete projection: callers cannot ask for one
/// item by key and thereby bypass whole-program coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PreparedLoopOperationRowV1 {
    pub(super) schedule: PreparedLoopOperationScheduleRowV1,
    pub(super) operation: LoopOperationV1,
}

impl PreparedLoopOperationRowV1 {
    pub(crate) const fn new(
        schedule: PreparedLoopOperationScheduleRowV1,
        operation: LoopOperationV1,
    ) -> Self {
        Self {
            schedule,
            operation,
        }
    }

    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.schedule.item
    }

    pub(crate) const fn schedule(self) -> PreparedLoopOperationScheduleRowV1 {
        self.schedule
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.schedule.block
    }

    pub(crate) const fn owner_loop(self) -> LoopNodeKeyV1 {
        self.schedule.owner_loop
    }

    pub(crate) const fn operation(self) -> LoopOperationV1 {
        self.operation
    }
}

impl PreparedLoopOperationScheduleRowV1 {
    pub(crate) const fn item(self) -> LoopItemKeyV1 {
        self.item
    }

    pub(crate) const fn block(self) -> LoopBlockKeyV1 {
        self.block
    }

    pub(crate) const fn owner_loop(self) -> LoopNodeKeyV1 {
        self.owner_loop
    }
}

/// Full-program ReadBinding projection. This is derived only from a complete
/// prepared program; it is not a single-operation extraction API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedLoopReadBindingRowV1 {
    pub(super) schedule: PreparedLoopOperationScheduleRowV1,
    pub(super) binding: LoopBindingKeyV1,
    pub(super) result: LoopValueKeyV1,
    pub(super) source_binding: BindingRefV1,
    pub(super) source_site: SourceExprSiteV1,
    pub(super) class: LoopValueClassV1,
}

/// Full-program WriteBinding projection. Like the Read projection, this is
/// derived only from a complete prepared program and never selects one item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreparedLoopWriteBindingRowV1 {
    pub(super) schedule: PreparedLoopOperationScheduleRowV1,
    pub(super) binding: LoopBindingKeyV1,
    pub(super) value: LoopValueKeyV1,
    pub(super) source_binding: BindingRefV1,
    pub(super) source_site: SourceExprSiteV1,
    pub(super) class: LoopValueClassV1,
}

impl PreparedLoopWriteBindingRowV1 {
    pub(crate) fn new(
        schedule: PreparedLoopOperationScheduleRowV1,
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
        source_binding: BindingRefV1,
        source_site: SourceExprSiteV1,
        class: LoopValueClassV1,
    ) -> Self {
        Self {
            schedule,
            binding,
            value,
            source_binding,
            source_site,
            class,
        }
    }
}

impl PreparedLoopWriteBindingRowV1 {
    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.schedule.item
    }
    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.schedule.block
    }
    pub(crate) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.schedule.owner_loop
    }
    pub(crate) const fn binding(&self) -> LoopBindingKeyV1 {
        self.binding
    }
    pub(crate) const fn value(&self) -> LoopValueKeyV1 {
        self.value
    }
    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }
    pub(crate) fn source_site(&self) -> &SourceExprSiteV1 {
        &self.source_site
    }
    pub(crate) const fn class(&self) -> LoopValueClassV1 {
        self.class
    }
}

impl PreparedLoopReadBindingRowV1 {
    pub(crate) fn new(
        schedule: PreparedLoopOperationScheduleRowV1,
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
        source_binding: BindingRefV1,
        source_site: SourceExprSiteV1,
        class: LoopValueClassV1,
    ) -> Self {
        Self {
            schedule,
            binding,
            result,
            source_binding,
            source_site,
            class,
        }
    }

    pub(crate) const fn item(&self) -> LoopItemKeyV1 {
        self.schedule.item
    }
    pub(crate) const fn block(&self) -> LoopBlockKeyV1 {
        self.schedule.block
    }
    pub(crate) const fn owner_loop(&self) -> LoopNodeKeyV1 {
        self.schedule.owner_loop
    }
    pub(crate) const fn binding(&self) -> LoopBindingKeyV1 {
        self.binding
    }
    pub(crate) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }
    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }
    pub(crate) fn source_site(&self) -> &SourceExprSiteV1 {
        &self.source_site
    }
    pub(crate) const fn class(&self) -> LoopValueClassV1 {
        self.class
    }
}
