---
Status: SSOT
Decision: provisional
Date: 2026-03-25
Scope: `stage2` hakorune の AOT/native fast-lane について、source layering を保ったまま execution layering だけを collapse する設計方向を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - docs/development/current/main/design/stage2-aot-native-external-consultation-question.md
  - docs/abi/vm-kernel.md
  - lang/src/runtime/collections/README.md
  - lang/src/runtime/host/host_facade_box.hako
  - lang/src/hako_alloc/README.md
---

# Stage2 AOT-Native Thin Path (SSOT)

## Goal

- `stage2` hakorune の性能主戦場を `AOT/native` に固定する。
- `.hako` 側の source layering は維持し、execution layering だけを `AOT/native` で collapse する。
- public canonical ABI は増やさず、hot path から generic host/plugin bridge を退かせる。
- この文書は external consultation を採用した設計正本だが、perf proof 自体はまだ未完了として読む。

## Fixed Reading

### Stage meaning

- `stage2` は current repo では build artifact kind ではない。
- `stage2` は `future distribution target / stage-axis label / compare label` として読む。
- 現在の daily build artifact はまだ `stage1-cli` / `launcher-exe` 系で、`stage2` は packaging family ではない。

### Primary target

- 最優先ターゲットは `AOT/native` だよ。
- `rust-vm` は correctness / parity / blocker capture の reference lane として維持する。
- したがって、設計判断は `AOT/native で per-call overhead を最小化できるか` を主基準にする。

### Adopted direction

- source layering は current repo の reading をそのまま採る。
  - `.hako semantic owner`
  - `.hako algorithm/capability substrate`
  - `native metal keep`
- collapse するのは execution layering だけだよ。
  - `AOT/native` では monomorphic route を早めに確定し、hot path の wrapper 連鎖を backend-private fast lane に潰す
  - `rust-vm` では layered route を semantics reference / debug lane として維持する
- つまり、いま必要なのは layering の作り直しではなく、AOT hot path の crossing 削減だよ。

## Current Boundary Map

### 1. `.hako` semantic owner

- current visible owner は `lang/src/runtime/collections/` にある。
- `ArrayCoreBox` / `MapCoreBox` / `RuntimeDataCoreBox` / `StringCoreBox` が user-visible semantics を持つ。
- `lang/src/runtime/kernel/` は string search や numeric loop などの pure control/algorithm owner で、host/plugin/ABI crossing の主戦場ではない。

### 2. `.hako` capability substrate

- current substrate staging root は `lang/src/runtime/substrate/` だよ。
- `raw_array` / `raw_map` / `mem` / `buf` / `ptr` / verifier boxes が、collection owner の一段下にある narrow capability seam を持つ。
- `hako_alloc` はまだ actual allocator ではなく、future alloc/policy root として予約されている。

### 3. Native/Rust metal keep

- current hot substrate はまだ Rust/C に多く残っている。
- 主な crossing は次。
  - `ArrayCoreBox` / `MapCoreBox` / `RuntimeDataCoreBox` から `nyash.array.*` / `nyash.map.*` / `nyash.runtime_data.*` に落ちる route
  - `MirCall/MethodCall/ExternCall` から `HostFacadeBox -> extern_provider/plugin loader` に落ちる route
  - `hako_mem_alloc/realloc/free`, handle registry, GC/barrier に触る allocation route

### 4. `rust-vm` reference lane

- `lang/src/vm/hakorune-vm/` は production-like nyvm orchestration を持つが、この設計では performance owner ではない。
- `rust-vm` は semantics proof / parity / debug observability の lane として残す。

## End-State Reading

### Source layering stays

1. `.hako semantic owner`
   - collection semantics
   - allocator policy
   - route/fallback/contract
2. `.hako algorithm/capability substrate`
   - `hako.mem`
   - `hako.buf`
   - `hako.ptr`
   - `RawArray`
   - `RawMap`
   - future `hakozuna` policy/state owner
3. `native metal keep`
   - final alloc/free/realloc backend
   - TLS/atomic/page/GC hooks
   - ABI entry stubs
4. `rust-vm`
   - reference lane only

### Execution layering collapses in AOT only

- `AOT/native` hot path は次の形を目標にする。
  - `.hako semantic owner`
  - `.hako substrate seam`
  - backend-private fast leaf
  - native metal keep
- `rust-vm` / debug/reference lane は current layered route を維持してよい。
- `HostFacade / extern_provider / plugin loader` は hot path owner ではなく、cold dynamic lane として扱う。

## Fast-Lane Rules

### Hot scalar operations

- per-instruction host crossing を禁止する。
- hot loop 中の `hostbridge` / `extern_provider` / stringly payload / ad-hoc `env.get` を禁止する。
- `len/get/set/has/probe/push-fast` は monomorphic direct fast entry を主線にする。
- `slot/probe/reserve/grow` は runtime generic dispatch ではなく、compile-time seam / naming schema として使う。
- batched interface は `copy/scan/flatten/rehash/reclaim` などの bulk-only lane に限る。

### Lowering / dispatch

- `AOT/native` では lowering/MIR generation が hot route を早めに確定する。
- runtime に残してよいのは cold fallback と debug/parity guard だけだよ。
- `generic box_call` / `generic extern_invoke` / provider dispatch を hot collection op の主線に残さない。

### ABI / fast-path boundary

- public canonical ABI は増やさない。
- public surface は current `Core C ABI / TypeBox ABI v2` を維持する。
- internal-only fast path は許可する。
  - backend-private fast leaf table / manifest
  - hidden leaf id / hidden symbol
  - monomorphic fast entry
- internal fast path は public contract に昇格させない。
- `selector/slot` は public canonical ABI ではなく、backend-private fast lane を組み立てる seam として使う。

### Allocator / hakozuna boundary

- `hako_alloc` / future `hakozuna` は policy/state owner として扱う。
  - size-class policy
  - bin/reclaim policy
  - locality / remote-free routing
  - TLS cache policy
- native keep は actual metal owner に限定する。
  - raw backend allocation
  - platform TLS/atomic
  - final GC integration
  - final ABI stubs
- ownership meaning は manifest / `.hako` 側に置き、retain/release/barrier の実行は compiler-inserted capability/native leaf に閉じる。
- allocator layerに collection semantics や generic plugin semantics を入れない。

## Design Classification

### Clearly safe

- `AOT/native` を性能本線、`rust-vm` を reference lane として固定する。
- source layering を保ったまま execution layering だけを `AOT/native` で collapse する。
- public canonical ABI を `Core C ABI / TypeBox ABI v2` の 2 面のまま維持する。
- collection hot path を raw seam (`slot/probe/reserve/grow`) に寄せ、`HostFacade/provider/plugin` を cold path に退かせる。
- `hako_alloc` を policy/state owner、native を metal keep として分ける。

### Safe only with an explicit internal contract change

- backend-private fast leaf manifest/table を生成する。
- manifest-first row に internal metadata を足す。
  - `leaf_id`
  - `may_alloc`
  - `may_barrier`
  - `cold_fallback`
  - `value_class_profile`
- retain/release/barrier を user-visible surface ではなく compiler-inserted capability op として固定する。

### Unsafe / likely wrong

- 第三の public canonical ABI を作ること。
- `selector/payload` 型の generic dispatcher を hot scalar path の主線にすること。
- runtime が AOT hot path の最終 route choice を毎回やること。
- `atomic/tls/gc` や final allocator backend body まで即座に `.hako` に寄せること。
- batched interface を `get/set/has/push` の canonical 主線にすること。

## Recommended Follow-Ups

### Lane A: collection/runtime hot path

- exact crossing inventory is now locked in
  [`stage2-aot-fast-lane-crossing-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-aot-fast-lane-crossing-inventory.md).
- `collections -> substrate -> native leaf` を 1 crossing に圧縮する。
- `HostFacade/extern_provider/plugin loader` を collection hot path から退かせる。

### Lane B: allocator/hakozuna

- `hako_alloc` を policy/state owner に育てる。
- actual allocator backend は metal keep に閉じる。
- handle/GC/barrier の責務境界を allocator policy から切り離す。

### Lane C: lowering/dispatch

- MIR lowering 時点で hot collection ops を monomorphic 化する。
- generic `box_call` / `extern_invoke` を lowering result に残さない。

### Lane D: perf proof

- perf measurement は次の 4 bucket を分けて読む。
  1. bridge cost
  2. allocation cost
  3. semantic-owner cost
  4. dynamic fallback cost
- benchmark ladder 自体の運用は `perf-optimization-method-ssot.md` を正本にする。

## Immediate Next Task

- backend-private fast leaf contract is now locked in
  [`stage2-fast-leaf-manifest-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md).
- `Array hot path collapse` is landed for the first AOT route-table slice.
- next exact implementation slice is now `Map hot path collapse`.

## Non-Goals

- `rust-vm` を最速 lane として設計し直すこと
- stage2 を新しい artifact kind として先に増やすこと
- generic plugin/extern surface を hot path に残したまま micro-opt でごまかすこと
- `hako_alloc` に actual OS/allocator backend まで持ち込むこと
