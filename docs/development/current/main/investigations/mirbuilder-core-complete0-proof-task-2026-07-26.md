---
Status: active proof task
Date: 2026-07-26
Decision: MIRBUILDER-CORE-COMPLETE0-P0
---

# MirBuilder core-complete proof

This is a read-only aggregation row after `NORMAL-FILE-VM0-G0`. It declares
only that the new compiler core has one sealed authority chain and one bounded
normal-file production lane. It does not change the default backend or claim
that every normal caller has migrated.

## Accepted core boundary

```text
canonical function exit owner = 1
canonical Script result owner = 1
canonical entry/result owner = 1
atomic draft/module publication = 1
supported Raw VM lane = 1
bounded normal file lane = 1
normal-file fallback = 0
normal-file production caller = 1
default route replacement = 0
JSON / REPL / LLVM / selfhost = non-blocking
```

The normative language boundary remains
`docs/reference/language/function-exit-and-entry-result.md`. The bounded
normal lane is `NormalFileNoImportVmReferenceV1`, selected only by
`--backend normal-file-vm-reference` with the `vm-reference` feature.

## Evidence owners

```text
SealedFunctionExitContractV1
PreparedFunctionDraftSealV1
ScriptLastExpressionOrUnit
SourceEntryResultV1 -> ProcessExitProjectionV1
compile_raw_published_v1
run_raw_vm_reference_owned_v1
NormalFileVmReferenceProductionRequestV1
reference::select_and_run
```

The production route guard proves one central selector, one Raw caller, one
normal caller, default-route isolation, no fallback, and the source/check file
budget. Real-binary P0b already proves status 42 for Integer, status 70 for
unsupported Bool, status 1 for invocation failure, status 2 for usage/feature
rejection, and stable diagnostics.

## Verification

```bash
python3 tools/checks/lib/mirbuilder_core_complete0_guard.py
python3 tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_owner_guard.py
python3 tools/checks/lib/entry_result_projection0_s3_execution_guard.py
bash tools/checks/current_state_pointer_guard.sh
```

This row must not add function capability, imports, dynamic carriers, JSON,
REPL, LLVM/native, selfhost, executor, fastmem, or default-route cutover.
