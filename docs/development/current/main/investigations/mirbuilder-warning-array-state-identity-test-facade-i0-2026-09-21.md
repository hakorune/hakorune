---
Status: closed__2026-09-21__WarningArrayStateIdentityTestFacade
Task: MIRBUILDER-WARNING-ARRAY-STATE-IDENTITY-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i90-2026-09-21.md
Implementation permission: true for the test-only ArrayStateIdentity observation facade only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I91
---

# Warning cleanup: array state identity test facade

## Six-line brief

```text
Decision: gate the ArrayStateIdentity observation surface to tests; preserve
  ArrayStateCell storage and all production array operations.
Source authority + canonical issuer: src/boxes/array/mod.rs together with
  the I90 quick-profile warning inventory and repository-wide caller census.
Non-authority: cargo-fix, warning-count guesses, runtime identity redesign,
  storage changes, or a production caller inferred from test code.
Fail-fast boundary: any production caller, compile error, focused red, or
  warning-count mismatch rejects the slice.
Smallest next slice: gate the identity type, field/initializer, and
  state_identity() accessor, then run the two identity-focused test modules
  and the fixed warning refresh.
Non-claims: no array storage semantics, sharing/clone behavior, ABI change,
  suppression, or broad warning cleanup.
```

## Precondition and caller census

I90 records lib **1,715** and lib-test **553**. The selected diagnostics are:

* `src/boxes/array/mod.rs:47` — private `ArrayStateCell::identity` field is
  never read in the lib build.
* `src/boxes/array/mod.rs:104` — `ArrayBox::state_identity()` is never used in
  the lib build.

The only callers are test code: the `#[cfg(test)]` assertions in
`src/boxes/array/runtime_contract.rs` and
`src/boxes/array/tests/state_identity.rs`. `rg` found no production caller of
`ArrayStateIdentity`, `ArrayStateCell::identity`, or `state_identity`. The
nearby `admitted_registry::branch_count` warning is explicitly excluded by
its production caller in `aot_admission.rs:153`.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib boxes::array::tests::state_identity -- --nocapture
cargo test --profile quick --lib boxes::array::runtime_contract -- --nocapture
```

The focused filters must execute nonzero named tests and pass. The stable
refresh must reduce lib warnings from **1,715** to **1,713**, keep lib-test at
**553** with no new diagnostics, and preserve the identity/share/clone
assertions. Run `cargo fmt --all -- --check`, `git diff --check`, and the
current-state pointer guard before closeout. Do not add `#[allow]`, delete a
test, or change production array behavior.

## Execution evidence

The identity observation surface is now test-only: `ArrayStateIdentity`, the
`ArrayStateCell::identity` field and initializer, and
`ArrayBox::state_identity()` are all gated with `cfg(test)`. Production array
storage, sharing, cloning, and element-contract behavior are unchanged.

Sequential acceptance passed:

* `cargo check --profile quick --lib -j4`: **1,713** lib warnings, exactly
  two fewer than I90; the retained production `branch_count` warning and the
  mixed-cfg `loop_phi_materializer` index warning remain classified.
* `cargo test --profile quick --lib --no-run -j4`: **553** lib-test warnings.
* `cargo test --profile quick --lib boxes::array::tests::state_identity
  -- --nocapture`: **1/1**.
* `cargo test --profile quick --lib boxes::array::runtime_contract
  -- --nocapture`: **4/4**.
* `cargo fmt --all -- --check`, `git diff --check`, and the current-state
  pointer guard all passed.

No suppression, test deletion, runtime semantic change, or production caller
was introduced. The next action is I91 warning baseline refresh.
