---
Status: closed__2026-09-22__DirectAccumEffectRoleTestFacade__DeletedAndVerified
Task: MIRBUILDER-WARNING-DIRECT-ACCUM-EFFECT-ROLE-TEST-FACADE-I137
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i136-2026-09-22.md
Implementation permission: true for deleting the production-zero accessor and retaining the existing witness
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I138
---

# MirBuilder warning cleanup I137

## Six-line brief

```text
Decision: delete DirectAccumBindingEffectEntryV1::role and rewrite the existing
  witness to inspect the sealed role-keyed entry lookup; retain the effect-plan shape.
Source authority + canonical issuer: DirectAccumBindingEffectPlanV1 and its
  existing DirectAccum profile witness.
Non-authority: warning text alone, a new role accessor, positional inference,
  Recipe keys, or an AST/source rescan.
Fail-fast boundary: any non-test caller, focused red, changed role inventory,
  new warning, or changed production plan behavior stops the slice.
Smallest next slice: remove exactly the unused accessor, replace its test-only
  map with existing ALL/entry assertions, and run stable warning gates.
Non-claims: no DirectAccum semantic change, route selection, backend change,
  fallback, old-edge retirement, or new receipt.
```

## Census and delete set

The warning is emitted at
`src/mir/loop_structural_facts/direct_accum_effect_plan.rs:44` for
`DirectAccumBindingEffectEntryV1::role`. The exact source census found one
caller: `src/mir/compiler/direct_accum_profile.rs` inside its existing
`#[cfg(test)]` module. Production consumes the role field through the sealed
`VerifiedDirectAccumBindingEffectPlanV1::entry(role)` lookup and does not call
the accessor. The five-role `ALL` inventory and all effect entries remain
unchanged. The delete set is exactly the accessor plus the test-only map that
called it; the witness remains and checks the same five role bindings through
the existing `entry(role)` API.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib direct_accum_profile_seals_effect_witness
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

Expected warning counts are lib **1,685** and lib-test **544**. The focused test must execute nonzero named tests and pass. Do not alter the
role enum, `ALL`, entry lookup semantics, or any production caller.

## Stop rule

If a non-test caller or production role observation appears, return to
`design_stop`; do not replace the accessor with another public façade.

## Closeout evidence

`DirectAccumBindingEffectEntryV1::role` was physically deleted. The existing
DirectAccum profile witness now checks every role in `ALL` through the sealed
`entry(role)` lookup and retains its explicit binding assertions; no test was
removed and no production caller changed. `cargo fmt --all -- --check` passed.
The focused witness passed **1/1**. `cargo check --profile quick --lib -j4`
passed with lib **1,685** warnings, and the test binary build passed with
lib-test **544** warnings. The qualified accessor search, `git diff --check`,
and current-state pointer guard passed. No Facts, Recipe, route, ABI,
fallback, or old-edge behavior changed.
