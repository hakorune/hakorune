# Nyash Applications Showcase

このディレクトリには、Nyashの実力を示す実用的なアプリケーションが含まれています。

開発・貢献に関する全体ガイドはリポジトリルートの`AGENTS.md`（Repository Guidelines）を参照してください。プロジェクト構成、ビルド/テスト、PR要件の要点を簡潔にまとめています。

## 🚀 実装済みアプリケーション

### 🎮 ゲーム・エミュレータ

#### CHIP-8エミュレータ
**場所**: `chip8_nyash/chip8_emulator.hako`  
**特徴**: 完全なゲーム機エミュレータ、グラフィック表示対応
```bash
./target/release/hakorune apps/chip8_nyash/chip8_emulator.hako
```

### 📝 エディタ・開発ツール

#### Enhanced Kilo Editor
**場所**: `kilo_nyash/enhanced_kilo_editor.hako`  
**特徴**: テキストエディタ（kilo改良版）、実用的なファイル編集機能
```bash
./target/release/hakorune apps/kilo_nyash/enhanced_kilo_editor.hako
```

### 🌐 ネットワークアプリ

#### TinyProxy
**場所**: `tinyproxy_nyash/proxy_server.hako`  
**特徴**: HTTPプロキシサーバー、Netプラグイン活用
```bash
./target/release/hakorune apps/tinyproxy_nyash/proxy_server.hako
```

### 🛠️ ユーティリティ・ベンチマーク

#### BoxTorrent Mini
**場所**: `boxtorrent-mini/main.hako`
内容アドレス化チャンク、重複排除、明示refcountを持つローカルBox store。

```bash
./target/release/hakorune --backend vm apps/boxtorrent-mini/main.hako
```

**特徴**:
- `ContentHash` による決定的チャンクID
- `BoxTorrentStore` で同一内容のchunkを再利用
- `BoxTorrentManifest` でpayloadを復元
- 参照カウントreleaseでcache lifecycleを確認

#### binary-trees
**場所**: `binary-trees/main.hako`
CLBG風の二分木生成・破棄・再帰checksumを行う小型ベンチ。

```bash
./target/release/hakorune --backend vm apps/binary-trees/main.hako
```

**特徴**:
- stretch tree と long-lived tree の両方を生成
- 複数depthで短命treeを大量生成
- 再帰 `itemCheck` で構造と値を検証
- 後続のGC/allocator評価へ拡張しやすい固定出力

#### mimalloc-lite
**場所**: `mimalloc-lite/main.hako`
本物allocator移植の前段として、page/free-list/reuseを持つ小型モデル。

```bash
./target/release/hakorune --backend vm apps/mimalloc-lite/main.hako
```

**特徴**:
- small/medium page の固定サイズ割り当て
- handle による release route
- free-list reuse と peak usage の集計
- 本物 allocator port 前に語彙とsmokeを固定

#### allocator-stress
**場所**: `allocator-stress/main.hako`
`hako_alloc` page/free-list seam の飽和・再利用・rejectを固定する小型ストレス。

```bash
./target/release/hakorune --backend vm apps/allocator-stress/main.hako
```

**特徴**:
- small/medium page の満杯状態を確認
- release 後の free-list reuse を確認
- oversize と double-free の reject を固定
- `hako_alloc` public seam 経由の deterministic accounting

#### hako-alloc-production-facade-proof
**場所**: `hako-alloc-production-facade-proof/main.hako`
M46 proof。`HakoAllocProductionFacade` が production allocator port の
public seam として既存 `HakoAllocHeap` page/free-list policy state へ委譲する
ことを固定する。

```bash
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
```

**特徴**:
- production-facing facade 名を `hako_alloc` 配下に固定
- allocate/release/reject の最小 accounting を検証
- backend allocator replacement / pointer fetch_add / native pointer attrs は追加しない

#### hako-alloc-local-page-policy-proof
**場所**: `hako-alloc-local-page-policy-proof/main.hako`
M47 proof。`HakoAllocProductionFacade` 経由で local page policy の
small/medium allocate/free/reject/reuse accounting を pure-first EXE で固定する。

```bash
bash tools/checks/k2_wide_hako_alloc_local_page_policy_exe_guard.sh
```

#### hako-alloc-usize-field-probe
**場所**: `hako-alloc-usize-field-probe/main.hako`
294x-18 probe。production migration とは別に、`hako_alloc` 配下の
probe-only box で capacity / used / alloc_count / requested_bytes の
`usize` stored field 形を VM reference で固定する。

```bash
bash apps/hako-alloc-usize-field-probe/test.sh
```

#### mimalloc-alloc-fast-path-proof
**場所**: `mimalloc-alloc-fast-path-proof/main.hako`
M167 proof。`HakoAllocPageQueue` の page selection と
`HakoAllocPageModel.acquire(...)` の free-list pop を
`HakoAllocFastPathHeap` で合成し、full queue 時の deterministic fallback
page creation を固定する。

```bash
bash tools/checks/k2_wide_mimalloc_alloc_fast_path_guard.sh
```

**特徴**:
- production facade から既存 `HakoAllocHeap` page/free-list state へ委譲
- oversize reject と double-free reject を public seam 経由で検証
- remote-free / OSVM page-source / allocator replacement hook はまだ扱わない

#### mimalloc-osvm-page-source-composition-proof
**場所**: `mimalloc-osvm-page-source-composition-proof/main.hako`
M168 proof。M167 の page queue + page-local free-list model を、
既存 `HakoAllocPageSourcePolicy` reserve/commit/decommit seam と合成する。

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_source_composition_guard.sh
```

**特徴**:
- fresh modeled page creation が OSVM page-source policy を通る
- M167 の `HakoAllocFastPathHeap` は OSVM-free のまま維持する
- local-free retire / remote-free / page-map / provider / hook はまだ扱わない

#### mimalloc-local-free-retire-proof
**場所**: `mimalloc-local-free-retire-proof/main.hako`
M169 proof。`HakoAllocPageModel` の same-thread `local_free` を reusable free
stack に戻し、empty-page retire state を page-local に観測できることを固定する。

```bash
bash tools/checks/k2_wide_mimalloc_local_free_retire_guard.sh
```

**特徴**:
- `acquire()` が必要時に `local_free` を再利用可能な free block に戻す
- final `releaseLocal()` が empty-page retire state を idempotent に記録する
- remote-free / abandoned reclaim / page-map / OSVM release はまだ扱わない

#### mimalloc-remote-free-page-integration-proof
**場所**: `mimalloc-remote-free-page-integration-proof/main.hako`
M170 proof。既存 `HakoAllocRemoteFreePolicy` の bounded pointer CAS retry
publish を `HakoAllocRemoteFreePageInbox` で page-owned `releaseLocal(...)`
state へ合成する。

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh
```

**特徴**:
- pointer load/store/CAS route facts は既存 remote-free policy 所有のまま
- page state mutation は `HakoAllocPageModel.releaseLocal(...)` に委譲する
- caller-provided block id proof seam なので page-map / arbitrary pointer
  free / provider / hook / replacement はまだ扱わない

#### mimalloc-page-map-proof
**場所**: `mimalloc-page-map-proof/main.hako`
M171 proof。caller-visible pointer id を `page_id` / `block_id` に解決する
`HakoAllocPageMap` model を固定する。

```bash
bash tools/checks/k2_wide_mimalloc_page_map_guard.sh
```

**特徴**:
- register / lookup / unregister の ownership map だけを扱う
- page release / realloc / pointer arithmetic / native metal はまだ扱わない
- M172 の page-map-backed release seam の前提になる

#### mimalloc-page-map-release-proof
**場所**: `mimalloc-page-map-release-proof/main.hako`
M172 proof。`HakoAllocPageMap.lookup(...)` で caller-visible pointer を
page/block identity に解決し、`HakoAllocPageModel.releaseLocal(...)` に
委譲してから `HakoAllocPageMap.unregister(...)` する release seam を固定する。

```bash
bash tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
```

**特徴**:
- pointer registration は M171 `HakoAllocPageMap.register(...)` の責務として残す
- M172 seam は lookup / page-local release / unregister の合成だけを扱う
- realloc / aligned allocation / huge allocation / secure-list / OSVM release /
  provider / hook / replacement はまだ扱わない

#### hako-alloc-remote-free-policy-proof
**場所**: `hako-alloc-remote-free-policy-proof/main.hako`
M48 proof。`HakoAllocProductionFacade` 経由で M43 の bounded CAS retry-loop
remote-free policy を pure-first EXE で固定する。

```bash
bash tools/checks/k2_wide_hako_alloc_remote_free_policy_exe_guard.sh
```

**特徴**:
- facade が `HakoAllocRemoteFreePolicy` へ委譲する public seam を検証
- pointer store/load/CAS route facts は substrate 所有のまま
- pointer fetch_add / native pointer attrs / OSVM page-source は追加しない

#### hako-alloc-page-source-policy-proof
**場所**: `hako-alloc-page-source-policy-proof/main.hako`
M49 proof。`HakoAllocProductionFacade` 経由で OSVM reserve/commit/decommit
page-source policy を pure-first EXE で固定する。

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_policy_exe_guard.sh
```

**特徴**:
- facade が `HakoAllocPageSourcePolicy` へ委譲する public seam を検証
- OSVM route facts は `OsVmCoreBox` / substrate 所有のまま
- OSVM unreserve/release / allocator replacement hook / native attrs は追加しない

#### hako-alloc-production-facade-stress
**場所**: `hako-alloc-production-facade-stress/main.hako`
M50 proof。既存 `allocator-stress` の small/medium saturation・reuse・reject
accounting shape を `HakoAllocProductionFacade` 経由で pure-first EXE 固定する。

```bash
bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
```

**特徴**:
- app から `HakoAllocHeap` / `HakoAllocPage` を直接触らない
- facade 経由で既存 allocator-stress と同じ deterministic accounting を検証
- allocator replacement hook / pointer fetch_add / native attrs は追加しない

#### mimalloc-raw-page-proof
**場所**: `mimalloc-raw-page-proof/main.hako`
M12 substrate proof。`RawBufCoreBox` と `RawArrayCoreBox` を明示的に使う
raw page/free-list fixture。

```bash
apps/mimalloc-raw-page-proof/test.sh
```

**特徴**:
- raw page allocation/free は `RawBufCoreBox` 経由
- free-list slot 操作は `RawArrayCoreBox` 経由
- fast path に `Contract(no_alloc/no_safepoint)` を付けて MIR verify
- Profile/Capability/unsafe/backend special-case は未使用

#### mimalloc-size-class-table-proof
**場所**: `mimalloc-size-class-table-proof/main.hako`
M21 substrate proof。source `static const u16[]` size-class tables と
raw page/free-list pure-first EXE route を合成する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh
```

**特徴**:
- size-class metadata は runtime Array/Map ではなく MIR `static_data_plans`
- table read は `static_data_load` 経由
- RawBuf/RawArray route は M14-M20 の既存 facts を再利用
- pure-first は MIR-owned static data facts だけを読む。新しい source syntax
  / allocator policy は追加しない

#### mimalloc-two-class-page-proof
**場所**: `mimalloc-two-class-page-proof/main.hako`
M22 substrate proof。static size-class table から small/medium の2ページを作り、
reject/release/reuse を pure-first EXE で固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_two_class_page_exe_guard.sh
```

**特徴**:
- small=32/4、medium=64/2 を MIR `static_data_plans` から読む
- 2つの raw page が M14-M20 RawBuf/RawArray route を再利用
- full-page reject、oversize reject、release 後 reuse を固定
- 新しい source syntax / allocator policy は追加しない

#### mimalloc-dynamic-bin-proof
**場所**: `mimalloc-dynamic-bin-proof/main.hako`
M23 substrate proof。実行時に選んだ class index で static size-class table
を読み、raw page sequence を pure-first EXE で固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_dynamic_bin_exe_guard.sh
```

**特徴**:
- `MI_SIZE_CLASS[class_idx]` / `MI_CLASS_CAP[class_idx]` の非定数 index を固定
- request=48 から medium class `64/2` を選ぶ
- RawBuf/RawArray route は既存 M14-M20 facts を再利用
- general `size_to_bin` / allocator policy は追加しない

#### mimalloc-size-to-bin-inline-proof
**場所**: `mimalloc-size-to-bin-inline-proof/main.hako`
M24 substrate proof。`Profile(allocator.fast)` の `size_to_bin` helper を
MIR optimizer で inline し、その結果を static size-class table load に流す fixture。

```bash
bash tools/checks/k2_wide_mimalloc_size_to_bin_inline_exe_guard.sh
```

**特徴**:
- `size_to_bin` は source helper のまま、pure-first backend 前に展開
- backend/.inc は `Profile(allocator.fast)` を読まない
- dynamic `static_data_load` と RawBuf/RawArray route を合成
- wider inline shape / general mimalloc bin algorithm は追加しない

#### mimalloc-size-class-policy-proof
**場所**: `mimalloc-size-class-policy-proof/main.hako`
M163 proof。`hako_alloc` の `SizeClassBox` が mimalloc-shaped
size-to-bin / bin-size policy を純粋関数として持ち、既存 `LayoutBox`
互換 facade が small/medium 行動を維持することを固定する。

```bash
bash tools/checks/k2_wide_mimalloc_size_class_policy_guard.sh
```

**特徴**:
- upstream mimalloc v3.3.2 の 8-byte word / 73-bin 形を `.hako` policy に固定
- allocator page state、free-list、RawBuf/RawArray、OSVM、TLS、atomic は触らない
- `LayoutBox` は現行 `mimalloc-lite` / production facade 互換のまま
- provider activation / hook / process allocator replacement は追加しない

#### mimalloc-osvm-page-proof
**場所**: `mimalloc-osvm-page-proof/main.hako`
M25 substrate proof。`OsVmCoreBox.reserve_bytes_i64/commit_bytes_i64/decommit_bytes_i64`
を pure-first EXE で実行し、OSVM page reservation seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
```

**特徴**:
- source は `hako.osvm` facade を使う
- MIR-owned extern route facts が reserve/commit/decommit を表す
- pure-first は route facts を emit するだけで、app-specific matcher を持たない
- page-size row / unreserve API / TLS / atomic は追加しない

#### mimalloc-tls-cache-slot-proof
**場所**: `mimalloc-tls-cache-slot-proof/main.hako`
M26 substrate proof。`TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64`
を pure-first EXE で実行し、allocator fast-path 前の TLS cache-slot seam
を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh
```

**特徴**:
- source は `hako.tls` facade を使う
- MIR-owned extern route facts が cache-slot get/set を表す
- pure-first は route facts を emit するだけで、app-specific matcher を持たない
- generic TLS cell / allocator policy / atomic remote-free は追加しない

#### mimalloc-worker-tls-cache-proof
**場所**: `mimalloc-worker-tls-cache-proof/main.hako`
MIMAP-TLS-001 substrate proof。`HakoAllocWorkerTlsCache` が
`HakoAllocWorkerIdentity` と `TlsCoreBox.cache_slot_get_i64/cache_slot_set_i64`
を合成し、allocator-internal worker cache-slot state を pure-first EXE で
固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh
```

**特徴**:
- source は `hako_alloc` の allocator-facing owner を使う
- worker id と cache-slot get/set の両方を MIR-owned route facts から emit する
- scalar proof state は slot/value/worker/count に限定する
- source-level worker-local / generic TLS cell / atomics / remote-free は追加しない

#### mimalloc-atomic-cas-proof
**場所**: `mimalloc-atomic-cas-proof/main.hako`
M27 substrate proof。`AtomicCoreBox.cas_i64/3` を pure-first EXE で実行し、
remote-free 前の最小 atomic CAS seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh
```

**特徴**:
- source は `hako.atomic` facade を使う
- MIR-owned extern route facts が fixed i64 CAS を表す
- pure-first は route facts を emit するだけで、app-specific matcher を持たない
- load/store/fetch_add / pointer CAS / memory-order args / remote-free policy は追加しない

#### mimalloc-atomic-load-proof
**場所**: `mimalloc-atomic-load-proof/main.hako`
M28 substrate proof。`AtomicCoreBox.load_i64/1` を pure-first EXE で実行し、
fixed i64 load seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh
```

**特徴**:
- source は `hako.atomic` facade を使う
- MIR-owned extern route facts が fixed i64 load を表す
- pure-first は route facts を emit するだけで、app-specific matcher を持たない
- store/fetch_add / pointer load-store / memory-order args / remote-free policy は追加しない

#### mimalloc-atomic-store-proof
**場所**: `mimalloc-atomic-store-proof/main.hako`
M29 substrate proof。`AtomicCoreBox.store_i64/2` を pure-first EXE で実行し、
fixed i64 store seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh
```

**特徴**:
- source は `hako.atomic` facade を使う
- MIR-owned extern route facts が fixed i64 store を表す
- pure-first は route facts を emit するだけで、app-specific matcher を持たない
- fetch_add / pointer store / memory-order args / remote-free policy は追加しない

#### mimalloc-atomic-fetch-add-proof
**場所**: `mimalloc-atomic-fetch-add-proof/main.hako`
M30 substrate proof。`AtomicCoreBox.fetch_add_i64/2` を pure-first EXE で実行し、
fixed i64 fetch-add seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh
```

**特徴**:
- source は `hako.atomic` facade を使う
- MIR-owned extern route facts が fixed i64 fetch-add を表す
- pure-first は route facts を emit するだけで、app-specific matcher を持たない
- pointer fetch-add / memory-order args / remote-free policy は追加しない

#### mimalloc-remote-free-i64-proof
**場所**: `mimalloc-remote-free-i64-proof/main.hako`
M31 composition proof。既存の `AtomicCoreBox.cas_i64/load_i64/store_i64/fetch_add_i64`
を使って、fixed-slot i64 remote-free push sketch を pure-first EXE で実行する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh
```

**特徴**:
- source は `hako.atomic` facade の既存 primitive だけを使う
- MIR-owned route facts の合成で LIFO head update / next-link storage / enqueue count を表す
- pure-first は既存 route facts を emit するだけで、新しい route row を持たない
- pointer atomics / memory-order args / production remote-free policy は追加しない

#### mimalloc-ptr-atomic-store-proof
**場所**: `mimalloc-ptr-atomic-store-proof/main.hako`
M35 route proof。`hako_atomic_ptr_store_ordered(cell_ptr, value_ptr, order)`
を direct extern route として pure-first EXE で実行し、最初の native pointer
atomic store seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh
```

**特徴**:
- source は direct externcall を使い、`AtomicCoreBox` pointer method は追加しない
- MIR-owned extern route facts が `extern.hako_atomic.ptr_store_ordered` を表す
- pure-first は native pointer transport を `ptr` 引数に変換して emit する
- pointer fetch_add / production remote-free policy は追加しない

#### mimalloc-ptr-atomic-load-proof
**場所**: `mimalloc-ptr-atomic-load-proof/main.hako`
M39 route proof。`hako_atomic_ptr_load_ordered(cell_ptr, order)` を direct
extern route として pure-first EXE で実行し、native pointer atomic load seam
を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh
```

**特徴**:
- source は direct externcall を使い、`AtomicCoreBox` pointer method は追加しない
- MIR-owned extern route facts が `extern.hako_atomic.ptr_load_ordered` を表す
- pure-first は native pointer return を i64 transport に変換して emit する
- pointer fetch_add / production remote-free policy は追加しない

#### mimalloc-ptr-atomic-cas-proof
**場所**: `mimalloc-ptr-atomic-cas-proof/main.hako`
M40 route proof。`hako_atomic_ptr_cas_ordered(cell_ptr, expected_ptr,
desired_ptr, success_order, failure_order)` を direct extern route として
pure-first EXE で実行し、native pointer atomic CAS seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh
```

**特徴**:
- source は direct externcall を使い、`AtomicCoreBox` pointer method は追加しない
- MIR-owned extern route facts が `extern.hako_atomic.ptr_cas_ordered` を表す
- pure-first は native pointer args/return を i64 transport として変換して emit する
- pointer fetch_add / production remote-free policy は追加しない

#### mimalloc-ptr-remote-free-list-proof
**場所**: `mimalloc-ptr-remote-free-list-proof/main.hako`
M41 composition proof。既存の `hako_atomic_ptr_store_ordered` /
`hako_atomic_ptr_load_ordered` / `hako_atomic_ptr_cas_ordered` route facts だけで
two-node remote-free list push を pure-first EXE で固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh
```

**特徴**:
- source は direct externcall を使い、`AtomicCoreBox` pointer method は追加しない
- block 先頭 word を next pointer cell として使う
- MIR-owned route facts の合成だけで head CAS publish と next link を表す
- 新しい route row / pointer fetch_add / production remote-free policy は追加しない

#### mimalloc-remote-free-list-policy-proof
**場所**: `mimalloc-remote-free-list-policy-proof/main.hako`
M42 policy integration proof。M41 の two-node remote-free list push 形を
`AllocatorRemoteFreeListPolicy` の same-module method に移し、pure-first EXE で
generic-i64 global route 経由の policy seam を固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh
```

**特徴**:
- same-module policy box が head init / push / peek を所有する
- method body 内で pointer load → next store → head CAS を実行する
- main は alloc/free と list shape 検証だけを持つ
- 新しい route row / pointer fetch_add / production retry loop は追加しない

#### mimalloc-remote-free-retry-loop-proof
**場所**: `mimalloc-remote-free-retry-loop-proof/main.hako`
M43 retry-loop proof。`AllocatorRemoteFreeRetryPolicy` が bounded CAS retry loop
を所有し、既存 pointer store/load/CAS route facts だけで retry 後の publish を
pure-first EXE で固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh
```

**特徴**:
- policy method 内で load → next store → CAS → retry を実行する
- 1回だけ competing push を注入して CAS failure を作る
- 最終形 `block_b -> block_c -> block_a -> null` を検証する
- 新しい route row / pointer fetch_add / production allocator policy は追加しない

#### mimalloc-tls-ptr-remote-free-proof
**場所**: `mimalloc-tls-ptr-remote-free-proof/main.hako`
M36 composition proof。TLS cache-slot に native mailbox pointer を置き、
`hako_atomic_ptr_store_ordered` で native block pointer を publish する
remote-free mailbox seam を pure-first EXE で固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh
```

**特徴**:
- M26 TLS cache-slot rows と M35 pointer-store row だけを合成する
- MIR-owned route facts が TLS helper と direct pointer store を表す
- pure-first は既存 route facts を emit するだけで、新しい route row を持たない
- pointer fetch_add / production allocator policy は追加しない

#### mimalloc-remote-free-policy-proof
**場所**: `mimalloc-remote-free-policy-proof/main.hako`
M37 policy integration proof。`AllocatorRemoteFreePolicy` が TLS mailbox の
install/publish/release を所有し、既存 M26/M35 route facts だけで pure-first
EXE 実行できることを固定する fixture。

```bash
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh
```

**特徴**:
- same-module policy box が remote-free mailbox seam の app-level policy を持つ
- MIR-owned route facts が TLS helper と direct pointer store を表す
- pure-first は既存 route facts を emit するだけで、新しい route row を持たない
- pointer fetch_add / full remote-free list policy は追加しない

#### allocator-fast-path-exe-proof
**場所**: `allocator-fast-path-exe-proof/main.hako`
M13 scalar EXE proof。`Profile(allocator.fast)` を MIR-owned verified
required InlinePlan として消費し、pure-first EXE には展開済み scalar MIR
だけを渡す fixture。

```bash
bash tools/checks/k2_wide_allocator_fast_path_exe_guard.sh
```

**特徴**:
- `Profile(allocator.fast)` は InlinePlan/EffectPlan/CapabilityPlan facts に展開
- verified required inline は MIR optimizer が消費
- backend/.inc は profile 名を読まない
- RawBuf/RawArray/native pointer EXE lowering は未受理

#### json-stream-aggregator
**場所**: `json-stream-aggregator/main.hako`
JSONL風イベントを逐次読み、userごとのbytes/ok/failを集計する小型アプリ。

```bash
./target/release/hakorune --backend vm apps/json-stream-aggregator/main.hako
```

**特徴**:
- narrow JSONL scanner
- `MapBox` による user stats 集計
- deterministic report output
- stream processing seam の real-app smoke

#### ny-echo - 最小CLI実装
**場所**: `ny-echo/main.hako`
標準入力を読み取り、オプションに応じて変換して出力する基本的なCLIツール。

```bash
# 基本使用
echo "Hello World" | nyash apps/ny-echo/main.hako

# 大文字変換
echo "hello" | nyash apps/ny-echo/main.hako --upper

# 小文字変換
echo "HELLO" | nyash apps/ny-echo/main.hako --lower
```

**特徴**:
- ConsoleBoxによるI/O処理
- StringBoxの変換メソッド活用
- VM/AOTで同一動作（JIT実行は現在封印）

### 2. ny-array-bench - 性能ベンチマーク
ArrayBoxの各種操作をベンチマークし、VM/JIT/AOTの性能比較を行うツール。

```bash
# ベンチマーク実行
nyash apps/ny-array-bench/main.hako

# 出力例（JSON形式）
{
  "create_1000": 1.23,
  "map_1000": 2.45,
  "reduce_1000": 0.98,
  "relative_performance": {"vm": 1.0, "aot": 5.0}
}
```

**特徴**:
- カスタムStatsBoxによる計測
- JSON形式でCI連携可能
- 性能改善の定量的測定

### 3. ny-jsonlint（開発中）
PyRuntimeBoxを使用してPythonのjsonモジュールでJSON検証を行うツール。

### 4. ny-filegrep（開発中）
ファイルシステムを検索し、パターンマッチングを行う実用的なツール。

### 5. ny-http-hello（開発中）
HTTPサーバーを実装し、Web対応を実証するデモアプリケーション。

## 🔧 ビルドと実行

### 実行方法
```bash
# インタープリター実行
nyash apps/APP_NAME/main.hako

# VM実行（高速）
nyash --backend vm apps/APP_NAME/main.hako

# JIT実行（封印中）
# 現在は無効です。Interpreter/VM か AOT(EXE) を使用してください。
```

### テスト実行
各アプリケーションにはtest.shが含まれています：

```bash
cd apps/ny-echo
./test.sh
```

## 🎯 予定アプリケーション（論文・ベンチマーク用）

### 📊 CLBG標準ベンチマーク
AI先生たちの推奨により、論文説得力向上のため以下を実装予定：

#### 1. binary-trees - メモリ・GC性能測定
**目的**: GC性能、メモリ割り当て速度測定
**期待性能**: Interpreter(1x) → VM(8x) → LLVM(20x)
```nyash
// 二分木大量生成・破棄でGC性能測定
box TreeNode {
    init { left, right, value }
    birth(depth, value) { ... }
}
```

#### 2. n-body - 数値計算の王道
**目的**: 浮動小数点演算、ループ最適化効果測定
**期待性能**: Interpreter(1x) → VM(10x) → LLVM(50x)
```nyash
// 太陽系シミュレーション、重力計算
// MathBoxを活用した数値計算ベンチマーク
```

#### 3. mandelbrot - 計算+画像出力
**目的**: 純粋計算性能、ファイル出力確認
**期待性能**: Interpreter(1x) → VM(15x) → LLVM(80x)
```nyash
// フラクタル計算、PPM/PNGファイル出力
// 視覚的にJIT/LLVM効果を確認可能
```

### 🌟 Nyash特色ベンチマーク

#### 4. JSON Stream Aggregator
**目的**: プラグイン統一性、「Everything is Box」実証
**特徴**: File/Netプラグインから同じコードで処理
```nyash
// FileBoxとNetBoxから同じAPIでJSONを読み取り
// 同一コードでローカルファイルとHTTP APIに対応
```

## 📊 性能指標（現在の実測値）

| アプリ | Interpreter | VM | LLVM(予定) | 用途 |
|--------|-------------|----|-----------| -----|
| ny-echo | 1.0x | 13.5x | 50x | I/O性能 |
| ny-array-bench | 1.0x | 13.5x | 40x | 計算性能 |
| chip8_emulator | 1.0x | 13.5x | 60x | ゲーム性能 |
| enhanced_kilo_editor | 1.0x | 13.5x | 45x | エディタ性能 |
| tinyproxy | 1.0x | 13.5x | 35x | ネットワーク性能 |

## 🎯 実装ロードマップ

### ✅ 完了済み
- [x] ny-echo（基本I/O検証）
- [x] ny-array-bench（性能基準）
- [x] chip8_emulator（ゲーム・グラフィック）
- [x] enhanced_kilo_editor（実用ツール）
- [x] tinyproxy（ネットワーク）
- [x] BoxTorrent Mini（内容アドレスBox + 参照カウント連携）
- [x] binary-trees（GC性能測定）
- [x] mimalloc-lite（allocator-shaped model）
- [x] hako-alloc-production-facade-proof（M46 production allocator facade boundary）
- [x] hako-alloc-local-page-policy-proof（M47 local page policy proof）
- [x] hako-alloc-remote-free-policy-proof（M48 remote-free policy proof）
- [x] hako-alloc-page-source-policy-proof（M49 OSVM page-source proof）
- [x] hako-alloc-production-facade-stress（M50 production facade stress parity）
- [x] mimalloc-raw-page-proof（M12 substrate proof）
- [x] mimalloc-size-class-table-proof（M21 static size-class table proof）
- [x] mimalloc-two-class-page-proof（M22 two-class page proof）
- [x] mimalloc-dynamic-bin-proof（M23 dynamic bin proof）
- [x] mimalloc-size-to-bin-inline-proof（M24 size_to_bin inline proof）
- [x] mimalloc-size-class-policy-proof（M163 size-class policy owner proof）
- [x] mimalloc-alloc-fast-path-proof（M167 alloc fast path proof）
- [x] mimalloc-osvm-page-source-composition-proof（M168 OSVM page-source composition proof）
- [x] mimalloc-local-free-retire-proof（M169 local-free retire proof）
- [x] mimalloc-remote-free-page-integration-proof（M170 remote-free page integration proof）
- [x] mimalloc-page-map-proof（M171 pointer-to-page ownership map proof）
- [x] mimalloc-osvm-page-proof（M25 OSVM page proof）
- [x] mimalloc-ptr-atomic-store-proof（M35 native pointer atomic store proof）
- [x] mimalloc-ptr-atomic-load-proof（M39 native pointer atomic load proof）
- [x] mimalloc-ptr-atomic-cas-proof（M40 native pointer atomic CAS proof）
- [x] mimalloc-ptr-remote-free-list-proof（M41 pointer CAS remote-free list proof）
- [x] mimalloc-remote-free-list-policy-proof（M42 allocator remote-free list policy integration proof）
- [x] mimalloc-remote-free-retry-loop-proof（M43 allocator remote-free retry-loop proof）
- [x] mimalloc-tls-ptr-remote-free-proof（M36 TLS pointer remote-free proof）
- [x] mimalloc-remote-free-policy-proof（M37 remote-free policy integration proof）

### 🚧 実装予定（論文・ベンチマーク用）
- [ ] n-body（数値計算）
- [ ] mandelbrot（視覚的ベンチマーク）
- [ ] JSON Stream Aggregator（プラグイン統一）

### 🔮 将来候補
- [ ] レイトレーサー（CPU集約的）
- [ ] Lispインタープリター（言語実装）
- [ ] 静的サイトジェネレータ（実用性）

## 🚀 Nyashメモリ管理の真価を示す革新的アプリケーション

### AI先生たちの提案（2025-08-31）

Gemini先生とChatGPT5先生から、Nyashの決定論的メモリ管理（スコープベース解放、finiシステム、weak自動nil化）がもたらす新しいプログラミングパラダイムについて革新的な提案を受けました。

### 🌟 最優先実装候補

#### 1. **分散ホットスワップ・パイプライン**
**概要**: NyaMesh上でセンサ→前処理→推論→配信の各段をプラグイン化し、無停止で更新可能なMLパイプライン

**Nyashならではの特徴**:
- 🔄 **無停止プラグイン更新**: finiシステムにより論理的に終了しても物理的に参照可能
- 🧹 **決定的メモリ管理**: スコープ解放と逆順カスケードで予測可能な解放
- ⚡ **性能維持**: p99レイテンシ悪化<5%、スループット維持

**他言語では困難な理由**:
- Rust/C++: 手動メモリ管理で複雑、ホットスワップ時にUAFリスク
- Python/Ruby: GILにより真の並行性が得られない

#### 2. **BoxTorrent - 内容アドレス化P2P配布基盤**
**概要**: 大容量データや中間生成物を「Box=DAGノード」として配布し、変換プラグインで処理

**Nyashならではの特徴**:
- 📦 **ゼロコピー共有**: Arc<Mutex>で安全にBoxを共有
- 🔍 **内容ハッシュ重複排除**: 同一内容のBoxを自動的に再利用
- 🗑️ **自然なキャッシュ管理**: 参照カウントで不要データを自動削除

#### 3. **Live Shared Heap - メッシュ越し共有ヒープ**
**概要**: 論理的に単一のShared HeapにBoxを配置し、P2Pネットワーク上で共有

**Nyashならではの特徴**:
- 🌐 **分散ロックの単純化**: 全Boxがスレッドセーフ前提
- 🔌 **プラグイン透過性**: ヒープ上の同一Boxをそのまま扱える
- 🔧 **ノード障害耐性**: 参照カウントで自然復旧

### 📊 実装による測定可能な優位性

| 指標 | 期待される優位性 |
|------|-----------------|
| **安全性** | UAF/データ競合/クラッシュ発生率 0% |
| **可用性** | ホットスワップ中断時間 0秒 |
| **効率性** | ゼロコピー率 90%以上 |
| **拡張性** | ピア数に対して線形スケール |
| **回復性** | ノード喪失下での自動復旧 |

### 🎯 実装ロードマップ（Nyash特化）

#### Phase 1: ミニマムPoC（1週間）
- [x] **BoxTorrentミニ版**: 内容アドレスBox + 参照カウント連携
- [ ] **測定基盤**: 参照グラフ可視化、メモリリーク監視

#### Phase 2: 分散デモ（2週間）
- [ ] **2段パイプライン**: センサ→処理のホットスワップ実証
- [ ] **性能計測**: p99レイテンシ、スループット監視

#### Phase 3: 論文向け完全版（3週間）
- [ ] **完全なMLパイプライン**: 4段以上の処理段
- [ ] **大規模ベンチマーク**: 100ノード規模での性能実証

### 💡 Nyashだからこそ可能な革新

**「他言語では危険だがNyashなら安全」な例**:
1. **ゼロコピー共有バッファの多段パイプ**: 大容量Box<ByteBuf>を複数プラグインが並列処理
2. **共有メモリマップファイルの安全クローズ**: 最後の参照が落ちた瞬間のみクローズ
3. **マルチプロデューサ・マルチコンシューマなリングバッファ**: 言語レベルでunsafe不要

これらの実装により、Nyashの「Everything is Box」哲学とArc<Mutex>統一アーキテクチャが、単なる理論ではなく実用的な価値を持つことを証明します。

### 🔮 将来候補
- [ ] レイトレーサー（CPU集約的）
- [ ] Lispインタープリター（言語実装）
- [ ] 静的サイトジェネレータ（実用性）

## 🤝 貢献方法

新しいアプリケーションのアイデアや改善提案は大歓迎です！

1. 新しいアプリディレクトリを作成
2. main.hakoとtest.shを実装
3. このREADMEに追加
4. PRを送信

すべてのアプリケーションは「Everything is Box」哲学に従い、プラグインシステムを活用することを推奨します。
