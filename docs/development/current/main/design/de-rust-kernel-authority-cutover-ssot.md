---
Status: SSOT
Decision: provisional
Date: 2026-03-18
Scope: `hakorune` の独り立ちを「repo から Rust を即 delete すること」ではなく、kernel meaning/policy の owner を `.hako` 側へ移すこととして固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/rep-mir-string-lowering-ssot.md
  - lang/README.md
  - lang/src/runtime/kernel/
---

# De-Rust Kernel Authority Cutover (SSOT)

## Purpose

- `kernel を .hako 化するか` を、wholesale rewrite の議論ではなく owner cutover の順番として固定する。
- `hakorune` の独り立ちを「Rust source が 1 行も残らないこと」ではなく、「kernel meaning/policy の最終 owner が `.hako` であること」として定義する。
- `0rust` は meaning owner zero を意味するが、Rust build/bootstrap route zero を意味しない。
- operational reading は `stage0 Rust bootstrap keep / stage2+ selfhost mainline` であり、kernel authority zero は後者の owner cutover を指す。
- raw substrate micro-optimization と kernel owner cutover を混ぜて、測定や責務境界を濁さない。

## 1. Boundary Lock

1. `kernel authority zero` は `substrate zero` ではない。
   - `kernel authority zero`:
     - method/box の意味
     - route/fallback policy
     - acceptance/contract
     - low-level string algorithm control structure
     - visible runtime kernel orchestration
     を `.hako` が owner する状態
   - `substrate zero`:
     - handle registry
     - GC
     - object layout
     - ABI/FFI substrate
     まで Rust/C から退役させる状態
2. current active reading is that collection owner cutover (`array -> map -> runtime_data cleanup`) has reached its current done-enough stop line, but not end-state completion.
3. raw substrate micro-opt may reopen only after the boundary is deeper than the remaining method-shaped Rust exports still used by the daily `.hako` path, or those exports are explicitly accepted as the long-term substrate cut.
4. `.hako` が先に持つべきなのは policy/contract であり、native substrate ではない。
5. Rust ベースの buildability は migration 中も維持する。

## Practical Substrate Target

- The preferred final shape is not "delete every native line immediately".
- The preferred final shape is: `.hako` owns kernel meaning/policy/control, while native substrate shrinks to the minimum required for bootstrap, ABI/transport, raw leaf memory, handle registry, GC hooks, and LLVM backend emission.
- If a concern can be expressed as policy, route, or control structure, prefer `.hako`.
- Keep LLVM as the primary backend substrate unless a separate SSOT says otherwise.

## 2. Current Truth

- `.hako` 側の runtime kernel edit lane は [`lang/src/runtime/kernel/`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/kernel/) である。
- ただし current runtime substrate はまだ Rust/C 側に多く残っている。
- backend-zero SSOT でも final shape は `.hako -> thin boundary` であって、Rust 全消しを immediate goal にはしていない。
- stage0 first-build / recovery lane としての Rust bootstrap keep は、この wave の失敗条件ではない。
- `string` は stop line 到達済みで parked。
- current active kernel lane is collection owner cutover under `lang/src/runtime/collections/`:
  - `ArrayBox` semantics first
  - `MapBox` semantics second
  - `RuntimeDataBox` cleanup as protocol / facade only
- current stop-line reading:
  - the collection owner cutover acceptance set is green
  - `array` / `map` / `runtime_data` are parked unless a new exact blocker appears
  - next fixed order is boundary-deepen work on the remaining transitional exports:
    - `nyash.array.len_h`
    - `nyash.array.push_hh`
    - `nyash.map.size_h`
  - raw substrate perf stays parked until that deeper boundary is fixed
- raw boundary naming / demotion contract is pinned in:
  - `docs/development/current/main/design/collection-raw-substrate-contract-ssot.md`
- したがって、次に固定すべきは `kernel authority zero` であり、`substrate zero` ではない。

## 3. What Moves First

先に `.hako` 側へ移すべき owner は次だよ。

1. method contract / acceptance contract
2. route selection / fallback policy
3. box-level orchestration
4. visible runtime proof / smoke owner
5. docs/SSOT/README の truth

具体例:

- `StringBox.length/indexOf/substring` の visible contract
- `StringBox.indexOf/contains/startsWith/endsWith` を支える low-level string algorithm control structure
- `ArrayBox` / `MapBox` の method acceptance, bounds/key normalization, visible fallback contract
- `RuntimeDataBox` の protocol/facade contract（owner growthではなく thin routing）
- fallback を許すか freeze するかの判断
- `.hako` kernel の high-level orchestration

## 4. What Stays For Now

当面 Rust/C 側に残してよいものは次だよ。

1. handle registry
2. GC hooks / allocation substrate
3. `StringBox` / `ArrayBox` / `MapBox` の object layout
4. Core C ABI / TypeBox ABI v2 / thin backend boundary
5. perf-critical native leaf

具体例:

- `Registry::alloc`
- `BoxBase::new`
- `host_handles`
- ABI export / marshal
- pointer/string helper の native leaf
- raw byte scan / compare / copy
- flat string allocation / flatten
- `freeze.str` leaf 実装
- stage1 / bootstrap build path

### temporary pilot allowance

exe optimization wave の narrow pilot として、Rust 側に backend-local lowering substrate を置くこと自体は許可する。
ただし条件は固定する。

1. AOT backend-local only
2. owner は docs / `.hako authority`
3. VM / plugin / FFI contract を広げない
4. pilot scope は narrow fixture に限定する
5. `.hako` へ戻せる命令/境界名を docs に先に書く

つまり、temporary pilot はよいが、**temporary pilot を Rust の新しい meaning owner にしない**。
string kernel について言い換えると、`.hako` が algorithm/control owner、Rust/C が raw leaf substrate owner のままに保つ。

### 1.5 0rust buildability lock

- `0rust` は Rust meaning owner zero を意味するが、Rust ベースの build/bootstrap route を消すことではない。
- migration slice を切っても、Rust から daily / compat / bootstrap build が再実行できる状態を保つ。
- Rust buildability が壊れる slice は authority migration の keep 条件を満たさない。

## 5. Fixed Order

1. `kernel authority inventory`
   - current kernel meaning/policy owner と substrate owner を分離して棚卸しする
2. `contract-first migration`
   - docs / smoke / acceptance row の truth を `.hako` 側へ寄せる
3. `shadow owner wave`
   - `.hako` 側で同じ contract を持つ shadow kernel owner を置く
   - ただし daily substrate はまだ Rust/C のまま比較する
4. `daily owner cutover`
   - visible owner を `.hako` に切り替える
   - Rust は substrate / compat keep に降格する
5. `substrate reconsideration`
   - perf / portability / ABI cost を見て、Rust substrate をさらに削るか再判定する

## 6. Active Trigger

この wave を active と読んでよい条件は、もう満たしている。

1. backend-zero の current owner/compat keep wave が stop line に届いている
2. `array_getset` inventory により method-shaped collection semantics が still-Rust substrate だと確認できている
3. collection owner map (`array/map/runtime_data`) が docs で 1 枚に固定されている

## 7. Done Shape

`kernel authority zero` done は次の状態だよ。

1. kernel meaning/policy の SSOT が `.hako` にある
2. low-level string algorithm control structure の SSOT が `.hako` / docs にある
3. daily collection owner は `.hako` ring1 collection core である
4. `RuntimeDataBox` は protocol / facade に限定されていて collection semantics owner ではない
5. Rust runtime は substrate / portability / compat keep に限定されている
6. Rust source が残っていても、それが meaning owner ではない

## 8. Non-goals

1. current collection owner cutover の途中で raw substrate micro-opt を主線に戻すこと
2. `repo から Rust を消すこと` を immediate goal にすること
3. perf hotspot を `.hako` 側 workaround で隠すこと
4. ABI substrate と kernel meaning owner を同じ波で切ること
