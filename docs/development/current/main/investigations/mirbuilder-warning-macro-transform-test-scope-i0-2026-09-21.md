---
Status: closeout__2026-09-21__Landed__FocusedSuitesGreen
Task: MIRBUILDER-WARNING-MACRO-TRANSFORM-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i79-2026-09-21.md
Implementation permission: true for cfg(test) scoping of transform_normal_callable_program_v1 only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I80
---

# MirBuilder macro transform test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for the caller-zero normal
  callable transform helper while retaining existing test facades.
Source authority + canonical issuer: macro/normal_callable_transform.rs.
Non-authority: parent macro-module reachability, warning counts, or a new
  transform owner.
Fail-fast boundary: any non-test caller, missing test import, changed transform
  behavior, or focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export for the v1 helper and
  run representative transform/LoopCond tests plus the quick library check.
Non-claims: no transform semantic change, no policy change, no production route
  change, no test deletion, and no blanket warning cleanup.
```

## Census and acceptance

`transform_normal_callable_program_v1` is defined in
`macro/normal_callable_transform.rs`. Repository census shows its imports through
`crate::macro` only in `#[cfg(test)]` modules; production code uses the
policy-aware transform path or does not call this helper. Keep the existing
legacy test imports and scope only this parent re-export to `cfg(test)`.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib loop_cond_break_continue_observation -- --nocapture
cargo test --profile quick --lib normal_callable_transform -- --nocapture
```

The focused suites must pass. Record the warning delta, preserve the I79
candidate correction, and run `cargo fmt --check`, `git diff --check`, and the
current-state pointer guard before commit/push.

## Closeout evidence

The parent `transform_normal_callable_program_v1` re-export is now scoped to
`cfg(test)`. The canonical implementation and policy-aware production path are
unchanged. Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warning baseline **1,742**.
- `cargo test --profile quick --lib -j4 loop_cond_break_continue_observation -- --nocapture`: **9/9**.
- `cargo test --profile quick --lib -j4 normal_callable_transform -- --nocapture`: **7/7**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

No semantic code path, test, suppression, or production route was changed.
The next action is a design-stop warning baseline refresh (I80).
