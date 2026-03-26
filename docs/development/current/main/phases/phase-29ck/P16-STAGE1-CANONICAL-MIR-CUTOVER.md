---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `Stage1 = ny-llvmc(boundary pure-first)` mainline lane の call dialect を `mir_call` へ寄せ、legacy `boxcall` emit を current producer owner から外す。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P15-STAGE1-MIR-DIALECT-INVENTORY.md
  - docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
  - docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
  - docs/development/current/main/design/mir-callsite-retire-lane-ssot.md
  - src/runner/mir_json_emit/emitters/calls.rs
  - src/runner/modes/common_util/selfhost/json.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - tools/dev/phase29ck_stage1_mir_dialect_probe.sh
---

# P16: Stage1 Canonical MIR Cutover

## Purpose

- `Stage1` mainline/perf lane の callsite family を `mir_call` へ寄せる。
- no-replay kilo を broad dual-dialect support で通さず、producer owner を canonical にする。
- `llvmlite` keep lane と `ny-llvmc` mainline lane の責務を code でも揃える。

## First Exact Owner

- `src/runner/mir_json_emit/emitters/calls.rs`

## Fixed Order

1. stop emitting method `boxcall` from the active Stage1 producer path
2. keep `jsonfrag_normalizer_box.hako` as pass-through in this wave
3. keep strict/dev reject contract in `src/runner/modes/common_util/selfhost/json.rs`
4. only after canonical producer output is confirmed, resume pure-first semantic widening in `hako_llvmc_ffi_pure_compile.inc`

## Current Target

1. Stage1 mainline route should no longer rely on `NYASH_MIR_UNIFIED_CALL=0` compatibility for method calls
2. current kilo mainline MIR should be probeable without `boxcall`
3. constructor/global/value callsite handling stays scoped to the same producer owner; do not widen consumer and producer in the same commit

## Acceptance

- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --route hako-helper --input benchmarks/bench_kilo_kernel_small.hako --strict-stage1`
- `cargo check --bin hakorune`
- `bash tools/checks/dev_gate.sh quick`
- `HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in /tmp/kilo_kernel_small.mir.json --emit exe -o /tmp/kilo_kernel_small.exe --quiet`

## Non-Goals

- broad `boxcall` acceptance inside `hako_llvmc_ffi_pure_compile.inc`
- `copy` / `newbox` retirement
- `.hako` mirbuilder route rework in the same wave
- keep-lane `llvmlite` removal

## Exit Condition

- current Stage1 producer no longer emits method `boxcall`
- next exact blocker is a real pure-first semantic unsupported family, not a dialect mismatch
