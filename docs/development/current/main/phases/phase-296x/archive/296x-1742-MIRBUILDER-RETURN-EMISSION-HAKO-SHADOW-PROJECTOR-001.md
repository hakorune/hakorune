---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001
---

# MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PROJECTOR-001

## Summary

`MirBuilder::finalize_module` ReturnEmission now has an ordinary Hako shadow
projector support module under `lang/src/compiler/lib/`. The projector keeps
the converter implementation split explicit:

- `input_json` records the ReturnEmission plan JSON
- `output_json` records the typed Hako shadow projection
- `python_oracle` stays the oracle reference
- `hako_shadow` stays the Hako shadow candidate input
- `parity_gate`, `promotion_token`, and `retirement_token` remain explicit

The module is compiler-library code only. It does not add a new ABI, host
facade, runtime fallback, or source selfhost claim.

## Authority

Semantic source:

```text
MirBuilderReturnEmissionPlanV1
  -> ReturnEmissionHakoProjector
  -> canonical JSON parity record
```

Implemented surface:

```text
lang/src/compiler/lib/return_emission_projector.hako
lang/src/compiler/hako_module.toml
lang/src/compiler/lib/README.md
```

The projector is a support-library landing, not a family adoption decision.

## Acceptance

```text
bash tools/bin/hako --backend mir --verify lang/src/compiler/lib/return_emission_projector.hako = green
git diff --check = green
bash tools/checks/rust_lifecycle_no_silent_hardcode_guard.sh = green
bash tools/checks/current_state_pointer_guard.sh = green
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
MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001
```
