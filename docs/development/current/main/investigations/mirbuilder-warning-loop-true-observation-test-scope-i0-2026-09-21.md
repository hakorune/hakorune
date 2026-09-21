---
Status: fast__2026-09-21__SelectedBoundedWarningCohort__ExecuteTestScope
Task: MIRBUILDER-WARNING-LOOP-TRUE-OBSERVATION-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i78-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the five LoopTrue observation re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I79
---

# MirBuilder LoopTrue observation test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for five caller-zero LoopTrue
  observation symbols while retaining existing test facades.
Source authority + canonical issuer: loop_true_break_continue_observation.rs.
Non-authority: parent-module reachability, warning counts, or a new observation
  owner.
Fail-fast boundary: any non-test caller, missing test import, changed owner, or
  focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run LoopTrue
  observation/admission tests plus the quick library check.
Non-claims: no LoopTrue observation semantic change, no admission change, no
  production route change, no test deletion, and no blanket warning cleanup.
```

## Census and acceptance

The five symbols are defined in `loop_true_break_continue_observation.rs`.
Production code uses the owner module directly; parent-facade imports are in
`loop_true_break_continue_observation_tests.rs`, `family_admission_tests.rs`,
and `family_selector_tests.rs`. Keep the tests and scope the parent re-export
to `cfg(test)`.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib loop_true_break_continue_observation -- --nocapture
cargo test --profile quick --lib family_admission -- --nocapture
```

The focused suites must pass. Record the warning delta, preserve known baseline
classification, and run `cargo fmt --check`, `git diff --check`, and the
current-state pointer guard before commit/push.
