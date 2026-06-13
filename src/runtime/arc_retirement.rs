//! Arc retirement family gates and first-family scaffold.
//!
//! This module records the post-ARC-RETIRE-005 contract for retiring Arc from
//! one Box family. It is intentionally reportable contract data. It does not
//! rewrite `dyn NyashBox`, plugin carriers, or global VM object carriers.

use crate::runtime::VMBoxRefCarrier;

/// Scope in which a family can claim Arc retirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArcRetirementScope {
    /// Only the VM value carrier is Arc-free for this family.
    VmValueCarrier,
    /// Full Box trait object carrier has been replaced.
    BoxTraitCarrier,
    /// Plugin instance carrier has been replaced.
    PluginCarrier,
}

/// Candidate families for the first Arc-retirement slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArcRetirementFamily {
    /// VM scalar values already use direct `VMValue` variants instead of BoxRef.
    VmScalarValueBoxes,
    /// Placeholder for future builtin identity families.
    BuiltinIdentityBox,
    /// Placeholder for future plugin-backed boxes.
    PluginBox,
}

/// Gate required before a family may claim Arc retirement in a chosen scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FamilyRetirementGate {
    pub object_identity_owner_exists: bool,
    pub refcount_storage_owner_exists: bool,
    pub atomic_free_on_zero_exists: bool,
    pub dispatch_route_owner_exists: bool,
    pub clone_share_semantics_preserved: bool,
    pub weak_behavior_defined: bool,
    pub fini_owner_defined: bool,
    pub backend_unsupported_surfaces_fail_fast: bool,
}

impl FamilyRetirementGate {
    /// Gate used by the VM scalar value family.
    ///
    /// This is a VM-carrier-only retirement: scalar values are already carried
    /// directly by `VMValue::{Integer,Bool,String,Float,Void}` and therefore do
    /// not require a runtime refcount cell or weak handle.
    pub const VM_SCALAR_VALUE_CARRIER: Self = Self {
        object_identity_owner_exists: true,
        refcount_storage_owner_exists: true,
        atomic_free_on_zero_exists: true,
        dispatch_route_owner_exists: true,
        clone_share_semantics_preserved: true,
        weak_behavior_defined: true,
        fini_owner_defined: true,
        backend_unsupported_surfaces_fail_fast: true,
    };

    pub const fn is_satisfied(&self) -> bool {
        self.object_identity_owner_exists
            && self.refcount_storage_owner_exists
            && self.atomic_free_on_zero_exists
            && self.dispatch_route_owner_exists
            && self.clone_share_semantics_preserved
            && self.weak_behavior_defined
            && self.fini_owner_defined
            && self.backend_unsupported_surfaces_fail_fast
    }
}

/// Selected family for the first Arc-retirement scaffold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FamilyRetirementCandidate {
    pub family: ArcRetirementFamily,
    pub scope: ArcRetirementScope,
    pub gate: FamilyRetirementGate,
    pub reason: &'static str,
}

impl FamilyRetirementCandidate {
    pub const VM_SCALAR_VALUE_BOXES: Self = Self {
        family: ArcRetirementFamily::VmScalarValueBoxes,
        scope: ArcRetirementScope::VmValueCarrier,
        gate: FamilyRetirementGate::VM_SCALAR_VALUE_CARRIER,
        reason:
            "VM scalar values are already direct VMValue carriers and do not use VMValue::BoxRef",
    };
}

/// Refcount storage strategy for an Arc-retired family.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RefcountStorageStrategy {
    /// Runtime object header stores the refcount.
    ObjectHeader,
    /// Runtime object table stores the refcount.
    SideTable,
    /// Immediate VM scalar values need no runtime refcount storage.
    ImmediateScalarNoRefcount,
}

/// Refcount storage owner selected by ARC-RETIRE-008.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RefcountStoragePrototype {
    pub strategy: RefcountStorageStrategy,
    pub storage_owner: &'static str,
    pub applies_to_first_family: bool,
}

impl RefcountStoragePrototype {
    pub const VM_SCALAR_NO_REFCOUNT: Self = Self {
        strategy: RefcountStorageStrategy::ImmediateScalarNoRefcount,
        storage_owner: "VMValue scalar variant",
        applies_to_first_family: true,
    };

    pub const FUTURE_OBJECT_STORAGE: Self = Self {
        strategy: RefcountStorageStrategy::ObjectHeader,
        storage_owner: "ownership substrate object header or object table",
        applies_to_first_family: false,
    };
}

/// Atomic retain/release/free contract for families that do require refcounts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AtomicRetainReleaseContract {
    pub retain_symbol: &'static str,
    pub release_symbol: &'static str,
    pub free_symbol: &'static str,
    pub release_uses_fetch_add_minus_one: bool,
    pub free_on_zero_owner: &'static str,
}

impl AtomicRetainReleaseContract {
    pub const HAKO_ATOMIC_AND_MEM: Self = Self {
        retain_symbol: "hako_atomic_slot_fetch_add_i64",
        release_symbol: "hako_atomic_slot_fetch_add_i64",
        free_symbol: "hako_mem_free",
        release_uses_fetch_add_minus_one: true,
        free_on_zero_owner: "ownership substrate",
    };
}

/// First-family scaffold for ARC-RETIRE-010.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FirstFamilyArcRetirementScaffold {
    pub candidate: FamilyRetirementCandidate,
    pub refcount_storage: RefcountStoragePrototype,
    pub atomic_contract: AtomicRetainReleaseContract,
    pub current_boxref_carrier: VMBoxRefCarrier,
    pub first_family_carrier: VMBoxRefCarrier,
    pub global_box_trait_arc_replaced: bool,
}

impl FirstFamilyArcRetirementScaffold {
    pub const VM_SCALAR_VALUE_BOXES: Self = Self {
        candidate: FamilyRetirementCandidate::VM_SCALAR_VALUE_BOXES,
        refcount_storage: RefcountStoragePrototype::VM_SCALAR_NO_REFCOUNT,
        atomic_contract: AtomicRetainReleaseContract::HAKO_ATOMIC_AND_MEM,
        current_boxref_carrier: VMBoxRefCarrier::ArcDynNyashBox,
        first_family_carrier: VMBoxRefCarrier::DirectVmScalar,
        global_box_trait_arc_replaced: false,
    };
}

pub fn family_retirement_gate() -> FamilyRetirementGate {
    FamilyRetirementGate::VM_SCALAR_VALUE_CARRIER
}

pub fn first_family_candidate() -> FamilyRetirementCandidate {
    FamilyRetirementCandidate::VM_SCALAR_VALUE_BOXES
}

pub fn refcount_storage_prototype() -> RefcountStoragePrototype {
    RefcountStoragePrototype::VM_SCALAR_NO_REFCOUNT
}

pub fn atomic_retain_release_contract() -> AtomicRetainReleaseContract {
    AtomicRetainReleaseContract::HAKO_ATOMIC_AND_MEM
}

pub fn first_family_arc_retirement_scaffold() -> FirstFamilyArcRetirementScaffold {
    FirstFamilyArcRetirementScaffold::VM_SCALAR_VALUE_BOXES
}

/// Stable report fields for ARC-RETIRE-006..010.
pub fn arc_retirement_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("arc_retirement_mode", "first_family_scaffold"),
        ("arc_retirement_family_gate_defined", "1"),
        ("arc_retirement_family_gate_satisfied", "1"),
        ("object_identity_owner_exists", "1"),
        ("refcount_storage_owner_exists", "1"),
        ("atomic_free_on_zero_exists", "1"),
        ("dispatch_route_owner_exists", "1"),
        ("clone_share_semantics_preserved", "1"),
        ("weak_behavior_defined", "1"),
        ("fini_owner_defined", "1"),
        ("backend_unsupported_surfaces_fail_fast", "1"),
        ("first_arc_retirement_candidate", "vm_scalar_value_boxes"),
        ("first_arc_retirement_scope", "vmvalue_carrier"),
        ("refcount_storage_owner_defined", "1"),
        ("refcount_storage_strategy", "immediate_scalar_no_refcount"),
        ("atomic_retain_release_contract_defined", "1"),
        ("retain_symbol", "hako_atomic_slot_fetch_add_i64"),
        ("release_symbol", "hako_atomic_slot_fetch_add_i64"),
        ("release_uses_fetch_add_minus_one", "1"),
        ("free_symbol", "hako_mem_free"),
        ("first_family_arc_retirement_scaffold", "1"),
        ("first_family_vm_carrier", "direct_vm_scalar"),
        ("first_family_vm_carrier_arc_free", "1"),
        ("first_family_box_trait_arc_replaced", "0"),
        ("global_arc_replaced", "0"),
        ("typeabi_identity_truth_count", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_gate_is_satisfied_for_vm_scalar_candidate() {
        let gate = family_retirement_gate();

        assert!(gate.is_satisfied());
        assert!(gate.object_identity_owner_exists);
        assert!(gate.backend_unsupported_surfaces_fail_fast);
    }

    #[test]
    fn first_candidate_is_vm_carrier_only() {
        let candidate = first_family_candidate();

        assert_eq!(candidate.family, ArcRetirementFamily::VmScalarValueBoxes);
        assert_eq!(candidate.scope, ArcRetirementScope::VmValueCarrier);
        assert!(candidate.gate.is_satisfied());
    }

    #[test]
    fn refcount_prototype_keeps_first_family_immediate() {
        let storage = refcount_storage_prototype();

        assert_eq!(
            storage.strategy,
            RefcountStorageStrategy::ImmediateScalarNoRefcount
        );
        assert!(storage.applies_to_first_family);
    }

    #[test]
    fn atomic_contract_uses_existing_substrate_symbols() {
        let contract = atomic_retain_release_contract();

        assert_eq!(contract.retain_symbol, "hako_atomic_slot_fetch_add_i64");
        assert_eq!(contract.release_symbol, "hako_atomic_slot_fetch_add_i64");
        assert!(contract.release_uses_fetch_add_minus_one);
        assert_eq!(contract.free_symbol, "hako_mem_free");
    }

    #[test]
    fn first_family_scaffold_does_not_claim_global_arc_replacement() {
        let scaffold = first_family_arc_retirement_scaffold();

        assert_eq!(
            scaffold.candidate.family,
            ArcRetirementFamily::VmScalarValueBoxes
        );
        assert_eq!(
            scaffold.first_family_carrier,
            VMBoxRefCarrier::DirectVmScalar
        );
        assert!(!scaffold.global_box_trait_arc_replaced);
    }

    #[test]
    fn report_fields_cover_006_to_010() {
        let fields = arc_retirement_report_fields();

        assert!(fields.contains(&("arc_retirement_family_gate_defined", "1")));
        assert!(fields.contains(&("first_arc_retirement_candidate", "vm_scalar_value_boxes")));
        assert!(fields.contains(&("refcount_storage_owner_defined", "1")));
        assert!(fields.contains(&("atomic_retain_release_contract_defined", "1")));
        assert!(fields.contains(&("first_family_arc_retirement_scaffold", "1")));
        assert!(fields.contains(&("first_family_vm_carrier", "direct_vm_scalar")));
        assert!(fields.contains(&("global_arc_replaced", "0")));
    }
}
