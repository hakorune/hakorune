---
Status: closed__2026-09-22__WarningLoopTargetPolicyRetireI139
Task: MIRBUILDER-WARNING-LOOP-TARGET-POLICY-RETIRE-I139
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i138-2026-09-22.md
Implementation permission: true; one stale BoxShape retirement
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I140
---

# MirBuilder warning loop-target policy retirement I139

## Six-line brief

```text
Decision: delete the stale five-name loop_target_policy and its wrapper;
  current two-target Case-A lowering remains the route owner.
Source authority + canonical issuer: lowering/loop_scope_shape/case_a.rs and
  the existing skip/trim target-specific lowerers.
Non-authority: the old five-name table, wrapper tests, warning text, and
  retired LowerOnly/If documentation.
Fail-fast boundary: any non-test caller, generated-manifest mismatch, changed
  skip/trim route, focused red, or unresolved current-doc authority.
Smallest next slice: remove the stale module/wrapper/tests, update current
  README/SSOT and generated inventory, then run focused JoinIR guards.
Non-claims: no semantic target expansion, backend or VM change, fallback,
  parser promotion, or LegacyCallV0 retirement.
```

## Finite census and delete set

Boundary: `src/mir/join_ir/lowering/mod.rs` ->
`src/mir/join_ir/lowering/loop_scope_shape/case_a.rs`.

Includes the caller-zero wrapper and module, their dedicated tests, the two
current documentation descriptions, and generated manifest/inventory rows.
It excludes archive/history documents, active `skip_ws` and
`funcscanner_trim` lowerers, and all unrelated warning cohorts.

The pre-change census is finite:

- `is_loop_lowered_function`: definition plus same-file tests only;
- `is_loop_lowering_target` and `LOOP_LOWERING_TARGETS`: module-local only;
- no source caller outside `loop_target_policy.rs` and `lowering/mod.rs`;
- active Case-A target registration is `Main.skip/1` and
  `FuncScannerBox.trim/1` in `loop_scope_shape/case_a.rs`.

Delete exactly:

1. `src/mir/join_ir/lowering/loop_target_policy.rs`;
2. its `pub(crate)` module declaration, wrapper, and dedicated test module in
   `src/mir/join_ir/lowering/mod.rs`;
3. the three obsolete generated test-manifest rows;
4. the two obsolete source-inventory rows.

Update `src/mir/join_ir/lowering/README.md` and the current
`joinir-target-lowerer-thinning-ssot.md` so the documented route authority is
the two-target Case-A owner. Archive documents are not rewritten.

## Acceptance

- focused Case-A/skip/trim tests remain green;
- `cargo fmt --all -- --check`, quick library check, and test-binary build are
  green when run one Cargo process at a time;
- current-state pointer guard and `git diff --check` are green;
- qualified absence confirms no source or current-doc reference to the retired
  policy remains, apart from this receipt and its parent census;
- the closeout records measured warning and test-manifest counts rather than
  estimating them.

## Closeout receipt

The stale module, wrapper, and dedicated tests were deleted. The generated
test manifest lost exactly three rows and the lifecycle inventory lost the two
source rows for the retired symbols. Current README and SSOT now identify
`loop_scope_shape/case_a.rs` as the two-target Case-A authority; no active
source, test, or tools reference to the retired names remains.

Measured local evidence:

- `case_a` focused guard: **5/5** (positive, negative, and structural-shape
  checks);
- `cargo fmt --all -- --check`: **pass**;
- `CARGO_BUILD_JOBS=4 cargo check --profile quick --lib`: **pass**, lib
  warnings **1,677** (I137 baseline 1,685);
- `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib --no-run`: **pass**,
  lib-test warnings **544**;
- `current_state_pointer_guard.sh`: **pass**; `git diff --check`: **pass**.

No backend, VM, parser, fallback, or LegacyCallV0 behavior was changed.
