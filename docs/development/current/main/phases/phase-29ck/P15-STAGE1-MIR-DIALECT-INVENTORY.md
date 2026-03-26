---
Status: Closed Inventory
Decision: accepted
Date: 2026-03-26
Scope: `pure-first + no-replay` current blocker を `Stage1 MIR dialect split` として固定し、next exact cutover owner を 1 本に絞る。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P14-PURE-FIRST-NO-REPLAY-CUTOVER.md
  - docs/development/current/main/phases/phase-29ck/P16-STAGE1-CANONICAL-MIR-CUTOVER.md
  - docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
  - docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
  - src/runner/mir_json_emit/emitters/calls.rs
  - lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - src/runner/modes/common_util/selfhost/json.rs
  - tools/dev/phase29ck_stage1_mir_dialect_probe.sh
---

# P15: Stage1 MIR Dialect Inventory

## Purpose

- `kilo` no-replay stop-line を broad pure-first widening と読まない。
- current mainline MIR producer / normalizer / consumer の dialect split を 1 画面で固定する。
- next exact code bucket を `src/runner/mir_json_emit/emitters/calls.rs` に絞る。

## Closed Evidence

1. current kilo mainline route emits legacy callsite shape.
   - `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --route hako-helper --input benchmarks/bench_kilo_kernel_small.hako`
   - observed current shape:
     - `boxcall=11`
     - `mir_call=0`
     - `newbox=1`
     - `copy=33`
2. current producer still owns legacy method emit.
   - `src/runner/mir_json_emit/emitters/calls.rs`
   - `NYASH_MIR_UNIFIED_CALL` OFF path emits `Callee::Method` as `boxcall`
3. current normalizer is not the canonicalization owner.
   - `lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako`
   - both `boxcall` and `mir_call` are passed through
4. current Stage1 consumer already reads stricter truth.
   - `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc` is `mir_call`-centric
   - `src/runner/modes/common_util/selfhost/json.rs` already rejects `boxcall` / `externcall` in strict/dev

## Inventory Judgment

- current exact blocker is `Stage1 MIR dialect split`
- this is a BoxShape issue, not a request to support both dialects forever
- the first cutover owner is:
  - `src/runner/mir_json_emit/emitters/calls.rs`
- the first cutover is not:
  - broad `boxcall` support in `hako_llvmc_ffi_pure_compile.inc`
  - ad-hoc rewrite inside `jsonfrag_normalizer_box.hako`

## Non-Goals

- `copy` retirement
- `newbox` retirement
- pure-first semantic widening for string/array families
- `llvmlite` keep-lane removal

## Acceptance

- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --route hako-helper --input benchmarks/bench_kilo_kernel_small.hako`
- `rg -n \"Emit as boxcall for compatibility|op == \\\"boxcall\\\"|callsite-retire:legacy-boxcall\" src/runner/mir_json_emit/emitters/calls.rs lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako src/runner/modes/common_util/selfhost/json.rs`
- `git diff --check`

## Exit Condition

- the repo can name the exact producer/normalizer/consumer split
- `P16-STAGE1-CANONICAL-MIR-CUTOVER.md` becomes the next exact front
