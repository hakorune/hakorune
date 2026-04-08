---
Status: investigation
Date: 2026-04-08
Scope: `phase-137x` の current front `kilo_micro_substring_only` で rejected にした substring-local perf cut を 1 本の rolling ledger に固定する
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/exports/string_helpers/cache.rs
  - crates/nyash_kernel/src/exports/string_helpers/materialize.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/tests/string.rs
---

# Phase137x Substring Rejected Optimizations (2026-04-08)

## Scope

- current `substring` perf wave の rejected attempts を 1 本の rolling ledger に残す
- short summary だけを `CURRENT_TASK.md` / `phase-137x/README.md` に残し、exact evidence はここへ集約する
- failed perf cuts は test ごとの folder に分けない
- current accept gate は `WSL 3 runs + perf` だよ

## Current Reading

- active front is `kilo_micro_substring_only`
- current exact baseline is:
  - `kilo_micro_substring_only: C 3 ms / AOT 5 ms`
  - `instr: 58,672,982`
  - `cycles: 9,979,794`
  - `cache-miss: 9,939`
  - split exact reread:
    - `kilo_micro_substring_views_only: instr=37,073,017 / cycles=6,804,272 / cache-miss=9,648`
    - `kilo_micro_len_substring_views: instr=22,672,209 / cycles=3,991,125 / cache-miss=8,789`
- current whole-kilo health is:
  - `tools/checks/dev_gate.sh quick`: green
  - `kilo_kernel_small_hk` strict accepted reread: `755 ms`
  - parity: ok
- current landed truth:
  - `substring_hii` can reissue a fresh handle from a cached `StringViewBox` object after transient drop-epoch churn if the source handle still names the same live source object
  - `str.substring.route` observe read is dominated by `view_arc_cache_handle_hit=599,998 / total=600,000`
  - the current keeper removed redundant `view_enabled` state from `SubstringViewArcCache`; that cache only runs on the `view_enabled` route
  - split exact reread separated `substring_hii` and `len_h`; the mixed-front keeper in this pass comes from `len_h`, not substring publication/reissue
  - the current keeper also keeps `len_h` trace-off steady state thin by tail-calling a tiny fast-return helper from `string_len_export_impl()`
- current stop-line:
  - do not widen substring runtime cache mechanics into `.hako` or `MIR`
  - keep `kilo_micro_substring_only` as the accept gate, but use split exact fronts before retrying substring-local structural cuts

## Operational Rule

- 1 cut = 1 local hypothesis に戻す
- exact front instruction win を primary gate にする
- whole-kilo strict が良くても、active exact baseline を beat できなければ keep しない
- probe code is reverted immediately when the cut is rejected

## Rejected Attempts

### 2026-04-08: broad `NyashBox` substring-source contract widening

**Hypothesis**

- `StringBox` / `StringViewBox` / planner branching を `NyashBox` trait hook に寄せれば
- `substring` source inspect と planner branching の helper density を減らせる
- ついでに `ViewSpan` birth 時の second source reread も消せる

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [string_view.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_view.rs)
- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- exact:
  - `instr=63,473,200`
  - `cycles=10,381,776`
  - `cache-miss=10,009`
  - `ny_aot_ms=5`
- whole:
  - `kilo_kernel_small_hk strict = 780 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- cycles は少し改善したが instruction baseline を超えられなかった
- strict whole も `735 -> 780 ms` で悪化した
- trait surface widening の cost と local cut の効果が混ざりすぎて、current lane の next move としては広すぎた

**Reopen Condition**

- substring runtime cache mechanics 自体を higher layer へ持ち上げる structural reason が別 lane で生まれた時だけ

### 2026-04-08: `substring_view_arc_cache_lookup` / `entry_hit` hot-path fusion

**Hypothesis**

- cache lookup helper を fused して
- `key match -> live handle return -> refresh` を straight-line に寄せれば
- `substring_hii` の hot branch instruction を減らせる

**Touched owner area**

- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)
- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- exact:
  - `instr=63,472,979`
  - `cycles=10,419,995`
  - `cache-miss=9,693`
  - `ny_aot_ms=5`
- whole:
  - `kilo_kernel_small_hk strict = 821 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- fused source はきれいでも exact baseline を beat できなかった
- whole strict は大きく悪化して `821 ms` まで戻った
- current machine では helper fusion だけで enough win が出ていない

**Reopen Condition**

- lookup counters で `live_handle_return` 比率が dominant と示せた上で
- hot branch shape をさらに狭くした second attempt が作れる時だけ

### 2026-04-08: birth-side second `with_handle(...)` removal via planner-local source metadata carry

**Hypothesis**

- planner result に source metadata を carry すれば
- `ViewSpan` birth 直前の second `with_handle(...)` を落とせる
- source reread cost を birth side から抜ける

**Touched owner area**

- [string_view.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_view.rs)
- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- exact:
  - `instr=63,472,398`
  - `cycles=10,327,503`
  - `cache-miss=10,078`
  - `ny_aot_ms=5`
- whole:
  - `kilo_kernel_small_hk strict = 711 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- strict whole は良化したが active exact baseline を超えられなかった
- current lane は `substring_hii` exact front が accept gate なので keeper にできない
- metadata carry 自体は narrow だが、current front では enough instruction win にならなかった

**Reopen Condition**

- planner-local carrier reshape を broader publication-shape cut に吸収できる時だけ

### 2026-04-08: reissue-side slot carry / `refresh_handle` rematch removal

**Hypothesis**

- cache slot が already knows source identity なら
- `refresh_handle(...)` での rematch を削って fresh handle reissue を細くできる
- dead-handle side instruction を減らせる

**Touched owner area**

- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)
- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- exact:
  - `instr=63,473,149`
  - `cycles=10,419,469`
  - `cache-miss=9,293`
  - `ny_aot_ms=5`
- whole:
  - `kilo_kernel_small_hk strict = 718 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- strict whole は healthy でも exact instructions が baseline を上回った
- refresh path carry を足しても hot exact front の keeper 条件を満たさなかった
- current front の main driver は refresh protocol 全体ではなく、more local な publication/read shape の可能性が高い

**Reopen Condition**

- refresh-side counters で `needs_refresh` / `reissue_hit` が dominant と示せた時だけ

### 2026-04-08: concrete `Arc<StringViewBox>` cache carrier narrowing

**Hypothesis**

- cache carrier を `Arc<dyn NyashBox>` から `Arc<StringViewBox>` に narrow すれば
- dynamic dispatch / trait-object shape を減らして
- view publication / cache reuse の fixed cost を削れる

**Touched owner area**

- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)
- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- exact:
  - `instr=63,472,647`
  - `cycles=10,287,843`
  - `cache-miss=10,104`
  - `ny_aot_ms=5`
- whole:
  - `kilo_kernel_small_hk strict = 703 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- whole strict はこの束で一番良かったが、exact instruction baseline を still beat できなかった
- carrier narrowing 単体は clean でも、current front では primary gate に届かなかった
- current lane では whole-only win より exact-front instruction win を優先する

**Reopen Condition**

- backend-private thinner `BorrowView` ticket cut の一部として carrier shape を再編する時だけ

### 2026-04-08: `len_h` cache-first reorder

**Hypothesis**

- `len_h` の entry で dispatch より先に fast cache を見れば
- split `kilo_micro_len_substring_views` で hot な stable-view length read をほぼ straight-line にできる
- exact front の instruction をさらに大きく削れる

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [materialize.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/materialize.rs)
- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- split exact:
  - `kilo_micro_len_substring_views: instr=23,272,882 / cycles=4,413,037 / cache-miss=9,776 / AOT 3 ms`
- mixed exact:
  - `kilo_micro_substring_only: instr=59,272,861 / cycles=10,111,083 / cache-miss=9,140 / AOT 4 ms`
- whole:
  - `kilo_kernel_small_hk strict = 898 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- split exact と mixed exact は強く良化した
- ただし whole strict が `898 ms` まで悪化した
- dispatch-active / miss-heavy lane でも cache probe を先に払う shape は、この lane の keeper 条件に合わない

**Reopen Condition**

- fast-hit bias が shared exact family で支配的と示せて
- dispatch-active whole lane への escape hatch を同時に作れる時だけ

### 2026-04-08: `drop_epoch_if_ready()` fast accessor probe

**Hypothesis**

- `string_len_fast_cache_lookup()` の steady-state が already ready なら
- `drop_epoch()` の full helper を通さずに fast accessor で済ませられる
- `kilo_micro_len_substring_views` の epoch compare cost をさらに削れる

**Touched owner area**

- [host_handles.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs)
- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- variant A (`None => miss`):
  - split exact:
    - `kilo_micro_len_substring_views: instr=22,072,405 / cycles=4,389,400 / cache-miss=10,257 / AOT 5 ms`
  - mixed exact:
    - `kilo_micro_substring_only: instr=58,072,406 / cycles=9,677,483 / cache-miss=9,908 / AOT 5 ms`
  - whole:
    - `kilo_kernel_small_hk strict = 863 ms`, probe outlier `1003 ms`
    - parity ok
- variant B (`None => drop_epoch()` fallback):
  - split exact:
    - `kilo_micro_len_substring_views: instr=23,272,188 / cycles=4,932,870 / cache-miss=9,730 / AOT 4 ms`
  - mixed exact:
    - `kilo_micro_substring_only: instr=59,272,204 / cycles=9,981,287 / cache-miss=9,306 / AOT 5 ms`
  - whole:
    - `kilo_kernel_small_hk strict = 749 ms`, rerun `764 ms`
    - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- fail-closed variant は split exact の instruction win があっても whole strict を大きく壊した
- fallback variant は whole を守れても exact win がほぼ消えた
- runtime handle API surface を増やす割に keeper 条件を満たさなかった

**Reopen Condition**

- host handle layer で drop-epoch read が cross-family hotspot と示せた時だけ

### 2026-04-08: global `dispatch` / `trace` false-state fast probes

**Hypothesis**

- `string_dispatch_raw()` と `jit_trace_len_enabled()` の false-state を
  `string_len_export_impl()` の外で fast probe すれば
- `len_h` steady-state の guard cost をさらに thin にできる
- split `kilo_micro_len_substring_views` と mixed exact の両方で keeper を狙える

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs)
- [string_debug.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_debug.rs)

**Observed result**

- split exact:
  - `kilo_micro_len_substring_views: instr=22,672,636 / cycles=4,057,151 / cache-miss=9,018 / AOT 4 ms`
- mixed exact:
  - `kilo_micro_substring_only: instr=58,672,598 / cycles=10,039,095 / cache-miss=9,574 / AOT 5 ms`
- whole:
  - `kilo_kernel_small_hk strict = 770 ms`, rerun `782 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- exact は current keeper と同じ帯まで動いた
- ただし same-machine baseline `749 ms` / `754 ms` に対して strict whole が `770 ms` / `782 ms` へ悪化した
- global false-state probes は `len_h` exact には効いても、whole lane での branch/layout side effect が still too large だった

**Reopen Condition**

- whole strict で同じ global helpers が actually hot と asm で示せる時だけ

### 2026-04-08: `len_h` dispatch-hit cold split

**Hypothesis**

- dispatch hit path を cold helper に逃がせば
- `string_len_export_impl()` の hot body から dispatch call setup を抜ける
- exact front の i-cache / branch layout が少し軽くなる

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- split exact:
  - `kilo_micro_len_substring_views: instr=22,672,426 / cycles=4,021,740 / cache-miss=8,743 / AOT 3 ms`
- mixed exact:
  - `kilo_micro_substring_only: instr=58,672,410 / cycles=9,909,575 / cache-miss=9,872 / AOT 5 ms`
- whole:
  - `kilo_kernel_small_hk strict = 734 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- whole strict は良かった
- ただし exact instruction の改善量が小さすぎて noise band を出なかった
- dispatch body を cold に逃がしただけでは `len_h` entry 先頭の main compare chain は残った

**Reopen Condition**

- dispatch-hit lane が別 exact front で支配的になった時だけ

### 2026-04-08: `trace_len_state()` helper / trace cache single-load probe

**Hypothesis**

- trace flag を `bool` ではなく state で返せば
- steady-state `len_h` fast return で `JIT_TRACE_LEN_ENABLED_CACHE` load を 1 回に寄せられる
- ideal asm の `trace-state load once` に近づける

**Touched owner area**

- [string_debug.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_debug.rs)
- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- split exact:
  - `kilo_micro_len_substring_views: instr=22,672,716 / cycles=4,051,113 / cache-miss=9,204 / AOT 4 ms`
- mixed exact:
  - `kilo_micro_substring_only: instr=58,672,971 / cycles=9,982,147 / cache-miss=9,782 / AOT 5 ms`
- asm:
  - `nyash.string.len_h` still reloads `JIT_TRACE_LEN_ENABLED_CACHE` on the non-zero path

**Verdict**

- rejected
- reverted immediately

**Why**

- exact gate を超えなかった
- source shape は変わっても emitted asm の double trace-cache load は残った
- current lane では helper vocabulary の追加より emitted hot block の実変化が必要

**Reopen Condition**

- trace-state helper が another hot family でも共通化できる時だけ

### 2026-04-08: `len_h` two-slot pre-match + single epoch-guard probe

**Hypothesis**

- primary/secondary handle match を先に決めて
- shared `drop_epoch` compare を 1 回へ寄せれば
- alternating 2-slot front の compare chain を short にできる

**Touched owner area**

- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- exact rerun A:
  - `kilo_micro_len_substring_views: instr=22,672,154 / cycles=4,036,620 / cache-miss=8,828 / AOT 4 ms`
  - `kilo_micro_substring_only: instr=58,672,650 / cycles=9,871,677 / cache-miss=9,501 / AOT 5 ms`
- exact rerun B:
  - `kilo_micro_len_substring_views: instr=22,672,879 / cycles=3,990,150 / cache-miss=8,978 / AOT 3 ms`
  - `kilo_micro_substring_only: instr=58,672,469 / cycles=9,864,252 / cache-miss=10,073 / AOT 4 ms`
- asm:
  - emitted `nyash.string.len_h` still keeps the same two epoch compares in the hot block

**Verdict**

- rejected
- reverted immediately

**Why**

- exact delta が small かつ rerun で揺れた
- source rewrite だけでは LLVM が hot compare order を変えなかった
- ideal asm の `epoch compare once` に届かなかった

**Reopen Condition**

- counter or codegen evidence で TLS slot shape を stronger に変えられる cut が見つかった時だけ

### 2026-04-08: local `dispatch_known_absent_fast` + cold dispatch probe combo

**Hypothesis**

- `len_h` entry で cached-absent dispatch state を 1 load だけ見て
- dispatch body 自体は cold helper に逃がせば
- ideal asm の `STRING_DISPATCH_STATE load once` に寄せられる

**Touched owner area**

- [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs)
- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)

**Observed result**

- split exact:
  - `kilo_micro_len_substring_views: instr=22,673,236 / cycles=3,962,213 / cache-miss=8,741 / AOT 4 ms`
- mixed exact:
  - `kilo_micro_substring_only: instr=58,672,979 / cycles=9,937,709 / cache-miss=9,887 / AOT 4 ms`
- whole:
  - `kilo_kernel_small_hk strict = 716 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- whole strict は強く良化した
- ただし exact gate は両 front とも baseline を超えられなかった
- local fast probe + cold split の組み合わせでも `len_h` entry 先頭の emitted compare chain 自体はまだ残った

**Reopen Condition**

- dispatch-state cached-absent path が another exact family でも dominant と示せた時だけ

### 2026-04-08: `drop_epoch_after_cache_hit()` ready-after-hit probe

**Hypothesis**

- `string_len_fast_cache_lookup()` で handle match が先に確定した後なら
- host-handle registry は already ready とみなせる
- `drop_epoch()` の full helper を避けて exact front の epoch read を軽くできる

**Touched owner area**

- [host_handles.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs)
- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- exact rerun A:
  - `kilo_micro_len_substring_views: instr=22,672,620 / cycles=3,981,913 / cache-miss=8,583 / AOT 4 ms`
  - `kilo_micro_substring_only: instr=58,672,145 / cycles=10,916,669 / cache-miss=9,409 / AOT 5 ms`
- exact rerun B:
  - `kilo_micro_len_substring_views: instr=22,672,586 / cycles=3,988,684 / cache-miss=8,913 / AOT 3 ms`
  - `kilo_micro_substring_only: instr=58,672,908 / cycles=9,903,816 / cache-miss=9,459 / AOT 4 ms`

**Verdict**

- rejected
- reverted immediately

**Why**

- mixed front は一度少し下がったが rerun で baseline 帯へ戻った
- local `len` split は両 rerun とも baseline `22,672,209` を超えた
- `REG.get()` fast path を after-hit に寄せただけでは enough win にならなかった

**Reopen Condition**

- `OnceCell` ready probe を完全に hot path から外せる stronger cut が見つかった時だけ

### 2026-04-08: `len_h` dispatch single-probe + raw trace-state split

**Hypothesis**

- `string_dispatch_raw()` と `jit_trace_len_enabled()` の helper shape を外して
- `len_h` entry が raw state を一度だけ読む形に寄せれば
- ideal asm の `dispatch load once` / `trace load once` に近づける

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [string_debug.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_debug.rs)
- [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs)

**Observed result**

- exact:
  - `kilo_micro_len_substring_views: instr=22,672,516 / cycles=3,985,063 / cache-miss=9,131 / AOT 4 ms`
  - `kilo_micro_substring_only: instr=58,672,529 / cycles=9,905,697 / cache-miss=9,299 / AOT 4 ms`
- asm:
  - `nyash.string.len_h` still reloads both `STRING_DISPATCH_STATE` and `JIT_TRACE_LEN_ENABLED_CACHE`

**Verdict**

- rejected
- reverted immediately

**Why**

- exact gate は両 front とも baseline を clear しなかった
- source shape は変わっても emitted hot asm はほぼ同じままだった
- helper vocabulary の置き換えだけでは current lane の win にならない

**Reopen Condition**

- same logic を another exact family にも適用できる common helper benefit が見えた時だけ

### 2026-04-08: `len_h` 1-probe hash-slot cache shape

**Hypothesis**

- `len_h` fast cache を 2-slot recent compare から 1-probe hash slot へ変えれば
- alternating 2-handle lane の secondary-hit cost を落とせる
- exact split と mixed の両方で instruction win を狙える

**Touched owner area**

- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- exact:
  - `kilo_micro_len_substring_views: instr=22,672,814 / cycles=4,015,092 / cache-miss=9,472 / AOT 5 ms`
  - `kilo_micro_substring_only: instr=58,672,735 / cycles=9,975,762 / cache-miss=10,125 / AOT 4 ms`
  - `kilo_micro_substring_views_only: instr=37,073,366 / cycles=6,825,751 / cache-miss=8,838 / AOT 4 ms`

**Verdict**

- rejected
- reverted immediately

**Why**

- alternating 2-handle micro でも hash/slot compute の cost を回収できなかった
- mixed / len / control の 3 front 全部で baseline を超えた
- current exact lane では cache shape rewrite より existing 2-slot compare の方が still better

**Reopen Condition**

- handle-distribution evidence が出て 2-slot compare 自体が another family でも limit だと示せた時だけ

### 2026-04-08: registry-pointer epoch read on len cache hits

**Hypothesis**

- store 側で raw registry pointer を capture しておけば
- `len_h` cache hit 後は `OnceCell REG.get()` を通らずに epoch を読める
- `drop_epoch_after_cache_hit()` より stronger に ready probe を hot path から外せる

**Touched owner area**

- [host_handles.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs)
- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)

**Observed result**

- exact:
  - `kilo_micro_len_substring_views: instr=22,673,076 / cycles=4,024,980 / cache-miss=8,992 / AOT 3 ms`
  - `kilo_micro_substring_only: instr=58,672,992 / cycles=10,205,799 / cache-miss=9,706 / AOT 4 ms`

**Verdict**

- rejected
- reverted immediately

**Why**

- `REG` state probe を raw pointer へ落としても exact front は悪化した
- safety surface を増やす割に current lane の hot block は still enough 動かなかった
- ready-probe elimination 単体では keeper 条件を満たさない

**Reopen Condition**

- host-handle layer で raw registry access が cross-family hotspot と示せた時だけ

### 2026-04-08: `len_h` `ReadOnlyScalarLane` separation-only slice

**Hypothesis**

- `len_h` を façade と `ReadOnlyScalarLane` に分けて
- fast hit を `FastHit(len)` / `Miss(reason)` だけ返す small lane に固定すれば
- 次の snapshot slice を入れる前に hot/cold ownership を clean にできる

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [len_lane.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/len_lane.rs)

**Observed result**

- serial exact rerun A:
  - `kilo_micro_substring_only: instr=58,672,896 / cycles=9,951,136 / cache-miss=9,763 / AOT 5 ms`
  - `kilo_micro_len_substring_views: instr=22,672,125 / cycles=4,113,347 / cache-miss=9,639 / AOT 3 ms`
  - `kilo_micro_substring_views_only: instr=37,076,670 / cycles=6,956,285 / cache-miss=8,874 / AOT 4 ms`
- serial exact rerun B:
  - `kilo_micro_substring_only: instr=58,673,042 / cycles=10,066,456 / cache-miss=9,578 / AOT 5 ms`
  - `kilo_micro_len_substring_views: instr=22,672,847 / cycles=4,222,683 / cache-miss=8,867 / AOT 4 ms`
- serial whole:
  - `kilo_kernel_small_hk strict = 1263 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- exact was only noise-band on the first serial reread and lost the baseline on the second
- whole strict regressed too far to justify landing a structure-only slice
- lane separation is still the right direction, but it cannot land alone; the next retry must combine the lane boundary with entry snapshots so the hot block actually changes

**Reopen Condition**

- retry only as a combined step with control/data snapshots in the same slice
  and require an asm-visible hot-block change before keeper evaluation

### 2026-04-08: `len_h` combined `ReadOnlyScalarLane` + entry snapshot slice

**Hypothesis**

- `len_h` を façade + `ReadOnlyScalarLane` に split した上で
- control-plane (`dispatch`, `trace`) と data-plane (`drop_epoch`) を entry snapshot すれば
- hot block の reread を潰して ideal asm に寄せられる

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [string_debug.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_debug.rs)
- [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs)
- [cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)
- [len_lane.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/len_lane.rs)

**Observed result**

- exact:
  - `kilo_micro_substring_only: instr=58,672,761 / cycles=10,080,628 / cache-miss=10,570 / AOT 5 ms`
  - `kilo_micro_len_substring_views: instr=22,672,605 / cycles=4,049,035 / cache-miss=9,201 / AOT 4 ms`
  - `kilo_micro_substring_views_only: instr=37,073,526 / cycles=6,906,063 / cache-miss=9,583 / AOT 4 ms`
- exact serial reread:
  - `kilo_micro_substring_only: instr=58,672,903 / cycles=9,964,932 / cache-miss=9,925 / AOT 5 ms`
  - `kilo_micro_len_substring_views: instr=22,672,830 / cycles=3,995,191 / cache-miss=8,755 / AOT 4 ms`
- whole:
  - `kilo_kernel_small_hk strict = 713 ms`
  - parity ok
- asm:
  - `nyash.string.len_h` still reloads both `STRING_DISPATCH_STATE` and `JIT_TRACE_LEN_ENABLED_CACHE`

**Verdict**

- rejected
- reverted immediately

**Why**

- whole strict は良化したが、local split `kilo_micro_len_substring_views` が baseline を clear しなかった
- lane + snapshot を足しても hot asm は細くならず、dispatch/trace の reread が残った
- current lane では structure の clean-up より emitted hot block change が先に必要

**Reopen Condition**

- dispatch/trace hot loads を実際に減らせる stronger control-plane cut があり
  その asm-visible change を先に確認できた時だけ

### 2026-04-08: `host_handles::drop_epoch()` global mirror probe

**Hypothesis**

- `drop_epoch()` を registry 本体から global mirror へ切り出せば
- `len_h` fast cache hit から `host_handles::REG` ready probe を外せる
- registry-side `OnceCell` path を経由しない epoch read にできる

**Touched owner area**

- [host_handles.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs)

**Observed result**

- exact:
  - `kilo_micro_len_substring_views: instr=22,672,032 / cycles=4,031,342 / cache-miss=9,168 / AOT 3 ms`
  - `kilo_micro_substring_only: instr=58,672,687 / cycles=10,181,019 / cache-miss=9,465 / AOT 5 ms`
- exact serial reread:
  - `kilo_micro_len_substring_views: instr=22,672,808 / cycles=4,265,903 / cache-miss=9,793 / AOT 4 ms`
- asm:
  - `nyash.string.len_h` annotate still shows the `host_handles::REG` ready probe / `OnceCell::initialize` branch

**Verdict**

- rejected
- reverted immediately

**Why**

- first exact run は良く見えたが、serial reread で local split が baseline を落とした
- more importantly、狙っていた `REG` probe removal が asm 上で確認できなかった
- source-level mirror 追加だけでは current codegen を enough 変えられない

**Reopen Condition**

- `drop_epoch()` source change が emitted `len_h` hot asm から `REG` probe を消したと確認できた時だけ

### 2026-04-08: `len_h`-specific 4-box slice (`façade + control snapshot + pure cache probe + cold path`)

**Hypothesis**

- `len_h` を reusable framework にせず lane-specific に 4 箱へ split すれば
- façade / control snapshot / pure cache probe / cold path の境界が立って
- `trace` reread と `drop_epoch()` registry path を同時に hot block から薄くできる

**Touched owner area**

- [string_helpers.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers.rs)
- [string_helpers/cache.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/cache.rs)
- [string_helpers/len_lane.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_helpers/len_lane.rs)
- [string_debug.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_debug.rs)
- [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs)
- [host_handles.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs)

**Observed result**

- exact:
  - `kilo_micro_substring_only: instr=58,672,662 / cycles=10,685,150 / cache-miss=9,814 / AOT 6 ms`
  - `kilo_micro_len_substring_views: instr=22,672,634 / cycles=4,061,970 / cache-miss=9,279 / AOT 4 ms`
- whole:
  - `kilo_kernel_small_hk strict = 787 ms`
  - parity ok
- asm:
  - `nyash.string.len_h` still rereads `STRING_DISPATCH_STATE`
  - `nyash.string.len_h` still rereads `JIT_TRACE_LEN_ENABLED_CACHE`
  - `host_handles::REG` ready probe is still visible in the hot block

**Verdict**

- rejected
- reverted immediately

**Why**

- mixed exact improved slightly, but the active local front `kilo_micro_len_substring_views` stayed above baseline
- whole strict regressed from the accepted `755 ms` band to `787 ms`
- most importantly, the emitted hot asm did not reflect the intended box separation; the same control-plane rereads remained
- this confirms that “cleaner source boxes” are not enough unless they physically change the hot CFG

**Reopen Condition**

- only retry after a narrower control-plane cut proves an asm-visible reduction in `STRING_DISPATCH_STATE` or `JIT_TRACE_LEN_ENABLED_CACHE`
- keep generic scalar-lane abstraction out until that happens

## Next Candidate

- keep substring runtime mechanics in Rust
- do not create more test-by-test artifact folders for this wave
- next local cut is:
  1. keep `kilo_micro_substring_only` as the accept gate
  2. use `kilo_micro_len_substring_views` for local `len_h` cuts
  3. keep substring runtime mechanics unchanged unless the split pair moves again
  4. helper/state rewrites and cache-shape rewrites did not move emitted `len_h` hot asm enough
  5. `ReadOnlyScalarLane` separation, `drop_epoch()` source reshapes, and the lane-specific 4-box split are all blocked until they prove an asm-visible hot-block change
  6. focus next on control-plane hot loads (`STRING_DISPATCH_STATE` / `JIT_TRACE_LEN_ENABLED_CACHE`) before retrying another structural lane slice
