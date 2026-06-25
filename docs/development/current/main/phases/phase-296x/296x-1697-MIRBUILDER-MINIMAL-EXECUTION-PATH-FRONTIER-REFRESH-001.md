# 296x-1697 MIRBUILDER-MINIMAL-EXECUTION-PATH-FRONTIER-REFRESH-001

Status: Landed
Date: 2026-06-25
Token: MIRBUILDER-MINIMAL-EXECUTION-PATH-FRONTIER-REFRESH-001

## Purpose

Refresh the minimal MirBuilder execution path frontier so the analyzer derives
the next unsupported live edge concretely, after the allocation-policy mainline
pilot became Available. This is a frontier-derivation slice, not an
implementation slice: no Hako artifact, backend route, ABI, or runtime
behavior is added.

The generic placeholder edge (`NextMinimalExecutionPathEdgeSelection`) is
replaced by a concrete capability edge derived from live source order plus an
existing explicit contract.

## Source Authority

```text
input:
  live Rust source order (MirBuilder::build_module / prepare_module)
  + explicit artifact contracts

grounding contract:
  mirbuilder-literal-integer-lowering-plan-v0.json
    non_claims.return_emission = 0
```

`LiteralIntegerLowering` explicitly does not claim return emission, and
`BoundedFinalizeComposition` is a composition plan, not a return-emission
provider. The minimal-execution smoke is a Rust-path observation, not a
capability provider. ReturnEmission is therefore an independent capability
that has no provider in the current contract set.

## Derived Frontier Result

The analyzer walks `ordered_source_edges`; the first edge with no provider is
the derived frontier. After this refresh it derives:

```text
edge:
  finalize_module.return_emission

callsite:
  MirBuilder::finalize_module -> append Return(result_value)

reason:
  UnsupportedDirectShape

detail:
  ReturnEmissionRequired

next slice:
  MIRBUILDER-RETURN-EMISSION-001
```

## Non-Claims

```text
return_emission_implemented = 0
bounded_finalize_as_return_provider = 0
smoke_as_capability_proof = 0
next_input_profile_advance = 0
full_mirbuilder_new_claim = 0
mainline_selection_widen = 0
generated_hako_change = 0
new_backend_route = 0
new_abi = 0
runtime_fallback = 0
source_selfhost_claim = 0
bundle_size_as_proof = 0
```

ReturnEmission is identified as the next red edge only; it is not implemented
in this slice (provider stays None).

## Acceptance

```text
python3 -m py_compile \
  tools/rust_lifecycle/mirbuilder_minimal_execution_path_selection.py
bash tools/checks/rust_lifecycle_mirbuilder_minimal_execution_path_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
