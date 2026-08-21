Status: landed fast BoxShape; behavior-neutral split complete
Task: MIR-LOOP-OPERATION-EMITTER-SPLIT-S0
Date: 2026-08-22
Priority: prerequisite
Parent: MIR-EMIT-CANONICAL-STRICTNESS-D0
Owner: `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/`
NextCard: MIR-LOOP-COMPARE-SESSION-TARGET-P0
---

# Loop pure-operation emitter split S0

## Readiness

```text
The existing prepared pure-operation input maps to the same existing
Const/Binary/Compare leaf and fails at the same existing boundary.
```

Classification: `BoxShape`. This row changes no accepted shape, authority,
instruction order, error vocabulary, ledger state, target validation, or
production reachability.

## Decision

`operation_emitter.rs` is 794 lines and may not receive semantic edits. Move
the complete existing pure-operation responsibility into one child before the
C-prime rows begin:

```text
loop_recipe_physicalizer/
  operation_emitter.rs       # Binding read/write owners
  pure_operation_emitter.rs  # prepared ConstI64/BinaryI64/CompareI64 owner
```

Move together:

- `PreparedLoopOperationEmissionV1`;
- `LoopOperationServicesV1`;
- pure target issuance; reuse the one existing target-error mapping without
  copying it (the Binding read/write owners also consume that mapping);
- `emit_prepared_operation_v1`;
- `emit_prepared_pure_operation_v1`;
- `emit_prepared_pure_operation_at_target_v1`;
- existing Const/Binary/Compare dispatch, result-type reread, ledger
  publication, receipts, and errors needed solely by those functions.

Do not improve those behaviors in S0. In particular, the existing Compare
must still call `loop_operation::emit_compare_i64_at`, and the existing
post-emission type and ledger checks must remain equivalent in observable
order. They are removed only by later P0/CONNECT0 rows.

Use narrow `pub(super)` re-exports or direct sibling imports. Do not make a
new crate-visible facade, duplicate a constructor, or preserve imports through
a broad barrel.

## Allowed files

```text
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_emitter.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/pure_operation_emitter.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/operation_dispatcher.rs
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/mod.rs
```

Test files may change only when a private path import must follow the moved
owner. No fixture or assertion meaning may change.

## Forbidden overlap

```text
no strict prepared receipt
no open-target witness
no CFG/SSA/session change
no ledger Reserved/Poisoned state
no row/target pairing change
no writer-core extraction
no new negative case
no caller connection or retirement
no move/clone optimization
no fallback or retry
```

## Acceptance

- `operation_emitter.rs` and `pure_operation_emitter.rs` are each below 760
  lines; 800 remains a hard stop;
- every moved symbol has one definition and unchanged visibility;
- the existing common Loop physicalizer focused tests pass unchanged;
- Callable and Generic G0 canaries keep their current results;
- the caller census remains zero outside `#[cfg(test)]`;
- `git diff --check` and the current-state pointer guard pass;
- the commit contains only this responsibility split and its card closeout.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib loop_recipe_physicalizer -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

If the move requires changing operation order, target issuance, error mapping,
ledger behavior, writer selection, or a public API, stop and return to the D0.

## Evidence

Landed as a pure owner split. `operation_emitter.rs` now owns Binding
read/write preparation and the shared target-error mapping; the new
`pure_operation_emitter.rs` owns the existing prepared Const/Binary/Compare
leaves, their services, receipts, and pure-operation rejection type. The
parent re-exports the moved private surface so existing dispatcher and canary
imports remain unchanged. No strict receipt, CFG/SSA/session, ledger state,
writer, fallback, or caller edge was added.

Measured source sizes after the split:

```text
operation_emitter.rs        501 lines
pure_operation_emitter.rs  326 lines
```

Focused evidence is green:

```text
RUSTFLAGS='-Awarnings' cargo test --lib loop_recipe_physicalizer -- --nocapture --test-threads=1
  28 passed, 0 failed
RUSTFLAGS='-Awarnings' cargo check --lib
  Finished successfully
rustfmt --edition 2021 --check <three touched Rust files>
  clean
git diff --check
  clean
```

The whole-worktree `cargo fmt --check` remains a known baseline failure in
unrelated files and was not used as an S0 acceptance claim. The caller census
remains caller-zero outside `#[cfg(test)]`.
