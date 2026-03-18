---
Status: SSOT
Scope: `kilo` / `micro kilo` を起点にした exe 最適化の測定順序・判断順序・止め線
Related:
- docs/development/current/main/DOCS_LAYOUT.md
- docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/phases/phase-29ck/README.md
- CURRENT_TASK.md
---

# Perf Optimization Method SSOT

## Goal

この文書は、`.hako` / C ABI / Rust bridge / micro leaf をまたぐ exe 最適化を、毎回同じ手順で進めるための正本だよ。

目的は 2 つだけ。

1. whole-program の差をまず安定 baseline で固定する。
2. そこから micro leaf を 1 本ずつ exact に削る。

## Owner Scope Lock

この wave で触る owner と、keep lane として読むだけに留める owner を最初に固定する。

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
   - route contract: AOT lane is `.hako -> ny-llvmc(boundary) -> C ABI`; `llvmlite` / `native` / harness は fail-fast

2. Micro ladder
   - 入口: `tools/perf/run_kilo_micro_machine_ladder.sh`
   - 役割: `indexof_line` / `substring_concat` / `array_getset` の leaf 密度を比較する
   - 使い方: `ratio_cycles` と `ratio_instr` を優先して順位を決める
   - route contract: same as stable baseline; explicit keep lanes are not valid perf comparisons here

3. ASM probe
   - 入口: `tools/perf/bench_micro_aot_asm.sh`
   - 役割: micro ladder で一番厚い leaf の原因関数を確認する
   - 使い方: `perf report --stdio --no-children` の top symbol を読む

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
4. それでも残るなら `@hint(inline)` を exact hot leaf にだけ試す

理由:

- env probe は leaf の形を変えずに固定費を落としやすい
- dispatch / registry は多くの benchmark に横断で効く
- `@hint(inline)` は最後に試す補助輪で、workaround ではない

## Current Wave Snapshot

2026-03-18 時点では、次の理解で進める。

- `kilo_kernel_small_hk` は whole-program baseline として固定済み
- latest fresh stable baseline is `c_ms=79`, `py_ms=111`, `ny_vm_ms=989`, `ny_aot_ms=804`, `ratio_c_aot=0.10`, `aot_status=ok`
- `kilo_micro_substring_concat` が最厚
- `kilo_micro_array_getset` が次
- `kilo_micro_indexof_line` が一番マシ
- micro profile で見えている `std::env::_var_os` は、まず bridge 側の per-call probe を疑う
- `substring_concat` の current exact leaf は kernel/runtime owner に固定する
- `crates/nyash_kernel/src/exports/string_view.rs` now owns `borrowed_substring_plan_from_handle(...)`, and `crates/nyash_kernel/src/exports/string.rs::substring_hii` is reduced to dispatch + match
- `crates/nyash_kernel/src/exports/string.rs::concat3_hhh` is now split file-locally into transient planning (`concat3_plan_from_parts`, `concat3_plan_from_fast_str`, `concat3_plan_from_spans`) plus birth sink (`freeze_concat3_plan`)
- `substring_hii` の hot path must stay on direct `with_handle(...)`; cache-backed span lookup is diagnostic-only here because it regressed `string_span_cache_get/put` back into the top symbols
- `src/runtime/host_handles.rs::Registry::alloc` now reads `policy_mode` before the write lock and keeps invariant failures in cold helpers; this is the current bridge/allocation slice
- current contract-change slice raises the short-slice eager materialize threshold to `<= 8 bytes`
- fresh micro recheck after the current slices is `266244455 cycles / 72 ms` for `kilo_micro_substring_concat`
- fresh stable recheck after the current slices is `798 ms` median for `kilo_kernel_small_hk` (`min=791`, `max=1607`)
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
  - `llvmlite/native/harness` are invalid and must fail-fast
- `kilo_micro_substring_concat`:
  - asm-guided slice first changed `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES` from `8` to `0`, then contract-change follow-up restored eager materialize for `<= 8 bytes`
  - short `substring_hii` results now materialize under FAST lane, while mid slice still stays `StringViewBox`
  - pre-structure-first checkpoint was `266891899 cycles / 73 ms`, while stable `kilo_kernel_small_hk` sat at `804 ms`
  - accepted structure-first follow-up: `concat3_hhh` now reads `plan -> freeze -> handle` inside `string.rs`; current recheck is `266244455 cycles / 72 ms` and stable `kilo_kernel_small_hk` median `798 ms`
  - rejected observer-only follow-up: explicit `string_len_from_handle` downcast fast paths reached `265893951 cycles / 68 ms`, but stable `kilo_kernel_small_hk` regressed to `1066 ms` median, so this wave keeps the previous observer path unchanged
  - rejected structure-first follow-up: planner-side `OwnedSubstring/ViewRecipe` plus `substring_hii`-side view freeze reached `267397179 cycles / 72 ms`, but stable `kilo_kernel_small_hk` regressed to `901 ms` median, so plan/birth separation needs a real transient carrier instead of a pure birth-site shuffle
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
