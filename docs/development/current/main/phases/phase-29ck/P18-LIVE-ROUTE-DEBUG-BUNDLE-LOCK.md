---
Status: Task Pack
Decision: accepted
Date: 2026-03-27
Scope: `phase-29ck` perf wave を blind fixed-cost reduction から live-route proof first に戻し、optimization debug bundle と semantic window probe を current exact front に固定する。
Related:
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md
  - docs/development/current/main/design/stage2-optimization-debug-bundle-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
  - tools/perf/trace_optimization_bundle.sh
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P18: Live-Route Debug Bundle Lock

## Purpose

- `P17` で proof vocabulary を固定したあと、perf wave を blind leaf guessing に戻さない。
- live route / live MIR / live IR / live symbol の 1 本通し evidence を current exact front に固定する。
- `array get/set` family の next cut を adjacency guess ではなく semantic window recipe に切り替える。

## Preconditions

1. `P17` は landed
2. `kilo_kernel_small_hk` は `pure-first + compat_replay=none` で green
3. array substrate の broad representation split は current wave で reject 済み
4. backend-private fused `get -> +const -> set -> get` leaf は trigger proof なしで reject 済み

## Fixed Reading

1. current blocker は「route が壊れていること」ではなく「live route evidence が薄いこと」だよ
2. current next task は new leaf implementation より先に debug bundle を固定することだよ
3. current array leaf は adjacency recipe ではなく semantic `array_rmw_window` として読む

## Fixed Order

1. stable trace tag を SSOT に追加する
2. reusable bundle script を landed させる
3. same bundle で
   - route trace
   - MIR window
   - IR
   - symbol
   - optional perf
   を 1 directory に束ねる
4. `kilo_micro_array_getset` current live route をその bundle で取り直す
5. その evidence を見てからだけ next leaf/peephole を切る

## Acceptance

- current front が `P18` へ進んでいる
- `[llvm-route/trace]` の stage names が docs と code で一致している
- bundle script で current live route の MIR/IR/symbol proof を 1 ディレクトリに残せる
- next leaf design が `array_rmw_window` で読める

## Non-Goals

- immediate new fused leaf keep
- lock swap の再試行
- broad `ArrayBox` storage redesign
- new public MIR/AOT-Core layer

## Exit Condition

- current wave の exact next patch が bundle evidence を起点に選べる
- `symbol miss` が `reason unknown` で残らない
- `array_rmw_window` を current live route で証明できる
