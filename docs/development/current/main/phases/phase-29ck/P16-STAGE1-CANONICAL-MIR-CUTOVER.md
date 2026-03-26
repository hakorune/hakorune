---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `Stage1 = ny-llvmc(boundary pure-first)` mainline lane の call dialect を `mir_call` へ寄せ、`.hako` producer を canonical owner に固定した上で Rust residual seam の legacy `boxcall` emit を外す。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P15-STAGE1-MIR-DIALECT-INVENTORY.md
  - docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
  - docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md
  - docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
  - docs/development/current/main/design/mir-callsite-retire-lane-ssot.md
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/mir/builder/MirBuilderBox.hako
  - lang/src/mir/builder/func_lowering/call_methodize_box.hako
  - src/runner/mir_json_emit/emitters/calls.rs
  - src/runner/modes/common_util/selfhost/json.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
  - tools/dev/phase29ck_stage1_mir_dialect_probe.sh
---

# P16: Stage1 Canonical MIR Cutover

## Purpose

- `Stage1` mainline/perf lane の callsite family を `mir_call` へ寄せる。
- no-replay kilo を broad dual-dialect support で通さず、`.hako` producer owner を canonical にする。
- Rust line は immediate delete ではなく、`dialect materializer` から `thin materializer/transport seam` へ降格する。
- `llvmlite` keep lane と `ny-llvmc` mainline lane の責務を code でも揃える。

## First Exact Owner

- `.hako` Stage1 canonical producer route
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/mir/builder/MirBuilderBox.hako`
  - `lang/src/mir/builder/func_lowering/call_methodize_box.hako`

## Current Materializer Seam

- `src/runner/mir_json_emit/emitters/calls.rs`

## Fixed Order

1. make the `.hako` Stage1 producer the canonical owner for method call dialect
2. stop letting the Rust materializer seam decide method call dialect so it follows the canonical owner
3. keep `jsonfrag_normalizer_box.hako` as pass-through in this wave
4. keep strict/dev reject contract in `src/runner/modes/common_util/selfhost/json.rs`
5. only after canonical producer output is confirmed, resume pure-first semantic widening in `hako_llvmc_ffi_pure_compile.inc`

## Current Target

1. Stage1 mainline route should no longer rely on Rust-side `NYASH_MIR_UNIFIED_CALL=0` compatibility for method calls
2. current kilo mainline MIR should be probeable without `boxcall`
3. constructor/global/value callsite handling stays scoped to the same producer matrix; do not widen consumer and producer in the same commit
4. Rust should end this wave as thin materializer/transport path, not the long-term dialect-policy owner

## Landed Slice

- `src/runner/mir_json_emit/emitters/calls.rs`
  - `Callee::Method` は `HAKO_MIR_BUILDER_METHODIZE=1` を authority signal として `mir_call` に固定する
  - `NYASH_MIR_UNIFIED_CALL=0` は Stage1 method dialect を戻さない
- `src/host_providers/mir_builder.rs`
  - Phase0 keep/export lane は `HAKO_MIR_BUILDER_METHODIZE=0` を明示して legacy route を固定する
- `tools/hakorune_emit_mir.sh`
  - Stage-B / selfhost-first route は `NYASH_MIR_UNIFIED_CALL=1` を internal pin して caller env の legacy toggle を吸わない
- `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc`
  - generic pure path は `copy`, `newbox(ArrayBox)`, `release_strong` を pure-first owner として受理する
  - current kilo mainline string family (`String const`, `RuntimeDataBox.length/substring/indexOf`, string `binop +`) を current canonical `mir_call` shape に合わせて受理する
  - phi array form `[value, pred]` を正しく読む
  - discard-only call emit は `%_ = call ...` をやめて plain `call ...` に揃え、 invalid LLVM IR を止める

## Current Result

1. direct/helper/mainline probe で Stage1 method dialect は `mir_call` に揃った
2. `kilo_kernel_small_hk` は `pure-first + compat_replay=none` で `aot_status=ok` に戻った
3. current stop-line は `Stage1 MIR dialect split` ではなくなった
4. 次の exact front は `P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md` だよ
   - route correction は landed
   - perf は blind substrate try へ戻さず、proof vocabulary lock のあとで integer array fast lane へ戻る

## Acceptance

- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --route hako-helper --input benchmarks/bench_kilo_kernel_small.hako --strict-stage1`
- `cargo check --bin hakorune`
- `bash tools/checks/dev_gate.sh quick`
- `HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none NYASH_LLVM_ROUTE_TRACE=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/ny_mir_builder.sh --in /tmp/kilo_kernel_small.mir.json --emit exe -o /tmp/kilo_kernel_small.exe --quiet`
- `PERF_VM_FORCE_NO_FALLBACK=1 PERF_REQUIRE_AOT_RESULT_PARITY=0 NYASH_LLVM_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small_hk 1 1`

## Non-Goals

- broad `boxcall` acceptance inside `hako_llvmc_ffi_pure_compile.inc`
- `copy` / `newbox` retirement
- keep-lane `llvmlite` removal

## Exit Condition

- `.hako` Stage1 producer is the documented canonical callsite owner
- current Rust materializer seam no longer chooses method call dialect
- next exact blocker is a real pure-first semantic unsupported family, not a dialect mismatch
