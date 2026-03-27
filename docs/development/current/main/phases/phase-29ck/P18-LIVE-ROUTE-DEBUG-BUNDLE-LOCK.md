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

## Fixed Facts

1. current blocker は「route が壊れていること」ではなく「live route evidence が薄いこと」だよ
2. `kilo_micro_array_getset` source route では semantic `array_rmw_window` が same artifact で証明済みだよ
3. current micro leaf proof はこれだよ
   - `array_rmw_window result=hit`
   - lowered IR contains `nyash.array.rmw_add1_hi`
   - built binary exports `nyash.array.rmw_add1_hi`
4. current main route has one same-artifact direct hit on `array_string_len_window`
5. rejected follow-up:
   - same-artifact `array_string_indexof_window result=hit` was proven
   - lowered IR still contained both `nyash.array.slot_load_hi` and `nyash.array.string_indexof_hih`
   - stable main regressed to `853 ms`
   - `kilo_micro_indexof_line = 9 ms`
6. current main route still has observed misses `post_len_uses_consumed_get_value` and `next_noncopy_not_len`
7. current array leaf は adjacency recipe ではなく semantic window recipe として読む
8. `leaf-proof micro` lane is now landed:
   - `kilo_leaf_array_rmw_add1 = 36 ms`
   - `kilo_leaf_array_string_len = 15 ms`
   - `kilo_leaf_array_string_indexof_const` currently fails AOT build with a pure-first coverage gap

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
5. micro route で same-artifact hit を取る
6. main route の hit/miss reason を bundle で固定する
7. その evidence を見てからだけ next observer leaf を widen する
8. do not reopen an observer cut that still leaves `slot_load_hi` in the same hot block
9. current next exact blocker is the leaf-proof `get -> indexOf("line")` shape before returning to `micro kilo`

## Acceptance

- current front が `P18` へ進んでいる
- `[llvm-route/trace]` の stage names が docs と code で一致している
- bundle script で current live route の MIR/IR/symbol proof を 1 ディレクトリに残せる
- current micro fused leaf が same artifact で証明されている
- current main direct string-observer leaf が same artifact で証明されている
- next leaf design が current main miss reason widening として読める

## Non-Goals

- lock swap の再試行
- broad `ArrayBox` storage redesign
- new public MIR/AOT-Core layer

## Exit Condition

- micro route の `symbol miss` が `reason unknown` で残らない
- `array_rmw_window` を current live route で証明できる
- current main route の direct observer leaf が same artifact で証明されている
- current main route の next widening target が miss reason 付きで固定されている
