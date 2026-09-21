---
Status: closed__2026-09-22__WarningForestBindingAccessors__ProductionBuildRestricted
Task: MIRBUILDER-WARNING-FOREST-BINDING-ACCESSORS-I133
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i132-2026-09-22.md
Implementation permission: true for restricting two production-zero accessors to test builds
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I134
---

# Warning cleanup: Loop source forest binding accessors

## Six-line brief

```text
Decision: compile VerifiedLoopSourceForestBindingMemberV1::path and
  VerifiedLoopSourceForestBindingV1::owner only for tests; retain the forest,
  member paths, owner field, parent checks, and Recipe conversion.
Source authority + canonical issuer: resolver-owned forest binding and its
  existing bind_resolved_loop_source_forest_v1 / into_source_binding owners.
Non-authority: warning text alone, a new forest projection, syntax rescans,
  Recipe inference, or compatibility route behavior.
Fail-fast boundary: any production caller, focused red, changed parent/path
  evidence, or new warning cancels the slice.
Smallest next slice: add cfg(test) to exactly the two unused accessors and run
  the existing forest/Recipe focused tests.
Non-claims: no forest schema, parser composite admission, LoopBreak cutover,
  backend, fallback, old-edge retirement, or warning suppression.
```

## Census and delete set

The selected warnings are emitted at
`src/mir/loop_structural_facts/resolved_source_adapter.rs:104` and `:118`:

```text
VerifiedLoopSourceForestBindingMemberV1::path
VerifiedLoopSourceForestBindingV1::owner
```

Repository-wide search found `path()` only in the owner module's
`loop_structural_facts/tests.rs` forest projection assertions. No production
caller or guard calls either accessor; `owner` has no direct callsite. The
forest binding types, `members`, `parent_index`, direct field use inside
`into_source_binding`, parent-index validation, and all issuers remain live.
The delete set is only production compilation of these two convenience
accessors; test builds retain them for structural evidence.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib mir::loop_structural_facts
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

The focused filter must execute nonzero named tests and pass. Record the new
lib/lib-test warning counts and confirm the forest/Recipe symbols remain.

## Stop rule

If a production consumer appears, or if the test-only boundary changes the
forest parent/path evidence, return to design_stop. Do not delete the forest
binding or add a replacement production accessor.

## Closeout evidence

Both accessors now have `#[cfg(test)]`, so they remain available to the
structural tests but are absent from the production lib build. The forest
binding, parent-index checks, owner field, and Recipe conversion are unchanged.
`cargo fmt --all -- --check` passed; the focused forest filter passed **28/28**;
`cargo check --profile quick --lib -j4` passed with lib **1,687** warnings;
`cargo test --profile quick --lib --no-run -j4` passed with lib-test **545**
warnings; and the pointer guard passed. The two target warning locations are
absent from the lib warning log. No forest schema, semantic route, test, or
fallback was deleted.
