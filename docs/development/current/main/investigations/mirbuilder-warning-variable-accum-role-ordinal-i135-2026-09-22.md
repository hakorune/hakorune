---
Status: closed__2026-09-22__WarningVariableAccumRoleOrdinal__DeletedAndVerified
Task: MIRBUILDER-WARNING-VARIABLE-ACCUM-ROLE-ORDINAL-I135
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i134-2026-09-22.md
Implementation permission: true for deleting the declaration-only accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I136
---

# Warning cleanup: variable-accumulator source-role ordinal accessor

## Six-line brief

```text
Decision: delete VariableAccumRecurrenceSourceRoleV1::ordinal(); retain the
  eleven-role enum, ALL inventory, operation-role coverage, and Recipe issuer.
Source authority + canonical issuer: the existing recurrence Facts issuer and
  variable_accum_recurrence Recipe producer.
Non-authority: warning text alone, ordinal guesses, AST rescans, or a new
  numeric/Recipe identity projection.
Fail-fast boundary: any caller, focused red, changed role inventory, or new
  warning cancels the slice.
Smallest next slice: remove exactly the declaration-only method and run the
  recurrence projection/Recipe focused tests.
Non-claims: no recurrence semantics, source admission, backend, fallback,
  old-edge retirement, or warning suppression.
```

## Census and delete set

The selected warning is emitted at
`src/mir/loop_structural_facts/variable_accum_recurrence.rs:60`:

```text
VariableAccumRecurrenceSourceRoleV1::ordinal
```

Repository-wide search across `src`, `tests`, `tools`, `lang`, `apps`, and
tracked current docs found only the declaration. The enum's `ALL` inventory is
consumed by the compiler projection, and `operation_roles` is retained in
Facts; neither relies on ordinal numbering. The delete set is exactly this
method body. No role variant, source observation, Recipe relation, or guard is
changed.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib mir::compiler::variable_accum_recurrence_projection
cargo test --profile quick --lib variable_recurrence_producer_seals_existing_recipe_core
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Both focused filters must execute nonzero named tests and pass. Record the new
lib/lib-test warning counts and confirm the role inventory remains intact.

## Stop rule

If a caller or role-identity dependency appears, return to design_stop. Do not
replace the accessor with another ordinal or infer a Recipe key from position.

## Closeout evidence

The declaration-only `ordinal` method was physically deleted. Repository
census found no caller or qualified reference; the eleven-role enum, `ALL`
inventory, operation-role coverage, Facts issuer, and Recipe producer remain
unchanged. `cargo fmt --all -- --check` passed. The source projection focused
suite passed **9/9**, and the exact Recipe producer test
`variable_recurrence_producer_seals_existing_recipe_core` passed **1/1**.
`cargo check --profile quick --lib -j4` passed with lib **1,686** warnings;
`cargo test --profile quick --lib --no-run -j4` passed with lib-test **544**
warnings. The pointer guard and `git diff --check` passed. No semantic route,
fallback, test deletion, or old-edge claim changed; the LoopBreak old-edge
remains `NoSafeSlice`.
