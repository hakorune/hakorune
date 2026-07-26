---
Status: active executable row
Date: 2026-07-26
Decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-LOWERER0-S0
Parent: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-S0
Scope: add one typed retaining resolved-function draft-lowering terminal and retain its exact Builder restoration receipt
ceremony_tier: T1 bounded owner/evidence refactor
series_mode: BoxShape only; no accepted source/result shape grows
Related:
  - docs/development/current/main/investigations/normal-callable-module0-tx0-draft-failure-d0-design-question-2026-07-26.md
  - docs/development/current/main/investigations/normal-callable-module0-tx0-handoff0-s0-execution-task-2026-07-26.md
  - src/mir/builder/resolved_lowering/
  - src/mir/compiler/normal_source_plan/normal_callable_transaction_handoff.rs
---

# NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-LOWERER0-S0

## Outcome

Create the single lower-function terminal that preserves the exact failure
stage and the proof that the Builder session has been restored:

```text
CanonicalTrivialBindingSsaPlanV1
  -> typed retaining lowerer
  -> MirFunction
     or RejectedNormalFunctionDraftLoweringV1
```

This subrow does not iterate helpers, lower Main through TX0, construct a
physical thunk, create a batch, or publish a module.

## Sole authority

The new terminal belongs beside the existing resolved trivial lowering and is
the only owner of these facts:

```rust
enum NormalFunctionDraftLoweringStageV1 {
    SessionOpen,
    BindingInstall,
    Skeleton,
    BodyLowering,
    DraftSeal(FunctionDraftSealStageV1),
    SessionRestore,
}

struct RejectedNormalFunctionDraftLoweringV1 {
    stage: NormalFunctionDraftLoweringStageV1,
    cause: NormalFunctionDraftLoweringCauseV1,
    restoration: NormalFunctionDraftBuilderRestorationReceiptV1,
}
```

`stage` is issued at each exact failure site. It must never be inferred by
matching a formatted error string. Bounded legacy detail may remain inside the
nested cause, but is not an outer-stage authority.

The existing `lower_resolved_trivial_function_draft` remains a compatibility
facade over the new typed terminal. No existing production caller changes in
this row.

## Required law

```text
failure after child session opens
  -> exact child terminal runs
  -> caller context is restored exactly once
  -> typed restoration receipt is issued
  -> candidate module mutation = 0

draft-seal rejection
  -> FunctionDraftSealStageV1 is retained directly
  -> no String stage parsing
  -> RejectedFunctionDraftSealV1::discard() closes the child state

success
  -> same MirFunction as current compatibility facade
```

The terminal does not retain or reconstruct a consumed lowering plan. That
receipt belongs to the outer TX0 rejection in the next subrow.

## Explicit non-authority

```text
helper loop/prefix                         = 0
Main source proof binding                  = 0
physical thunk                             = 0
batch/schema/publication                   = 0
source/profile/entry classification        = 0
AST clone/rewrite                          = 0
retry/resume/fallback                      = 0
runner/backend/process changes             = 0
```

## Focused fixtures

```text
body-lowering failure:
  typed BodyLowering stage
  restoration receipt

draft-seal failure:
  typed DraftSeal(FunctionDraftSealStageV1)
  no formatted-string stage classification

failure -> same MirBuilder later success:
  caller restoration exact
  candidate module unchanged on failure

compatibility facade:
  unchanged success and public error behavior
```

Place tests beside the new terminal. Do not grow `main_transaction.rs` or
introduce a per-row shell guard; extend the existing transaction guard in the
later G0 row.

## Series after this row

```text
NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-LOWERER0-S0
  typed retaining function lowerer only

-> NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-HELPER-PREFIX0-S0
  consume HANDOFF0 schedule once
  retain exact successful helper draft prefix and consumed-operation receipt

-> NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-MAIN-PHYSICAL0-S0
  bind one Main proof, prepare source Main and physical thunk drafts
  retain the exact prefix on later failure

-> NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
  schema/correspondence/verification only

-> NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
  one infallible candidate drain

-> NORMAL-CALLABLE-MODULE0-TX0-G0
  atomicity/reuse/structural closeout
```

## Acceptance

```text
typed retaining lowerer terminal                 = 1
compatibility facade                              = 1
outer-stage String parsing                        = 0
lowering-plan reconstruction                      = 0
partial module publication                        = 0
helper/Main/thunk/batch caller                    = 0
all modified/new source/check files              < 800 lines
```

Run:

```bash
cargo check --lib --features vm-reference
cargo test -q --lib resolved_lowering --features vm-reference
python3 tools/checks/lib/normal_source_plan0_transaction_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
