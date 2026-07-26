# Canonical Source-Entry Publication 0 — Execution Task

```text
Decision: CANONICAL-SOURCE-ENTRY-PUBLICATION0-prime-r1
Status: accepted
Ceremony tier: T2
Active executable row: NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0
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

Status: closed by `2fbf5a722a` and `1c470bc017`.

Run actual Script VM/process parity for Unit origins, scalar results,
unsupported process results, and VM faults; run the unchanged Main matrix
through the same core.

### 6. `CANONICAL-SOURCE-ENTRY-PUBLICATION0-G0`

Status: closed by `e38e064629`.

Merge the structural assertions into existing canonical-core/source-entry
guards. Do not add a per-cell shell wrapper.

## Callable evidence row

### `NORMAL-CALLABLE0-PUBLICATION-EVIDENCE0-S0`

Status: closed by this commit.

The callable candidate now retains the facts sealed before candidate commit:

```text
source-Main to physical main/0 relation
sealed source-result contract
complete normal-module schema
prepared helper topology receipt
source identity
candidate function/schema verification receipt
MirModule
```

The commit consumes the existing prepared batch once. It does not rescan the
module, Return instructions, signatures, source AST, or callable graph.

## Next row

### `NORMAL-CALLABLE0-CANONICAL-PUBLICATION0-S0`

Status: closed by this commit.

Callable now projects its retained candidate evidence into the existing shared
canonical publication core:

```text
Canonical(Callable { source_owner }) membership = 1
shared publication prepare / infallible commit  = unchanged
canonical VM executor                           = unchanged, still 1
```

The projection checks retained schema, topology, candidate verification, and
exact physical entry facts. It never scans the module or AST. The focused
proof constructs a callable candidate through the normal transaction and
confirms its published target/result/family projection.

## Next row

`CANONICAL-CORE-DISPATCH-PUBLICATION0-S0` must connect the already-sealed
`CallableModule` source plan to its existing callable transaction and then to
this publication core. It replaces only the current typed
`FamilyCapabilityPending(CallableModule)` rejection; no second dispatcher,
publication owner, VM executor, or retry path is permitted.

### Dispatch policy

`NormalFileCanonicalCoreVmReferenceV1` owns exactly one source file and
admits no imports. Its sole callable-catalog compilation-unit identity is
therefore the named canonical-core single-file ordinal `0`. This is source
identity issuance only: it is not a caller option, ambient value, retry key,
or backend selection.

The dispatch sequence is fixed:

```text
CallableModule source
  -> callable-source validation
  -> helper catalog (single-file ordinal 0)
  -> Main-with-catalog resolution
  -> Main direct-call preflight
  -> helper graph resolution
  -> helper draft prefix
  -> Main + physical main transaction
  -> callable candidate commit
  -> existing shared canonical publication
```

Every intermediate typed rejection retains its exact existing owner. The
outer dispatch only classifies that rejection as `Callable`; it neither
rebuilds a source plan nor tries another family.

The first dispatch slice admits a Main fallthrough plus top-level static
`i64 -> i64` helpers. A Main direct call remains the existing typed
first-family rejection until `NORMAL-MAIN-DIRECT-CALL0-S0`; dispatch must not
broaden that capability incidentally.

### `CANONICAL-CORE-DISPATCH-PUBLICATION0-S0`

Status: closed by this commit.

The sole `CallableModule` dispatch now consumes the fixed sequence through
callable-source validation, catalog, Main preflight, helper resolution, draft
transaction, candidate commit, and shared publication. Every rejection
retains the exact owner returned by that existing layer; the outer dispatch
only records the `Callable` stage.

```text
second source classification = 0
second publication owner     = 0
family-specific VM executor  = 0
CallableModule pending reject = 0
```

## Next row

`NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a` runs the real VM-reference matrix
over the admitted Script, Main, and callable-first-family forms. It must
separately prove the expected pre-execution typed rejection for Main direct
calls, rather than widening that capability or falling back.

### `NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a`

Status: closed by this commit.

The focused canonical-core matrix now proves:

```text
Script / Main existing VM parity                  = green
Callable: static i64 helper + Main fallthrough    = VM status 0 / AppMain0
Callable: Main direct call                         = MainPlan typed rejection
direct-call rejection -> later callable success    = green
```

No fallback or profile reselection occurs. The one shared publication and VM
owner remains the only execution path.

## Next row

`NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0` expands reuse evidence from the
focused direct-call case to a single compiler sequence covering Script/Main/
Callable success, callable catalog/preflight rejection, and VM process Fault.
It changes no accepted source shape.

### `NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0`

Status: closed by this commit.

One compiler instance now proves the complete bounded sequence:

```text
Script success
  -> Main success
  -> Callable success
  -> Callable direct-call MainPlan rejection
  -> Script VM Fault 70
  -> Callable success
```

No failure changes the fixed profile, retries another family, or leaves a
published partial module.

## Next row

`NORMAL-FILE-CANONICAL-CORE0-REQUEST0-S0` creates the one explicit,
default-off, crate-private normal-file canonical-core VM-reference request.
Selection is CLI-shaped but has no production caller yet; it owns one existing
normal-file request and accepts no backend/optimization/profile reconstruction.

### `NORMAL-FILE-CANONICAL-CORE0-REQUEST0-S0`

Status: closed by this commit.

`NormalFileCanonicalCoreVmReferenceProductionRequestV1` now owns exactly one
existing canonical-core front-door request. Its selector spelling is
`normal-file-canonical-core-vm-reference`, but this row does not add it to the
central selector or any production caller.

```text
file I/O / parse / compile / VM execution = 0
profile reconstruction                     = 0
non-default optimization                  = typed usage rejection
```

## Next row

`NORMAL-FILE-CANONICAL-CORE0-REPORT0-S0` consumes this request through the
existing front door, canonical-core compiler dispatch, shared publication,
and VM terminal. It returns the common reference outcome vocabulary and does
not yet connect the CLI selector.

### `NORMAL-FILE-CANONICAL-CORE0-REPORT0-S0`

Status: closed by this commit.

The runner-neutral report owner consumes the sealed canonical-core request
through one file read, one canonical parse, one source-plan classification,
the shared canonical compiler/publication/VM chain, and the existing common
reference terminal vocabulary. It owns neither selector nor process exit.

```text
pre-execution source/plan/dispatch/publication rejection = Invocation
executed program result / Fault                           = Program
status reconstruction                                     = 0
fallback / Raw re-entry                                   = 0
```

## Next row

`NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a` adds the production-shaped matrix
for Script, Main, and the admitted callable slice before any CLI selection or
production caller is connected.

### `NORMAL-FILE-CANONICAL-CORE0-PARITY0-P0a`

Status: closed by this commit.

The report-shaped matrix covers Script Unit/scalar results, range and
unsupported-result process Faults, VM division fault, Main fallthrough, the
admitted helper-plus-Main callable slice, parse/using rejection, and the
existing typed direct-call rejection. No case reaches CLI selection or a
process terminal.

## Next row

`NORMAL-FILE-CANONICAL-CORE0-REUSE0-P0` records the compiler reuse evidence
for success, rejection, and program Fault before the explicit selector is
allowed to gain a production caller.

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
