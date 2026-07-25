---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-SOURCE-PLAN0-S0
Scope: disconnected source-owned normal family classifier
ceremony_tier: T2 new source authority
sunset_id: NORMAL-SOURCE-PLAN0-PROOF-SUNSET-001
proof_inventory_before: accepted D0 decision and existing normal-file/Raw source fixtures
new_proofs: one source-family classifier fixture matrix and one reusable family guard
retired_or_merged_proofs: none in S0
net_proof_delta: positive one bounded T2 scaffold
sunset_budget: one disconnected source-plan proof
sunset_row: NORMAL-SOURCE-PLAN0-G0
retire_when: canonical-core caller is live and the normal route guard absorbs durable classifier assertions
budget_repayment_evidence: NORMAL-FILE-CANONICAL-CORE0-G0 with disconnected proof consumer zero
Related:
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-file-vm0-frontdoor-forge-task-2026-07-26.md
---

# NORMAL-SOURCE-PLAN0-S0

## Outcome

Build one disconnected, source-only classifier:

```text
owned parsed AST
  -> NormalSourceSurfaceInventoryV1
  -> NormalSourcePlanClassifierV1
  -> SealedNormalSourcePlanV1
```

This row is Builder-free, MIR-free, backend-free, runner-free, and
profile-admission-free.

## Products

```rust
pub(crate) struct PreparedNormalSourcePlanInputV1 { /* owned AST + identity */ }

pub(crate) enum SealedNormalSourcePlanV1 {
    ScalarRoot(SealedNormalScalarRootV1),
    CallableModule(SealedNormalCallableModuleSourceV1),
}

pub(crate) enum SealedNormalScalarRootV1 {
    Script(SealedNormalScriptSourceV1),
    Main0(SealedNormalMainSourceV1),
}

pub(crate) struct RejectedNormalSourcePlanV1 {
    owner: PreparedNormalSourcePlanInputV1,
    stage: NormalSourcePlanStageV1,
    error: NormalSourcePlanErrorV1,
}
```

The source input does not carry a second profile authority. The existing
`SealedNormalEntryProfileV1` stays outside the classifier and will be consumed
by `NORMAL-SOURCE-PLAN0-ADMISSION0-S0`.

Normal site vocabulary is source-only:

```text
NormalTopLevelSiteV1:
  top-level statement index

NormalMainMethodSiteV1:
  Main top-level site
  sorted method key
  declared arity
```

It does not carry a physical symbol, `ValueId`, `MirType`, or callable-catalog
identity.

## File split

```text
src/mir/compiler/normal_source_plan/
  mod.rs
  product.rs
  inventory.rs
  classifier.rs
  rejection.rs
  tests.rs
```

`src/mir/compiler/mod.rs` receives only the module declaration. It is already
near the file-size boundary and must not receive implementation or fixture
logic.

Suggested limits:

```text
mod.rs        < 120
product.rs    < 220
inventory.rs  < 220
classifier.rs < 280
rejection.rs  < 160
tests.rs      < 320
guard         < 260
```

Every modified/new source or check file remains below 800 lines.

## Exact fixture matrix

S0 unit fixtures exercise the classifier without file I/O or the front door:

| Source | Expected |
| --- | --- |
| empty Program | `ScalarRoot::Script` |
| scalar Script | `ScalarRoot::Script` |
| static `Main.main/0` only | `ScalarRoot::Main0` |
| Main plus top-level function | `CallableModule` |
| Main plus Main-box helper | `CallableModule` |
| function only | `MissingSourceEntry` |
| Script plus Main | `MixedSourceFamilies` |
| Script plus function | `MixedSourceFamilies` |
| two Main boxes | `DuplicateMain` |
| instance Main | `MainMustBeStatic` |
| Main without main | `MainMethodMissing` |
| `Main.main/1` | `MainArityMismatch` |
| unsupported declaration | `UnsupportedTopLevelSurface` |
| non-Program root | `RootNotProgram` |

Add permutation witnesses for mixed-family and duplicate-Main ordering.
Main-box helper keys are sorted before sealing. Same-box duplicate method
spelling remains parser responsibility and is not claimed by S0.

## Rejection law

S0 errors are source-only:

```text
RootSurface
SourceEntry
FamilyClosure
```

The rejection owner exposes only:

```text
stage()
error()
discard(self)
```

It has no `into_owner`, `resume`, `retry`, `reclassify`, fallback, profile
selection, or Legacy terminal.

## Structural gate

Use one reusable manifest-backed family guard:

```text
tools/checks/run_row_guard.sh --only normal-source-plan0
```

The implementation commit may add:

```text
tools/checks/lib/normal_source_plan0_guard.py
tools/checks/guard_rows.toml entry: normal-source-plan0
```

Do not add a per-subrow shell guard. Add the check index entry only after the
guard exists.

The guard fixes:

```text
classifier definition/producer             = 1
sealed plan definition/producer            = 1
ScalarRoot / CallableModule variants       = 1 each
production consumer                        = 0
profile match in classifier                = 0
Builder/MIR/runtime/runner/backend imports = 0
AST clone/rewrite/source rewrite           = 0
module/function symbol scan                = 0
retry/fallback/reclassification            = 0
Clone/Copy on owner products               = 0
all touched source/check files             < 800 lines
```

Avoid a blanket `.clone()` text ban; target AST/source cloning and rewrite
constructors so site-string operations do not produce false positives.

## Acceptance

```bash
cargo check --lib
cargo test -q --lib mir::compiler::normal_source_plan
tools/checks/run_row_guard.sh --only normal-source-plan0
bash tools/checks/current_state_pointer_guard.sh
```

## Immediate continuation

```text
NORMAL-SOURCE-PLAN0-S0
-> NORMAL-SOURCE-PLAN0-INPUT0-S0
-> NORMAL-SOURCE-PLAN0-G0
```

INPUT0 adds one consuming front-door projection without a bare AST accessor.
It does not alter the existing narrow profile's production terminal.

## Non-claims

```text
profile admission
normal production consumer
Main/function lowering
callable catalog construction
entry thunk/publication
VM/process execution
existing VM0 route change
default route change
imports/dynamic result support
```
