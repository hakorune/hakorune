---
Status: Provisional SSOT
Decision: accepted-for-phased-rollout
Date: 2026-04-19
Scope: phase-137x で受け入れた string value-model redesign を、北極星設計と段階導入順に分けて固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - crates/nyash_kernel/src/exports/string_plan.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_classify.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_materialize.rs
  - crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs
  - crates/nyash_kernel/src/plugin/array_string_slot.rs
  - crates/nyash_kernel/src/plugin/array_runtime_facade.rs
  - crates/nyash_kernel/src/plugin/array_slot_store.rs
---

# String Value-Model Phased Rollout SSOT

## Quick Scan

- `String` は language meaning では immutable value で、handle/object は boundary representation として読む
- 北極星は `handle-based public ABI` を維持しながら、hot path では text を handle/object world で運ばないこと
- `publish` は boundary effect で、`freeze.str` は唯一の birth sink として分ける
- phase 1 は `TextLane` まで行かない
- phase 1 で正本にする carrier は、今 repo に既にある:
  - `VerifiedTextSource`
  - `TextPlan`
  - `OwnedBytes`
  - `KernelTextSlot`
- `TextOutcome` / `TextCell` / `TextLane` は rollout vocabulary として採用する
- `TextLane` は semantic truth ではなく future storage specialization として扱う
- ただし code では最初から巨大 enum や全面 storage rewrite を入れない
- ordering を間違えると `owner shift only` になって revert しやすい
- next narrow card after the deferred-slot landing is read-side alias lane split, not full `TextLane`
  - `TextReadOnly`
  - `EncodedAlias`
  - `StableObject`
  - stable objectize stays cold and cache-backed, not per-read

## Goal

この文書の目的は 1 つだけだよ。

- phase-137x の accepted redesign を、`北極星` と `今すぐ landing する段` と `まだ deferred にする段` に分けて固定する

これにより、次の設計/実装判断を helper 名ではなく value model と owner boundary で行う。

## Problem Statement

現状の問題は `Rust string` 一般ではない。
phase-137x の hot path が、same-corridor text を途中で public object world に戻していることだよ。

今の whole/meso で繰り返し見えている defect は次。

1. producer が text を作る責務と publish する責務を兼ねている
2. array store が text sink ではなく handle/object sink として振る舞いがち
3. publish が representation になっていて、effect として隔離されていない
4. meaning / boundary / birth sink / storage specialization の層が docs 上で混ざると second truth になりやすい

この 4 つを分けるのが rollout の中心になる。

## North-Star Design

北極星の一枚絵は次で固定する。

```text
Public world
  StringHandle / ArrayHandle / Box<dyn NyashBox>
        ^
        | publish only on escape
        |
Execution world
  VerifiedTextSource
    -> TextPlan
    -> OwnedTextBuf
    -> TextCell / TextLane
```

この読みでは、

- `String` の semantic truth は language world に残す
- public ABI は handle のまま
- execution world では handle/object world を steady-state carrier にしない
- publish は境界 effect
- `freeze.str` は唯一の birth sink
- array は internal storage で text sink に specialize できる
- `TextLane` は future storage specialization であって semantic truth ではない

## Canonical Rollout Vocabulary

この rollout では、設計語彙として次を canonical に使う。

| rollout term | meaning | repo shape today |
| --- | --- | --- |
| `VerifiedTextSource` | source として合法に読める text | `VerifiedTextSource`, borrowed source metadata, source keep mechanics |
| `TextPlan` | bytes 未確定でもよい transient plan | `TextPlan`, `TextPiece`, `StringSpan`, `Pieces3` 相当 |
| `OwnedTextBuf` | alloc/copy 済み unpublished text | `OwnedBytes` |
| `TextCell` | sink に入った unpublished residence | `KernelTextSlot` が最初の canonical shape |
| `PublishedStringHandle` | public handle/object world | `StringBox`, `Arc<dyn NyashBox>`, fresh handle |
| `TextLane` | text-specialized array storage | future internal storage only, semantic truth ではない |

Lock:

- `TextOutcome` は docs vocabulary として採用する
- phase 1 では code に public/general enum を必須化しない
- current code の first-class carrier は既存型の読み替えで足りる
- `publish` と `freeze.str` は別責務のまま保つ

## Owner Split

北極星では owner を 3 つに分ける。

### Producer owner

```text
VerifiedTextSource -> TextPlan -> OwnedTextBuf
```

- text を読む
- plan を正規化する
- 必要なら 1 回だけ owned bytes に freeze する

### Sink owner

```text
TextPlan or OwnedTextBuf -> TextCell
```

- array/string sink に text を住まわせる
- ここでは publish しない
- phase 1 の canonical sink residence は `KernelTextSlot`

### Publish owner

```text
OwnedTextBuf or TextCell -> PublishedStringHandle
```

- `objectize`
- `issue_fresh_handle`
- public ABI replay

問題だったのは、producer owner と publish owner が同じ corridor で潰れていたことだよ。

## Non-Negotiable Constraints

次は phase-137x の拘束条件として固定する。

1. public handle ABI は変えない
2. hot leaf の bytes math / copy は generic helper にまとめない
3. `KernelTextSlot` を side path ではなく canonical sink の first shape として使う
4. `TextLane` は phase 1 に入れない
5. MIR legality を runtime consume capability より先に強化しない
6. registry/TLS/objectize を unpublished carrier に流用しない
7. `publish` と `freeze.str` を別の truth として増殖させない

## Why Ordering Matters

順番を間違えると `revert` になりやすい理由を先に固定する。

### Why not `TextLane` first

- array internal storage を先に変えると、producer/sink/publish の責務分離前に storage rewrite が先行する
- この段階では owner が array facade に移るだけで、本丸の producer publication を消した証明にならない

### Why not MIR legality first

- store consumer が transient/unpublished text をまだ consume できない状態で legality だけ上げると、runtime 側で早期 publish するしかなくなる
- これは `owner shift only` の典型

### Why not generic helper widening first

- `const_suffix`, `Pieces3`, `substring_concat` の hot leaf は専用のまま残すべき
- 最初から巨大 `TextBuilder` を作ると branch と match が hot helper に戻る

### Why `KernelTextSlot` first

- 既に repo にある unpublished sink residence だから
- exact で narrow keeper が出ているから
- whole の active owner を消すには、まず producer result が canonical sink まで unpublished で届く必要があるから

## Phase Boundaries

## Phase 0: Current Baseline Lock

これは実装済み/観測済みの前提段だよ。

- docs-first semantic lock:
  - `String = value`
  - `publish = boundary effect`
  - `freeze.str = only birth sink`
  - `TextLane = future storage specialization`
- exact front では `KernelTextSlot` direct-set / shared-receiver bridge が keeper
- meso はまだ `57 ms` 帯で open
- whole は `856 ms` で open
- current carriers は実質次:
  - `VerifiedTextSource`
  - `TextPlan`
  - `OwnedBytes`
  - `KernelTextSlot`
  - published handle world

Phase 0 の意味:

- `KernelTextSlot` は special-case ではなく、phase 1 の canonical sink seed として扱ってよい

## Phase 1: Producer-First Unpublished Contract

### Goal

- producer が handle を返さなくても corridor が閉じる形を作る
- `KernelTextSlot` を canonical sink residence として固定する

### What lands in phase 1

1. producer result は internal に `TextOutcome` 読みを持てる
2. 実体は既存 carrier を使う
   - `VerifiedTextSource`
   - `TextPlan`
   - `OwnedBytes`
   - `KernelTextSlot`
3. `const_suffix` / direct `Pieces3` / direct `substring->store` は:
   - early handle publish を避ける
   - `KernelTextSlot` まで unpublished で渡す
4. store consumer は:
   - `KernelTextSlot` / verified source / owned unpublished text を consume できる
   - `set_his` fast path を壊さない

### What phase 1 explicitly does not do

- `ArrayStorage::TextLane` 導入
- generic `TextOutcome` enum の全面配線
- global `publish_if_escape(...)` 集約の完成
- MIR verifier による publish legality 強制
- non-text consumer まで含む全面 widening

### Phase 1 repo mapping

| role | current repo shape |
| --- | --- |
| producer source | `borrowed_handle.rs` source/retarget path |
| plan | `string_plan.rs`, substring/piecewise planning |
| owned unpublished | `OwnedBytes` in `string_materialize.rs` |
| sink residence | `KernelTextSlot` in `string_materialize.rs` |
| sink consumer | `array_string_slot.rs`, `array_slot_store.rs` |

### Phase 1 keeper criteria

- exact:
  - no regression
  - current closed front stays closed
- meso:
  - no regression is minimum
  - keeper threshold is `>= 15%` win when the cut targets source/store continuity
- whole:
  - keeper threshold is `>= 10%` win
  - publish/object-world symbols must not move upward
- structural:
  - landed route reaches sink without eager `StringBox -> fresh handle`
  - no new registry-backed unpublished carrier appears

### Phase 1 revert criteria

- exact front reopens
- sink path bypasses `set_his` legality/alias behavior
- producer still materializes public handle before canonical sink
- whole symbols simply move from producer helper to slot helper with no ms win

## Phase 2: Publish Effect Isolation

### Goal

- publish を representation ではなく boundary effect にする

### What lands in phase 2

1. publish sink を site-local helper 群から集約する
2. `OwnedBytes -> PublishedStringHandle`
3. `KernelTextSlot/TextCell -> PublishedStringHandle`
4. objectize/fresh-handle path は cold adapter に押し込む

### Deferred inside phase 2

- `TextLane` storage specialization
- verifier hard error

## Phase 2.5: Read-Side Alias Lane

### Goal

- keep cheap reads on an alias lane instead of promoting to stable/public on every `array.get`

### What lands in phase 2.5

1. split `array.get` demand into:
   - `TextReadOnly`
   - `EncodedAlias`
   - `StableObject`
2. keep alias encode cheap and cache-backed
3. make stable objectize cold and one-shot per cell unless identity is explicitly demanded
4. first landed slice: `BorrowedHandleBox` caches the encoded runtime handle for unpublished keeps

### What phase 2.5 explicitly does not do

- full `TextLane` storage rewrite
- public `TextOutcome` enum rollout
- allocator redesign before read-side aliasing is fixed

### Keeper criteria

- `array.get` no longer pays fresh stable object creation on the common read path
- whole improves without reopening exact or meso
- `StableObject` promotion stays cold and cache-backed

### Structural success signal

- whole の `IPC collapse` が改善し始める
- hot top から `objectize_*` / `issue_fresh_handle` / generic publish branching が落ちる
- `const_suffix` / `Pieces3` helper body が publish owner を持たなくなる

## Phase 3: TextLane Introduction

### Goal

- array internal storage を text-heavy corridor 向けに specialize する

### What lands in phase 3

1. internal storage vocabulary:
   - `Generic`
   - `TextLane`
2. `TextLane` cell vocabulary:
   - `Source`
   - `Owned`
   - `Published`
   - `Empty`
3. degrade path:
   - non-text / mixed semantics が来たら generic に降格できる

### Why this is deferred

- phase 1/2 なしで `TextLane` を入れると storage rewrite が先行しすぎる
- first owner was producer publication, not array storage

### Keeper criteria

- whole で object-world indirection がさらに落ちる
- `array_string_store_kernel_text_slot_at` / `array_get_index_encoded_i64` family の whole cost が縮む
- public semantics remains unchanged

## Phase 4: MIR Legality And Sink-Aware AOT

### Goal

- publish 禁止/許可を contract に上げる
- AOT が sink-aware leaf を吐けるようにする

### What lands in phase 4

1. MIR truth:
   - transient text result class
   - sink consume capability
   - publish boundary
   - stable identity demand
2. legality:
   - same-corridor store
   - loopcarry
   - `Pieces3 -> store`
   - `const_suffix -> store`
   では publish 禁止
3. sink-aware leaf:
   - `concat_suffix_into_text_cell(...)`
   - `piecewise_into_text_cell(...)`

### Keeper criteria

- hot path leaf contains only:
  - source read
  - size calc
  - one alloc
  - memcpy/memmove
  - cell header store
- leaf no longer contains:
  - objectize
  - fresh handle issue
  - registry/TLS branching

## Relationship To Current Repo Carriers

この rollout は「全部作り直す」ではなく、今ある carrier を昇格させる読みだよ。

### `VerifiedTextSource`

- keeps source legality and retarget continuity
- first owner for meso/whole source corridor
- must not pay public publish tax just to stay readable

### `TextPlan`

- planning truth only
- `Pieces3`, substring spans, concat planning
- must not become public truth by default

### `OwnedBytes`

- minimal owned unpublished text
- phase 1/2 の主役
- move-only carrier readingを守る

### `KernelTextSlot`

- phase 1 canonical sink residence
- no longer treated as side optimization only
- seed shape for future `TextCell`

### `TextLane`

- future internal array specialization
- not required to prove phase 1 keeper
- introduced only after producer/sink/publish split is already real

## Allowed And Forbidden Moves

### Allowed

- specialized producers with common carrier contract
- site-local bridges from producer to `KernelTextSlot`
- source/slot continuity before publish
- cold publish adapter consolidation after sink continuity proves out
- temporary coexistence with legacy helper paths during phased rollout when needed to protect the active accept fronts

### Forbidden

- general helper widening before carrier proof
- registry-backed transient carrier
- public ABI widening on phase-137x
- `TextLane` first
- legality-first without runtime consume capability
- keeping permanent dual-routing after phase 2/3 keepers prove the new contract

## Taskization

phase-137x の next implementation queue は次で固定する。

1. Phase 1 completion
   - widen producer-first unpublished contract where current narrow keeper already exists
   - keep `KernelTextSlot` as canonical sink
2. Phase 2 preparation
   - isolate publish owner so producer helpers stop owning handle birth
   - once the new path is keeper-grade, delete legacy helper coexistence instead of preserving dual routing
3. Phase 3 planning only after phase 2 keeper
   - design `TextLane` against real post-phase-2 owner data
4. Phase 4 only after phase 3 direction is proven
   - legality and sink-aware AOT

## Final Lock

この redesign の accepted reading は次だよ。

- `String` は language meaning では immutable value として扱う
- text は public object として運ぶのではなく、execution world では text として運ぶ
- array は phase 1 では `KernelTextSlot` を通じて text sink になり、phase 3 で `TextLane` に進化する
- publish は最後にだけ起こる effect であり、producer/helper の steady-state representation ではない
- `freeze.str` は唯一の birth sink であり、publish policy の second truth ではない

したがって phase-137x の first move は `TextLane` 全面導入ではなく、
`VerifiedTextSource -> TextPlan -> OwnedBytes -> KernelTextSlot` を canonical corridor として固定することだよ。
