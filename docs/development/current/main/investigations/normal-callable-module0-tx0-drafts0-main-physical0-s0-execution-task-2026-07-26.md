---
Status: active executable row
Date: 2026-07-26
Decision: NORMAL-CALLABLE-MODULE0-TX0-DRAFT-FAILURE-prime-r1
Row: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-MAIN-PHYSICAL0-S0
Parent: NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-S0
Scope: consume one retained Main proof, prepare one source-Main draft and one physical entry-thunk draft, retaining the helper prefix on every later rejection
ceremony_tier: T1 bounded owner/evidence refactor
series_mode: BoxShape only; no accepted source/result shape grows
Related:
  - docs/development/current/main/investigations/normal-callable-module0-tx0-drafts0-helper-prefix0-s0-execution-task-2026-07-26.md
  - src/mir/builder/normal_module_transaction/
  - src/mir/compiler/normal_source_plan/normal_callable_transaction_handoff.rs
---

# NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-MAIN-PHYSICAL0-S0

## Outcome

Extend the completed helper prefix with the one source Main draft and the one
synthetic physical entry thunk, without creating a batch or publishing a
module:

```text
PreparedNormalHelperDraftPrefixV1
  -> consume Main lowering proof once
  -> exact source Main draft
  -> exact physical main thunk draft
  -> retained Helpers + Main + Physical drafts
     or a typed rejection retaining every earlier draft
```

## Authority and order

```text
helper prefix       = prior row's immutable completed prefix
Main source proof   = OpenNormalCallableModuleTransactionV1::take_main_lowering_proof()
source Main input   = retained source authority borrows it once
source Main lowerer = existing F1 prepared draft-seal chain
physical thunk      = existing normal physical-thunk owner
```

Two bounded bridges are required before the lowerer is called:

```text
consumed Main proof + borrowed exact Main input
  -> one CanonicalTrivialBindingSsaPlanV1
  (bind already-sealed facts; do not rerun CanonicalLoweringPreflightV1)

sealed Main header + completion + terminal profile
  -> reusable physical relation
  (factor the existing thunk-plan relation; do not rederive result/entry facts
   inside the transaction)
```

The Main-only legacy transaction is not a reusable owner here because it also
owns schema, shell, candidate verification, and publication preparation.

No second Main classifier, function completion verifier, entry selector,
signature inference, or physical-symbol inference may be introduced. The
already-sealed Main-to-physical relation is the only entry authority.

## Failure retention

```text
Main source draft failure:
  helper prefix retained
  Main proof consumed and represented only by typed failure evidence
  physical thunk = not attempted

physical thunk failure:
  helper prefix + verified source Main draft retained
  no module batch or publication
```

Each lowerer rejection must retain its typed stage directly. The rejection
owner exposes inspection and `discard(self)` only. It has no retry/resume,
source reconstruction, profile reselection, or legacy fallback terminal.

## Explicit non-authority

```text
batch/schema/correspondence/full verification = 0
candidate-module insertion/publication          = 0
helper schedule re-consumption                  = 0
new source/result/entry semantics               = 0
runner/backend/process changes                  = 0
```

## Focused fixtures

```text
helper prefix + Main success:
  source Main and physical draft exact identities
  helper prefix unchanged

Main failure:
  helpers retained, physical = 0, exact stage retained

physical failure:
  helpers + source Main retained, exact stage retained

failure -> later success:
  same Builder context restored
  partial module publication = 0
```

## Acceptance

```text
one Main proof consumer                         = 1
one source-Main draft owner                     = existing F1 only
one physical thunk owner                        = existing owner only
helper schedule re-consumption                  = 0
batch/publication caller                        = 0
retry/resume/fallback                           = 0
all modified/new source/check files            < 800 lines
```

Run:

```bash
cargo check --lib --features vm-reference
cargo test -q --lib normal_module_transaction --features vm-reference
python3 tools/checks/lib/normal_source_plan0_transaction_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next rows

```text
NORMAL-CALLABLE-MODULE0-TX0-DRAFTS0-MAIN-PHYSICAL0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-BATCH0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-COMMIT0-S0
  -> NORMAL-CALLABLE-MODULE0-TX0-G0
```
