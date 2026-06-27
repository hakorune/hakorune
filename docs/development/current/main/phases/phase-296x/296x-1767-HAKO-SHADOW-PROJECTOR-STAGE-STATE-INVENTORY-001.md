---
Status: Landed
Date: 2026-06-28
Card: HAKO-SHADOW-PROJECTOR-STAGE-STATE-INVENTORY-001
---

# HAKO-SHADOW-PROJECTOR-STAGE-STATE-INVENTORY-001

## Summary

Add an executable inventory guard for the Hako shadow projector support
library lane so the current projector stage-state vocabulary stays explicit
and machine-checked.

The guard inventories the existing compiler-library shadow projector support
modules under `lang/src/compiler/lib/` and verifies that they record the
required stage-state fields:

- `family_id`
- `stage_id`
- `python_oracle`
- `hako_shadow`
- `parity_gate`
- `promotion_token`
- `retirement_token`

The surface remains ordinary Hako compiler-library code. The guard does not
promote the lane to a new ABI, host facade, package ABI, or language syntax.

## Authority

Semantic source:

```text
rust-to-hako-converter-implementation-role-ssot.md
  -> every Hako shadow projector must record the stage-state fields
  -> inventory guard over existing projector support modules
```

Implemented surface:

```text
tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh
lang/src/compiler/lib/return_emission_projector.hako
lang/src/compiler/lib/function_region_stack_pop_projector.hako
lang/src/compiler/lib/slot_registry_release_projector.hako
lang/src/compiler/lib/module_metadata_publication_projector.hako
lang/src/compiler/lib/record_packed_layout_refresh_projector.hako
lang/src/compiler/lib/typed_object_plan_refresh_projector.hako
lang/src/compiler/lib/direct_state_plan_refresh_projector.hako
lang/src/compiler/lib/all_functions_phi_materialization_projector.hako
lang/src/compiler/lib/README.md
```

The guard is an inventory and placement verifier, not a new semantic
projector and not an ABI surface.

## Acceptance

```text
bash tools/checks/rust_lifecycle_hako_shadow_projector_stage_state_inventory_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
git diff --check = green
```

## Non-Claims

```text
new_abi = 0
host_abi_compiler_semantics = 0
source_selfhost_claim = 0
runtime_fallback = 0
PythonSemanticProjectorGrowth = 0
TypeBoxABI = 0
language_syntax_change = 0
```

## Next

```text
MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PARITY-001
```
