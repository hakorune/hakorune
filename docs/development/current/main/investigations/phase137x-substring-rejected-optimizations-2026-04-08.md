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
  - `instr: 59,272,932`
  - `cycles: 10,007,852`
  - `cache-miss: 8,699`
  - split exact reread:
    - `kilo_micro_substring_views_only: instr=37,073,398 / cycles=6,880,057 / cache-miss=9,746`
    - `kilo_micro_len_substring_views: instr=23,272,760 / cycles=4,123,725 / cache-miss=9,284`
- current whole-kilo health is:
  - `tools/checks/dev_gate.sh quick`: green
  - `kilo_kernel_small_hk` strict accepted reread: `744 ms`
  - parity: ok
- current landed truth:
  - `substring_hii` can reissue a fresh handle from a cached `StringViewBox` object after transient drop-epoch churn if the source handle still names the same live source object
  - `str.substring.route` observe read is dominated by `view_arc_cache_handle_hit=599,998 / total=600,000`
  - the current keeper removed redundant `view_enabled` state from `SubstringViewArcCache`; that cache only runs on the `view_enabled` route
  - split exact reread separated `substring_hii` and `len_h`; the mixed-front keeper in this pass comes from `len_h`, not substring publication/reissue
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

### 2026-04-08: dispatch/trace false-state helper split

**Hypothesis**

- `string_dispatch_raw()` と `jit_trace_len_enabled()` の false-state を cold helper へ逃がせば
- `len_h` steady-state の guard cost をもっと薄くできる
- split `kilo_micro_len_substring_views` と mixed exact の両方で勝てる

**Touched owner area**

- [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs)
- [string_debug.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string_debug.rs)

**Observed result**

- split exact:
  - `kilo_micro_len_substring_views: instr=22,071,954 / cycles=4,012,624 / cache-miss=9,216 / AOT 4 ms`
- mixed exact:
  - `kilo_micro_substring_only: instr=57,172,575 / cycles=9,700,690 / cache-miss=9,623 / AOT 5 ms`
- whole:
  - `kilo_kernel_small_hk strict = 754 ms`, rerun `789 ms`
  - parity ok

**Verdict**

- rejected
- reverted immediately

**Why**

- exact はこの wave で一番強く動いた
- それでも strict whole が accepted band を外れた
- global false-state helper split は exact 用には強いが、whole lane への branch/layout side effect がまだ大きい

**Reopen Condition**

- whole strict で同じ helpers が actually hot と asm で示せる時だけ

## Next Candidate

- keep substring runtime mechanics in Rust
- do not create more test-by-test artifact folders for this wave
- next local cut is:
  1. keep `kilo_micro_substring_only` as the accept gate
  2. use `kilo_micro_len_substring_views` for local `len_h` cuts
  3. keep substring runtime mechanics unchanged unless the split pair moves again
  4. measure again with `3 runs + perf`
