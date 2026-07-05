---
Status: Landed
Date: 2026-07-05
Scope: AOT blocker inventory for ProgramJSON v0 scanner execution.
---

# MIRBUILDER-PROGRAMJSON-V0-SCANNER-AOT-LOWERING-BLOCKER-INVENTORY-001

## Result

Added a focused inventory guard for the existing ProgramJSON v0 scanner:

```text
guard=tools/checks/rust_lifecycle_mirbuilder_programjson_v0_scanner_aot_blocker_inventory_guard.sh
owner=ProgramJsonV0ScannerBox
```

The guard proves that scanner helpers verify as MIR but are not currently
executable through the AOT/pure route.

## Evidence

```text
probe=read_char                  blocker_symbol=ProgramJsonV0ScannerBox._read_char/2
probe=seek_after                 blocker_symbol=StringHelpers.to_i64/1
probe=seek_obj_end               blocker_symbol=ProgramJsonV0ScannerBox.seek_obj_end_unescaped/2
probe=seek_obj_field_value_start blocker_symbol=ProgramJsonV0ScannerBox.seek_obj_field_value_start/3
probe=seek_obj_field_obj_start   blocker_symbol=ProgramJsonV0ScannerBox.seek_obj_field_obj_start/3
reason=module_generic_prepass_failed
```

All probes:

```text
mir_verify=green
aot_emit=blocked
```

## Interpretation

The ProgramJSON snapshot owner from 2976 is not blocked by its own shape
contract first. It is blocked before runtime parity by the execution substrate
for `ProgramJsonV0ScannerBox`.

This means the next parity card cannot honestly claim runtime equality until
one of these is selected:

```text
option=AOT scanner lowering slice
option=non-AOT runner/parity route with explicit scope
```

## Non-Claims

```text
programjson_snapshot_parity_claim=0
source_selfhost_claim=0
hako_adopted_decision=0
rust_astnode_projector_retired=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
new_backend_route=0
```

## Next

This is a design boundary, not a small implementation continuation. Choose one
route before the parity card:

```text
MIRBUILDER-PROGRAMJSON-SNAPSHOT-PARITY-RUNNER-ROUTE-DECISION-001
```
