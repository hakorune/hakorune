---
Status: closed__2026-09-21__WarningAdmittedRegistryBranchCountTestFacade
Task: MIRBUILDER-WARNING-ADMITTED-REGISTRY-BRANCH-COUNT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i95-2026-09-21.md
Implementation permission: true for the coupled test-only branch-count accessor chain
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I96
---

# Warning cleanup: admitted registry branch-count test facade

## Six-line brief

```text
Decision: gate the test-only branch-count observation chain
(`AdmittedTextScanRegistryV1::branch_count()` and its
`PreparedAotExecutableAdmissionV1::registry_branch_count()` facade) to tests;
preserve the admitted registry rows and all production admission semantics.
Source authority + canonical issuer: src/box_callable/provider_admission/
admitted_registry.rs and the I95 quick-profile warning census.
Non-authority: similarly named interpreter/MIR/reporting counters, warning
suppression, cargo-fix, or a guessed production consumer.
Fail-fast boundary: any unaccounted production caller, focused red, or
warning-count mismatch rejects the facade gating.
Smallest next slice: add cfg(test) to both observation accessors, run the
admission focused test, and refresh the fixed warning baseline.
Non-claims: no admission redesign, route switch, old-edge deletion, or ABI
change.
```

## Caller census

I95 records lib **1,709** and lib-test **552**. The exact
`AdmittedTextScanRegistryV1::branch_count()` accessor has its defining-module
test caller and one production-side caller from the unused
`PreparedAotExecutableAdmissionV1::registry_branch_count()` facade. That
facade is itself called only by the semantic-package test. The two accessors
therefore form one test-only observation chain and are gated together. The
other `branch_count` names in the repository belong to independent
interpreter, MIR-facts, and report structures and are outside this row.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib box_callable::provider_admission::admitted_registry::tests
```

The focused filter must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,709** to **1,708**, keep lib-test at **552**, and
preserve the deterministic registry assertions. Run fmt, diff, and the
current-state pointer guard before closeout. Do not add an allow or alter
admission semantics.

## Execution evidence

The first single-accessor attempt was rejected by the compiler: the
production-side `registry_branch_count()` facade still called `branch_count()`.
The bounded design was corrected to gate both accessors together; that facade
has a test caller only and no remaining production path.

Sequential acceptance then passed: lib check **1,708** warnings, lib-test
build **552**, and the focused admission filter **2/2**. Fmt, diff, and the
current-state pointer guard passed. Admission rows, generation, and plan-stamp
semantics are unchanged.
