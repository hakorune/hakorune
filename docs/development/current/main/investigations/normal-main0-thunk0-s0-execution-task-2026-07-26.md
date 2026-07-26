---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN0-THUNK0-S0
Scope: disconnected exact source-Main to physical-main thunk plan
ceremony_tier: T1 bounded capability inside accepted NORMAL-CANONICAL-CORE0
series_mode: BoxCount only; one physical-entry thunk shape
sunset_id: NORMAL-CANONICAL-CORE0-PROOF-SUNSET-001
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
Related:
  - docs/development/current/main/investigations/normal-main0-f1-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-module-tx0-l0-execution-task-2026-07-26.md
  - src/mir/compiler/normal_source_plan/
  - src/mir/compiler/capability/resolved_owner_header.rs
  - src/mir/builder/normal_module_transaction/
---

# NORMAL-MAIN0-THUNK0-S0

## Outcome

Seal one exact relation between the already verified canonical source
`Main.main/0` function and one future synthetic physical VM entry:

```text
VerifiedNormalMainFunctionPlanV1
  -> existing VerifiedResolvedOwnerHeaderV1
  -> exact source result/representation relation
  -> VerifiedNormalMainThunkPlanV1
       source key    = CanonicalResolvedOwner(owner)
       source symbol = existing canonical owner header
       source arity  = 0
       physical key  = Main
       physical symbol/arity = sealed physical entry identity
       call target   = exact source header, never symbol lookup
```

S0 is disconnected. It does not lower the source Main, emit a Call or Return,
construct a `MirFunction`, mutate the normal-module schema, publish a module,
execute a VM, or add a runner/profile consumer.

## Authority boundaries

Reuse:

```text
source function semantics:
  VerifiedNormalMainFunctionPlanV1
  CanonicalTrivialBindingSsaPlanV1

source identity:
  VerifiedResolvedOwnerHeaderV1

source result:
  VerifiedFunctionCompletionV1
  SealedFunctionExitContractV1
  existing trivial representation profile

future transaction identity:
  FunctionDraftKeyV1::CanonicalResolvedOwner
  FunctionDraftKeyV1::Main
  NormalModuleEntryRelationV1
```

Do not derive authority from:

```text
the string "main"
module function inventory
NYASH_ENTRY
VMValue
last ValueId
physical Return scan
Raw Main slot/policy
```

The source symbol comes from `VerifiedResolvedOwnerHeaderV1`; it is not rebuilt
from the AST name in the thunk owner.

## First thunk shape

```text
source Main:
  static
  arity 0
  exact sealed Main role
  direct calls 0

physical entry:
  synthetic
  arity 0
  one exact call target
  one exact completion matching the source result representation

supported result relation:
  Unit  -> physical Unit/Void return contract
  i64   -> exact Integer
  Bool  -> exact Bool
  Float -> exact Float
```

String, object/dynamic carriers, multiple/nested returns, helpers, and direct
source calls remain typed exclusions. S0 does not widen the F1 matrix.

## Product

Conceptual shape:

```rust
pub(crate) struct VerifiedNormalMainThunkPlanV1<'unit> {
    source: VerifiedNormalMainFunctionPlanV1<'unit>,
    source_header: VerifiedResolvedOwnerHeaderV1,
    source_result: VerifiedNormalMainThunkResultV1,
    entry: VerifiedNormalMainEntryRelationV1,
    _seal: VerifiedNormalMainThunkPlanSealV1,
}

pub(crate) enum VerifiedNormalMainThunkResultV1 {
    Unit,
    Integer,
    Bool,
    Float,
}
```

The exact result vocabulary must be derived from the sealed F1/value-profile
product. A second AST classifier or `MirType` guess is forbidden.

## Failure retention

```rust
pub(crate) enum NormalMainThunkPlanErrorV1 {
    Header(ResolvedOwnerHeaderSealErrorV1),
    SourceRoleMismatch,
    SourceArityMismatch,
    UnsupportedResultCarrier,
    CompletionRepresentationMismatch,
    EntryRelationMismatch,
}
```

A rejected owner retains the complete Main F1 plan and typed cause. It exposes
inspection and `discard(self)` only. No role/profile retry or Raw fallback is
allowed.

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

typed rejection / invariant:
  foreign role/header
  nonzero source/physical arity
  source/header owner mismatch
  unsupported carrier
  completion/profile disagreement
  source/physical relation drift

structural:
  exact source header producer = 1
  physical entry plan producer = 1
  symbol/module/NYASH entry scan = 0
  MIR writer / VM consumer = 0
```

## File boundary

Keep the compiler-side semantic plan beside the Main owner:

```text
src/mir/compiler/normal_source_plan/
  main_thunk_plan.rs
  main_thunk_plan_tests.rs
```

If the physical entry identity needs a neutral vocabulary, add one bounded
file beside `normal_module_transaction`; do not put source semantics in the
builder collector.

## Guard

Extend the existing `normal-source-plan0` family guard. No row-specific shell.

```text
Main thunk plan producer                  = 1
existing resolved-owner header consumer  = 1
existing F1/result authority consumer    = 1

AST result reclassification              = 0
symbol/module/environment entry inference= 0
MirBuilder/MirInstruction/ValueId        = 0
module mutation/publication              = 0
VM/process/runner consumer               = 0
fallback/retry                           = 0
all modified/new source/check files      < 800 lines
```

## Acceptance

```bash
cargo check --lib
cargo test -q --lib mir::compiler::normal_source_plan
tools/checks/run_row_guard.sh --only normal-source-plan0
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Immediate continuation

```text
NORMAL-MAIN0-THUNK0-S0
-> NORMAL-CANONICAL-MODULE-BATCH0-S0
-> NORMAL-MAIN0-TX0-I0
```

`NORMAL-CANONICAL-MODULE-BATCH0-S0` connects completed source/thunk drafts to
the already closed passive transaction schema. It remains unpublished until
the later atomic I0 terminal.

## Reconsult boundary

Reopen design only if the existing sealed Main F1/value-profile products cannot
determine the physical result relation without inspecting AST, MIR, or runtime
values. A missing accessor is an implementation seam, not a design conflict.

## Non-claims

```text
source Main lowering
physical thunk MIR emission
module collector/batch mutation
atomic publication
VM execution/process projection
helpers/Main direct calls
profile admission/dispatch
CLI/default caller
imports/using
dynamic/object result
```
