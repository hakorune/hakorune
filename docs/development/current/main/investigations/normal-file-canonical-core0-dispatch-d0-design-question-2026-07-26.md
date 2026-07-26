---
Status: closed; accepted design
Date: 2026-07-26
Decision: NORMAL-FILE-CANONICAL-CORE0-DISPATCH-prime-r1
Row: NORMAL-FILE-CANONICAL-CORE0-DISPATCH-D0
Scope: choose the sole consuming dispatch owner from a sealed canonical-core normal-file source plan to the existing Script, Main, and callable-candidate owners
ceremony_tier: T2 new authority
Next executable row: NORMAL-FILE-CANONICAL-CORE0-DISPATCH0-S0
Related:
  - docs/development/current/main/investigations/normal-file-canonical-core0-profile0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-callable-module0-tx0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-file-canonical-core0-dispatch-series-execution-task-2026-07-26.md
---

# NORMAL-FILE-CANONICAL-CORE0-DISPATCH-D0

## Accepted decision

```text
Q1:
  A — compiler-layer consuming dispatcher

Q2:
  family evidenceを保持したunpublished candidateを先に作る
  -> one canonical publication boundary
  -> existing neutral VM-reference terminal

Q3:
  existing RawScriptBodyRecipeV1 remains the Script semantic owner
  RawPublishedInvocationV1 / Raw invocation brand are forbidden
  canonical-core Script waits for a canonical candidate/publication adapter

Q4:
  initial connection = Main0 only
  Script / CallableModule = typed pre-Builder FamilyCapabilityPending
  all three families must be connected before full PARITY0-P0a and a caller
```

The runner/front door may issue one consuming compiler-neutral handoff, but it
does not inspect the source-plan variant. The sole family match lives in the
compiler layer.

```text
ClassifiedNormalFileSourcePlanV1
  -> CanonicalCoreSourcePlanCompileRequestV1
  -> NormalCanonicalCoreSourcePlanCompilerV1
  -> CompletedCanonicalCoreSourceEntryCandidateV1
  -> PreparedCanonicalCorePublicationV1
  -> PublishedSourceEntryInvocationV1
  -> existing neutral VM-reference terminal
```

The candidate is unpublished so compilation, publication, execution, and
process projection remain separate authorities.

The first executable row is tracked in:

```text
normal-file-canonical-core0-dispatch-series-execution-task-2026-07-26.md
```

## Why this is a design stop

`NORMAL-FILE-CANONICAL-CORE0-PROFILE0-S0` is closed. It provides a separate
canonical-core profile and carries it through exactly one file read, one
Canonical parse, and one sealed source-family plan. The pre-existing narrow
profile remains frozen and can be the only Raw handoff producer.

The next requested row was named `PARITY0-P0a`, but the current code has no
owner allowed to consume this product:

```text
ClassifiedNormalFileSourcePlanV1
  = SealedNormalSourcePlanV1
  + SealedNormalEntryProfileV1
  + one-read/one-parse receipt
```

Existing lower-level pieces are separately sealed:

```text
Script             -> existing Raw Script result/physical owner
Main.main/0        -> existing normal Main F1/candidate/publication owner
CallableModule     -> existing Main+helper atomic candidate owner
VM/process         -> existing neutral source-entry VM-reference owner
```

Adding ad-hoc `match plan()` logic in the runner, a second front door, or a
fallback from canonical to Raw would create competing route authorities. This
is therefore not a test-only P0a gap.

## Options considered

Choose one owner for the one-shot dispatch.

### A — compiler-layer consuming dispatcher (accepted)

```rust
PreparedCanonicalCoreNormalDispatchV1
  owns ClassifiedNormalFileSourcePlanV1

NormalCanonicalCoreSourcePlanCompilerV1::prepare(self, &mut MirCompiler)
  -> PreparedCanonicalCoreNormalDispatchV1

PreparedCanonicalCoreNormalDispatchV1::commit(self)
  -> PublishedSourceEntryInvocationV1<...>
```

The dispatcher matches the sealed family once:

```text
ScalarRoot::Script  -> existing Script owner
ScalarRoot::Main0   -> existing canonical Main owner
CallableModule      -> existing callable transaction owner
```

It owns only plan consumption, family-to-owner selection, and typed rejection
retention. Function result, callable graph, physical entry, VM execution, and
process status remain owned by their existing layers.

### B — runner-layer dispatch (rejected)

The runner consumes `ClassifiedNormalFileSourcePlanV1` and chooses a compiler
entry. This is not recommended: runner configuration would become semantic
route authority and would have to retain compiler failures/source ownership.

### C — profile-specific separate front doors (rejected)

Create Script/Main/Callable canonical-core request types. This is rejected
unless new evidence proves the source plan cannot retain its single Program
owner: it duplicates source selection and invites profile/family retry.

## Invariants for every acceptable answer

```text
source family classification                        = exactly once
profile selects capability, not source family       = exactly once
Script/Main/Callable fallback or retry              = 0
AST clone/rewrite                                   = 0
source/physical symbol scan                         = 0
NYASH_ENTRY / execute_module discovery              = 0
Raw handoff for canonical-core profile              = 0
second VM executor/result decoder/status owner      = 0
failure retains complete classified source owner    = 1
production CLI/default caller                       = 0 in first dispatch row
```

## Exact questions for review

```text
Q1. Is A the accepted owner boundary?

Q2. Should the dispatcher produce a family-neutral unpublished candidate first,
    then use the existing publication/neutral VM adapter, or may it produce an
    already-published source-entry invocation directly?

Q3. Does Script remain admitted in canonical-core through the existing Raw
    Script result owner, or must it wait for a canonical Script publication
    adapter so canonical-core never crosses a Raw publication owner?

Q4. For initial P0a, should Main0 be the only connected family and Script /
    CallableModule receive typed pre-Builder profile rejection, or should all
    three be required before any connection?
```

## Non-claims

```text
dispatcher implementation
new profile widening
CLI/default route activation
VM/process/diagnostic change
Raw fallback
callable result-carrier widening
imports/using
```

## Closeout evidence

The code audit found two mandatory connection gaps:

```text
Script:
  source-result semantics can reuse RawScriptBodyRecipeV1
  canonical publication owner is absent

CallableModule:
  CompletedNormalCallableCandidateV1 currently retains only MirModule
  exact schema / entry / result / verification evidence must survive commit
  before a publication terminal may exist
```

Neither gap may be repaired by a module scan, symbol inference, Raw handoff,
or result reconstruction.
