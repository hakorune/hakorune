# 296x-1543 GENERATED-TO-NATIVE-ADOPTION-MATRIX-001

Status: landed
Date: 2026-06-22

## Purpose

Summarize the generated route selection and native semantic-authority
adoption status for the current MirBuilder slices in one small report.

The matrix covers:

```text
BindingContextNative
VariableContextNative simple-map
VariableContextNative snapshot/restore
CarrierInfoNative snapshot APIs
```

The report distinguishes:

```text
generated derived_hako route
native_hako source existence
native behavior EXE guard
source_selfhost_claim=0 unless explicitly promoted
```

## Scope

```text
BoxCount: one adoption matrix report
owner: generated/native adoption summary
input: family route manifest + native source files + native EXE guards
output: one machine-readable adoption matrix report
```

## Required Checks

```text
bash tools/checks/rust_mirbuilder_generated_to_native_adoption_matrix_guard.sh
bash tools/checks/rust_mirbuilder_binding_context_native_guard.sh
bash tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh
bash tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh
bash tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh
```

## Acceptance

```text
one adoption matrix report prints the generated route, native source, native
EXE smoke, and source_selfhost_claim for each listed slice
binding_context native source exists and EXE smoke is green
variable_context simple-map native source exists and EXE smoke is green
variable_context snapshot/restore native source exists and EXE smoke is green
carrier_info native snapshot APIs source exists and EXE smoke is green
generated routes stay derived_hako
source_selfhost_claim stays 0 unless explicitly promoted
```

## Stop Line

```text
do_not_claim_source_selfhost=1
do_not_remove_rust_bootstrap_oracle_routes=1
do_not_open_runtime_try_hako_then_rust_fallback=1
do_not_summarize_without_native_smoke=1
```
