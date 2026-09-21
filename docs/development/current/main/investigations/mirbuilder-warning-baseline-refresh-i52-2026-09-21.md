---
Status: closed__2026-09-21__WarningBaselineRefreshI52__CallableSemanticBatchFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I52
Date: 2026-09-21
Parent: mirbuilder-warning-compare-i64-writer-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-CALLABLE-SEMANTIC-BATCH-FACADE-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I53
---

# MirBuilder warning baseline refresh I52

## Six-line brief

```text
Decision: refresh both warning surfaces after the I51 compare-I64 facade cohort
  before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,772/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,772** and lib-test **561** after I51.


## Selection evidence

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,772 | `/tmp/hakorune-warning-i52-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i52-lib-test-20260921.log` |

The selected caller-zero cohort is the four forwarding exports in
`src/mir/callable_semantic_batch/mod.rs:20-24`:

* `issue_resolved_callable_semantic_batch_v1`
* `issue_resolved_callable_semantic_batch_with_brand_catalog_v1`
* `issue_resolved_callable_semantic_batch_with_policy_v1`
* `DirectCallObservationBatchPolicyV1`

The finite source census found no production consumer through this facade. The
issuer calls its own child functions directly; the only external references
are test modules, and the apparent common-v2 caller is inside that file's
`#[cfg(test)] mod tests`. The issuer, model, S6C relation, and production
semantic behavior are outside this slice.

The bounded execution slice gates only these four forwarding exports under
`#[cfg(test)]`; no issuer logic, public batch contract, or production route
changes.

## Closeout evidence

The four selected forwarding exports were gated under `#[cfg(test)]`. Both
fixed gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,771 | `/tmp/hakorune-warning-i0-callable-semantic-batch-facade-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-callable-semantic-batch-facade-lib-test-20260921.log` |

I52 closes with no issuer, model, S6C relation, semantic batch, or production
route change. The four imports formed one compiler warning diagnostic, so the
lib reduction is one warning.
