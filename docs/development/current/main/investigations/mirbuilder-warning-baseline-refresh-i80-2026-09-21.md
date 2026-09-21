---
Status: closeout__2026-09-21__BaselineStable__I81Selected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I80
Date: 2026-09-21
Parent: mirbuilder-warning-macro-transform-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: select one caller-zero facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I80

## Six-line brief

```text
Decision: refresh the warning surfaces after the macro transform test-scope
  closeout and choose one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,742/557, census one
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
owners. A selected deletion must prove caller-zero before physical removal,
then run its focused gate and the stable warning refresh at its parent/current
pair.

## I79 and macro closeout evidence

I79 restored the rejected LoopTrue mapper candidate after focused compile failure
and recorded it as `NoSafeSlice`. The macro transform slice then scoped one
parent re-export to `cfg(test)`, leaving the canonical implementation and
policy-aware production path unchanged. Its focused suites passed LoopCond
**9/9** and normal-callable transform **7/7**; the library warning baseline is
now **1,742** and lib-test remains **557**.

## Refresh result and selected bounded edge

The sequential refresh completed with lib **1,742** warnings and lib-test
**557**, matching the macro-transform closeout. No new red or command drift was
observed. The finite candidate is the parent re-export of
`map_loop_true_source_binding_reject` in `src/mir/loop_structural_facts/mod.rs`.
The canonical mapper remains required by the test-only compiler adapter, so the
function itself is not deletable. Only its non-test parent export edge may be
scoped to `cfg(test)`; the test compiler module and focused route tests retain
the test-time path.

Selected successor:
`MIRBUILDER-WARNING-LOOP-TRUE-REJECT-MAPPER-TEST-SCOPE-I0`.

## Closeout result

The selected I81 slice first rejected an over-broad `cfg(test)` scope because
production route-policy modules consume the LoopTrue observation types. The
corrected slice split the mapper from those types, scoped only the mapper export,
and passed lib check plus LoopTrue **9/9**. Lib warnings decreased to **1,741**.
