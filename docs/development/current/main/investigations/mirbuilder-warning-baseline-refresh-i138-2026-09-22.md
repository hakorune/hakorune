---
Status: design_stop__2026-09-22__WarningBaselineRefreshI138__NoSafeSlice
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I138
Date: 2026-09-22
Parent: mirbuilder-warning-direct-accum-effect-role-test-facade-i137-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: owner-decision__joinir-loop-target-policy-or-new-warning-census
---

# MirBuilder warning baseline refresh I138

## Six-line brief

```text
Decision: refresh the warning baseline after I137 and select one finite
  production-zero cleanup candidate only after a fresh caller census.
Source authority + canonical issuer: quick-profile Cargo diagnostics, warning
  classification policy, and the selected owner's source-level references.
Non-authority: warning-count guesses, blanket allow, cargo-fix, AST rescans,
  inferred route identity, or new semantic receipts.
Fail-fast boundary: unclassified red, a non-test caller, changed role/route
  inventory, or a missing focused guard returns to design_stop.
Smallest next slice: one declaration-only production-zero item or one
  caller-zero old edge with an explicit successor proof.
Non-claims: no broad warning sweep, parser semantic expansion, VM repair,
  fallback, backend promotion, or LegacyCallV0 retirement.
```

## I137 closeout baseline

I137 physically deleted the DirectAccum effect-entry `role` accessor and
rewrote the existing witness to use `ALL` plus the sealed `entry(role)` lookup.
The focused test was **1/1**, lib was **1,685 warnings**, and lib-test was
**544 warnings**. Format, check, test binary build, pointer guard, qualified
absence search, and diff checks passed. The LoopBreak old-edge remains
`NoSafeSlice` and is not reopened by this refresh.

## Selection gate

Run one quick-profile baseline refresh and classify newly measured warnings as
`current-change failure`, `known baseline debt`, or `informational census`.
For a candidate, record its declaration, all callers, owner authority,
focused guard, and exact delete set. Select at most one bounded fast slice;
otherwise remain in `design_stop` with the blocking premise recorded.

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
bash tools/checks/current_state_pointer_guard.sh
```

## Selection result

The post-I137 baseline is lib **1,685** and lib-test **544**. The remaining
candidate with the clearest caller-zero shape is the
`is_loop_lowered_function`/`loop_target_policy` family under
`src/mir/join_ir/lowering/`. Its Rust production caller census is empty, but
the tracked JoinIR thinning SSOT and module README explicitly name
`loop_target_policy.rs` as the active five-name classification authority.
Deleting or test-gating it would therefore change the documented authority
chain, even though the current compiler does not call the wrapper. That is a
design decision rather than a warning-only cleanup, so this candidate is
`NoSafeSlice` and is not selected.

The other measured warnings either have production references, belong to
future source/Recipe owners, or are grouped diagnostics whose delete set is
not finite without a semantic decision. No additional fast cohort is selected
until the target-policy owner is reconciled or a new declaration-only row is
proven with the same caller census.
