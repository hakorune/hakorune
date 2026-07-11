---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-SHADOW-PROJECTOR-001
---

# MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-SHADOW-PROJECTOR-001

## Summary

`MirBuilder::finalize_module` SlotRegistryRelease now has an ordinary Hako
shadow projector support module under `lang/src/compiler/lib/`. The projector
keeps the converter implementation split explicit:

- `input_json` records the SlotRegistryRelease plan JSON
- `output_json` records the typed Hako shadow projection
- `python_oracle` stays the oracle reference
- `hako_shadow` stays the Hako shadow candidate input
- `parity_gate`, `promotion_token`, and `retirement_token` remain explicit

The module is compiler-library code only. It does not add a new ABI, host
facade, runtime fallback, or source selfhost claim.

## Authority

Semantic source:

```text
MirBuilderSlotRegistryReleasePlanV1
  -> SlotRegistryReleaseHakoProjector
  -> canonical JSON parity record
```

Implemented surface:

```text
lang/src/compiler/lib/slot_registry_release_projector.hako
lang/src/compiler/hako_module.toml
lang/src/compiler/lib/README.md
docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md
```

The projector is a support-library landing, not a family adoption decision.

## Acceptance

```text
bash tools/bin/hako --backend mir --verify lang/src/compiler/lib/slot_registry_release_projector.hako = green
git diff --check = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh = green
cargo check --release = green
```

## Non-Claims

```text
new_abi = 0
host_abi_compiler_semantics = 0
source_selfhost_claim = 0
runtime_fallback = 0
HakoAdopted = 0
PythonSemanticProjectorGrowth = 0
```

## Next

```text
MIRBUILDER-MODULE-METADATA-PUBLICATION-DERIVED-HAKO-ARTIFACT-001
```
