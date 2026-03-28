---
Status: SSOT
Scope: `kilo` / `micro kilo` を起点にした exe 最適化の測定順序・判断順序・止め線
Related:
- docs/development/current/main/DOCS_LAYOUT.md
- docs/development/current/main/design/optimization-tag-flow-ssot.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md
- docs/development/current/main/design/stage2-optimization-debug-bundle-ssot.md
- docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
- docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
- docs/development/current/main/design/stage2-string-route-split-plan.md
- docs/development/current/main/design/transient-text-pieces-ssot.md
- docs/development/current/main/design/kilo-meso-benchmark-ladder-ssot.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/phases/phase-29ck/README.md
- docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-29ck/P8-PERF-REOPEN-JUDGMENT.md
- docs/development/current/main/design/stage1-mir-dialect-contract-ssot.md
---

# Perf Optimization Method SSOT

## Goal

この文書は、`.hako` / C ABI / Rust bridge / micro leaf をまたぐ exe 最適化を、毎回同じ手順で進めるための正本だよ。

目的は 2 つだけ。

1. whole-program の差をまず安定 baseline で固定する。
2. そこから micro leaf を 1 本ずつ exact に削る。

## Current Scheduling Status

- `phase-21_5` perf reopen judgment is now landed with `reopen allowed`.
- reopen order is fixed like this:
  1. confirm the pre-perf runway in `phase-29ck/P7-PRE-PERF-RUNWAY-TASK-PACK.md` is closed
  2. confirm the boundary mainline route is stable on `.hako -> ny-llvmc(boundary) -> C ABI`
  3. land the explicit `perf/kilo` reopen judgment
  4. if that judgment is green, reopen `kilo` / `micro kilo`
- current `P8/P9` evidence now allows reopen:
  - `bench_compare_c_vs_hako.sh method_call_only_small 1 1` returns `aot_status=ok`
  - `phase21_5_perf_loop_integer_hotspot_contract_vm.sh` is green
  - `phase21_5_perf_strlen_ir_contract_vm.sh` is green
- `P10-SMALL-PERF-REENTRY-TASK-PACK.md` is now closed.
- current small-entry truth is:
  - `method_call_only_small` mainline AOT IR is a pure `+5` loop
  - `box_create_destroy_small` mainline AOT IR is a pure `+1` loop
  - short microasm may still be startup/loader sensitive, but `bench_micro_aot_asm.sh` now uses a direct C runner instead of a bash loop
- startup-subtracted small-entry evidence is now `method_call_only_small=1 ms`, `box_create_destroy_small=0 ms`
- `P11-SMALL-ENTRY-STARTUP-INVENTORY.md` is now closed.
- current perf-kilo design front has moved to `transient-text-pieces-ssot.md`; the measurement snapshots below remain historical evidence until the next proof lands.
- current docs-first perf-kilo design front is `string-birth-sink-ssot.md`: `freeze.str` remains the canonical sink target, but the attempt to move the canonical sink into `string_store.rs` was rejected; the active lane stays on substring boundary cleanup rather than more route/helper splitting.
- when the lane is on `freeze.str`, do not mix sink canonicalization with route/helper splitting in the same commit series.
- current narrow implementation order is fixed: shrink `BorrowedSubstringPlan` into recipe-only / boundary-only placement, keep `array_set` as the consumer boundary, re-run the same-artifact meso/main proof, and only then sink-local `Registry::alloc/get` / `BoxBase::new` tuning.
- landed planner cleanup: const-suffix / insert recipe helpers are isolated in `crates/nyash_kernel/src/exports/string_plan.rs`.
- rejected follow-up: moving `freeze.str` into `string_store.rs` regressed stable main (`834 ms` / `909 ms` back-to-back), so keep the shared `freeze_text_plan(...)` helper local to `string.rs` until new asm evidence appears.
- code has now landed the shared `freeze_text_plan(...)` sink helper for `concat_hs` and `insert_hsi`; keep the current proof reading as historical evidence until the next sink-local tuning lands.
- therefore the perf lane may stay reopened, `P12-SMALL-ENTRY-GC-SECTIONS-CANDIDATE.md` and `P13-SMALL-ENTRY-RAW-NET-REFRESH.md` are now closed, and the current small-entry lane is `none (monitor-only)`.
- current boundary-mainline `method_call_only_small` exe shape is `5,375,880` bytes / `61` relocations.
- refreshed raw 1x1 evidence is `method_call_only_small=9 ms`, `box_create_destroy_small=8 ms`.
- `llvmlite` / harness stays outside the perf judge even when the lane reopens.
- until that reopen happens, the quick chip8 crosslang smoke is monitor-only for AOT:
  - keep `[bench4]` / `[bench4-route]` shape and timing keys pinned
  - allow `aot_status=skip`
  - do not treat that smoke as proof that perf lane is open

## Owner Scope Lock

この wave で触る owner と、keep lane として読むだけに留める owner を最初に固定する。

- this owner list applies when the perf lane is explicitly reopened; it is not the current mainline implementation front today.

- active edit owners:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/plugin/string.rs`
  - `src/runtime/host_handles.rs`
  - `lang/c-abi/shims/hako_aot.c`
  - `lang/c-abi/shims/hako_aot_shared_impl.inc`
- keep-lane owners for this wave:
  - `src/llvm_py/**`
  - `tools/llvmlite_harness.py`
  - `crates/nyash-llvm-compiler/src/harness_driver.rs`
  - explicit keep-lane selectors and their docs/tests
- operational rule:
  - start from `bench_micro_aot_asm.sh` top symbols and follow the symbol owner
  - do not pivot into `llvm_py` just because keyword grep finds matching names
  - only reopen keep-lane owners when the route contract itself is broken

## Measurement Ladder

最適化は、必ずこの順で進める。

1. Stable baseline
   - 入口: `tools/perf/bench_compare_c_py_vs_hako_stable.sh`
   - 役割: C / Python / Hako / AOT の whole-program 差を見る
   - 使い方: `PERF_AOT_SKIP_BUILD=0` の fresh build を baseline にする
  - route contract: AOT lane is `.hako -> ny-llvmc(boundary) -> C ABI`
  - `llvmlite` / harness is a correctness/compat keep, not a perf baseline
  - `native` direct keep lane is also outside the perf judge

2. Leaf-proof micro ladder
   - 入口: `tools/perf/run_kilo_leaf_proof_ladder.sh`
   - 役割: 1 leaf の shape が same route で本当に薄くなったかを見る
   - 使い方:
     - old crossing が hot block から消えているかを優先して確認する
     - `leaf-proof -> micro kilo -> main kilo` の順を崩さない
   - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

3. Micro ladder
   - 入口: `tools/perf/run_kilo_micro_machine_ladder.sh`
   - 役割: `indexof_line` / `substring_concat` / `array_getset` の leaf 密度を比較する
   - 使い方: `ratio_cycles` と `ratio_instr` を優先して順位を決める
  - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

4. Meso ladder
   - 入口: `tools/perf/run_kilo_meso_machine_ladder.sh`
   - 役割: `micro` と `kilo_kernel_small_hk` の間にある `len -> array_set -> loopcarry` 境界で gap がどこで開くかを見る
   - 使い方:
     - `substring+concat+len`
     - `substring+concat+array_set`
     - `substring+concat+array_set+loopcarry`
     の順で差が開く地点を読む
   - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

5. ASM probe
   - 入口: `tools/perf/bench_micro_aot_asm.sh`
   - 役割: micro ladder で一番厚い leaf の原因関数を確認する
   - 使い方: `perf report --stdio --no-children` の top symbol を読む
   - runner contract: bash loop は使わず、direct C runner で exe を繰り返し起動する

6. MIR call family probe
   - 入口: `tools/perf/report_mir_hotops.sh`
   - 役割: `mir_call` がどの callee family に寄っているかを構造化表示する
   - 使い方: `[mir-shape/call]` を見て `RuntimeDataBox.substring` / `indexOf` / `get/set/length` などの次 leaf を決める

7. Optimization debug bundle
   - 入口: `tools/perf/trace_optimization_bundle.sh`
   - 役割: route trace / MIR window / IR / symbol / optional micro perf を same artifact で束ねる
   - 使い方:
     - `symbol miss` を unexplained のままにしない
     - new leaf は bundle で `route -> MIR window -> IR -> symbol` が揃ってからだけ再試行する

## Tag Coverage

最適化 tag / knob の到達範囲は [optimization-tag-flow-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/optimization-tag-flow-ssot.md) を正本にする。

この wave での短い読みは次。

- language annotations (`@hint` / `@contract` / `@intrinsic_candidate`) are not backend-active yet
- AotPrep knobs act before `ny-llvmc`
- boundary compile/link knobs are the only tags that truly cross into `ny-llvmc(boundary)` / object / exe generation
- perf-only knobs are measurement controls, not backend optimization tags

## Classification

Hotspot は次の分類で読む。

- startup
  - process 起動、引数解析、runner 配線
- driver
  - `ny-llvmc` / runner / bridge の選択
- bridge
  - FFI、handle registry、dispatch、env probe
- allocation
  - `StringBox` / `ArrayBox` / `Registry::alloc`
- algorithm
  - `find_substr_byte_index` などの pure leaf
- cache
  - OnceLock / handle cache / span cache

判断に迷ったら、まず bridge と allocation を疑う。

## Stage2 Thin-Path Reading

`stage2` / `hako_alloc` / `hakozuna` 側の AOT fast-lane 検討では、最初に source layering を疑わない。

- 先に疑うもの:
  - bridge cost
  - allocation cost
  - semantic-owner cost
  - dynamic fallback cost
- 先に疑わないもの:
  - source owner/substrate layering そのもの
- route collapse は perf lane の対象にしてよいが、source relayering は別 SSOT の責務だよ。
- hot path が `HostFacade / extern_provider / plugin loader` に入るなら、それは perf miss として扱う。
- dual-lane reading:
  - `ny-llvm` / `ny-llvmc` = perf/mainline judge
  - `llvmlite` = keep lane only
  - keep lane breaks are correctness/compat issues, not performance evidence

## Stage2 Per-Slice Rule

- stage2 AOT thin-path work is measured one accepted slice at a time.
- route-table change commits and perf-retune commits do not mix.
- after each accepted slice, re-run the ladder in order:
  1. stable baseline
  2. micro ladder
  3. ASM probe
- String waves are measured separately:
  - first `String search/slice route split`
  - then `String concat route split`
- `llvmlite` remains outside the perf judge even when stage2 String waves are active.
- current scheduling consequence:
  - if `phase-29ck` reopens a new exact `ny-llvm` front, do not reopen this perf lane yet
- current preferred next owner is the boundary link owner from `P12`, not a runtime string/box leaf and not an immediate medium/full `kilo` retune

## Small-Entry Stop Line

- if the reopened small-entry lane dumps to pure-loop IR, do not edit runtime string/box leaves in that same series
- if the short asm probe is dominated by loader/startup symbols, move to startup/loader inventory before reopening medium/full `kilo`

## Stop Line

次の条件なら、その lane は「最適化より構造修正」を優先する。

- route がまだ揺れている
- contract テストが落ちている
- benchmark が startup dominated で leaf を見分けられない
- 1回の変更で 2 枚以上の leaf を同時に触りたくなった

`ratio_c_aot >= 0.95` かつ `aot_status=ok` なら、その lane は monitor-only へ落とす。

## Recommended Order

最適化の順番は原則こうだよ。

1. hot path の per-call env probe を cache 化する
2. hot bridge の dispatch / registry を薄くする
3. `substring_concat` や `array_getset` の exact leaf を 1 本だけ削る
4. それでも残るなら native leaf-local hint を exact hot leaf にだけ試す

理由:

- env probe は leaf の形を変えずに固定費を落としやすい
- dispatch / registry は多くの benchmark に横断で効く
- language-level `@hint(inline)` is not backend-active in the current wave; only leaf-local native hints count here

## Current Wave Snapshot

2026-03-27 current stop-line は次だよ。

- `Stage0 = llvmlite` explicit compat/probe keep lane only
- `Stage1 = ny-llvmc(boundary pure-first)` daily/mainline/perf owner
- current `kilo` route is back to `pure-first + compat_replay=none + aot_status=ok`
- `Stage1 MIR dialect split` is retired for the current kilo entry
- current exact front is now `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
- future `AOT-Core MIR` is treated as `future-needed but not a new layer now`
- current action is to add proof vocabulary to the existing MIR/lowering/manifest path
  - `value_class`
  - `escape_kind`
  - `effect`
  - `cold_fallback`
- current array reject reading is now sharper:
  - adjacent fused-leaf guess was rejected
  - live no-replay route shows current semantic window is `get -> copy* -> const 1 -> add -> set`
  - next exact front is reusable route/window/IR/symbol bundle before another leaf attempt
- first concrete consumer after docs remains integer-heavy `ArrayBox.get/set/len` fast lane
- rejected array-substrate tries are now recorded in a rolling investigation ledger instead of staying as ephemeral shell history
- no-replay `kilo` が green になるまで、`src/llvm_py/**` は perf owner work に使わない
- 下の micro snapshot は historical evidence として保持するが、current exact front ではない

2026-03-22 時点の micro snapshot は次の理解だった。

- `kilo_kernel_small_hk` は whole-program baseline として固定済み
- latest fresh stable baseline is `c_ms=76`, `py_ms=105`, `ny_vm_ms=974`, `ny_aot_ms=740`, `ratio_c_aot=0.10`, `aot_status=ok`
- `kilo_micro_substring_concat` が最厚
- `kilo_micro_array_getset` が次
- `kilo_micro_indexof_line` が一番マシ
- `kilo_micro_array_getset` の current exact leaf is now Rust substrate: the first read-seam slice (`crates/nyash_kernel/src/plugin/array_slot_load.rs::array_slot_load_encoded_i64`) is landed, and the current probe target is the write/TLS seam:
  - `crates/nyash_kernel/src/plugin/array_slot_store.rs::array_slot_store_i64`
  - `crates/nyash_kernel/src/plugin/handle_cache.rs::with_array_box`
  - `src/boxes/array/mod.rs::ArrayBox::try_set_index_i64_integer`
- `crates/nyash_kernel/src/plugin/array_index_dispatch.rs` / `array_write_dispatch.rs` are now thin wrappers, so they are no longer the primary exact leaf target
- fresh `kilo_micro_array_getset` recheck after the read-seam keep is `ny_aot_ms=44`
- rejected probes (reverted immediately):
  - dedicated i64 write helper: `47 ms`
  - `try_set_index_i64_integer` cold-split: `48 ms`
  - `with_array_box` cache-hit inline probe: `46 ms`; asm top stayed on `array_slot_store_i64` closure + `LocalKey::with`
- fresh microasm now concentrates on `array_slot_store_i64` closure + `LocalKey::with`, so the next cut must be measurement-led rather than another blind helper split
- micro profile で見えている `std::env::_var_os` は、まず bridge 側の per-call probe を疑う
- `substring_concat` の current exact leaf は kernel/runtime owner に固定する
- `crates/nyash_kernel/src/exports/string_view.rs` now owns `borrowed_substring_plan_from_handle(...)`, and `crates/nyash_kernel/src/exports/string.rs::substring_hii` is reduced to dispatch + match
- `crates/nyash_kernel/src/exports/string.rs::concat3_hhh` is now split file-locally into transient planning (`concat3_plan_from_parts`, `concat3_plan_from_fast_str`, `concat3_plan_from_spans`) plus birth sink (`freeze_concat3_plan`)
- `substring_hii` の hot path must stay on direct `with_handle(...)`; cache-backed span lookup is diagnostic-only here because it regressed `string_span_cache_get/put` back into the top symbols
- `src/runtime/host_handles.rs::Registry::alloc` now reads `policy_mode` before the write lock and keeps invariant failures in cold helpers; this is the current bridge/allocation slice
- current contract-change slice raises the short-slice eager materialize threshold to `<= 8 bytes`
- fresh micro recheck after the current slices is `266244455 cycles / 72 ms` for `kilo_micro_substring_concat`
- fresh stable recheck after the current slices is `740 ms` median for `kilo_kernel_small_hk` (`min=738`, `max=744`)
- rejected variant: `root StringBox <= 16 bytes` / `nested StringViewBox <= 8 bytes` improved isolated `substring_concat` to `262468757 cycles / 69 ms`, but stable `kilo_kernel_small_hk` regressed to `819 ms`; do not keep this split while stable is the primary metric
- rejected observer-only variant: `crates/nyash_kernel/src/exports/string.rs::string_len_from_handle(...)` explicit `StringBox` / `StringViewBox` downcast fast paths reached `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed to `1066 ms` median (`min=786`, `max=1841`); revert immediately and do not reopen this cut before a stronger owner-level reason appears
- rejected structure-first variant: `BorrowedSubstringPlan::{OwnedSubstring,ViewRecipe}` moved `StringViewBox` birth from `borrowed_substring_plan_from_handle(...)` into `substring_hii`, but without a real transient carrier this only shuffled the birth site; isolated `substring_concat` landed at `267397179 cycles / 72 ms`, while stable `kilo_kernel_small_hk` regressed to `901 ms` median (`min=794`, `max=1146`); do not reopen this cut until a larger `TStr`/freeze-boundary design is ready
- current asm top is:
  - `BoxBase::new`
  - `Registry::alloc`
  - `nyash.string.substring_hii`
  - `nyash.string.concat3_hhh`
  - `string_len_from_handle` / `string_handle_from_owned`
- `BoxBase::new` is the current stop-line: it is tied to box identity via `next_box_id()`, so the next safe cut must reduce `StringViewBox::new` call count or another upstream owner instead of reusing IDs
- adopted design reading after external consultation:
  - this is a birth-density problem, not a `BoxBase::new` micro-cost problem
  - the next wave should separate `authority / transient / birth boundary / substrate`
  - read-only observer chains may stay transient; only substrate-visible / retained values should birth
- interpretation:
  - keep the short-slice materialize change if whole-program stable is the primary metric
  - do not treat isolated micro regression as automatic revert when the stable lane improves
  - next queued wave is transient string chain design-first with a future `freeze` boundary, not another threshold experiment

## Evidence To Record

最適化を 1 slice 終えたら、必ず次を更新する。

- `CURRENT_TASK.md`
- 該当 phase README
- その wave の design / investigation doc

記録する内容は最小でよい。

- baseline 数値
- top symbol
- 変更した exact leaf
- 再実行した gate / smoke

## Non-goals

- 大きな route rewrite を先にやること
- benchmark を見ずに一般論だけで最適化すること
- hint を workaround として使うこと
- Route contract for this wave:
  - perf AOT lane is `.hako -> ny-llvmc(boundary) -> C ABI`
  - `llvmlite/harness` are invalid as perf comparators but valid as explicit keep lanes
  - `native` direct keep lane is also outside the perf judge
- `kilo_micro_substring_concat`:
  - asm-guided slice first changed `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES` from `8` to `0`, then contract-change follow-up restored eager materialize for `<= 8 bytes`
  - short `substring_hii` results now materialize under FAST lane, while mid slice still stays `StringViewBox`
  - pre-structure-first checkpoint was `266891899 cycles / 73 ms`, while stable `kilo_kernel_small_hk` sat at `804 ms`
  - accepted structure-first follow-up: `concat3_hhh` now reads `plan -> freeze -> handle` inside `string.rs`; current recheck is `266244455 cycles / 72 ms` and stable `kilo_kernel_small_hk` median `798 ms`
  - rejected observer-only follow-up: explicit `string_len_from_handle` downcast fast paths reached `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed to `1066 ms` median, so this wave keeps the previous observer path unchanged
  - rejected structure-first follow-up: planner-side `OwnedSubstring/ViewRecipe` plus `substring_hii`-side view freeze reached `267397179 cycles / 72 ms`, but stable `kilo_kernel_small_hk` regressed to `901 ms` median, so plan/birth separation needs a real transient carrier instead of a pure birth-site shuffle
  - rejected 2026-03-28 follow-up: direct `concat_hs` / `concat3` copy materialization moved stable `kilo_kernel_small_hk` from the current `736 ms` line to `757 ms` and did not improve micro; keep `TextPlan`-backed concat routes until a new asm reason appears
  - rejected 2026-03-28 follow-up: piece-preserving `insert_inline` plus store/freeze reshaping regressed stable `kilo_kernel_small_hk` to `895 ms`; do not reopen that cut until `concat_hs` / `array_set_by_index_string_handle_value` stop being the active hot leafs
  - rejected 2026-03-28 follow-up: blanket `#[inline(always)]` on host registry / hako-forward string wrappers held stable main near `740 ms` and did not beat the current `736 ms` line, so the slice stays reverted
  - rejected 2026-03-28 follow-up: `concat_hs` duplicate span-resolution removal plus span-resolver inlining regressed stable `kilo_kernel_small_hk` to `796 ms`, so the existing `TextPlan::from_handle(...)` route stays active
  - current top symbols are:
    - `BoxBase::new`
    - `Registry::alloc`
    - `nyash.string.substring_hii`
    - `nyash.string.concat3_hhh`
    - `string_len_from_handle` / `string_handle_from_owned`
- Keep-lane diagnostic note:
  - worker inventory found likely `loop self-carry PHI` string pointer loss under `src/llvm_py/**`
  - this is diagnostic evidence only in the current wave
  - next edits still stay on kernel/runtime/C-boundary owners until asm top symbols move away from `substring_hii` / `Registry::alloc` / `BoxBase::new`
