---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: `hakorune` の独り立ちを「repo から Rust を即 delete すること」ではなく、kernel meaning/policy の owner を `.hako` 側へ移し、`K-axis` / task pack / owner cutover の読みを混線させない policy に固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/kernel-replacement-axis-ssot.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/rep-mir-string-lowering-ssot.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/design/recipe-scope-effect-policy-ssot.md
  - lang/README.md
  - lang/src/runtime/kernel/
---

# De-Rust Kernel Authority Cutover (SSOT)

## Purpose

- `kernel を .hako 化するか` を、wholesale rewrite の議論ではなく owner cutover の順番として固定する。
- `hakorune` の独り立ちを「Rust source が 1 行も残らないこと」ではなく、「kernel meaning/policy の最終 owner が `.hako` であること」として定義する。
- `0rust` は meaning owner zero を意味するが、Rust build/bootstrap route zero を意味しない。
- operational reading は `stage0 Rust bootstrap keep / stage1 bridge/proof line / stage2-mainline daily mainline / stage2+ umbrella` であり、kernel authority zero は owner cutover 軸を指す。
- `K-axis` reading is `K0 = all-Rust hakorune / K1 = .hako kernel migration stage / K2 = .hako kernel mainline / zero-rust daily-distribution stage`.
- `K2-core` / `K2-wide` are task packs inside `K2`, and the current active order is `stage / docs / naming` -> `K1 done-enough stop-line` -> `K2-core accepted stop-line` -> `K2-wide next structural follow-up` -> `zero-rust default`.
- raw substrate micro-optimization と kernel owner cutover を混ぜて、測定や責務境界を濁さない。
- phase plan SSOT is `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`.

## 0. Axis Lock

- `stage0/stage1/stage2-mainline/stage2+` と `owner/substrate` は別軸で読む。
- current matrix SSOT is `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`.
- replacement milestone is owned by `docs/development/current/main/design/kernel-replacement-axis-ssot.md`.
- `phase-29cm done-enough` は owner axis の局所 stop-line であり、stage2-mainline end-state completion を意味しない。

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
2. current active reading is that collection owner cutover is expressed as `Array phase -> Map phase -> RuntimeData cleanup phase`; it has reached its current done-enough stop line, but not end-state completion.
3. raw substrate micro-opt may reopen only after the boundary is deeper than the remaining method-shaped Rust exports still used by the daily `.hako` path, or those exports are explicitly accepted as the long-term substrate cut.
4. `.hako` が先に持つべきなのは policy/contract であり、native substrate ではない。
5. Rust ベースの buildability は migration 中も維持する。

## Practical Substrate Target

- The preferred final shape is not "delete every native line immediately".
- The preferred final shape is: `.hako` owns kernel meaning/policy/control, while native substrate shrinks to the minimum required for bootstrap, ABI/transport, raw leaf memory, handle registry, GC hooks, and LLVM backend emission.
- The preferred design reading is also `meaning / scope / effect / policy` in `.hako`, with native code reduced to hidden leaf.
- The next step after the current collection stop-line is a capability ladder:
  - `hako_kernel` as the `.hako` semantic owner
  - `hako_substrate` as the `.hako` algorithm substrate
  - capability substrate (`hako.abi`, `hako.value_repr`, `hako.mem`, `hako.buf`, `hako.ptr`, `hako.atomic`, `hako.tls`, `hako.gc`, `hako.osvm`)
  - native metal keep
- If a concern can be expressed as policy, route, or control structure, prefer `.hako`.
- same-boundary の daily replacement code は `hako_kernel` / `hako_substrate` と呼び、`plugin` は cold loader lane に残す。
- Keep LLVM as the primary backend substrate unless a separate SSOT says otherwise.

## 2. Current Truth

- `.hako` 側の runtime kernel edit lane は [`lang/src/runtime/kernel/`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/kernel/) である。
- ただし current runtime substrate はまだ Rust/C 側に多く残っている。
- backend-zero SSOT でも final shape は `.hako -> thin boundary` であって、Rust 全消しを immediate goal にはしていない。
- stage0 first-build / recovery lane としての Rust bootstrap keep は、この wave の失敗条件ではない。
- `string` は stop line 到達済みで parked。
- current `K-axis` reading is `K0 baseline / K1 done-enough on the current collection wave / K2-core RawArray pilot not yet entered`.
- current active kernel lane is collection owner cutover under `lang/src/runtime/collections/`:
  - `Array phase`
  - `Map phase`
  - `RuntimeData cleanup phase`
  - `ArrayBox` semantics first
  - `MapBox` semantics second
  - `RuntimeDataBox` cleanup as protocol / facade only
- current stop-line reading:
  - the collection owner cutover acceptance set is green
  - `array` / `map` / `runtime_data` are parked unless a new exact blocker appears
  - next fixed order is boundary-deepen work on the remaining transitional exports:
    - `nyash.map.entry_count_i64`
  - landed:
    - daily array observer route now uses `nyash.array.slot_len_h`
    - `nyash.array.len_h` is compat-only
    - daily array append route now uses `nyash.array.slot_append_hh`
    - `nyash.array.push_hh` is compat-only
    - daily map observer route now uses `nyash.map.entry_count_i64`
    - `nyash.map.entry_count_h` / `nyash.map.size_h` are compat-only
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

1. `K0 = all-Rust hakorune`
   - current kernel meaning/policy owner と substrate owner を Rust baseline 上で棚卸しする
   - `hako.abi` / `hako.value_repr` / ownership-layout / fail-fast verifier contract を swap truth に固定する
2. `K1 = .hako kernel migration stage`
   - docs / smoke / acceptance row の truth を `.hako` 側へ寄せる
   - `.hako` 側で同じ contract を持つ semantic kernel owner を置き、visible owner を `.hako` に切り替える
   - Rust は substrate / compat keep に降格する
3. `K2 = .hako kernel mainline / zero-rust daily-distribution stage`
   - `K2-core`: `RawArray` を first truthful pilot にする
   - `K2-wide`: `RawMap` を second target にする
   - `K2-wide`: capability widening と metal keep review を同じ stage 内 task pack として進める
   - `RuntimeDataBox` は facade-only のままに固定する

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
7. `K2-wide` の deeper substrate replacement は `.hako substrate module` を daily owner にできる条件でのみ reopen する

## 8. Non-goals

1. current collection owner cutover の途中で raw substrate micro-opt を主線に戻すこと
2. `repo から Rust を消すこと` を immediate goal にすること
3. perf hotspot を `.hako` 側 workaround で隠すこと
4. ABI substrate と kernel meaning owner を同じ波で切ること
5. `K2` を future note のまま凍らせること
6. `K2-wide` を public top-level milestone として再分裂させること
