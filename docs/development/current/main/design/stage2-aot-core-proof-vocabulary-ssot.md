---
Status: SSOT
Decision: provisional
Date: 2026-03-27
Scope: future `AOT-Core MIR` を急造せず、current MIR/lowering/manifest path に staged proof vocabulary を導入するための正本。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md
  - docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
---

# Stage2 AOT-Core Proof Vocabulary (SSOT)

## Goal

- `AOT-Core MIR` が future に必要になりうることを先に固定する。
- ただし current wave では、新しい独立 IR layer を追加しない。
- 先に必要なのは、current MIR/lowering/manifest path に `AOT/native` 用の proof vocabulary を staged に差し込むことだよ。
- perf wave を「箱そのものを速くする」から「証明できる hot path では箱を collapse する」へ寄せる。

## Fixed Reading

1. `.hako` Stage1 MIR は引き続き semantic owner だよ。
2. `ny-llvmc(boundary pure-first)` は引き続き Stage1/Stage2 mainline-perf owner だよ。
3. current exact front は、新しい IR layer を入れることではなく、proof vocabulary を docs と narrow carrying seam に固定することだよ。
4. `AOT-Core MIR` は future concept として予約するが、いまは current MIR path を壊さず staged に進める。

## Minimal Proof Vocabulary

current wave で first-class に扱う語彙は次だけで固定する。

### `value_class`

- 既存の [`value-repr-and-abi-manifest-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md) を再利用する。
- initial reading:
  - `imm_i64`
  - `imm_bool`
  - `handle_owned`
  - `handle_borrowed_string`
  - `boxed_local`
- current perf wave では、まず integer-heavy array fast lane が `imm_i64` を見分けられることを優先する。

### `escape_kind`

- initial decision axis は次で十分だよ。
  - `local_non_escaping`
  - `return_escape`
  - `field_or_container_escape`
  - `closure_capture`
  - `boundary_escape`
- first purpose は「local non-escaping hot value を heap/handle lane に戻さない」ことだよ。

### `effect`

- initial decision axis は次で固定する。
  - `readonly`
  - `mutates_local`
  - `may_alloc`
  - `may_barrier`
- first purpose は observer/mutator fast path と cold generic lane の分離だよ。

### `cold_fallback`

- 既存の [`stage2-fast-leaf-manifest-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md) を再利用する。
- initial values:
  - `none`
  - `generic_box_call`
  - `host_loader`
- hot path は `cold_fallback=none` を理想にし、slow path は explicit lane 名で押し出す。

## Deferred Vocabulary

次は future wave まで defer する。

- `capture_kind`
- `identity_required`

これらは必要になりうるけれど、current `array get/set/len` stop-line を切るための first vocabulary には入れない。

## First Carrying Seam

current wave の carrying seam は新 IR ではなく、existing backend-private seams だよ。

1. `value_class`
   - current value-repr / ABI manifest truth を使う
2. `cold_fallback`
   - current `FastLeafManifest` row metadata を使う
3. `escape_kind` / `effect`
   - lowering / verifier / route selection 側の internal metadata として持つ

current phase では、public MIR JSON syntax を増やさない。

## First Consumer

first consumer はこれで固定する。

- integer-heavy `ArrayBox.get/set/len` fast lane

current exact target は array substrate hot path だよ。

- `crates/nyash_kernel/src/plugin/array_slot_store.rs`
- `crates/nyash_kernel/src/plugin/array_slot_load.rs`
- `crates/nyash_kernel/src/plugin/handle_cache.rs`
- `src/boxes/array/mod.rs`

この wave では broad generic `Box` optimization へ広げない。

## Promotion Rule To Future AOT-Core MIR

distinct `AOT-Core MIR` layer へ昇格してよいのは、次の条件が揃ったときだけだよ。

1. current MIR path が semantic-owner truth と AOT proof-owner truth の両方を clean に運べなくなった
2. 複数 hot-path consumer が shared proof fields を要求し、local metadata だけでは整理できなくなった
3. separate layer を入れる方が policy/ownership/debug contract を減らせる

つまり、future `AOT-Core MIR` は「今すぐ追加する層」ではなく「current path では clean に保てなくなったら切り出す層」だよ。

## Fixed Order

1. docs で proof vocabulary を固定する
2. current carrying seam を 1 本だけ決める
3. first consumer として integer array fast lane に結ぶ
4. そこで concrete win が出てからだけ、次の proof field や wider consumer を追加する

## Non-Goals

- 今すぐ distinct `AOT-Core MIR` layer を実装すること
- parser/annotation 側の `@hint` / `@contract` を backend-active にすること
- broad generic `Box` deboxing を一気に進めること
- public ABI や external MIR syntax を増やすこと

## Acceptance

- `AOT-Core MIR` は future-needed だが `not-now` と一意に読める
- current wave の proof vocabulary は `value_class / escape_kind / effect / cold_fallback` に固定される
- first carrying seam と first consumer が phase docs だけで辿れる
- `phase-29ck` の exact front が ad hoc perf tries ではなく staged proof-vocabulary lock へ寄る
