---
Status: closed__2026-09-21__WarningBaselineRefreshI88__ParserBodySourceFacadeSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I88
Date: 2026-09-21
Parent: mirbuilder-warning-admitted-text-scan-method-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: MIRBUILDER-WARNING-PARSER-BODY-SOURCE-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I88

## Six-line brief

```text
Decision: refresh warning surfaces after the admitted-registry method scope and
  select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,736/553, census one
  candidate, and select Delete or NoSafeSlice.
Non-claims: no semantic refactor, suppression, production switch, or broad
  warning cleanup.
```

## Acceptance

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record warning class, file and line, owner, role, grouped-diagnostic
membership, and caller inventory. Select at most one caller-zero warning
facade whose production edge can be removed with a focused guard; otherwise
record `NoSafeSlice`. Keep dead-code and private-interface rows with their
owners. A selected edge must prove caller-zero before physical removal, then
run its focused gate and the stable warning refresh at its parent/current pair.

## I88 evidence

The admitted-registry slice scopes two test-only observation methods. Lib is
**1,736**, lib-test is **553**, and the focused suite passed **2/2**; production
`branch_count` remains unchanged.

## Refresh result and selected bounded cohort

The sequential refresh completed at lib **1,736** and lib-test **553**, with no
new red and no unused-import warnings. The apparent `branch_count` candidate is
rejected: `src/box_callable/provider_admission/aot_admission.rs:153` is a
production caller, so its warning remains with the admitted-registry owner.

The next finite caller-zero cohort is the parser body-source transaction
facade. `ParserResolverBodyTransactionV1`, its error/helpers,
`parse_from_string_with_resolver_body_source`, and `release_source` are
referenced only by existing `#[cfg(test)]` paths. The production
`instance_method_body_source.rs` consumer uses the body envelope/row types,
and `InstanceMethodFunctionCarrierIssuerV1` consumes
`ParserBoxInstanceMethodSyntaxLeaseV1`; those three production surfaces and
their parser re-exports remain unconditional. I0 may gate only the test
transaction support and companion release catalog; any further production
consumer or compile error is a fail-fast stop.
