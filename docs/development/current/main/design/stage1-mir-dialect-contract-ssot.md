---
Status: SSOT
Scope: `Stage0 keep` / `Stage1 mainline` で受け入れる MIR call dialect を固定し、pure-first no-replay cutover の exact blocker を 1 本化する。
Decision: accepted
Updated: 2026-03-26
Related:
- docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
- docs/development/current/main/design/mir-callsite-retire-lane-ssot.md
- docs/development/current/main/design/stage1-mir-authority-boundary-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/selfhost-compiler-structure-ssot.md
- docs/development/current/main/phases/phase-29ck/README.md
- docs/development/current/main/phases/phase-29ck/P14-PURE-FIRST-NO-REPLAY-CUTOVER.md
- lang/src/runner/stage1_cli_env.hako
- lang/src/mir/builder/MirBuilderBox.hako
- lang/src/mir/builder/func_lowering/call_methodize_box.hako
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
- `Stage1 -> Stage2` の MIR dialect policy は `.hako` mainline 側へ寄せ、Rust は current materializer residue を持つ thin seam へ降格する。

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

### Producer authority

- preferred Stage1 canonical owner:
  - `lang/src/runner/stage1_cli_env.hako`
  - `lang/src/mir/builder/MirBuilderBox.hako`
  - `lang/src/mir/builder/func_lowering/call_methodize_box.hako`
- current truth:
  - `.hako` mirbuilder 側には `call -> mir_call(Method)` へ寄せる canonicalization parts がある
  - Stage1/Stage2 migration policy としては、call dialect meaning should live here

### Producer materializer seam

- active Rust MIR JSON producer:
  - `src/runner/mir_json_emit/emitters/calls.rs`
- current truth:
  - `NYASH_MIR_UNIFIED_CALL` が OFF のとき、`Callee::Method` は `boxcall` を emit する
  - `Callee::Constructor` / `Callee::Global` は v0 `call` + `callee` を keep している
  - this seam is still live today and still materializes visible call dialect choice
  - therefore it is not yet a pure serializer; it is a materializer seam that must be demoted

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
4. `.hako` side already has methodize/canonical pieces, so keeping dialect policy in Rust is not the target end state

したがって、first cut は `boxcall` を pure-first owner に広く足すことではない。

## Canonicalization Rule

- canonicalization point は 1 owner に寄せる
- preferred first owner:
  - `.hako` Stage1 mainline producer route
  - concretely:
    - `lang/src/runner/stage1_cli_env.hako`
    - `lang/src/mir/builder/MirBuilderBox.hako`
    - `lang/src/mir/builder/func_lowering/call_methodize_box.hako`
- current materializer seam owner:
  - `src/runner/mir_json_emit/emitters/calls.rs`
- do not:
  - widen `hako_llvmc_ffi_pure_compile.inc` to broad `boxcall` support as a parallel Stage1 dialect
  - spread ad-hoc callsite rewrites across producer, normalizer, and consumer in the same wave

## Fixed Order

1. expose the dialect split in docs + probe
2. make the `.hako` Stage1 producer the canonical owner for method call dialect
3. demote `src/runner/mir_json_emit/emitters/calls.rs` from dialect materializer to a thin materializer/transport seam that follows the canonical owner
4. keep the normalizer pass-through in that wave
5. only after the producer/residual seam are canonical, continue pure-first semantic coverage widening

## Acceptance

- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --route hako-helper --input benchmarks/bench_kilo_kernel_small.hako`
- `tools/dev/phase29ck_stage1_mir_dialect_probe.sh --mir-json <canonical-mir.json> --strict-stage1`
- `HAKO_BACKEND_COMPILE_RECIPE=pure-first HAKO_BACKEND_COMPAT_REPLAY=none ... tools/ny_mir_builder.sh ...`

## Exit Condition

- the repo can name exactly:
  - which `.hako` owner should own Stage1 canonical call dialect
  - which Rust seam still materializes legacy `boxcall`
  - who merely passes it through
  - who refuses to consume it as Stage1 mainline
- the next exact code front is `.hako` canonical producer cutover plus Rust materializer demotion, not broad pure-first dual-dialect support
