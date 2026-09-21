---
Status: closed__2026-09-21__WarningModuleTokenId__DeletedAndVerified
Task: MIRBUILDER-WARNING-MODULE-TOKEN-ID-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i106-2026-09-21.md
Implementation permission: true for the production-zero module identity accessors
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I108
---

# Warning cleanup: unused module token ID accessor

## Six-line brief

```text
Decision: delete ModuleInvocationTokenV1::id and its now-unreachable test-only
ModuleInvocationIdV1::ordinal helper; use the existing brand accessor and token
PartialEq in the identity fixtures without adding a new authority.
Source authority + canonical issuer: src/mir/module_invocation_identity.rs;
the candidate came from the I106 quick-profile warning census.
Non-authority: warning suppression, cargo-fix, a replacement ID wrapper, or
changing the identity brand semantics.
Fail-fast boundary: any production caller, focused identity red, or warning
count mismatch rejects the row.
Smallest next slice: replace four fixture reads, remove the two dead accessors,
and run the identity-focused tests plus sequential warning checks.
Non-claims: no identity ABI change, route switch, parser expansion, fallback,
old-edge deletion, or LegacyCallV0 retirement.
```

## Caller census

I106 reproduced lib **1,701** and lib-test **551** warnings. The accessor
`ModuleInvocationTokenV1::id` has four fixture callers in
`module_invocation_identity_p0.rs` and `module_invocation_identity_idkernel_p0.rs`;
there is no production caller in `src`, and no tools or tracked documentation
invoke it. The fixture checks can use `brand()` for issuer observations and the
existing derived `PartialEq` for token distinction. Removing `id()` makes the
test-only `ModuleInvocationIdV1::ordinal` helper caller-zero as well; it is
removed in this same bounded row rather than leaving a new warning behind.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo fmt --all -- --check
cargo test --profile quick --lib mir::builder::module_invocation_identity_p0
cargo test --profile quick --lib mir::module_invocation_identity_idkernel_p0
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

The focused filters must execute nonzero tests and pass. The warning refresh
must reduce lib warnings from **1,701** to **1,700** and keep lib-test at
**551**; no `#[allow]` or unrelated identity changes are permitted.

## Execution evidence

`ModuleInvocationTokenV1::id` and the newly caller-zero test-only
`ModuleInvocationIdV1::ordinal` helper were deleted. The combined identity
focused filter matched both fixtures and passed **7/7**. Sequential warning
checks passed with lib **1,700** warnings and lib-test **551** warnings.
`cargo fmt --all -- --check`, `git diff --check`, and the current-state pointer
guard passed. Brand issuance, brand comparison, and token route semantics are
unchanged.
