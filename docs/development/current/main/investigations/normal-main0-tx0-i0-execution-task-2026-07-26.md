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
Related:
  - docs/development/current/main/investigations/normal-main0-f1-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-thunk0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-canonical-module-batch0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-module-tx0-l0-execution-task-2026-07-26.md
---

# NORMAL-MAIN0-TX0-I0

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
  - CompletedFunctionDraftV1 source Main
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
```

The I0 owner does not re-read AST, return annotation, terminal profile, or
completion coverage. It consumes the already sealed plan.

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
    owner: OpenNormalMainModuleTransactionV1<'unit>,
    stage: NormalMainModuleTransactionStageV1,
    error: NormalMainModuleTransactionErrorV1,
}
```

Every rejection retains the complete open batch plus any unpublished prepared
drafts. Public terminals are inspection and `discard(self)` only.

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
shell wrapper.

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
