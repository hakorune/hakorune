---
Status: Active task
Date: 2026-07-20
Scope: behavior-preserving LocalSSA post-success metadata split
Related:
  - docs/development/current/main/investigations/mirbuilder-clean-architecture-consolidation-task-2026-07-19.md
  - docs/development/current/main/investigations/mirbuilder-copy-unknown-origin-consultation-2026-07-20.md
  - src/mir/builder/ssa/local.rs
  - src/mir/builder/calls/unified_emitter/temporal_witness_tests.rs
---

# COPY-UNKNOWN0: behavior-preserving LocalSSA metadata split

## Decision

Candidate C′ is selected. One successful LocalSSA materialization has four
separate lanes, while preserving every currently observable destination state.

```text
successful LocalSSA materialization
  -> exact type-fact transfer
  -> legacy Unknown-entry compatibility
  -> origin transfer
  -> receiver-only Box fallback decision
```

`Unknown` is not an exact FACT0 type fact. It is retained only as an explicit
legacy compatibility entry until its separate retirement row. In particular:

```text
source type = StoredUnknown
source origin = Owner
kind = Recv

destination type = Unknown
destination origin = Owner
receiver Box(Owner) fallback = suppressed
```

The suppression comes from a named receiver-compatibility decision, not from
an accidental `Unknown` map write preceding a generic origin fallback.

## Fixed task order

```text
COPY-UNKNOWN0-S0
  -> COPY-UNKNOWN0-M0
  -> COPY-UNKNOWN0-P0
  -> COPY-UNKNOWN0-I0
  -> COPY-UNKNOWN0-G0

  -> COPY0-S0
  -> COPY0-P0
  -> COPY0-I0
  -> COPY0-G0
```

`COPY-UNKNOWN0-S0` is the sole next code-facing row. `COPY0-S0` remains
forbidden until `COPY-UNKNOWN0-G0` is green.

## Durable vocabulary

S0 adds private, map-free vocabulary under `src/mir/builder/ssa/local/` so the
existing 718-line `local.rs` does not grow into another mutable-world owner.
Suggested physical home:

```text
src/mir/builder/ssa/local/post_success.rs
```

The module may define only these structural products and pure decisions:

```rust
enum LocalSsaSourceTypeEntryV1 {
    Missing,
    StoredUnknown,
    Exact(MirType),
}

enum LocalSsaMaterializationKindV1 {
    RematerializedConst,
    RematerializedBinOp,
    RematerializedCompare,
    RematerializedSelect,
    PhysicalCopy(LocalSsaPhysicalCopyReasonV1),
}

enum ReceiverOriginCompatibilityV1 {
    Inactive,
    PublishBoxFromMissingType { owner: String },
    SuppressedByStoredUnknown { owner: String },
    SuppressedByExactType { owner: String },
}

struct PreparedLocalSsaPostSuccessV1 {
    materialization: LocalSsaMaterializationKindV1,
    exact_type: Option<MirType>,
    legacy_unknown: bool,
    origin: Option<String>,
    receiver_compat: ReceiverOriginCompatibilityV1,
}
```

`Missing`, stored `Unknown`, and every other `MirType` (including `Void`) are
classified once. `exact_type` and `legacy_unknown` are mutually exclusive.
The product is a prepared decision, not a second `ValueId` fact map.

## Behavior-preserving matrix

| Source type | Origin | Kind | Required destination state |
| --- | --- | --- | --- |
| missing | missing | Arg | no type, no origin |
| exact `T` | missing | Arg | `T` |
| Unknown | missing | Arg | Unknown |
| missing | Owner | Arg | origin only |
| Unknown | Owner | Arg | Unknown + origin |
| exact `T` | Owner | Arg | `T` + origin |
| missing | Owner | Recv | `Box(Owner)` + origin |
| Unknown | Owner | Recv | Unknown + origin |
| exact `T` | Owner | Recv | `T` + origin |
| missing | Owner | FieldBase | origin only |
| Unknown | Owner | FieldBase | Unknown + origin |

`FieldBase` shares a cache tag with `Recv`, but it must not acquire receiver
Box synthesis. The current equality check on `LocalKind::Recv` remains the
sole fallback gate.

## Transaction and failure law

Every selected checked materialization follows this order:

```text
1. snapshot/classify source type entry and origin
2. prepare exact/Unknown/origin/receiver decision
3. allocate destination ValueId
4. emit or rematerialize the instruction
5. after success only: commit exact type, legacy Unknown, origin, receiver fallback
6. insert the LocalSSA cache entry
```

Commit is non-fallible. Fresh-destination conflict validation belongs before
emission. An emission failure commits none of these:

```text
instruction
exact destination type
Unknown destination entry
destination origin
receiver Box fallback
LocalSSA cache
```

ValueId cursor rollback is not claimed. Existing `ensure()` continues its
legacy best-effort behavior; a new checked core may return its typed error, but
selected COPY0 consumers may not turn that error into an original-ValueId
fallback.

## Row contracts

### `COPY-UNKNOWN0-S0` — closed (2026-07-20)

```text
production behavior delta = 0
production consumers = 0
MirBuilder parameter = 0
type/origin/cache writes = 0
```

Implement the pure vocabulary and decision tests only. It must not inspect
source syntax, runtime tags, final metadata, or environment configuration.

Closed evidence:

```text
new owner:
  src/mir/builder/ssa/local/post_success.rs

source state:
  Missing | StoredUnknown | Exact(MirType), including Exact(Void)

receiver law:
  Missing + owner + Recv -> named Box(owner) publication decision
  StoredUnknown + owner + Recv -> named suppression decision
  FieldBase -> no receiver fallback

materialization vocabulary:
  physical Copy distinct from Const/BinOp/Compare/Select rematerialization

production connection / Builder / fact-map / origin / cache writes:
  0

focused proof:
  cargo test -q --lib post_success

behavior-parity proof:
  cargo test -q --lib local_statement_parity
```

`COPY-UNKNOWN0-M0` is now the sole next row. It must inventory the actual six
materialization outcomes and extract the checked-core boundary without wiring
this prepared product into production.

### `COPY-UNKNOWN0-M0` — closed (2026-07-20)

Inventory the six actual materialization outcomes and extract the checked-core
boundary without connecting the prepared product. Classify existing callers of
`ensure`/`try_ensure`; retain legacy facade behavior exactly.

Closed inventory:

| Successful LocalSSA outcome | Existing `ensure_inner` arm | COPY0 status |
| --- | --- | --- |
| rematerialized Const | `MirInstruction::Const` | parked non-Copy |
| rematerialized BinOp | `MirInstruction::BinOp` | parked non-Copy |
| rematerialized Compare | `MirInstruction::Compare` | parked non-Copy |
| rematerialized Select | `MirInstruction::Select` | parked non-Copy |
| rematerialized Copy | `MirInstruction::Copy` | future PhysicalCopy |
| dominating fallback Copy | wildcard arm after the existing dominance check | future PhysicalCopy |

There are no other post-success paths in `ensure_inner`; all six converge on
the present direct metadata/cache block. `schedule::block` owns separate
after-PHI/before-call materialization and remains outside this LocalSSA row.

Caller classification is also fixed:

```text
legacy best-effort facade:
  ensure
  recv / arg / cond / field_base / cmp_operand
  MirBuilder local_* convenience wrappers
  raw method receiver, binary, scheduler, and qmark callers

strict error-propagating facade:
  try_ensure
  finalize_args
  GenericLoop exit-edge, body Copy, BinOp, and effect-emission callers

not a LocalSSA caller:
  schedule::block direct Copy/rematerialization
```

The extracted checked-core boundary is deliberately distinct from the existing
`try_ensure` spelling. Today `try_ensure` propagates only strict
non-rematerializable/dominance errors: block-creation failure and instruction
emission failure remain legacy `Ok(original ValueId)` outcomes. Therefore I0
must introduce a private `try_materialize_local_v1`-style core and keep both
existing facades on their current best-effort behavior. The future checked
consumer may receive the new typed emission error directly, but it must not
convert that error back into an original-ValueId fallback. No production
consumer is added in M0, and `PreparedLocalSsaPostSuccessV1` remains
unconnected.

Evidence:

```text
source inventory:
  src/mir/builder/ssa/local.rs::ensure_inner

caller inventory:
  src/mir/builder/ssa/local/finalize.rs
  src/mir/builder/control_flow/plan/lowerer/{body_processing,effect_emission,exit_lowering}.rs
  src/mir/builder/{builder_emit,receiver,ops,utils/local_ssa}.rs

excluded direct owner:
  src/mir/builder/schedule/block.rs
```

`COPY-UNKNOWN0-P0` is now the sole next row. It must prove the complete pure
decision and synthetic transaction matrix before I0 changes the existing
post-success block.

### `COPY-UNKNOWN0-P0` — closed (2026-07-20)

Prove pure decision and synthetic transaction parity for the table above,
physical Copy and non-Copy materialization families, cache hit/miss, and
failure isolation. Downstream observations must retain stored-Unknown presence:
CopyTypePropagator, PHI diagnostics, same-root receiver proof, type hints, and
receiver route behavior.

Closed proof:

```text
pure synthetic commit:
  full Missing / StoredUnknown / Exact / origin / Recv / FieldBase matrix
  exact and Unknown lanes are mutually exclusive
  failure before commit leaves type/origin/cache empty
  cache hit leaves existing metadata untouched
  Const/BinOp/Compare/Select/Copy/fallback-Copy share the same prepared law

actual observation:
  a method receiver whose source type entry is StoredUnknown and whose origin
  is the method owner materializes through LocalSSA Recv as:
    emitted Copy
    destination type = Unknown
    destination origin = owner
    no Box(owner) overwrite

downstream preservation boundary:
  the destination remains a present Unknown entry; this row changes neither
  CopyTypePropagator, PHI publication, same-root provenance, type hints, nor
  receiver routing.
```

Focused evidence:

```text
cargo test -q --lib post_success
cargo test -q --lib temporal_witness
cargo test -q --lib local_statement_parity
```

`COPY-UNKNOWN0-I0` is now the sole next row. It may replace only the current
LocalSSA post-success metadata block with one prepared-then-commit owner; it
must retain every successful destination state proven here and leave all six
materialization families outside COPY0 except the two explicitly classified
physical-Copy variants.

### `COPY-UNKNOWN0-I0` — closed (2026-07-20)

One prepared decision now snapshots the source type entry and origin before
LocalSSA reserves its destination, classifies the materialization definition,
and commits only after the instruction succeeds. The one commit owner transfers
exact type, stored-Unknown compatibility, origin, and receiver-only Box
synthesis; existing string/map/record transfers and cache insertion remain in
their former owner after that commit.

The new private checked materialization entry preserves typed block-creation
and instruction-emission failures for a future COPY0 consumer. The existing
`ensure` and `try_ensure` facades deliberately retain their legacy best-effort
result for those failures, including recursive Select/Copy materialization;
contract failures retain their prior error behavior. No selected checked
consumer exists yet.

Closed evidence:

```text
successful LocalSSA:
  one PreparedLocalSsaPostSuccessV1::commit owner
  exact / StoredUnknown / origin / receiver fallback lanes unchanged

actual commit matrix:
  Missing, Exact, StoredUnknown, Recv, and FieldBase destination state parity

checked boundary:
  legacy facade recovers only BlockCreation / InstructionEmission
  checked future path preserves those typed failures

COPY0 scope:
  only RematerializedCopy and DominatingFallbackCopy are PhysicalCopy
  Const/BinOp/Compare/Select remain non-COPY0
```

Focused evidence:

```text
cargo test -q --lib post_success
cargo test -q --lib temporal_witness
cargo test -q --lib local_statement_parity
cargo test -q --lib variable_assignment_parity
cargo test -q --lib if_statement_parity
cargo test -q --lib return_statement_parity
cargo check --all-targets
```

`COPY-UNKNOWN0-G0` is now the sole next row. It freezes the one-owner counts
and direct-write exclusions before COPY0 may start.

### `COPY-UNKNOWN0-G0` — closed (2026-07-20)

One existing manifest-backed row guard now freezes these counts:

```text
post-success decision owners = 1
post-success commit owners = 1
source type-entry classifiers = 1
legacy Unknown compatibility lanes = 1
receiver fallback decision lanes = 1
physical materialization classifiers = 1
non-Copy rematerialization COPY0 consumers = 0
```

It additionally rejects direct LocalSSA destination type/origin writes,
`metadata::propagate` reuse, and any premature `TypeFactDecisionV1` consumer.
It checks the touched source/check files remain below 800 lines.

Focused evidence:

```text
tools/checks/run_row_guard.sh --only mirbuilder-copy-unknown-authority
```

`COPY0-S0` is now the sole next row. Only there may the existing
`TypeFactDecisionV1` become the disconnected exact physical-Copy publisher.

## COPY0 contract after this prerequisite

COPY0 will handle only successful `PhysicalCopy` materializations:

```text
source Missing       -> no exact fact
source StoredUnknown -> no exact fact
source Exact(T)      -> exact T candidate
```

The legacy Unknown entry is owned by `COPY-UNKNOWN0`, not by the later exact
publisher. Const, BinOp, Compare, and Select rematerializations are negative
fixtures for COPY0.

### `COPY0-S0` — closed (2026-07-20)

`local/copy_type.rs` now owns one disconnected, map-free
`PreparedLocalSsaPhysicalCopyTypeV1`. It accepts only the two existing
`PhysicalCopy` classifications, sends an exact source type through the existing
`TypeFactDecisionV1`, and keeps Missing/StoredUnknown as no-proposal inputs.
Non-Copy rematerialization rejects before any decision. Matching existing facts
remain idempotent and concrete conflicts retain the existing typed error.

```text
TypeContext / Builder / ValueId / MIR instruction ownership = 0
production LocalSSA consumer = 0
Unknown sentinel write / origin transfer / receiver fallback ownership = 0
```

Focused evidence:

```text
cargo test -q --lib copy_type
cargo test -q --lib post_success
cargo check --all-targets
```

`COPY0-P0` is now the sole next row. It must prove physical fallback and
rematerialized Copy timing, Missing/StoredUnknown/exact decisions, matching and
conflicting prepublication, and failure isolation before I0 connects one
successful physical-Copy producer.

## Parked authorities

```text
ORIGIN0: global value_origin_newbox policy
UNKNOWN-RET0-D0: eventual Unknown sentinel retirement
CopyTypePropagator / finalization repair
metadata::propagate
direct Copy emitters outside LocalSSA
string/map/record metadata transfer
FieldGet and Call result publication
source-shape, runtime, backend, and ownership widening
```

## Stop conditions

Stop this series if it needs to:

```text
treat Unknown as an exact TypeFactDecision proposal
drop the Unknown destination entry and silently change fallback behavior
derive Box type generally from origin
widen Recv fallback to FieldBase
put non-Copy rematerialization into COPY0
read final metadata during lowering
change CopyTypePropagator, type_hint_providers, or metadata::propagate
add a persistent ValueId -> type/owner map
retry after a selected checked-core failure
change grammar, runtime, backend, or ownership semantics
exceed 800 lines in any source/check file
```

## Decision lock

> **COPY-UNKNOWN0 selects C′: one behavior-preserving prepared transaction
> separates exact type transfer, legacy Unknown compatibility, origin transfer,
> and receiver-only Box synthesis. Stored Unknown explicitly suppresses the
> Recv-origin fallback, preserving `Unknown + origin` without publishing
> `Box(owner)`. The first code-facing row is disconnected S0; COPY0 remains
> blocked through G0 and will later consume only successful physical Copy plus
> already-exact source types.**
