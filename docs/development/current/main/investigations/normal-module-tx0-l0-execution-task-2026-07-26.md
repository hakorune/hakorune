---
Status: closed
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MODULE-TX0-L0
Scope: disconnected heterogeneous canonical normal-module transaction schema
ceremony_tier: T1 bounded owner extension inside accepted NORMAL-CANONICAL-CORE0
series_mode: BoxShape only; no lowering, publication, VM, profile, or caller activation
sunset_id: NORMAL-CANONICAL-CORE0-PROOF-SUNSET-001
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
Related:
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-f1-plan0-s0-execution-task-2026-07-26.md
  - src/mir/builder/module_draft_collector.rs
  - src/mir/builder/canonical_physical_drain.rs
  - src/mir/builder/resolved_lowering/callable_module_transaction.rs
---

# NORMAL-MODULE-TX0-L0

## Closeout

Closed on `main` with one builder-private passive schema:

```text
source Main row       = exactly 1
helper rows           = zero or more, canonical-key sorted
physical entry row    = exactly 1
entry relation        = exact row correspondence
canonical disposition = structurally sealed
Legacy policy         = unrepresentable

lowering/mutation/publication/runtime consumer = 0
```

Preparation owns and normalizes the proposed rows only after cardinality,
role/key, arity, key/symbol uniqueness, and entry-relation verification.
Failure retains the complete draft plus typed cause and exposes inspection plus
`discard(self)` only.

Evidence:

```text
mir::builder::normal_module_transaction = 4/4
cargo check --lib                       = green
normal-source-plan0 row guard           = green
current pointer / MIR root / VM0 route  = green
all touched source/check files          < 800 lines
```

## Outcome

Define one passive, move-only schema that can later own the complete
unpublished normal-module batch:

```text
source Main draft
+ zero or more canonical helper drafts
+ one synthetic physical main thunk
+ exact source-entry relation
  -> PreparedNormalModuleTransactionV1
  -> one future atomic candidate-module commit
```

L0 does not lower a function, construct a thunk, mutate a collector, create a
module, publish a draft, execute a VM, or add a production consumer.

## Why this row exists

The existing canonical physical vocabulary is split between:

```text
Single
Callable
```

The normal canonical-core route needs a heterogeneous batch containing:

```text
FunctionDraftKeyV1::CanonicalResolvedOwner(main_owner)
FunctionDraftKeyV1::CanonicalCallable(helper_key)*
FunctionDraftKeyV1::Main
```

Main must not be inserted into the helper catalog, and the Raw
`LegacyReplaceWholePair` Main policy must not be reused. The common schema is
defined before Main-only or callable lowering so both routes consume one
transaction authority.

## Reused authorities

```text
function identity:
  FunctionDraftKeyV1

completed function:
  CompletedFunctionDraftV1

collector:
  ModuleDraftCollectorV1

canonical admission:
  DraftPublicationPolicyV1::CanonicalRejectDuplicate

final module mutation:
  MirModule::try_add_functions_atomic
```

L0 may name these authorities and seal their correspondence. It must not
duplicate their key, symbol, arity, verification, collision, or commit rules.

## Minimum vocabulary

Conceptual shape:

```rust
pub(in crate::mir) enum NormalModuleDraftRoleV1 {
    SourceMain {
        owner: FunctionOwnerIdV1,
    },
    Helper {
        key: CanonicalCallableKeyV1,
    },
    PhysicalEntry,
}

pub(in crate::mir) struct NormalModuleDraftExpectationV1 {
    role: NormalModuleDraftRoleV1,
    key: FunctionDraftKeyV1,
    symbol: Box<str>,
    arity: usize,
}

pub(in crate::mir) struct SealedNormalModuleTransactionSchemaV1 {
    rows: Box<[NormalModuleDraftExpectationV1]>,
    source_entry: VerifiedNormalEntryRelationV1,
    _seal: SealedNormalModuleTransactionSchemaSealV1,
}
```

Names may vary to fit existing module boundaries. The laws may not:

```text
source Main row       = exactly 1
physical entry row    = exactly 1
helper rows           = zero or more
key/symbol pair       = unique
all arities           = exact
all publication policy= CanonicalRejectDuplicate
deterministic order   = source Main, helpers by canonical key, physical entry
Raw replacement policy= zero
```

The landed L0 vocabulary remains `builder`-private because
`FunctionDraftKeyV1` is itself a builder-private authority. Widening that key
only to expose a disconnected schema would create a private-interface drift.
The later activation row must expose a consuming facade, not the raw schema or
key vocabulary.

The schema carries `CanonicalInsertedDispositionV1`; therefore
`LegacyReplaceWholePair` is unrepresentable rather than accepted and rejected
at runtime.

The entry relation may initially be a passive exact identity product if its
later `NORMAL-MAIN0-THUNK0-S0` producer does not yet exist. It must not be
reconstructed from symbol text or module inventory.

## Failure retention

Preparation failure retains the complete proposed schema and typed cause:

```rust
pub(in crate::mir) enum NormalModuleTransactionSchemaErrorV1 {
    MissingSourceMain,
    DuplicateSourceMain,
    MissingPhysicalEntry,
    DuplicatePhysicalEntry,
    DuplicateKey(FunctionDraftKeyV1),
    DuplicateSymbol(Box<str>),
    RoleKeyMismatch,
    ArityMismatch,
    EntryRelationMismatch,
}
```

The rejection exposes inspection and `discard(self)` only.

Forbidden:

```text
retry
fallback
Legacy replacement
symbol-based role inference
module scan
partial collector mutation
partial module publication
```

## Fixture matrix

```text
success:
  Main + physical entry
  Main + one helper + physical entry
  Main + multiple helpers + physical entry
  declaration reorder -> same deterministic normalized schema

typed rejection:
  missing/duplicate Main
  missing/duplicate physical entry
  duplicate key
  duplicate symbol
  Main role with CanonicalCallable key
  helper role with Main/CanonicalResolvedOwner key
  physical role with non-Main key
  symbol/arity/entry-relation drift
  any LegacyReplaceWholePair policy
```

## File boundary

Prefer a bounded neutral module beside the existing collector:

```text
src/mir/builder/normal_module_transaction/
  mod.rs
  schema.rs
  rejection.rs
  tests.rs
```

Do not grow `module_draft_collector.rs`, `canonical_physical_drain.rs`, or
`compiler/mod.rs` with the schema implementation.

## Structural guard

Extend the existing `normal-source-plan0` family guard or add one manifest
entry to it; do not create a row-specific shell.

```text
normal-module transaction schema producer = 1
source Main role                           = 1
helper role                                = 1
physical entry role                        = 1
canonical publication policy               = 1
Legacy replacement policy consumer         = 0

MirBuilder/function lowering                = 0
collector/module mutation                   = 0
publication/VM/process/runner               = 0
production consumer                         = 0
fallback/retry                              = 0
all modified/new source/check files         < 800 lines
```

## Acceptance

```bash
cargo check --lib
cargo test -q --lib mir::builder::normal_module_transaction
tools/checks/run_row_guard.sh --only normal-source-plan0
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Immediate continuation

```text
NORMAL-MODULE-TX0-L0
-> NORMAL-MAIN0-THUNK0-S0
-> NORMAL-CANONICAL-MODULE-BATCH0-S0
-> NORMAL-MAIN0-TX0-I0
```

The later callable route reuses the same schema:

```text
NORMAL-CALLABLE-SOURCE0-S0
-> NORMAL-MAIN-DIRECT-CALL0-S0
-> NORMAL-HELPER-MODULE-PLAN0-S0
-> NORMAL-CALLABLE-MODULE0-A0-S0
-> NORMAL-CALLABLE-MODULE0-TX0-S0
```

Recursive SCC activation remains after the first acyclic canonical-core
production route and is not a blocker for the first vertical slice.

## Non-claims

```text
function draft lowering
physical thunk construction
collector admission/commit
module publication
VM execution/process projection
Main direct calls
helper catalog or graph activation
profile admission/dispatch
CLI/default caller
imports/using
recursive SCC activation
```
