---
Status: closed__2026-09-21__WarningBaselineRefreshI46__ResolvedLoweringFacadeTestExports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I46
Date: 2026-09-21
Parent: mirbuilder-warning-selected-dynamic-emitter-test-facade-i0-2026-09-21.md
Implementation permission: selection recorded; execution delegated to MIRBUILDER-WARNING-RESOLVED-LOWERING-FACADE-TEST-EXPORTS-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I47
---

# MirBuilder warning baseline refresh I46

## Six-line brief

```text
Decision: refresh both warning surfaces after the I45 selected dynamic emitter
  test facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,782/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## Selection evidence

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,782 | `/tmp/hakorune-warning-i46-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i46-lib-test-20260921.log` |

The selected caller-zero cohort is the test-only facade exports in
`src/mir/builder/resolved_lowering/mod.rs`:

* `:67` re-exports `CommonV2S6CTextCursorPreheaderRejectV1`; production
  child code owns and imports this type directly.
* `:71` re-exports the scalar-equality leaf shape and issuer; the child
  `common_v2_session/mod.rs:67` re-export is part of the same chain. Observed
  parent consumers are test-only content-root fixtures, while session internals
  use their own module path.
* `:78` re-exports `with_common_v2_physical_entry_session`; all observed
  consumers are resolved-lowering test modules.
* `:92` re-exports four dynamic capability types; observed external consumers
  are test-only normal-callable semantic package fixtures.

The bounded execution slice gates these four parent facade groups and the
scalar child re-export with `#[cfg(test)]`; child issuers, physical session
owners, and capability semantics remain unchanged.

## Closeout evidence

The delegated resolved-lowering facade slice completed with the fixed gates,
sequentially:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,778 | `/tmp/hakorune-warning-i0-resolved-lowering-facade-test-exports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-resolved-lowering-facade-test-exports-lib-test-20260921.log` |

Both commands exited 0. Four parent facade groups and the dependent scalar
child re-export are now test-only; child issuers, physical session owners, and
capability semantics remain unchanged. I46 is closed, and I47 is the next
design-stop baseline refresh.
