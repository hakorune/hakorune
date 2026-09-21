---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-LOOP-TRUE-REJECT-MAPPER-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i80-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the parent mapper re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I81
---

# MirBuilder LoopTrue reject mapper test scope I0

## Six-line brief

```text
Decision: remove the non-test parent export edge for the LoopTrue reject mapper
  while retaining the canonical mapper and its test-only compiler consumer.
Source authority + canonical issuer: loop_true_break_continue_observation.rs.
Non-authority: the loop_structural_facts facade, warning-count guesses, or
  deletion of the canonical mapper.
Fail-fast boundary: any non-test caller, missing test import, changed reject
  mapping, or focused test failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run the quick
  library check plus the LoopTrue focused suite.
Non-claims: no semantic mapping change, no LoopTrue route change, no function
  deletion, no test deletion, and no suppression.
```

## Census boundary and acceptance

The canonical `map_loop_true_source_binding_reject` mapper is defined in
`src/mir/loop_structural_facts/loop_true_break_continue_observation.rs`. The
only consumer of the parent path is
`src/mir/compiler/loop_true_break_continue_observation.rs`, which is a
`#[cfg(test)]` adapter. The parent export is therefore a production warning
edge, while the mapper and test consumer remain required.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib -j4 loop_true_break_continue_observation -- --nocapture
```

The focused suite must pass and the lib warning count must drop from **1,742**
without changing lib-test behavior. Then run `cargo fmt --all -- --check`,
`git diff --check`, and the current-state pointer guard before closeout.

## Candidate correction and closeout evidence

An initial attempt to scope the whole LoopTrue observation re-export to
`cfg(test)` was rejected by `cargo check`: the production route-policy modules
consume the observation types. That change was reverted immediately. The final
slice scopes only `map_loop_true_source_binding_reject` to `cfg(test)` and leaves
all production observation types unchanged.

Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warning baseline **1,741**.
- `cargo test --profile quick --lib -j4 loop_true_break_continue_observation -- --nocapture`: **9/9**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

The canonical mapper, test adapter, and reject mapping are unchanged. One
production parent export edge was removed; no function or test was deleted and
no suppression or route change was introduced. The next action is the I82
warning baseline refresh.
