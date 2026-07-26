---
Status: active design stop
Date: 2026-07-26
Decision: pending
Row: NORMAL-FILE-CANONICAL-CORE0-DISPATCH-D0
Scope: choose the sole consuming dispatch owner from a sealed canonical-core normal-file source plan to the existing Script, Main, and callable-candidate owners
ceremony_tier: T2 new authority
Related:
  - docs/development/current/main/investigations/normal-file-canonical-core0-profile0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-callable-module0-tx0-s0-execution-task-2026-07-26.md
---

# NORMAL-FILE-CANONICAL-CORE0-DISPATCH-D0

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

## Required decision

Choose one owner for the one-shot dispatch.

### A — compiler-layer consuming dispatcher (recommended)

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

### B — runner-layer dispatch

The runner consumes `ClassifiedNormalFileSourcePlanV1` and chooses a compiler
entry. This is not recommended: runner configuration would become semantic
route authority and would have to retain compiler failures/source ownership.

### C — profile-specific separate front doors

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
