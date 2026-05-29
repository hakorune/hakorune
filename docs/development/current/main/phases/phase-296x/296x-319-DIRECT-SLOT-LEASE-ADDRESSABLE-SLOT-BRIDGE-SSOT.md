---
Status: Landed
Date: 2026-05-29
Scope: define the helper-free addressable slot bridge before retrying DirectSlotLease lowering.
Blocker: DIRECT-SLOT-LEASE-ADDRESSABLE-SLOT-BRIDGE-SSOT-296X-001
Related:
  - docs/development/current/main/design/direct-slot-lease-addressable-slot-bridge-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-318-DIRECT-SLOT-LEASE-LOWERING-PILOT-FEASIBILITY-CLOSEOUT.md
---

# 296x-319 Direct Slot Lease Addressable Slot Bridge SSOT

## Purpose

Select the addressable slot bridge shape required before DirectSlotLease lowering
can be retried.

Row318 found that the current runtime-internal lease token is not a compiler
consumable address bridge. This row accepts a dedicated stable direct-slot cell
substrate as the bridge direction and keeps implementation closed.

## Contract

```text
output_contract=direct-slot-lease-addressable-slot-bridge-ssot-v0
input_contract=direct-slot-lease-lowering-pilot-feasibility-v0
design_ssot=docs/development/current/main/design/direct-slot-lease-addressable-slot-bridge-ssot.md
selected_bridge=direct_slot_cell_storage
selected_reason=current_runtime_token_is_not_llvm_consumable_address_bridge
hako_alloc_policy_state_owner=unchanged
representation_owner=compiler_direct_lowering
storage_substrate_owner=typed_object_direct_slot_storage
helper_path=fallback_materialization_debug
stable_cell_layout_required=1
llvm_consumable_slot_address_required=1
handle_resolution_contract_required=1
generation_or_identity_validation_required=1
cell_storage_classes=i64|u64|handle
raw_runtime_vec_pointer_exposure=0
thread_local_refcell_pointer_exposure=0
rust_enum_layout_direct_load=0
c_abi_load_writeback_helper_bridge=0
by_name_hako_alloc_special_case=0
selected_plan_silent_fallback_allowed=0
existing_helper_abi_unchanged=1
default_backend_direct_slot_emission=0
lowering_open=0
native_direct_open=0
direct_load_store_open=0
implementation_open=0
optimization_open=0
selected_next=direct_slot_cell_storage_layout_selection
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

DirectSlotLease lowering remains closed until a stable direct-slot cell storage
layout exists.

The next row should select the minimum cell layout and storage owner. It must not
touch LLVM lowering yet.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_lease_addressable_slot_bridge_ssot_guard.sh
```
