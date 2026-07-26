---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN0-TX0-I0
Scope: atomic canonical Main source draft plus physical thunk candidate
ceremony_tier: T2 activation inside accepted NORMAL-CANONICAL-CORE0
series_mode: one atomic two-draft materialization/commit authority
sunset_id: NORMAL-CANONICAL-CORE0-PROOF-SUNSET-001
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
proof_inventory_before: passive two-row batch schema plus existing function-draft and module-shell proofs
new_proofs: one normal Main transaction fixture family plus one reusable Python transaction-guard helper
retired_or_merged_proofs: none in I0; merge into canonical-core route proof at G0
net_proof_delta: +2 bounded T2 proofs
sunset_budget: repay both additions in NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core production route owns the same two-draft correspondence, failure, reuse, and caller evidence
budget_repayment_evidence: route guard absorbs the transaction helper and disconnected-only fixtures have zero consumers
docs_only_closeout: forbidden
code_or_artifact_delta_required: 1
Related:
  - docs/development/current/main/investigations/normal-main0-f1-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-thunk0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-canonical-module-batch0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-module-tx0-l0-execution-task-2026-07-26.md
---

# NORMAL-MAIN0-TX0-I0

## Audit clarification

```text
Decision:
  NORMAL-MAIN0-TX0-RETENTION-prime-r1

Status:
  accepted

Choice:
  retained semantic evidence + restored Builder
  no recoverable source-plan owner
```

The pre-I0 audit found one real mismatch between the first card wording and
the existing canonical lowerer:

```text
CanonicalTrivialBindingSsaPlanV1
  -> consumed lowering parts
  -> success: verified unpublished MirFunction
  -> failure: restored/discarded function session + typed build error
```

The existing lowerer cannot return the original non-Clone plan after a
`SourceDraft` failure. Adding a recoverable/retryable lowering owner only to
reconstruct that plan would conflict with this card's own prohibition on
retry, resume, and source-plan recovery.

I0 therefore retains the durable evidence required to diagnose and discard a
failed transaction:

```text
exact source unit identity
two-row schema
source header
sealed result and entry relation
typed stage and nested cause
Builder/session restoration receipt
every draft already completed before the failure
```

It does not retain or reconstruct the consumed lowering plan. This is the
only weakened sentence; atomicity, no fallback, no retry, and same-Builder
reuse remain mandatory.

## Outcome

Consume one `PreparedNormalCanonicalModuleBatchV1` and produce one unpublished,
fully verified candidate module containing exactly:

```text
1. canonical source Main.main/0 draft
   key    = CanonicalResolvedOwner(owner)
   symbol = main/0

2. synthetic physical entry draft
   key    = Main
   symbol = main
   body   = exact call source Main.main/0
            + exact Return matching the sealed thunk result
```

Both drafts are prepared and verified before one atomic candidate commit.
Neither draft may be visible alone.

This row still does not publish the candidate to the caller, execute VM,
project process status, select a profile, or add a production/default route.

## Sole owner chain

```text
PreparedNormalCanonicalModuleBatchV1
  ↓ consume
OpenNormalMainModuleTransactionV1
  ↓ fallible preparation only
PreparedNormalMainModuleTransactionV1
  - opaque VerifiedNormalMainSourceDraftV1
  - verified physical thunk draft
  - exact two-row schema correspondence
  - collision/cardinality preflight
  ↓ one infallible commit
CompletedNormalMainModuleCandidateV1
  - unpublished candidate MirModule
  - exact entry relation
  - result/decode evidence
  - verification receipt
```

No second finalizer, collector path, or rollback clone is allowed.

## Source Main draft

Reuse the existing canonical chain:

```text
VerifiedNormalMainFunctionPlanV1
  -> CanonicalTrivialBindingSsaPlanV1
  -> existing resolved lowering
  -> PreparedFunctionDraftSealV1
  -> CompletedFunctionDraftV1
  -> verified detached MirFunction
  -> opaque VerifiedNormalMainSourceDraftV1
```

The I0 owner does not re-read AST, return annotation, terminal profile, or
completion coverage. It consumes the already sealed plan. The outer
transaction does not widen or expose `CompletedFunctionDraftV1`; it
immediately wraps the existing verified detached result in a normal-specific
opaque owner with no mutable draft escape.

Accepted source result carriers remain:

```text
Unit / Integer / Bool / Float
```

No direct call, helper, String function result, object/dynamic carrier,
multiple/nested/all-path Return, or cleanup is added.

## Physical thunk draft

The physical thunk is synthetic and has exactly one call target:

```text
VerifiedResolvedOwnerHeaderV1
```

It must not perform symbol lookup. The header supplies exact target symbol and
arity. The thunk's physical name/arity comes only from the opaque canonical
entry target.

Result relation:

```text
Unit:
  Call source Main
  discard/consume exact Unit according to the sealed call contract
  Return(Void/Unit)

Integer:
  Call source Main -> exact Integer
  Return same value

Bool:
  Call source Main -> exact Bool
  Return same value

Float:
  Call source Main -> exact Float
  Return same value
```

The physical thunk owns no process-exit policy. Bool/Float remain source
results and are rejected only by the later `ProcessExitProjectionV1`.

## Prepare/commit law

All fallible work occurs before candidate commit:

```text
source Main lowering
source Main draft sealing
physical thunk draft preparation
physical thunk verification
schema/draft correspondence
key/symbol/arity collision checks
entry relation checks
candidate module verification plan
```

Commit performs only already prepared moves:

```text
insert exact source Main draft
insert exact physical entry draft
install exact entry/result evidence
return completed unpublished candidate
```

No `Result`, lookup, inference, fallback, verification, or source observation
may occur after commit begins.

## Failure retention

```rust
pub(in crate::mir) enum NormalMainModuleTransactionStageV1 {
    SourceDraft,
    PhysicalThunk,
    BatchCorrespondence,
    CandidateVerification,
}

pub(in crate::mir) struct RejectedNormalMainModuleTransactionV1<'unit> {
    evidence: RetainedNormalMainTransactionEvidenceV1<'unit>,
    stage: NormalMainModuleTransactionStageV1,
    error: NormalMainModuleTransactionErrorV1,
    prepared: RetainedNormalMainPreparedDraftsV1,
    restoration: NormalMainBuilderRestorationReceiptV1,
}
```

Every rejection retains exact semantic evidence plus any unpublished prepared
drafts that existed before the failing stage. A `SourceDraft` rejection
retains no recoverable lowering plan; all later stages retain the completed
source draft. Public terminals are inspection and `discard(self)` only.

Forbidden:

```text
retry/resume
Raw or Legacy fallback
partial candidate return
source plan recovery
post-commit repair
```

Later success with the same `MirCompiler`/Builder context must remain possible
after every rejection.

## Fixture matrix

```text
success:
  empty/fallthrough Unit
  return;
  return void
  return null
  return Integer
  return Bool
  return Float
  :void + Unit
  :i64 + Integer

exact physical relation:
  source symbol main/0
  physical symbol main
  one physical Call
  one physical Return
  exact result/signature relation
  no symbol/module entry scan

failure:
  source draft failure
  physical thunk preparation failure
  key collision
  symbol collision
  arity drift
  source/physical relation drift
  candidate verification failure
  late second-draft failure publishes zero

reuse:
  success -> success
  rejection -> success
```

## Structural gate

```text
source Main draft writer                    = 1 existing canonical owner
physical Main thunk writer                  = 1
normal two-draft commit owner               = 1

source AST/result reclassification          = 0
Return scan / last ValueId inference        = 0
NYASH_ENTRY/module function scan            = 0
Raw Main slot/policy                        = 0

partial module publication                  = 0
post-commit fallible operation              = 0
fallback/retry                              = 0

VM/process/profile/runner consumer          = 0
all modified/new source/check files         < 800 lines
```

Extend the existing `normal-source-plan0` guard; do not add a row-specific
shell wrapper. The existing guard is already near the 800-line boundary, so
TX0 checks belong in one imported reusable Python helper rather than another
shell entry.

## Internal implementation order

```text
TX0-A EVIDENCE0
  consuming batch/thunk splits
  retained semantic evidence
  opaque verified source-draft wrapper

TX0-B PHYSICAL0
  exact one-block physical thunk
  sealed header/result/entry only
  completed typed-definition verification + full MirVerifier

TX0-C PREPARE0
  source draft then physical draft
  schema/key/symbol/arity/entry correspondence
  collision/cardinality and empty-shell drain preflight

TX0-D COMMIT0
  PreparedModuleLoweringShellDrainV1::commit_preflighted pattern
  ownership-only infallible candidate commit

TX0-E FAILURE-REUSE0
  four typed failure stages
  retained evidence/drafts by stage
  rejection -> later success on the same Builder

TX0-F G0
  focused fixture family
  imported Python transaction guard helper
  current pointer and proof-budget closeout
```

The following existing routes remain forbidden:

```text
root_batch / LegacyReplaceWholePair
Raw main pending draft
callable collector rejection that drops input drafts
DrainedModuleCandidateV1 root/condition policy
try_add_functions_atomic inside commit
```

## Acceptance

```bash
cargo check --lib
cargo test -q --lib mir::compiler::normal_source_plan
cargo test -q --lib mir::builder::normal_module_transaction
tools/checks/run_row_guard.sh --only normal-source-plan0
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Immediate continuation

```text
NORMAL-MAIN0-TX0-I0
-> SOURCE-ENTRY-VMREF-NEUTRAL0-L0
-> SOURCE-ENTRY-VMREF-RAW-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF-ADAPTER0-I0
-> NORMAL-MAIN0-VMREF0-P0
```

The Rust `MirInterpreter` remains the explicit semantic-reference executor.
Product/default backend selection remains the separate
`NORMAL-ENTRY-PRODUCT-BACKEND-D0` decision. No reference lane is retired until
the final explicit keep/retire decision after product cutover parity.

## Reconsult boundary

Reopen design if any of the following is required:

```text
source Main cannot use the existing draft-seal owner
physical thunk needs result inference outside the sealed thunk plan
two drafts cannot be prepared before any candidate mutation
existing collector requires Legacy replacement policy
commit cannot be made infallible
```

A missing narrow adapter/accessor or a private test fixture is not a design
conflict.

## Non-claims

```text
VM execution
process projection
publication to a runner
canonical-core profile/caller
default/product backend
helpers/direct calls
imports/using
JSON/LLVM/native
Legacy retirement
```
