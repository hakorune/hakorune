---
Status: closed__2026-09-22__WarningBaselineRefreshI138__SelectedI139
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I138
Date: 2026-09-22
Parent: mirbuilder-warning-direct-accum-effect-role-test-facade-i137-2026-09-22.md
Implementation permission: false; refresh diagnostics and select one bounded next cohort
NextCard: MIRBUILDER-WARNING-LOOP-TARGET-POLICY-RETIRE-I139
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

The post-I137 baseline is lib **1,685** and lib-test **544**. The read-only
owner audit and an independent source census now resolve the earlier design
stop: `is_loop_lowered_function`, `is_loop_lowering_target`, and the
five-name table have no Rust production caller outside their own tests. The
current route owner is `loop_scope_shape/case_a.rs`, which registers only
`Main.skip/1` and `FuncScannerBox.trim/1`; the former LowerOnly/If lanes were
retired by the recorded JoinIR refactors. The current README and SSOT still
describe the stale five-name module as active, so the smallest safe slice is
to retire that BoxShape and repair the current docs and generated inventory in
the same change. The selected card is
`MIRBUILDER-WARNING-LOOP-TARGET-POLICY-RETIRE-I139`.

The other measured warnings either have production references, belong to
future source/Recipe owners, or are grouped diagnostics whose delete set is
not finite without a semantic decision. They remain outside I139.
