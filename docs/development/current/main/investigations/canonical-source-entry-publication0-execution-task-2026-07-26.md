# Canonical Source-Entry Publication 0 — Execution Task

```text
Decision: CANONICAL-SOURCE-ENTRY-PUBLICATION0-prime-r1
Status: accepted
Ceremony tier: T2
Active executable row: NORMAL-SCRIPT0-VMREF0-P0
```

## Boundary

Canonical Script and Main already produce complete unpublished module
candidates. Publication is a consuming typestate promotion only; it neither
inserts functions nor reinterprets source, Return operands, signatures, or VM
values.

```text
family-private candidate evidence
  -> shared canonical publication prepare
  -> infallible published-owner commit
  -> existing PublishedSourceEntryInvocationV1
  -> existing VM-reference execution / ProcessExitProjectionV1
```

The neutral invocation remains the sole owner of target, result, and
membership. The canonical published owner holds the module, family evidence,
admission, source receipt, and publication-verification receipt.

## Authority

```text
Script result / physical Return      = shared Script exit completion evidence
Main result / physical Return        = existing Main candidate evidence
candidate-to-published promotion     = PreparedCanonicalSourceEntryPublicationV1
neutral target/result/membership     = PublishedSourceEntryInvocationV1
VM execution                         = existing generic VM-reference owner
process status                       = ProcessExitProjectionV1
```

Forbidden:

```text
Raw brand / ledger in canonical Script
Script-as-Main membership
dummy FunctionOwnerId or SourceMain row
publication AST / Return / signature / VMValue inference
family-specific canonical VM executors
fallback, retry, profile reselection
```

## Required products

```text
CompletedScriptPhysicalFunctionV1
  retains CompletedScriptPhysicalExitCoreV1

CompletedNormalScriptModuleCandidateV1
  retains exact physical main/0 target, sealed Script result,
  physical result/Return evidence, schema, source identity, and verification

PreparedCanonicalSourceEntryPublicationV1
PublishedCanonicalSourceEntryOwnerV1
PublishedSourceEntryInvocationV1<PublishedCanonicalSourceEntryOwnerV1>
```

Canonical membership is nested in the existing neutral membership field:

```text
Raw { brand }
Canonical(Main { source_owner } | Script)
```

## Commit series

### 1. `NORMAL-SCRIPT0-PUBLICATION-EVIDENCE0-S0`

Status: closed by `dec19959af`.

Preserve existing Script exit evidence through physical function, detached
exit, and one-row module candidate. Seal a total Script-entry result contract
from the completed receipt and draft correspondence.

```text
publication / VM execution / AST re-observation = 0
```

### 2. `CANONICAL-SOURCE-ENTRY-PUBLICATION0-S0-A`

Status: closed by `49e9e2c3e3`.

Create shared Main+Script publication prepare/commit owners. Every fallible
pairing check completes before one infallible move into the existing neutral
invocation.

### 3. `CANONICAL-SOURCE-ENTRY-PUBLICATION0-S0-B`

Status: closed by `8dcb939543`.

Move Main to the shared core. Replace flat canonical Main membership and the
Main-specific VM-owner variant with one canonical family owner and one VM
executor implementation. Raw remains unchanged.

### 4. `NORMAL-SCRIPT0-PUBLISHED-OWNER0-S0`

Status: closed by `74875d16fa`.

Connect the completed Script candidate to the shared core with exact
`Canonical(Script)` membership. Production callers remain zero.

### 5. `NORMAL-SCRIPT0-VMREF0-P0`

Status: active.

Run actual Script VM/process parity for Unit origins, scalar results,
unsupported process results, and VM faults; run the unchanged Main matrix
through the same core.

### 6. `CANONICAL-SOURCE-ENTRY-PUBLICATION0-G0`

Merge the structural assertions into existing canonical-core/source-entry
guards. Do not add a per-cell shell wrapper.

## Acceptance gates

```text
canonical publication prepare / commit = 1 each
Main + Script publication producers    = 1 each
Callable publication producer           = 0

Script candidate target/result/physical/verification evidence = 1 each
publication-side source or physical re-inference               = 0
Script-as-Main / dummy owner / dummy row                        = 0

canonical VM executor implementation = 1
Main/Script-specific canonical executor = 0
Raw route delta / default caller delta / fallback = 0
all modified source/check files < 800 lines
```

## Failure law

Every publication prepare failure retains the complete unpublished canonical
candidate plus its family evidence, admission, and one-read/one-parse receipt.
The public surface is `stage()`, `cause()`, bounded report, and `discard()`.
Publication commit itself is infallible; VM execution and process projection
are later owners.

## Parked follow-up

Callable joins only after `NORMAL-CALLABLE0-PUBLICATION-EVIDENCE0-S0` equips
its current module-only candidate with target, result, topology, schema, and
verification evidence.
