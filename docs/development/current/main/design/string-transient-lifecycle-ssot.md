---
Status: SSOT
Decision: provisional
Date: 2026-03-19
Scope: `substring -> concat3 -> length` hot chain のために、string runtime を `authority / transient / birth boundary / substrate` の 4 層で読む設計を固定する
Related:
- CURRENT_TASK.md
- docs/development/current/main/design/string-birth-sink-ssot.md
- docs/development/current/main/design/transient-text-pieces-ssot.md
- docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md
- docs/development/current/main/design/perf-optimization-method-ssot.md
- docs/development/current/main/design/rep-mir-string-lowering-ssot.md
- docs/development/current/main/design/box-identity-view-allocation-design-note.md
- docs/development/current/main/design/substring-view-materialize-boundary-ssot.md
- crates/nyash_kernel/src/exports/string.rs
- crates/nyash_kernel/src/exports/string_view.rs
- src/box_trait.rs
- src/runtime/host_handles.rs
- benchmarks/bench_kilo_micro_substring_concat.hako
---

# String Transient Lifecycle SSOT

## Goal

string hot path を

1. authority / contract
2. transient
3. birth boundary
4. substrate

の 4 層で明示的に分けて、perf と設計の両方で見通しを固定する。

この文書の目的は 2 つだけだよ。

1. `BoxBase::new` を直接いじらず、birth 密度の問題として読む。
2. `substring -> concat3 -> length` の chain を、`plan` と `birth` が混ざらない構造で進める。

## Core Rule

この wave の中心ルールは次だよ。

- `observable` だから birth する、ではない
- `substrate-visible / retained` になった時だけ birth する

つまり:

- read-only observer (`length`, `size`, `indexOf`, read-only substring chain) は transient のまま流してよい
- loop-carried / retained / FFI-visible になった地点で初めて box/handle birth を許す

## The Four Layers

## 1. Authority / Contract

ここは「何が escape か」を決める層だよ。

持つべき責務:

- escape boundary の意味
- view/materialize policy
- low-level string algorithm control structure
- intrinsic contract naming
- benchmark/workload に対する禁止事項
- `.hako` / docs 側の owner truth

この層では具体的な `StringBox` / `StringViewBox` / handle を作らない。

具体例:

- `str.find_byte_from`
- `str.eq_at`
- `str.len`
- `freeze.str`

SSOT 上の canonical 名は `str.*` / `freeze.str` だよ。
`.hako` 側の internal spellings は `__str.*` にしてよい。

## 2. Transient

ここは box でも handle でもない、内部の文字列 transport 層だよ。

要件:

- identity を持たない
- GC/finalization key に参加しない
- read-only chain を保持できる
- `substring` だけでなく `concat3` も表現できる

この repo の current substrate では `StringSpan` が基礎になる。
ただし `left + "xx" + right` のような非連続 chain を扱うには、単一 span だけでは足りない。

この wave で採る設計方向は:

- transient は `TextPlan` / `TText` として読む
- carrier は `Piece = RootSlice | InlineLit` の normalized small piece list にする
- `Concat2` / `Concat3` / `Insert` は owner ではなく normalize step として扱う
- `TText = View1 | PiecesN | OwnedTmp` に寄せ、`PiecesN` を backend-local carrier にする
- transport token を導入する場合でも、それは意味の owner ではなく transient 表現の容器に留める
- narrow pilot としては `RepMIR` / `RepKind` の AOT backend-local representation を許可する
- ただしその場合でも owner は docs / `.hako authority` に置き、Rust private enum を truth にしない
- `RepMIR` は runtime の新しい層ではなく、AOT consumer が一瞬だけ使う shadow representation として扱う
- compiler-local な transport に留め、VM / plugin / FFI / host handle へ見せない

## 3. Birth Boundary

ここは transient を substrate-visible value に freeze する唯一の層だよ。

current docs-first cut では [`string-birth-sink-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/string-birth-sink-ssot.md) を、この層の narrow parent SSOT として使う。

期待する形:

```rust
fn freeze_string(/* transient repr */, boundary: BoundaryKind) -> i64
```

この層だけが次に触ってよい:

- `StringBox`
- `StringViewBox`
- `BoxBase::new`
- `handles::to_handle_arc(...)`
- shared empty handle
- materialize/copy

つまり「plan は plan」「birth は freeze」に分けるのが基本だよ。

この境界は 3 分割で読む。

- policy owner: `.hako` / docs
- placement owner: AOT compile-time (`BirthPlacement`)
- sink owner: runtime `freeze.str`

runtime に第三の observable layer は増やさない。

## 4. Substrate

ここは correctness-bearing 実装層だよ。

含まれるもの:

- `StringBox`
- `StringViewBox`
- `BoxBase`
- handle registry
- GC / finalization
- native string helpers
- raw byte scan / compare / copy / flatten
- `freeze.str` leaf 実装

この層は hot でも、今 wave では原則としていじらない。

## Escape Boundary Rule

現 wave の exact boundary は次で固定する。

### birth してよい地点

- loop-carried assignment
- array/map 格納
- FFI / C ABI visible point
- clone/share/retention boundary

### transient のままでよい地点

- `substring` result that is consumed immediately by another read-only helper
- `concat3` input/output inside the same read-only chain
- `length` / `size` / `indexOf` observer path

### Current benchmark lock

[`benchmarks/bench_kilo_micro_substring_concat.hako`](/home/tomoaki/git/hakorune-selfhost/benchmarks/bench_kilo_micro_substring_concat.hako)
では、この判断を採る。

- `left = text.substring(...)` -> transient candidate
- `right = text.substring(...)` -> transient candidate
- `out = left + "xx" + right` -> transient candidate
- `acc = acc + out.length()` -> transient observer
- `text = out.substring(1, len + 1)` -> first escape boundary

## Current Owner Reading

現在の live owner をこの 4 層に対応させるとこうだよ。

### Authority / Contract

- `docs/development/current/main/design/transient-string-chain-boxless-wave-ssot.md`
- `docs/development/current/main/design/substring-view-materialize-boundary-ssot.md`
- `CURRENT_TASK.md`
- `lang/src/runtime/kernel/string/README.md`

### Transient

- `crates/nyash_kernel/src/exports/string_view.rs::StringSpan`
- `docs/development/current/main/design/transient-text-pieces-ssot.md`
- `TextPlan` / `TText`
- 将来の transient piece / recipe 表現

### Birth Boundary

- 現状はまだ分離されていない
- `borrowed_substring_plan_from_handle(...)` と `string_handle_from_owned(...)` / `concat3_hhh` の間に散っている
- 次 wave ではここを 1 箇所に寄せる

### Substrate

- `StringBox`
- `StringViewBox`
- `BoxBase`
- `host_handles`
- `Registry::alloc`

## Immediate Refactor Direction

いまの code-side で first target にするのはこれだよ。

1. `borrowed_substring_plan_from_handle(...)` から substrate-specific birth を薄くする
2. `substring_hii` / `concat3_hhh` の中で直接 `string_handle_from_owned(...)` する責務を、将来の freeze 境界へ寄せる
3. `BoxBase::new` / id semantics には触らない
4. placement は runtime helper 常設ではなく、AOT consumer 側の shadow lowering を優先する
5. low-level string algorithm control structure は `.hako` owner へ寄せ、raw byte leaf を substrate に残す

## Pilot Lock

`RepMIR` をやるなら、いきなり runtime 全体へ広げない。

最初の pilot は次だけに限定する。

1. AOT backend only
2. `kilo_micro_substring_concat`
3. `thaw.str -> str.slice -> str.cat3 -> str.len -> freeze.str`
4. runtime / VM / plugin / FFI には transient token を見せない

これを超える widening は、別判断にする。

## Non-goals

1. `BoxBase::new` を generic に最適化すること
2. `StringViewBox` を alias object にすること
3. generic handle reuse
4. current flat `<= 8 bytes` short-slice policy を再び動かすこと
5. `src/llvm_py/**` keep lane を reopen すること
