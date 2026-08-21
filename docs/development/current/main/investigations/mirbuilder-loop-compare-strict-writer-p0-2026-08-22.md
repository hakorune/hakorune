Status: landed fast task; focused writer contract complete; caller-zero remains intentional
Task: MIR-LOOP-COMPARE-STRICT-WRITER-P0
Date: 2026-08-22
Priority: next bounded physical contract
Parent: MIR-EMIT-CANONICAL-STRICTNESS-D0
PreviousCard: MIR-LOOP-COMPARE-RESULT-LEDGER-P0
NextCard: MIR-LOOP-COMPARE-CONNECT0-D0
---

# Loop Compare strict writer P0

## Six-line brief

```text
Decision: admit one PreparedCanonicalCompareAppendV1 and append exactly one CompareI64 into one already-open target; the strict commit returns the writer-owned definition source required by the result ledger.
Source authority + canonical issuer: the Loop strict issuer co-seals the existing open-target/operand witnesses, fresh canonical destination capability, CompareOp, and precomputed Bool plan; builder_emit owns the sole physical append and emits the writer-owned definition source.
Non-authority: current_block, ensure_block_exists, emit_instruction_at, LocalSSA receiver repair, PHI repair/materialization, type inference, legacy emitters, raw ValueId, and post-append verification.
Fail-fast boundary: every target/owner/operand/destination/Bool-plan check and writer preparation finishes before append; strict commit has no Result, repair, block creation, or fallback.
Smallest next slice: add a private strict front door plus one shared append primitive, and prove one prepared Compare append/definition handoff without connecting dispatcher or a caller.
Non-claims: general dominance, cross-block operands, Binary/Const strictness, ledger reservation connection, operation dispatch, fallback removal, CONNECT0, production I0/R0, and performance.
```

## Fixed boundary

The result-ledger P0 now supplies an owner-bound affine reservation and accepts
only a writer-owned definition source at infallible commit. This card fixes the
writer half without consuming the reservation in production:

```text
verified target + verified lhs/rhs + fresh destination + Bool plan
    -> PreparedCanonicalCompareAppendV1
    -> one exact Compare instruction append
    -> CanonicalCompareDefinitionSourceV1
```

The strict front door is a sibling of the legacy repair-capable emitter. Both
front doors delegate the physical mutation to one private append primitive;
the strict front door may not call the legacy `emit_instruction_at` path.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| canonical CFG session | same-session open target | current-block repair or target creation |
| canonical SSA session | destination capability and operand witnesses | Compare selection or append |
| Loop strict issuer | operation/target/operand/destination/Bool co-seal | MIR mutation or fallback |
| strict builder writer | one prepared Compare append and definition source | target/type inference, repair, ledger state |
| result ledger | reserve/commit lifecycle | MIR inspection or physical allocation |

## Allowed files

```text
src/mir/builder/builder_emit.rs
src/mir/builder/builder_emit_core.rs (only if builder_emit exceeds 700 lines)
src/mir/builder/resolved_lowering/canonical_ssa/session.rs
src/mir/builder/resolved_lowering/canonical_ssa/session/destination.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_writer.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/compare_i64_writer_tests.rs
src/mir/builder/emission/compare_type.rs (visibility-only if required)
```

`builder_emit.rs` owns the shared `append_instruction_core` and the strict
prepare/commit facade; target 600 lines and split at 700 if needed. The Loop
child owns strict orchestration only. Do not edit `operation_dispatcher.rs`,
legacy Compare callers, or production selection. If the actual module layout
requires a different child path, record the path change in this card before
editing.

## Finite state table

| State | Authority | Effect | Allowed transition |
| --- | --- | ---: | --- |
| `Unprepared` | strict issuer | none | all preflight checks |
| `Prepared` | strict writer | none | one private commit |
| `Committed` | strict writer | exactly one append | definition source to ledger |
| `RejectedBeforeEffect` | strict issuer/writer | none | outer unpublished session discard |
| `Poisoned` | outer session | no further use | terminal discard only |

`Prepared` is private and non-`Clone`. `CanonicalCompareDefinitionSourceV1`
retains owner, target block, physical value, and the append proof while
implementing the ledger's writer-definition handoff trait; it cannot be made
from a free raw `ValueId`. The append primitive must not create a
missing block, change the current insertion point, materialize a receiver,
normalize PHI inputs, or infer a type. The returned definition source is the
only writer-to-ledger handoff; it is not a second target or value authority.

## Acceptance

- strict preparation accepts only the already-co-sealed C-prime witnesses;
- destination allocation is session-owned and wrapped before the writer sees it;
- the strict front door and legacy front door share one physical append point;
- all strict rejects leave instruction count, type context, and ledger state
  unchanged;
- strict commit appends exactly one Compare and returns one definition source
  without a post-append `Result` path;
- no `emit_instruction_at`, `current_block`, `ensure_block_exists`, LocalSSA,
  PHI repair, `compute_def_blocks`, or `compute_dominators` is reachable from
  strict commit;
- focused positive/negative tests, source-size, diff, and caller-zero guards
  are green.

## NoSafeSlice

Return to the strictness D0 if the strict path must use the legacy emitter,
if shared append cannot be isolated as the single physical mutation point, if
destination/type preparation remains fallible after append, if a raw
`ValueId` or block can escape instead of a private capability, if the strict
child requires a second MIR/CFG/SSA authority, or if any Compare caller must
be connected before the writer contract is complete.

## Implementation evidence

The bounded writer is landed without opening CONNECT0:

```text
builder_emit.rs (517 lines)
  legacy front door + shared-core delegation
        ↓
builder_emit_core.rs (232 lines)
  PreparedCanonicalCompareAppendV1
        ↓ commit once
CanonicalCompareDefinitionSourceV1 (non-Clone, non-Copy)
        ↓
compare_i64_writer.rs
  strict orchestration only
```

`builder_emit_core.rs` was split before the parent crossed the 700-line
trigger. The only direct `add_instruction_with_span` call is in the shared
append core. The strict child has no caller outside focused tests, and it does
not call `emit_instruction_at`, `emit_instruction`, `ensure_block_exists`,
ambient `current_block`, or dominance utilities. The canonical destination
capability and builder-private target/operand witnesses remain owned by the
existing CFG/SSA sessions.

Focused evidence:

```text
cargo check --profile quick --lib                         PASS
cargo test --profile quick --lib compare_i64_writer      3 passed
cargo test --profile quick --lib loop_recipe_physicalizer 36 passed
tools/checks/rust_mirbuilder_loop_compare_strict_writer_p0_guard.sh PASS
```

No dispatcher, production caller, result-ledger reservation, fallback
removal, or production I0/R0 claim was made. The next bounded task is
`MIR-LOOP-COMPARE-CONNECT0-D0`, which must first census the named caller and
design the dispatcher handoff.
