---
Status: SSOT
Scope: `phase-29ck` native perf wave で、route miss / trigger miss / symbol miss を 1 本の evidence bundle で説明できるようにする。
Related:
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md
  - docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
  - tools/perf/trace_optimization_bundle.sh
---

# Stage2 Optimization Debug Bundle SSOT

## Goal

この文書は、`ny-llvmc(boundary pure-first)` の最適化 wave で
「leaf symbol が出ない」「route が合っていない」「MIR 形が想定と違う」
を shell history ではなく 1 本の evidence bundle で説明するための正本だよ。

目的は 2 つだけ。

1. perf reject を `reason unknown` で終わらせない。
2. 次の leaf/recipe を live route の証拠付きで切る。

## Fixed Reading

1. `asm` は嘘をつかないが、asm だけでは route miss / trigger miss / DCE miss を区別できない。
2. `symbol not found` は backend bug と読まない。
3. まず証明すべき順番は次。
   1. source/MIR route
   2. MIR window shape
   3. emitted IR
   4. binary symbol proof
   5. repeated perf

## Bundle Contract

optimization bundle は最低でも次を 1 ディレクトリに残す。

1. source or MIR input copy
2. route-aware emitted MIR JSON
3. MIR op/callee histogram
4. MIR window probe
5. no-replay build log
6. LLVM IR dump
7. symbol proof
8. optional micro perf top report

保存先の既定は次。

- `target/perf_state/optimization_bundle/<timestamp>-<label>/`

## Window Reading

window probe は「隣接 3 命令列」を前提にしない。

- `const`
- `copy`
- `phi`

は transparent carrier として扱ってよい。

逆に、window の witness は次で固定する。

1. same semantic receiver
2. same semantic index/key
3. load result からの pure transform
4. write back sink
5. optional reread / accumulator use

current `phase-29ck` の array family では、
adjacent-op 仮説よりも semantic `array RMW window` を先に見る。

## Stable Trace Tags

route/bundle wave で使う stable trace tags は次。

- `[llvm-route/select]`
- `[llvm-route/replay]`
- `[llvm-route/trace] stage=<...> result=<...> reason=<...> extra=<...>`

`phase-29ck` current window probe stage names は次で固定する。

- `generic_walk`
- `mir_call`
- `mir_call_method`
- `array_rmw_window`

新しい stage を増やすときは
`ai-handoff-and-debug-contract.md`
へ先に追記する。

## Current Phase-29ck Reading

current `kilo_micro_array_getset` で重要なのは次。

1. compat replay を切った live route を前提に読む
2. `push` / `get` / `set` の `recv_org` は `origin` だけでなく `scan_origin` も見る
3. current miss は「symbol が mysteriously 消えた」ではなく、
   live MIR shape が previous fused leaf 仮説と違っていたことにある

current known live shape は semantic にこう読む。

- `get(idx)`
- transparent `copy/phi` carriers
- `const 1`
- `add(load, 1)`
- `set(idx, added)`

したがって next leaf は adjacency ではなく
`array_rmw_window`
として再設計する。

## Tooling Contract

bundle owner は次だよ。

- `tools/perf/trace_optimization_bundle.sh`

この script は current wave で次をやる。

1. emit MIR or ingest existing MIR
2. summarize MIR shape + windows
3. build with `pure-first + compat_replay=none + route trace`
4. save `.ll`
5. save symbol inventory
6. optionally run micro perf on the same built exe

## Non-Goals

- new public MIR syntax
- immediate new `AOT-Core MIR` layer
- permanent broad fused-leaf library before live trigger proof exists
- using keep-lane artifacts as perf evidence

## Exit Condition

- `phase-29ck` reject rows can point to one bundle directory
- `symbol miss` rows have route/window/IR proof
- next accepted leaf is chosen from live evidence, not shell-memory or adjacency guesses
