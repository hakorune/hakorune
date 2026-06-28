---
Status: Landed
Date: 2026-06-28
Card: MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PARITY-001
---

# MIRBUILDER-RETURN-EMISSION-HAKO-SHADOW-PARITY-001

## Summary

Keep `ReturnEmissionHakoProjector` as ordinary compiler-library code under
`lang/src/compiler/lib/`, and pin its canonical JSON parity surface with an
explicit shadow-result fixture plus guard.

The projector remains a support-library landing:

- `input_json` records the ReturnEmission plan JSON
- `output_json` records the typed Hako shadow candidate
- `python_oracle` stays the oracle reference
- `hako_shadow` stays the Hako shadow candidate input
- `parity_gate`, `promotion_token`, and `retirement_token` remain explicit

This card does not promote the projector lane to a new ABI, host facade,
runtime fallback, or source selfhost claim.

## Authority

Semantic source:

```text
MirBuilderReturnEmissionPlanV1
  -> ReturnEmissionHakoProjector
  -> canonical JSON parity record
```

Implemented surface:

```text
tools/rust_lifecycle/mirbuilder_return_emission_artifacts.py
lang/src/compiler/lib/return_emission_projector.hako
lang/src/compiler/lib/projector_support.hako
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-return-emission-hako-shadow-result-v0.json
tools/checks/rust_lifecycle_mirbuilder_return_emission_hako_shadow_parity_guard.sh
lang/src/compiler/lib/README.md
```

The guard is a support-library parity check, not a family adoption decision.

## Acceptance

```text
python3 tools/rust_lifecycle/mirbuilder_return_emission_artifacts.py --check = green
bash tools/checks/rust_lifecycle_mirbuilder_return_emission_hako_shadow_parity_guard.sh = green
bash tools/checks/rust_lifecycle_mirbuilder_return_emission_guard.sh = green
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
HakoAdopted = 0
PythonSemanticProjectorGrowth = 0
TypeBoxABI = 0
language_syntax_change = 0
```

## Next

```text
MIRBUILDER-FUNCTION-REGION-STACK-POP-DERIVED-HAKO-ARTIFACT-001
```
