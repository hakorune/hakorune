---
Status: SSOT
Decision: provisional
Date: 2026-03-18
Scope: `kilo_micro_substring_concat` を起点に、`substring -> concat3 -> length` の inner chain を transient/span-first にほどく次 wave の設計を固定する。
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/string-transient-lifecycle-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/substring-view-materialize-boundary-ssot.md
- docs/development/current/main/design/box-identity-view-allocation-design-note.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/optimization-ssot-string-helper-density.md
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - src/runtime/host_handles.rs
  - benchmarks/bench_kilo_micro_substring_concat.hako
---

# Transient String Chain Boxless Wave SSOT

## Goal

`substring_concat` 系の hot loop で、観測されない中間文字列まで box/handle を作っている密度を下げる。
ただし `BoxBase::new` / `box_id` / handle registry の substrate 契約は壊さない。

この wave の目的は 2 つだけだよ。

1. `substring -> concat3 -> length` の inner chain を transient/span-first に切り分ける。
2. loop-carried state と escape boundary を先に固定して、benchmark-shaped workaround を防ぐ。

この wave の architectural reading は [`string-transient-lifecycle-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/string-transient-lifecycle-ssot.md) に従う。
つまり、current code は次の 4 層で読む。

1. authority / contract
2. transient
3. birth boundary
4. substrate

## Exact Target Chain

最初にほどく対象は [`benchmarks/bench_kilo_micro_substring_concat.hako`](/home/tomoaki/git/hakorune-selfhost/benchmarks/bench_kilo_micro_substring_concat.hako) の 1 loop 内にあるこの chain だよ。

1. `left = text.substring(0, split)`
2. `right = text.substring(split, len)`
3. `out = left + "xx" + right`
4. `acc = acc + out.length()`

ここでは `left` / `right` / `out` は loop 内で read-only に消費されるだけなので、最初に transient/non-box 候補へ寄せる。

一方で、

- `text = out.substring(1, len + 1)`

は loop を跨いで次 iteration に持ち越される state なので、**最初の escape boundary** として扱う。
この地点では current substrate 上の materialize/handle 化を許可する。

## Boundary Lock

### Transient にしてよいもの

- read-only `substring` result
- `concat3` の入力としてだけ使われる一時 span
- `length`/`size` で即消費される一時 span

### Escape とみなすもの

- loop-carried assignment
- array/map への格納
- FFI / C ABI へ渡す地点
- clone/share/retention 境界

### 現 wave で触らないもの

- `BoxBase::new`
- `box_id` semantics
- generic handle reuse
- `StringViewBox` を alias 化すること
- `src/llvm_py/**`
- current flat short-slice policy `<= 8 bytes`

### current adoption from external consultation

- `BoxBase::new` の generic 削減ではなく、birth 密度の削減を本線にする
- `observable` ではなく `substrate-visible / retained` を birth ルールに置く
- `length` / `size` / read-only chain は transient のままでよい
- `text = out.substring(1, len + 1)` は first escape boundary として維持する

## Current Owner Split

### `.hako` / docs 側が先に owner するもの

- method contract
- view/materialize policy
- escape boundary の意味
- visible proof / smoke owner

### Rust substrate に当面残すもの

- `StringBox`
- `StringViewBox`
- `StringSpan`
- `BoxBase`
- `Registry::alloc`
- `host_handles`
- `string_handle_from_owned`
- native string/C ABI leaf

要するに、**authority は `.hako` / SSOT に寄せるが、substrate はまだ Rust に残す** がこの wave の前提だよ。

## Next Structural Shape

current code では `plan` と `birth` がまだ混ざっている。
次に目指す形はこれだよ。

1. transient planning
   - substring / concat / observer chain を substrate-independent に表す
2. freeze boundary
   - escaped/retained point だけが `StringBox` / `StringViewBox` / handle を作る

言い換えると、`BorrowedSubstringPlan::Materialize/CreateView` のような substrate-specific branch は最終的に freeze 側へ寄せたい。

## Fixed Order

1. docs-first
   - inner transient chain と escape boundary をこの文書と `string-transient-lifecycle-ssot.md` で固定する
2. inventory
   - `substring_hii`, `concat3_hhh`, `string_len_from_handle`, `string_handle_from_owned` のどこで box/handle birth が起きるかを 1 枚で棚卸しする
3. structure-first code change
   - current `<= 8 bytes` policyを変えず、plan と birth を薄く分ける exact slice だけを試す
   - do not repeat the rejected minimal split `BorrowedSubstringPlan::{OwnedSubstring,ViewRecipe}`: moving `StringViewBox` birth from planner to `substring_hii` without a real transient carrier regressed stable to `901 ms` median even though the code shape looked cleaner
4. perf proof
   - micro と stable の両方で keep/discard を決める
5. authority prep
   - `.hako` 側の string contract / orchestration owner を shadow 化する

## Acceptance

1. `cargo test -q -p nyash_kernel substring_hii -- --nocapture`
2. `cargo test -q -p nyash_kernel string_concat3_hhh_contract -- --nocapture`
3. `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_substring_concat 1 9`
4. `PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat '' 15`
5. `PERF_VM_FORCE_NO_FALLBACK=1 PERF_AOT_SKIP_BUILD=0 bash tools/perf/bench_compare_c_py_vs_hako_stable.sh kilo_kernel_small_hk auto 5 5 11`

Keep rule:

- stable median を `804 ms` 以下に維持、または改善する
- micro-only 改善は診断扱いに留め、stable が悪化したら discard する
- planner/birth separation is only keep-worthy if it also reduces birth density; a pure birth-site relocation is reject-by-default

## Stop Lines

次のどれかに触りたくなったら、この wave ではなく別設計に切り出す。

1. `BoxBase::new` / `box_id`
2. generic id reuse
3. `StringViewBox` alias 化
4. flat `<= 8 bytes` policy の再変更
5. kernel wholesale rewrite into `.hako`

## Non-goals

1. string kernel を今すぐ wholesale `.hako` rewrite すること
2. perf 問題を `.hako` workaround で隠すこと
3. `llvmlite` / `llvm_py` keep lane を reopen すること
4. benchmark 専用の provenance-sensitive threshold を常設すること
