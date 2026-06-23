# 296x-1652: MetadataContext Region Parent AOT Reopen

Status: Complete
Date: 2026-06-24
Token: METADATA-CONTEXT-REGION-PARENT-AOT-REOPEN-001

## Decision

Reopen the existing `MetadataContext region-parent` generated artifact on top
of the generic boxed-sum I64 payload ABI.

This is a consumer acceptance slice only. The boxed-sum ABI is already owned by
`BOXED-SUM-I64-PAYLOAD-ABI-001`.

## Scope

```text
existing generated artifact:
  metadata_context_region_parent

expected change:
  generated_hako_exe_aot:
    skipped_pending_boxed_i64_payload -> green

allowed:
  guard truth update
  artifact acceptance rerun
  closeout status update
```

## Non-Claims

```text
boxed ABI implementation change = 0
new operation kind = 0
new converter rule = 0
MetadataContext backend branch = 0
general Option payload support = 0
runtime fallback = 0
```

## Acceptance

```text
generator --check = green
MIR emit = green
EXE/AOT = green
stdout contains:
  metadata_context_region_parent_direct_artifact=ok
return code = 0
generated_hako_exe_aot_claim=1
boxed_i64_payload_claim=1
```

## Closeout Evidence

```text
bash tools/checks/rust_lifecycle_metadata_context_region_parent_derived_artifact_guard.sh
  deterministic_regeneration=green
  generated_hako_mir_emit=green
  generated_hako_exe_aot_claim=1
  generated_hako_exe_aot=green
  boxed_i64_payload_claim=1
  runtime_try_hako_then_rust_fallback=0
  summary=ok

bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
  metadata_context_region_parent=green
  metadata_context_region_parent_backend=green
  summary=ok
```
