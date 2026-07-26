---
Status: closed
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-CANONICAL-MODULE-BATCH0-S0
Scope: disconnected canonical Main-only pre-draft batch manifest
ceremony_tier: T1 bounded BoxShape inside accepted NORMAL-CANONICAL-CORE0
series_mode: BoxShape only; no new accepted source/result shape
sunset_id: NORMAL-CANONICAL-CORE0-PROOF-SUNSET-001
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
Related:
  - docs/development/current/main/investigations/normal-main0-thunk0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-module-tx0-l0-execution-task-2026-07-26.md
  - src/mir/compiler/normal_source_plan/
  - src/mir/builder/normal_module_transaction/
---

# NORMAL-CANONICAL-MODULE-BATCH0-S0

## Outcome

Consume one closed `VerifiedNormalMainThunkPlanV1` into one builder-private,
pre-draft batch manifest which co-owns:

```text
exact source Main semantic plan
exact source header/result relation
exact canonical physical-entry target
exact NormalModuleTransactionSchemaV1
```

Main-only S0 has exactly two expected rows:

```text
source Main:
  key    = CanonicalResolvedOwner(source owner)
  symbol = existing source header symbol
  arity  = existing source header arity (0)

physical entry:
  key    = Main
  symbol = sealed canonical physical target
  arity  = sealed canonical physical target (0)
```

S0 does not lower either function, construct a `MirFunction`, open a Builder
session, mutate a collector/module, publish, execute, select a profile, or add
a caller.

## Why this row is pre-draft

`NORMAL-MAIN0-THUNK0-S0` closed semantic identity/result authority.
`NORMAL-MODULE-TX0-L0` closed passive role/key/symbol/arity validation.
Neither row owns physical drafts.

Therefore this row seals only their exact correspondence:

```text
semantic thunk plan
  -> deterministic transaction manifest
  -> retained batch owner
```

Actual source-Main lowering, physical thunk MIR, and atomic candidate commit
belong together in the later `NORMAL-MAIN0-TX0-I0`. Creating partial drafts in
this S0 would introduce a second transaction authority.

## Product

Conceptual shape:

```rust
pub(in crate::mir::builder) struct PreparedNormalCanonicalModuleBatchV1<'unit> {
    thunk: VerifiedNormalMainThunkPlanV1<'unit>,
    schema: NormalModuleTransactionSchemaV1,
    _seal: PreparedNormalCanonicalModuleBatchSealV1,
}
```

The sole constructor consumes the thunk plan:

```rust
NormalCanonicalModuleBatchV1::prepare(
    thunk: VerifiedNormalMainThunkPlanV1<'unit>,
) -> Result<
    PreparedNormalCanonicalModuleBatchV1<'unit>,
    RejectedNormalCanonicalModuleBatchV1<'unit>,
>
```

No terminal may expose a bare source plan, bare AST, mutable schema, or caller-
supplied row vector. The later I0 terminal consumes the complete batch.

## Authority projection

The batch owner may read only:

```text
thunk.source_header().owner()
thunk.source_header().symbol()
thunk.source_header().arity()
thunk.entry().physical target identity
thunk.source_result()
```

The result is retained for the later physical thunk builder, but it does not
change the transaction schema's identity laws.

Forbidden authority:

```text
AST/source text
last ValueId
physical Return scan
MirType inference
module function inventory
the string "main" in the batch owner
NYASH_ENTRY
RawMainEntryTargetV1
RawRootBatchSlotV1
LegacyReplaceWholePair
```

The batch owner never reconstructs physical identity. It consumes the sealed
canonical target carried by the thunk plan.

## Failure retention

```rust
pub(in crate::mir::builder) struct RejectedNormalCanonicalModuleBatchV1<'unit> {
    owner: VerifiedNormalMainThunkPlanV1<'unit>,
    error: NormalCanonicalModuleBatchErrorV1,
}

pub(in crate::mir::builder) enum NormalCanonicalModuleBatchErrorV1 {
    Schema(NormalModuleTransactionSchemaErrorV1),
    SourceRelationDrift,
    PhysicalRelationDrift,
    CardinalityDrift,
}
```

The rejection surface is:

```text
error()
discard(self)
```

There is no `into_owner`, retry, resume, Raw conversion, or Legacy fallback.
The original complete thunk plan remains owned until discard.

## Fixture matrix

```text
success:
  Unit Main thunk
  Integer Main thunk
  Bool Main thunk
  Float Main thunk
  exact deterministic order: source Main, physical entry

typed invariant:
  source owner/symbol/arity drift
  physical symbol/arity drift
  missing/duplicate source Main
  missing/duplicate physical entry
  role/key mismatch
  symbol collision
  retained owner after rejection

structural:
  batch producer = 1
  schema producer remains 1
  caller-supplied row vector = 0
  draft/MIR/collector mutation = 0
```

Malformed identity fixtures must use private test-only schema seams. Production
constructors remain unforgeable.

## File boundary

Keep the batch owner beside the passive schema:

```text
src/mir/builder/normal_module_transaction/
  canonical_batch.rs
  canonical_batch_tests.rs
```

Do not place compiler source semantics in the collector or add another module
transaction directory.

## Guard

Extend the reusable `normal-source-plan0` family guard:

```text
canonical batch producer                     = 1
thunk-plan consumer                          = 1
transaction-schema consumer                  = 1

AST/source reclassification                  = 0
Raw/Legacy entry identity consumer           = 0
MirBuilder/MirFunction/MirInstruction        = 0
collector/module mutation                    = 0
publication/VM/process/runner                = 0
fallback/retry                               = 0
all modified/new source/check files          < 800 lines
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
NORMAL-CANONICAL-MODULE-BATCH0-S0
-> NORMAL-MAIN0-TX0-I0
-> SOURCE-ENTRY-VMREF-NEUTRAL0-L0
```

`NORMAL-MAIN0-TX0-I0` is the sole row allowed to materialize the source Main
draft and physical thunk draft, verify both, and commit the complete two-row
candidate atomically.

## Reconsult boundary

Reopen design only if the complete thunk plan cannot be projected into the
existing passive schema without:

```text
re-observing source/MIR/runtime state
exposing FunctionDraftKeyV1 outside the builder boundary
reusing Raw/Legacy Main policy
or inventing a second batch/collector transaction
```

A missing narrow accessor on the sealed thunk plan or passive schema is an
implementation seam, not a design conflict.

## Implementation closeout

Closed on 2026-07-26.

```text
canonical batch producer =
  1

input =
  one consumed VerifiedNormalMainThunkPlanV1

output =
  one PreparedNormalCanonicalModuleBatchV1
  + one sealed two-row NormalModuleTransactionSchemaV1

row order =
  source Main
  physical entry

result coverage =
  Unit / Integer / Bool / Float

retained malformed-schema rejection =
  green

MirFunction / collector / module mutation / publication / VM =
  0
```

Focused passive transaction fixtures are 7/7 green. The reusable family guard,
`cargo check --lib`, and the below-800 boundary are green.

## Non-claims

```text
source Main lowering
physical thunk MIR emission
CompletedFunctionDraftV1 production
collector/module mutation
atomic candidate commit
publication
VM execution/process projection
profile admission
production/default caller
helpers/callable module
imports/JSON/LLVM/native
Legacy retirement
```
