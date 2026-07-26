---
Status: accepted
Date: 2026-07-26
Decision: NORMAL-SCRIPT0-PHYSICAL-ENTRY-prime-r1
Row: NORMAL-SCRIPT0-PHYSICAL-ENTRY-D0
Scope: select the canonical, brand-free physical-entry materialization owner
for a source-classified Script candidate
ceremony_tier: T2 new physical/session/completion authority
Blocks: NORMAL-FILE-CANONICAL-CORE0-DISPATCH0-S0 / SCRIPT0
Accepted: A-prime Script-specific unbranded outer transaction plus one shared
brand-free Script exit kernel
First executable row: NORMAL-SCRIPT0-PHYSICAL-ENTRY0-S0
Related:
  - normal-file-canonical-core0-dispatch-series-execution-task-2026-07-26.md
  - normal-script0-physical-entry0-s0-execution-task-2026-07-26.md
  - normal-file-canonical-core0-dispatch-d0-design-question-2026-07-26.md
  - docs/reference/language/function-exit-and-entry-result.md
  - docs/reference/language/semantic-kernel.md
---

# External Consultation: Canonical Script Physical Entry

## Accepted resolution

```text
outer lifecycle owner =
  Script-specific and unbranded

shared inner owner =
  brand-free Script lowering / completion / result / Return kernel

Raw relation =
  Raw Script consumes the same inner kernel
  Raw brand / tracker / witness / ledger remain in a post-kernel adapter

module relation =
  one PhysicalEntry row
  no SourceMain row
  no synthetic callable owner
  no Main-to-main thunk
```

Implementation audit adds five mandatory refinements without changing the
accepted semantics:

1. `VerifiedNormalScriptRecipeV1` retains the sealed Script source opaquely so
   every rejection can retain the complete classified owner. Only the exact
   Script recipe is a lowering authority.
2. The outer transaction owns a candidate-only
   `ScriptPhysicalEntrySessionV1`; it does not receive a terminal that can
   replace the live compiler Builder.
3. The shared kernel emits one `VerifiedScriptEntryResultContractV1`, so Raw
   and canonical adapters cannot reconstruct result/origin policy separately.
4. Raw `Print` / `Local` / assignment Unit origins are an intentional
   conformance correction. Status and physical Void behavior remain unchanged.
5. The Script schema is a thin one-row wrapper over shared canonical row-set
   validation; Main schema validation is not copied.

Options B and C are rejected. The executable owner chain and commit order are
fixed in the related execution task.

## Original decision question

The consultation question below is retained as decision evidence. It is no
longer an active design stop.

The consultation asked which owner chain should turn an already
source-classified Script recipe into one unpublished canonical `main/0`
candidate.

```text
SealedNormalScriptSourceV1
  -> VerifiedNormalScriptRecipeV1
  -> ?
  -> CompletedNormalScriptModuleCandidateV1
  -> later canonical publication
  -> existing neutral VM-reference execution
```

The answer must not add a Script-tail classifier. `RawScriptBodyRecipeV1`
already owns the source result distinction:

```text
EmptyUnit | ValueExpression | UnitExpression | UnitStatement
```

`Print` / `Local` / assignment forms remain Unit.

## Confirmed boundary

Landed source handoff:

```text
one parsed Script source
  -> SealedNormalScriptSourceV1
  -> one consuming shared RawScriptBodyRecipeV1 projection
```

Forbidden in canonical Script:

```text
RawPublishedCompileRequestV1
RawPublishedInvocationV1
Raw invocation token / brand / ledger
Raw publication/fallback
AST clone/rewrite or source reclassification
```

The existing Raw materializer is not a candidate owner:

```text
InstalledRawRootEnvironmentV1::drive_root_body()
  owns token/brand, tracker, ledger, and Raw witness/publication pairing.
```

## Required result

```text
physical entry:
  one exact main/0
  return/signature from the sealed Script result
  no source Main callable and no Main-to-main thunk

unpublished candidate:
  MirModule
  exact main/0 target evidence
  source-result evidence
  physical-result/signature evidence
  complete verification receipt
```

Existing canonical function sessions require a resolved callable authority.
Script has neither a callable declaration nor a resolved owner. Existing Raw
body completion is brand-bound. Neither can become the Script owner as-is.

## Options

### A — Script-specific unbranded entry transaction (recommended)

```text
VerifiedNormalScriptRecipeV1
  -> OpenScriptPhysicalEntryV1
  -> PreparedScriptPhysicalExitV1
  -> CompletedScriptPhysicalExitV1
  -> CompletedNormalScriptModuleCandidateV1
```

Properties:

```text
one source-entry recipe session
one unbranded completion witness
one physical Return writer
one-function shell transaction
candidate-only; publication follows later
```

The generic Script completion/exit logic is extracted from Raw root plumbing.
Raw may later adapt that generic engine, but canonical Script never owns Raw
brand/token/ledger types.

### B — Reuse Raw root body with a synthetic/neutral brand (reject?)

```text
canonical Script
  -> RawRootBodyPhysical* / RawRootBodyExitWitnessV1
  -> discard brand later
```

Risk: Raw invocation identity leaks into canonical lifecycle and can reopen a
Raw publication route.

### C — Treat Script as a synthetic resolved callable (reject?)

```text
Script source
  -> invented FunctionOwnerId / resolved header
  -> existing resolved-function session
  -> physical entry
```

Risk: implementation scaffolding becomes source semantic authority and Script
is conflated with `Main.main/0`.

## Questions for review

```text
Q1. Is A the accepted owner boundary?

Q2. Should the generic owner be Script-only now, or a broader brand-free
    SourceEntryRecipe session from its first implementation?

Q3. Is a new unbranded Script completion witness required, rather than making
    RootBodyCompletionTrackerV1 optionally/unbranded?

Q4. Must Raw Script lowering adapt to the generic Script exit engine in the
    same series to enforce one physical Script Return writer, or may that be
    the immediate named follow-up?

Q5. Is a one-row Script module schema correct, separate from
    NormalModuleTransactionSchemaV1 which requires SourceMain plus physical
    entry?

Q6. Give the smallest safe implementation order and exact fail-fast rejection
    boundaries.
```

## Non-negotiable invariants

```text
Script result authority                         = RawScriptBodyRecipeV1 only
canonical Script Raw publication consumer       = 0
canonical Script Raw invocation brand consumer  = 0
canonical Script AST reclassification           = 0
canonical Script physical Return writer         = 1
candidate publication before verification       = 0
fallback/retry/profile reselection              = 0
VM/result/process policy duplication            = 0
```

## Requested answer format

```text
Decision: accepted/rejected option
Owner chain: exact types and consuming boundaries
Raw relation: shared versus forbidden elements
Failure law: retained owner, stage/cause, effects boundary
Implementation: 3–6 buildable commits
Structural gate: producer/consumer and zero-count assertions
Non-claims: excluded work
```
