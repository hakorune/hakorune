---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
Scope: move the existing Raw VM-reference production lane through the sole neutral published-entry and VM projection owners
ceremony_tier: T1 behavior-preserving adapter cutover
sunset_id: SOURCE-ENTRY-VMREF-RAW-DIRECT-SUNSET-001
sunset_row: SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
proof_inventory_before: existing S3 Raw execution matrix plus closed neutral L0 fixture
new_proofs: zero; reuse and extend the existing S3 execution fixture/guard
retired_or_merged_proofs: direct Raw-only activation/execution/outcome authority
net_proof_delta: non-positive
sunset_budget: no new disconnected proof
retire_when: production Raw runner uses the neutral owner once and all old direct execution/status callers are zero
budget_repayment_evidence: existing S3 parity remains green after the sole caller cutover
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/source-entry-vmref-neutral0-l0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/entry-result-projection0-s3-raw-vm-activation-execution-task-2026-07-25.md
---

# SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0

## Outcome

Move the supported Raw VM-reference lane through:

```text
RawPublishedInvocationV1
  -> exact Raw family adapter
  -> PublishedSourceEntryInvocationV1<RawPublishedInvocationV1>
  -> PreparedVmReferenceSourceEntryInvocationV1<RawPublishedInvocationV1>
  -> one exact fresh MirInterpreter execution terminal
  -> SourceEntryResultV1
  -> ProcessExitProjectionV1
  -> existing bounded diagnostic/report
```

Behavior, status, diagnostic spelling, CLI selection, and Raw compile policy
must remain unchanged.

## Mandatory first edit

`source_entry_vm_execution.rs` is already near the source-file boundary.
Before adding adapter code:

```text
move its cfg(test) module to source_entry_vm_execution_tests.rs
production behavior delta = 0
test inventory delta = 0
```

No new shell guard is allowed.

## Raw adapter authority

The adapter consumes the complete `RawPublishedInvocationV1` and validates:

```text
invocation brand retained
selected entry route retained
root target witness matches selected target
target is exact arity zero
existing Raw decode evidence is available
```

It then issues the neutral target/result/membership facts exactly once.

The current Raw decode vocabulary may be mapped losslessly to the neutral
result contract inside this adapter. That mapping is migration transport, not
a second source classifier:

```text
Unit(origin, requires_void)
  -> Unit(origin, ExactVoid | CompatiblePayload)
Integer -> Integer
Bool    -> Bool
Float   -> Float
String  -> String
```

Forbidden:

```text
AST/source re-observation
VMValue result inference
MirType/Return scan
module inventory scan
NYASH_ENTRY
execute_module
compatibility-module authority erasure
fallback/retry
```

## One executor

The exact fresh-interpreter execution and VM error classification move behind
the VM-specific neutral product. The Raw family owner supplies an opaque exact
module execution loan; it does not select the target.

There must be one:

```text
execute_function_with_args(exact symbol, [])
decode_vm_value
vm_error_to_source_fault
ProcessExitProjectionV1
diagnostic adapter
```

Raw-specific wrappers may retain their public report names, but must not own a
second execution or process policy.

## Failure retention

Adapter rejection retains the complete `RawPublishedInvocationV1`:

```text
stage()
error()
discard(self)
```

No owner recovery, retry, alternate entry, or compatibility erasure terminal.

Compile rejection remains `RejectedRawPublishedCompileV1`. VM execution faults
remain executed source Faults and proceed to canonical status 70.

## Implementation order

```text
RAW-A FILE-SPLIT0
  extract existing cfg(test) module

RAW-B EVIDENCE0
  bounded brand/route/target/decode accessors
  complete Raw owner retention

RAW-C ADAPTER0
  one consuming Raw -> neutral published-entry transition

RAW-D EXECUTOR0
  move exact fresh VM execution behind passive VM projection

RAW-E CUTOVER0
  run_raw_vm_reference_owned_v1 uses the neutral chain once

RAW-F RETIRE0
  remove old direct Raw-only activation/execution/outcome authority

RAW-G PARITY/G0
  existing semantic/status/diagnostic/decoy/reuse matrix
  caller and guard census
```

## Required parity

```text
empty / void
Integer 0 / 255 / -1 / 256
Bool / Float / String
Print / Local / Assignment / CompoundAssignment
App empty and scalar fallthrough
division-by-zero source Fault
ABI mismatch
NYASH_ENTRY decoy ignored
success -> success
compile rejection -> success
execution Fault -> success
```

The comparison is old production Raw behavior versus the new neutral route
over the same typed source/result law. It is not Legacy runner parity.

## Structural gate

```text
Raw -> PublishedSourceEntryInvocation producer      = 1
neutral VM projection production consumer           = 1
exact fresh VM executor                              = 1
ProcessExitProjectionV1 status authority             = 1
diagnostic adapter authority                         = 1

old direct prepare_vm_reference_activation caller    = 0
old CompletedRawVmReferenceExecutionV1 caller         = 0
VmReferenceProjectedOwnerV1::Raw                      = 0 after cutover
from_raw_vm_reference                                 = 0 after cutover

execute_module / NYASH_ENTRY / module scan            = 0
compatibility module erasure in adapter               = 0
fallback / retry                                      = 0

canonical Main adapter caller                         = 0
default/product route delta                           = 0
all modified/new source/check files                   < 800 lines
```

## Immediate continuation

```text
SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF0-P0
```

The canonical adapter must consume the completed Main candidate through an
explicit publication transition. It may not clone or expose its `MirModule`.

## Non-claims

```text
canonical Main publication/activation
default/product backend cutover
helper/direct-call support
imports/using
JSON/LLVM/native
reference CLI retirement
Legacy retirement
```
