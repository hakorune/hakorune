---
Status: closed__2026-09-21__WarningProviderImpossibleReject
Task: MIRBUILDER-WARNING-PROVIDER-IMPOSSIBLE-REJECT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i104-2026-09-21.md
Implementation permission: true for the unreachable provider admission reject variant
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I105
---

# Warning cleanup: provider impossible rejection

## Six-line brief

```text
Decision: delete ProviderAdmissionRejectV1::MissingCoreRow, which has no
issuer under the current required-core-row admission API; retain all reachable
provider rejection states and the existing CoreRowMismatch boundary.
Source authority + canonical issuer: src/box_callable/provider_admission/seal.rs,
selected from the I104 quick-profile warning census.
Non-authority: guessed future Option inputs, warning suppression, cargo-fix,
or a replacement reject variant.
Fail-fast boundary: any named constructor/match, focused red, or warning-count
mismatch rejects the deletion.
Smallest next slice: remove the one impossible enum variant and run the
provider-admission semantic-package tests.
Non-claims: no provider ABI change, alias policy change, parser expansion,
route switch, fallback, old-edge deletion, or LegacyCallV0 retirement.
```

## Caller census

I104 records lib **1,703** and lib-test **552**. `MissingCoreRow` has zero
callers in `src`, tests, tools, and tracked docs. The admission owner accepts
two required `&CoreMethodContractRowV2` inputs and emits `CoreRowMismatch` for
shape drift; no absent-row representation exists in this bounded API.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib mir::normal_callable_semantic_package::tests
```

The focused filter must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,703** to **1,702**, reduce lib-test from **552** to
**551**, and
preserve the provider admission and selected-package tests. Run fmt, diff, and
the current-state pointer guard before closeout. Do not add an allow or alter
provider admission semantics.

## Execution evidence

`ProviderAdmissionRejectV1::MissingCoreRow` was deleted. The current admission
owner still requires both core rows and keeps `CoreRowMismatch` as the reachable
shape rejection. Sequential acceptance passed: lib check **1,702** warnings,
lib-test build **551**, and the selected semantic-package filter **21/21**.
The test warning baseline changed by one because the impossible variant was
also reported in the test build. No warning suppression was added; fmt and diff
checks passed.
