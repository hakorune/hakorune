//! Pre-effect prepared ingress for one source-backed Dynamic callable Loop.
//!
//! The issuer consumes already verified source coverage and operation
//! relations while borrowing the current Dynamic-origin state. It has no
//! Builder/CFG handle and therefore cannot allocate a block, value, or
//! instruction while rejecting an incomplete or stale relation.

use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::ValueId;

use super::normal_callable_dynamic_operation_source::VerifiedDynamicLoopOperationSourceSetV1;
use super::normal_callable_dynamic_origin::CallableDynamicOriginLoweringStateV1;
use super::normal_callable_loop_handoff::{
    CallableLoopBindingClassV1, CallableSemanticLoopHandoffPreEffectReceiptV1,
    VerifiedCallableSemanticLoopBindingScheduleV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExactLoopCarrierClassV1 {
    I64,
}

/// Closed semantic representation family accepted by the later Loop skeleton.
///
/// `Exact` requires a future exact-class issuer. This P0 row issues only the
/// source-backed Dynamic arm; raw physical Unknown is not a variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoopCarrierRepresentationKindV1 {
    Exact(ExactLoopCarrierClassV1),
    SourceBackedDynamic { origin: BindingRefV1 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PreparedLoopCarrierRepresentationV1 {
    kind: LoopCarrierRepresentationKindV1,
}

impl PreparedLoopCarrierRepresentationV1 {
    const fn source_backed_dynamic(origin: BindingRefV1) -> Self {
        Self {
            kind: LoopCarrierRepresentationKindV1::SourceBackedDynamic { origin },
        }
    }

    pub(super) const fn dynamic_origin(&self) -> Option<BindingRefV1> {
        match self.kind {
            LoopCarrierRepresentationKindV1::SourceBackedDynamic { origin } => Some(origin),
            LoopCarrierRepresentationKindV1::Exact(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreparedLoopIncomingRoleV1 {
    Enter,
    Backedge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PreparedDynamicLoopEntryBindingV1 {
    binding: BindingRefV1,
    class: CallableLoopBindingClassV1,
    current: ValueId,
    representation: PreparedLoopCarrierRepresentationV1,
}

impl PreparedDynamicLoopEntryBindingV1 {
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn class(self) -> CallableLoopBindingClassV1 {
        self.class
    }

    pub(super) const fn current(self) -> ValueId {
        self.current
    }

    pub(super) const fn representation(&self) -> &PreparedLoopCarrierRepresentationV1 {
        &self.representation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PreparedDynamicLoopCarrierV1 {
    binding: BindingRefV1,
    entry: ValueId,
    representation: PreparedLoopCarrierRepresentationV1,
    expected_roles: [PreparedLoopIncomingRoleV1; 2],
}

impl PreparedDynamicLoopCarrierV1 {
    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn entry(self) -> ValueId {
        self.entry
    }

    pub(super) const fn representation(&self) -> &PreparedLoopCarrierRepresentationV1 {
        &self.representation
    }

    pub(super) const fn expected_roles(&self) -> [PreparedLoopIncomingRoleV1; 2] {
        self.expected_roles
    }
}

/// Move-only, Builder-free input accepted by the future Dynamic Loop skeleton.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct PreparedSourceBackedDynamicLoopIngressV1 {
    owner: FunctionOwnerIdV1,
    loop_site: SourceNodeSiteV1,
    source_coverage: CallableSemanticLoopHandoffPreEffectReceiptV1,
    operations: VerifiedDynamicLoopOperationSourceSetV1,
    entry_bindings: Box<[PreparedDynamicLoopEntryBindingV1]>,
    carrier: PreparedDynamicLoopCarrierV1,
}

impl PreparedSourceBackedDynamicLoopIngressV1 {
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn loop_site(&self) -> &SourceNodeSiteV1 {
        &self.loop_site
    }

    pub(super) const fn source_coverage(&self) -> &CallableSemanticLoopHandoffPreEffectReceiptV1 {
        &self.source_coverage
    }

    pub(super) const fn operations(&self) -> &VerifiedDynamicLoopOperationSourceSetV1 {
        &self.operations
    }

    pub(super) fn entry_bindings(&self) -> &[PreparedDynamicLoopEntryBindingV1] {
        &self.entry_bindings
    }

    pub(super) const fn carrier(&self) -> &PreparedDynamicLoopCarrierV1 {
        &self.carrier
    }

    pub(super) fn entry_binding(
        &self,
        binding: BindingRefV1,
    ) -> Option<&PreparedDynamicLoopEntryBindingV1> {
        self.entry_bindings
            .iter()
            .find(|row| row.binding() == binding)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DynamicLoopPrepareIssueV1 {
    OwnerMismatch,
    LoopSourceMismatch,
    CarrierCardinality,
    CarrierOperationMismatch,
    MissingCurrentDynamicOrigin(BindingRefV1),
    ForeignDynamicOrigin(BindingRefV1),
    DuplicateCurrentValue(ValueId),
    SourceCoverage(String),
}

pub(super) struct DynamicLoopPrepareIssuerV1;

impl DynamicLoopPrepareIssuerV1 {
    pub(super) fn issue(
        schedule: VerifiedCallableSemanticLoopBindingScheduleV1,
        operations: VerifiedDynamicLoopOperationSourceSetV1,
        origins: &CallableDynamicOriginLoweringStateV1,
        parent_site: &SourceNodeSiteV1,
        condition_site: &SourceNodeSiteV1,
        body_site: &SourceNodeSiteV1,
    ) -> Result<PreparedSourceBackedDynamicLoopIngressV1, DynamicLoopPrepareIssueV1> {
        if operations.owner() != origins.owner() {
            return Err(DynamicLoopPrepareIssueV1::OwnerMismatch);
        }
        if operations.loop_site() != schedule.loop_site() || operations.loop_site() != parent_site {
            return Err(DynamicLoopPrepareIssueV1::LoopSourceMismatch);
        }
        let carrier_binding = operations.add_rebind().carrier();
        if operations.comparison().carrier() != carrier_binding {
            return Err(DynamicLoopPrepareIssueV1::CarrierOperationMismatch);
        }
        let carrier_rows = schedule
            .rows()
            .iter()
            .filter(|row| row.class() == CallableLoopBindingClassV1::Carrier)
            .collect::<Vec<_>>();
        let [carrier_row] = carrier_rows.as_slice() else {
            return Err(DynamicLoopPrepareIssueV1::CarrierCardinality);
        };
        if carrier_row.binding() != carrier_binding {
            return Err(DynamicLoopPrepareIssueV1::CarrierOperationMismatch);
        }

        let mut current_values = BTreeSet::new();
        let mut entry_bindings = Vec::new();
        for row in schedule.rows() {
            if row.class() == CallableLoopBindingClassV1::IterationLocal {
                continue;
            }
            let (current, origin) = origins.current_binding(row.binding()).ok_or(
                DynamicLoopPrepareIssueV1::MissingCurrentDynamicOrigin(row.binding()),
            )?;
            if origin.owner() != origins.owner() || row.binding().owner() != origins.owner() {
                return Err(DynamicLoopPrepareIssueV1::ForeignDynamicOrigin(
                    row.binding(),
                ));
            }
            if !current_values.insert(current) {
                return Err(DynamicLoopPrepareIssueV1::DuplicateCurrentValue(current));
            }
            entry_bindings.push(PreparedDynamicLoopEntryBindingV1 {
                binding: row.binding(),
                class: row.class(),
                current,
                representation: PreparedLoopCarrierRepresentationV1::source_backed_dynamic(origin),
            });
        }
        let carrier_entry = entry_bindings
            .iter()
            .copied()
            .find(|row| row.binding() == carrier_binding)
            .ok_or(DynamicLoopPrepareIssueV1::CarrierCardinality)?;
        let carrier = PreparedDynamicLoopCarrierV1 {
            binding: carrier_binding,
            entry: carrier_entry.current(),
            representation: *carrier_entry.representation(),
            expected_roles: [
                PreparedLoopIncomingRoleV1::Enter,
                PreparedLoopIncomingRoleV1::Backedge,
            ],
        };
        let source_coverage = schedule
            .consume_pre_effect(parent_site, condition_site, body_site)
            .map_err(DynamicLoopPrepareIssueV1::SourceCoverage)?;
        Ok(PreparedSourceBackedDynamicLoopIngressV1 {
            owner: origins.owner(),
            loop_site: parent_site.clone(),
            source_coverage,
            operations,
            entry_bindings: entry_bindings.into_boxed_slice(),
            carrier,
        })
    }
}

#[cfg(test)]
#[path = "normal_callable_dynamic_loop_prepare_tests.rs"]
mod tests;
