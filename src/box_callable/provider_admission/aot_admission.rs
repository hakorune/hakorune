//! Owned symbolic AOT admission, before object generation and link.

use crate::abi::text_scan_aot_export_facts::{
    TextScanAotEntryIdV1, TextScanLeaseCapabilityV1, TextScanValueLaneV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

use super::admitted_registry::{AdmittedTextScanRegistryV1, TextScanAdmittedRoleV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextScanEntryContractV1 {
    entry: TextScanAotEntryIdV1,
    arity: u32,
    receiver_lane: TextScanValueLaneV1,
    argument_lanes: &'static [TextScanValueLaneV1],
    result_lane: TextScanValueLaneV1,
    lease: TextScanLeaseCapabilityV1,
}

impl TextScanEntryContractV1 {
    pub(super) const fn from_fact(
        entry: TextScanAotEntryIdV1,
        arity: u32,
        receiver_lane: TextScanValueLaneV1,
        argument_lanes: &'static [TextScanValueLaneV1],
        result_lane: TextScanValueLaneV1,
        lease: TextScanLeaseCapabilityV1,
    ) -> Self {
        Self {
            entry,
            arity,
            receiver_lane,
            argument_lanes,
            result_lane,
            lease,
        }
    }

    pub(crate) const fn entry(self) -> TextScanAotEntryIdV1 {
        self.entry
    }

    pub(crate) const fn arity(self) -> u32 {
        self.arity
    }

    pub(crate) const fn receiver_lane(self) -> TextScanValueLaneV1 {
        self.receiver_lane
    }

    pub(crate) const fn argument_lanes(self) -> &'static [TextScanValueLaneV1] {
        self.argument_lanes
    }

    pub(crate) const fn result_lane(self) -> TextScanValueLaneV1 {
        self.result_lane
    }

    pub(crate) const fn lease(self) -> TextScanLeaseCapabilityV1 {
        self.lease
    }
}

/// Move-only pre-link product.  Final image/address resolution belongs to the
/// post-object AOT link owner, not this MIR-side admission.
#[derive(Debug)]
pub(crate) struct PreparedAotExecutableAdmissionV1 {
    contract_id: &'static str,
    profile: u32,
    abi_revision: u32,
    canonical_receiver: &'static str,
    aliases: [&'static str; 2],
    registry: AdmittedTextScanRegistryV1,
    substring: TextScanEntryContractV1,
    index_of: TextScanEntryContractV1,
    plan_stamp: ModuleInvocationBrandV1,
}

impl PreparedAotExecutableAdmissionV1 {
    pub(crate) const fn contract_id(&self) -> &'static str {
        self.contract_id
    }

    pub(crate) const fn profile(&self) -> u32 {
        self.profile
    }

    pub(crate) const fn abi_revision(&self) -> u32 {
        self.abi_revision
    }

    pub(crate) const fn canonical_receiver(&self) -> &'static str {
        self.canonical_receiver
    }

    pub(crate) const fn aliases(&self) -> [&'static str; 2] {
        self.aliases
    }

    pub(crate) const fn registry_branch_count(&self) -> usize {
        self.registry.branch_count()
    }

    pub(crate) const fn registry_generation(&self) -> u64 {
        self.registry.generation()
    }

    pub(crate) fn registry_slot_for(
        &self,
        role: TextScanAdmittedRoleV1,
    ) -> u16 {
        self.registry.row(role).slot()
    }

    pub(crate) const fn plan_stamp(&self) -> ModuleInvocationBrandV1 {
        self.plan_stamp
    }

    pub(crate) const fn entry_for(
        &self,
        role: TextScanAdmittedRoleV1,
    ) -> TextScanEntryContractV1 {
        match role {
            TextScanAdmittedRoleV1::TextSliceRange => self.substring,
            TextScanAdmittedRoleV1::TextFindNeedle => self.index_of,
        }
    }
}

pub(super) fn build(
    contract_id: &'static str,
    profile: u32,
    abi_revision: u32,
    canonical_receiver: &'static str,
    aliases: [&'static str; 2],
    registry: AdmittedTextScanRegistryV1,
    substring: TextScanEntryContractV1,
    index_of: TextScanEntryContractV1,
    plan_stamp: ModuleInvocationBrandV1,
) -> PreparedAotExecutableAdmissionV1 {
    PreparedAotExecutableAdmissionV1 {
        contract_id,
        profile,
        abi_revision,
        canonical_receiver,
        aliases,
        registry,
        substring,
        index_of,
        plan_stamp,
    }
}
