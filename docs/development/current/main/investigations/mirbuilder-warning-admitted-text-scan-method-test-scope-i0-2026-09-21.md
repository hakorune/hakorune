---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-ADMITTED-TEXT-SCAN-METHOD-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i87-2026-09-21.md
Implementation permission: true for cfg(test) scoping of three admitted-registry observation methods only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I88
---

# MirBuilder admitted text-scan method test scope I0

## Six-line brief

```text
Decision: compile the three admitted-registry observation methods only for tests;
  retain production admission construction and all test assertions.
Source authority + canonical issuer: box_callable/provider_admission/admitted_registry.rs.
Non-authority: production registry construction, AOT admission, warning guesses,
  or registry semantics.
Fail-fast boundary: any non-test caller, missing test method, changed admission
  behavior, or warning classification drift stops the slice.
Smallest next slice: add cfg(test) to entry, branch_count, and row, then run
  lib check, test build, and the admitted-registry focused suite.
Non-claims: no admission ABI change, no test deletion, no suppression, and no
  production route change.
```

## Census boundary and acceptance

`AdmittedTextScanRowV1::entry`,
`AdmittedTextScanRegistryV1::branch_count`, and
`AdmittedTextScanRegistryV1::row` are called only by
`admitted_registry.rs` tests. Production consumers use registry construction,
`generation`, `plan_stamp`, and the AOT entry projection; those remain
unchanged.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib -j4 admitted_registry -- --nocapture
```

The focused suite must run at least one test and pass, lib warnings must drop
from **1,737**, lib-test behavior must remain green, and fmt/diff/pointer guards
must pass before closeout.

## Candidate correction and closeout evidence

An initial attempt to scope `branch_count` together with the two observation
methods was rejected by `cargo check`: `aot_admission.rs` consumes
`branch_count` in production. That attribute was reverted immediately. The
final slice scopes only `entry` and `row`, whose callers are test-only.

Sequential acceptance passed:

- `cargo check --profile quick --lib -j4`: lib warnings **1,736** (down from **1,737**).
- `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **553**.
- `cargo test --profile quick --lib -j4 admitted_registry -- --nocapture`: **2/2**.
- `cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
  guard all passed.

No admission behavior, AOT route, test body, or suppression changed. Two
verification methods were scoped to tests; production `branch_count` remains.
The next action is the I89 warning baseline refresh.
