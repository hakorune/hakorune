//! Owned symbolic AOT admission, before object generation and link.

use crate::abi::text_scan_aot_export_facts::{
    TextScanAotEntryIdV1, TextScanCallAbiFactV1, TextScanLeaseCapabilityV1,
    TextScanValueLaneV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

use super::admitted_registry::{AdmittedTextScanRegistryV1, TextScanAdmittedRoleV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextScanEntryContractV1 {
    entry: TextScanAotEntryIdV1,
    symbol: &'static str,
    arity: u32,
    receiver_lane: TextScanValueLaneV1,
    argument_lanes: &'static [TextScanValueLaneV1],
    result_lane: TextScanValueLaneV1,
    lease: TextScanLeaseCapabilityV1,
    call_abi: TextScanCallAbiFactV1,
}

/// Provider-owned projection consumed by the neutral MIR site-plan issuer.
/// It carries only checked ABI facts; it does not issue MIR IDs or shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextScanCheckedCallOutFactsV1 {
    entry_code: u32,
    arity: u32,
    call_abi_revision: u32,
    wire_revision: u32,
    end_authorized_handle: bool,
    immediate_i64: bool,
}

impl TextScanCheckedCallOutFactsV1 {
    pub(crate) const fn entry_code(self) -> u32 {
        self.entry_code
    }

    pub(crate) const fn arity(self) -> u32 {
        self.arity
    }

    pub(crate) const fn call_abi_revision(self) -> u32 {
        self.call_abi_revision
    }

    pub(crate) const fn wire_revision(self) -> u32 {
        self.wire_revision
    }

    pub(crate) const fn is_end_authorized_handle(self) -> bool {
        self.end_authorized_handle
    }

    pub(crate) const fn is_immediate_i64(self) -> bool {
        self.immediate_i64
    }
}

impl TextScanEntryContractV1 {
    pub(super) const fn from_fact(
        entry: TextScanAotEntryIdV1,
        symbol: &'static str,
        arity: u32,
        receiver_lane: TextScanValueLaneV1,
        argument_lanes: &'static [TextScanValueLaneV1],
        result_lane: TextScanValueLaneV1,
        lease: TextScanLeaseCapabilityV1,
        call_abi: TextScanCallAbiFactV1,
    ) -> Self {
        Self {
            entry,
            symbol,
            arity,
            receiver_lane,
            argument_lanes,
            result_lane,
            lease,
            call_abi,
        }
    }

    pub(crate) const fn entry(self) -> TextScanAotEntryIdV1 {
        self.entry
    }

    pub(crate) const fn symbol(self) -> &'static str {
        self.symbol
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

    pub(crate) const fn call_abi(self) -> TextScanCallAbiFactV1 {
        self.call_abi
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

    pub(crate) const fn checked_callout_facts(
        &self,
        role: TextScanAdmittedRoleV1,
    ) -> TextScanCheckedCallOutFactsV1 {
        let entry = self.entry_for(role);
        TextScanCheckedCallOutFactsV1 {
            entry_code: entry.entry as u32,
            arity: entry.arity,
            call_abi_revision: entry.call_abi.abi_revision,
            wire_revision: entry.call_abi.out_wire_revision,
            end_authorized_handle: matches!(
                (entry.result_lane, entry.lease),
                (
                    TextScanValueLaneV1::HostHandle,
                    TextScanLeaseCapabilityV1::EndAuthorized
                )
            ),
            immediate_i64: matches!(
                (entry.result_lane, entry.lease),
                (
                    TextScanValueLaneV1::ImmediateI64,
                    TextScanLeaseCapabilityV1::None
                )
            ),
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
