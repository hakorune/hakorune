---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-CALLABLE-SEMANTIC-BATCH-REEXPORT-DELETE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i83-2026-09-21.md
Implementation permission: true for removal of two caller-zero test-only re-export items only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I84
---

# MirBuilder callable semantic batch re-export delete I0

## Six-line brief

```text
Decision: remove two unused test-only parent re-export items while retaining
  the canonical issuer and all used batch entry points.
Source authority + canonical issuer: callable_semantic_batch/issuer.rs.
Non-authority: parent re-export reachability, warning guesses, or issuer internals.
Fail-fast boundary: any caller, unresolved import, changed batch behavior, or
  warning classification drift stops the slice.
Smallest next slice: remove the two unused items and run lib check, test build,
  and the callable semantic batch focused suite.
Non-claims: no issuer semantic change, no test deletion, no suppression, and no
  production route change.
```

## Census boundary and acceptance

`issue_resolved_callable_semantic_batch_with_policy_v1` and
`DirectCallObservationBatchPolicyV1` are defined and consumed internally by
`callable_semantic_batch/issuer.rs`. The parent `#[cfg(test)]` re-export has no
other source or test caller; the used issue and brand-catalog entry points stay
exported.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib -j4 callable_semantic_batch -- --nocapture
```

The focused suite must run at least one test and pass, lib-test warnings must
decrease from **556**, and fmt/diff/pointer guards must be green before closeout.

## Closeout evidence

The two unused test-only parent re-export items were removed; the issuer and
used batch entrypoints remain unchanged. Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warnings **1,741**.
- `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **555**.
- `cargo test --profile quick --lib -j4 callable_semantic_batch::tests -- --nocapture`: **13/13**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

No semantic issuer, production route, or test body changed. Two unused facade
items were physically deleted. The next action is the I84 warning baseline
refresh.
