---
Status: closed__2026-09-21__WarningModuleIdSame__DeletedAndVerified
Task: MIRBUILDER-WARNING-MODULE-ID-SAME-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i105-2026-09-21.md
Implementation permission: true for the production-zero ModuleInvocationIdV1::same method
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I106
---

# Warning cleanup: unused module identity comparison

## Six-line brief

```text
Decision: delete ModuleInvocationIdV1::same; compare IDs through the existing
derived PartialEq in its one test caller and retain Brand::same as the identity
kernel comparison authority.
Source authority + canonical issuer: src/mir/module_invocation_identity.rs;
the candidate came from the I105 quick-profile warning census.
Non-authority: warning suppression, cargo-fix, a new comparison helper, or
changing the brand comparison semantics.
Fail-fast boundary: any production caller, focused identity red, or warning
count mismatch rejects the row.
Smallest next slice: change one test assertion, remove one unused method, and
run the identity-focused tests plus sequential warning checks.
Non-claims: no identity ABI change, route switch, parser expansion, fallback,
old-edge deletion, or LegacyCallV0 retirement.
```

## Caller census

I105 reproduced lib **1,702** and lib-test **551** warnings. The method
`ModuleInvocationIdV1::same` has one textual caller in the identity fixture
`src/mir/builder/module_invocation_identity_p0.rs`; no production caller exists
in `src`, and no tools or tracked documentation invoke it. The test compares
the same derived identity fields, so `assert_ne!(first.id(), second.id())`
preserves the assertion without retaining a dead method.

`ModuleInvocationBrandV1::same` has independent callers and remains untouched.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib mir::builder::module_invocation_identity_p0
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

The focused filter must execute nonzero tests and pass. The warning refresh must
reduce lib warnings from **1,702** to **1,701** and keep lib-test at **551**;
no `#[allow]` or unrelated identity changes are permitted.

## Execution evidence

`ModuleInvocationIdV1::same` was deleted and its sole test assertion now uses
the existing derived `PartialEq` comparison. The focused identity filter passed
**4/4**. Sequential warning checks passed with lib **1,701** warnings and
lib-test **551** warnings. `cargo fmt --all -- --check`, `git diff --check`,
and the current-state pointer guard also passed. `ModuleInvocationBrandV1::same`
and all production identity issuance remain unchanged.
