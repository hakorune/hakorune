---
Status: SSOT
Scope: `Stage0 keep` / `Stage1 mainline` で受け入れる MIR call dialect を固定し、pure-first no-replay cutover の exact blocker を 1 本化する。
Decision: accepted
Updated: 2026-03-26
Related:
- docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
- docs/development/current/main/design/mir-callsite-retire-lane-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/phases/phase-29ck/README.md
- docs/development/current/main/phases/phase-29ck/P14-PURE-FIRST-NO-REPLAY-CUTOVER.md
- src/runner/mir_json_emit/emitters/calls.rs
- lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako
- lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
- src/runner/modes/common_util/selfhost/json.rs
---

# Stage1 MIR Dialect Contract (SSOT)

## Purpose

- `llvmlite` keep lane と `ny-llvmc(boundary pure-first)` mainline lane の責務を MIR dialect でも分離する。
- `pure-first + compat_replay=none` の current blocker を「generic coverage 不足」ではなく、まず `Stage1 dialect split` として固定する。
- pure-first owner に legacy `boxcall` を広く足す前に、mainline producer / normalizer / consumer の責務を 1 本化する。

## Contract

### Stage0 keep lane

- `llvmlite` / harness / explicit compat probe は legacy callsite を保持してよい。
- accepted surface:
  - `boxcall`
  - `externcall`
  - `call`
  - `newbox`
  - `copy`
- これは keep/debug/probe lane であり、perf/mainline judge ではない。

### Stage1 mainline lane

- `ny-llvmc(boundary pure-first)` が `Stage1` daily/mainline/perf owner だよ。
- canonical call dialect は `mir_call` に固定する。
- accepted surface for the callsite family:
  - `mir_call`
- non-goal for the first cutover:
  - `copy` は generic data op としてこの lane の問題にしない
  - `newbox` は別 judgment に分ける
- not allowed as a Stage1 mainline callsite:
  - `boxcall`
  - `externcall`

## Current Split Matrix

### Producer

- active Rust MIR JSON producer:
  - `src/runner/mir_json_emit/emitters/calls.rs`
- current truth:
  - `NYASH_MIR_UNIFIED_CALL` が OFF のとき、`Callee::Method` は `boxcall` を emit する
  - `Callee::Constructor` / `Callee::Global` は v0 `call` + `callee` を keep している

### Normalizer

- `lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako`
- current truth:
  - `boxcall` と `mir_call` をどちらも pass-through する
  - canonicalization owner ではない

### Consumer

- pure-first generic owner:
  - `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc`
- current truth:
  - generic lane は `mir_call` を読む
  - broad `boxcall` generic acceptance は持たない

### Stage1 strict/dev guard

- `src/runner/modes/common_util/selfhost/json.rs`
- current truth:
  - strict/dev では `boxcall` / `externcall` を `[freeze:contract][callsite-retire:*]` で reject する

## Current Exact Blocker

`pure-first + compat_replay=none` の kilo stop-line は、まず coverage widening ではなく `Stage1 MIR dialect split` だよ。

1. current mainline kilo MIR still emits legacy callsites
   - observed shape is `newbox/copy/boxcall`
2. current pure-first generic owner is `mir_call`-centric
3. strict/dev selfhost route already treats `boxcall` as retired

したがって、first cut は `boxcall` を pure-first owner に広く足すことではない。

## Canonicalization Rule

- canonicalization point は 1 owner に寄せる
- preferred first owner:
  - `src/runner/mir_json_emit/emitters/calls.rs`
- do not:
  - widen `hako_llvmc_ffi_pure_compile.inc` to broad `boxcall` support as a parallel Stage1 dialect
  - spread ad-hoc callsite rewrites across producer, normalizer, and consumer in the same wave

## Fixed Order

1. expose the dialect split in docs + probe
2. make the active Stage1 producer stop emitting method `boxcall`
3. keep the normalizer pass-through in that wave
4. only after the producer is canonical, continue pure-first semantic coverage widening

## Acceptance

- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --route hako-helper --input benchmarks/bench_kilo_kernel_small.hako`
- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --mir-json <canonical-mir.json> --strict-stage1`
- `HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none ... tools/ny_mir_builder.sh ...`

## Exit Condition

- the repo can name exactly:
  - who emits legacy `boxcall`
  - who merely passes it through
  - who refuses to consume it as Stage1 mainline
- the next exact code front is Stage1 producer cutover, not broad pure-first dual-dialect support
