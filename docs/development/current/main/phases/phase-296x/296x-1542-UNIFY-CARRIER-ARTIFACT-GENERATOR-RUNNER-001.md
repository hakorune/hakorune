# 296x-1542 UNIFY-CARRIER-ARTIFACT-GENERATOR-RUNNER-001

Status: landed
Date: 2026-06-22

## Purpose

Consolidate the VariableContext carrier snapshot and explicit carrier
snapshot artifact runners onto the shared validated generator path used by
the other MirBuilder families.

The carrier generator still has family-specific validation and spec building,
but the execution path now flows through the shared validated generator
helper instead of a bespoke write-outputs branch.

## Scope

```text
BoxCount: one runner-path consolidation
owner: carrier snapshot / explicit carrier snapshot generators
input: validated facts + plan + oracle + generated spec
output: shared validated generator execution path
```

## Required Checks

```text
python3 tools/rust_lifecycle/generate_variable_context_carrier_snapshot_artifact.py --check
python3 tools/rust_lifecycle/generate_variable_context_explicit_carrier_snapshot_artifact.py --check
bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_artifact_guard.sh
bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
```

## Acceptance

```text
carrier snapshot and explicit carrier snapshot both use the shared validated generator path
generated .hako and artifact manifests stay byte-identical
carrier snapshot and explicit carrier EXE guards stay green
no route-selection drift
no silent hardcode introduced
```

## Stop Line

```text
do_not_reintroduce_separate_write_outputs_flow=1
do_not_change_carrier_behavior=1
do_not_add_special_case_requested_names=1
do_not_open_nightly_rustc_adapter_path=1
```
