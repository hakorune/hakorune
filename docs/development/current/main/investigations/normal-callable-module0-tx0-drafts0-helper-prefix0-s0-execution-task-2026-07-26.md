---
Status: closed executable row
Date: 2026-07-26
Decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-HELPER-PREFIX0-S0
Parent: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-S0
Scope: consume the HANDOFF0 helper schedule once and retain only the exact successful helper-draft prefix on failure
ceremony_tier: T1 bounded owner/evidence refactor
series_mode: BoxShape only; no accepted source/result shape grows
Related:
  - docs/development/current/main/investigations/normal-callable-module0-tx0-drafts0-lowerer0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-callable-module0-tx0-handoff0-s0-execution-task-2026-07-26.md
  - src/mir/compiler/normal_source_plan/normal_callable_transaction_handoff.rs
  - src/mir/builder/resolved_lowering/
---

# NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-HELPER-PREFIX0-S0

## Closeout

Closed on 2026-07-26. The Builder-side prefix owner consumes the existing
HANDOFF0 `BTreeMap` once in canonical-key order, verifies each completed draft
against the retained catalog ABI before admission, and retains only completed
drafts plus the exact failing key/ordinal/stage. The schedule borrow ends
before the open transaction is bound into a success or rejection owner. Main,
physical thunk, batch, publication, caller, and backend behavior remain
unchanged.

The next row is
`NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-MAIN-PHYSICAL0-S0`.

## Outcome

Create the one-shot helper-draft prefix owner.  It consumes the already
selected HANDOFF0 schedule in canonical-key order and either yields every
helper draft or retains only the completed prefix plus the exact consumed
operation failure:

```text
CompletedNormalMainHelperResolutionV1
  -> callback-scoped owned helper schedule
  -> CanonicalTrivialBindingSsaPlanV1 (once per helper)
  -> verified helper draft prefix
     or typed helper-draft rejection
```

The parent source/catalog authority stays durable outside the temporary
schedule borrow.  The schedule borrow ends before a success or rejection
owner is issued; no self-referential owner is permitted.

## Sole authority

This row owns exactly these transition facts:

```rust
struct RetainedNormalHelperDraftPrefixV1 {
    topology: NormalHelperTopologyReceiptV1,
    drafts: Vec<VerifiedNormalHelperDraftV1>,
}

struct VerifiedNormalHelperDraftV1 {
    key: CanonicalCallableKeyV1,
    draft: MirFunction,
    _seal: VerifiedNormalHelperDraftSealV1,
}

struct ConsumedNormalHelperLoweringReceiptV1 {
    key: CanonicalCallableKeyV1,
    role: NormalFunctionRoleV1, // Helper only in this row
    ordinal: usize,
    stage: NormalFunctionDraftLoweringStageV1,
    _seal: ConsumedNormalHelperLoweringReceiptSealV1,
}
```

`CanonicalTrivialBindingSsaPlanV1` is consumed exactly once.  It is never
reconstructed or retained after a failure.  The consumed receipt is evidence,
not a retry capability.

## Required order

```text
one selected topology receipt
  -> its owned schedule
  -> exact canonical-key iteration order
  -> lower one plan through the retaining lowerer
  -> verify key/symbol/arity against the sealed catalog
  -> append the completed draft to the prefix
```

The existing HANDOFF0 schedule's `BTreeMap` remains the sole execution-order
authority; its consuming iteration is canonical-key order. The retained
topology is correspondence evidence, not a second execution-order algorithm.
Do not add another sort, declaration-order loop, or a second helper inventory.

## Failure law

```text
helper k fails
  -> drafts [0..k) retained exactly
  -> k's plan is consumed, represented only by its receipt
  -> helpers (k+1..) are not lowered
  -> Main and physical thunk are not lowered
  -> caller Builder context is restored by the retaining lowerer
  -> candidate-module publication = 0
```

The rejection owner retains the original completed source resolution only
after the callback-scoped schedule borrow has ended.  It exposes inspection
and `discard(self)` only; `retry`, `resume`, plan reconstruction, profile
reselection, and legacy fallback are forbidden.

## Explicit non-authority

```text
Main source draft                           = 0
physical main thunk                         = 0
batch/schema/correspondence/publication     = 0
new helper catalog/index/topology            = 0
AST clone/rewrite                            = 0
new source/result capability                 = 0
retry/resume/fallback                        = 0
runner/backend/process changes               = 0
```

## Focused fixtures

```text
acyclic helpers:
  canonical-key schedule is used exactly once
  all drafts retain catalog-matching key/symbol/arity

middle helper lowering failure:
  exact prefix retained
  exact failed key/ordinal/stage retained
  later helper/Main/thunk execution = 0

draft-seal failure:
  inner FunctionDraftSealStageV1 is retained directly
  no String stage classification

rejection -> later success:
  same MirBuilder has restored caller state
  partial module publication = 0
```

Use a private `#[cfg(test)]` loop-boundary injector only if a natural fixture
cannot reach the required failure point.  It must not add a production branch,
environment gate, or fallback.

## Acceptance

```text
one helper schedule consumer                    = 1
canonical-key order authority                   = existing HANDOFF0 BTreeMap only
typed retaining lowerer consumer                = 1
consumed-plan reconstruction                    = 0
helper prefix owner                             = 1
Main/thunk/batch/publication caller             = 0
retry/resume/fallback                            = 0
all modified/new source/check files            < 800 lines
```

Run:

```bash
cargo check --lib --features vm-reference
cargo test -q --lib normal_source_plan --features vm-reference
python3 tools/checks/lib/normal_source_plan0_transaction_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next rows

```text
NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-HELPER-PREFIX0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-MAIN-PHYSICAL0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-G0
```
