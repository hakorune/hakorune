---
Status: Active
Decision: provisional
Date: 2026-03-20
Scope: `kernel-mainline`（`.hako` kernel）authority migration の fixed order と collection owner growth rule を 1 枚で固定する（中途半端な境界いじりを止める）。
Related:
  - CURRENT_TASK.md
  - lang/src/runtime/kernel/README.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/build-lane-separation-ssot.md
  - docs/development/current/main/design/rep-mir-string-lowering-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
---

# Phase 29cm: Kernel Authority Migration (kernel-mainline)

## Goal

- kernel の「意味/contract/policy/control structure」の owner を `.hako` / docs 側へ寄せる。
- Rust/C は substrate（allocation / handle registry / GC / ABI / raw leaf）に固定し、meaning owner に戻さない。
- `0rust` は Rust meaning owner zero を意味するが、Rust ベースの build/bootstrap route を消すことではない。
- “境界だけいじって進まない” 状態を防ぐため、collection owner cutover の fixed order を SSOT 化する。

## Non-Goals

- Rust substrate の wholesale delete はしない（authority migration と混ぜない）。
- Rust ベースの buildability を壊す slice は、この phase の mainline に入れない。
- raw substrate の perf/asm optimization を主線にしない（collection owner cutover 完了後の follow-up）。
- silent fallback を許可しない（`NYASH_VM_USE_FALLBACK=0` を前提）。
- `runtime_data` を collection owner に育てない（protocol / facade に固定）。

## Fixed Order (Migration)

1. `string`
   - landed: `lang.runtime.kernel.string.search` (`find_index/contains/starts_with/ends_with/split_once_index`)
   - rule: further widening is paused until a new exact blocker appears; if none appears, stop the lane and move to inventory or the next fixed order

2. `array`
   - first active owner cutover
   - target: `.hako` ring1 collection core owns `ArrayBox.{get,set,push,len/length/size}`, bounds policy, index normalization, visible fallback/error contract
   - Rust side shrinks to raw storage/cache/load/store/downcast/layout substrate only

3. `map`
   - second active owner cutover
   - target: `.hako` ring1 collection core owns `MapBox.{get,set,has,len/length/size}`, key normalization, visible fallback/error contract
   - Rust side shrinks to raw hash/probe/rehash/layout substrate only

4. `runtime_data cleanup`
   - keep `RuntimeDataBox` as protocol / facade only
   - it may route to array/map owners, but must not become a collection-semantics god-box

5. `numeric`
   - landed: `MatI64.mul_naive` loop/body owner split (`lang/src/runtime/kernel/numeric/`)
   - rule: collection owner cutover が落ち着くまで parked; new narrow ops only reopen on a new exact blocker

## Migration Checklist

- [x] `string` lane reached the stop line
  - further widening is paused until a new exact blocker appears
- [ ] `array` owner cutover is active
  - first slice landed: `ArrayCoreBox` owns ArrayBox push/get/set/size fallback routing and `mir_call_v1_handler.hako` no longer carries Array-specific size/push branches
  - second slice landed: Rust `array` helper ownership is split into raw `slot_load` / `slot_store` modules while legacy method-shaped helper names remain thin wrappers
  - third slice landed: `ArrayCoreBox.get_i64/set_i64` retarget to raw `slot_load/slot_store` exports while legacy `get_hi/set_hii` stay compat-only
- [ ] `map` owner cutover follows `array`
  - first slice landed: `MapCoreBox` is now the single visible owner frontier for handler-side `MapBox.{set,get,has,size/len/length}` routing and `mir_call_v1_handler.hako` no longer carries inline MapBox set fallback logic
  - current adjacent blocker is lane C / `.hako VM`: `RVP-C17 MapBox.set(key,value)`, `RVP-C18 MapBox.size()`, `RVP-C19 MapBox.get(key)`, `RVP-C20 MapBox.has(key)`, `RVP-C21 MapBox.delete(key)`, `RVP-C22 MapBox.keys()`, `RVP-C23 MapBox.clear()`, `RVP-C24 MapBox.get(missing-key)`, `RVP-C25 MapBox.get(non-string key)`, `RVP-C26 MapBox.set(non-string key, value)`, and `RVP-C27 MapBox.getField(non-string key)` are ported, and the next hard blocker is `RVP-C28 MapBox.setField(non-string key, value)` stale unimplemented route
- [ ] `runtime_data` cleanup keeps protocol/facade-only shape
- [x] `numeric` inventory was rechecked and remains parked as a narrow pilot

## Latest Inventory (2026-03-20)

- `array`
  - `lang/src/runtime/collections/array_core_box.hako` / `array_state_core_box.hako` / `crates/nyash_kernel/src/plugin/array*.rs` are already split at the natural seams.
  - `array_core_box.hako` already owns method-shaped aliases/orchestration; this is now treated as the natural owner-growth frontier, not a reason to keep deferring.
  - `array_state_core_box.hako` remains ring1 state bookkeeping; it should support the `.hako` owner, not replace it with Rust-side semantics.
  - Rust `array` plugin/helpers currently still own method-shaped leaves on the hot path; this is the reason to cut over now rather than continue micro-optimizing them in place.
- `map`
  - `map_core_box.hako` already owns key normalization plus method-shaped aliases/orchestration.
  - first owner-lock slice is to keep `mir_call_v1_handler.hako` orchestration-only and route MapBox visible semantics through `MapCoreBox.try_handle(...)`.
  - `MapBox` follows `array`; do not treat it as forever-defer just because it lives under `collections/`.
  - Rust `map` plugin/helpers should end at raw hash-table substrate, not method-shaped semantics.
- `runtime_data`
  - `runtime_data_core_box.hako` is narrow dispatch today and should stay that way.
  - do not move array/map semantics into `RuntimeDataBox`; use it as protocol / facade / routing only.
- `numeric`
  - `lang/src/runtime/kernel/numeric/matrix_i64.hako` plus `lang/src/runtime/numeric/{mat_i64_box.hako,intarray_core_box.hako}` are already thin enough.
  - `MatI64.mul_naive` remains the only credible kernel pilot; no second narrow op is justified by the current inventory.
  - stop here until a new exact blocker appears after collection owner cutover settles.

## Current Reading

- kernel lane is active again as collection owner cutover.
- the current order is `array -> map -> runtime_data cleanup`; `string` is parked at stop line and `numeric` is parked as a narrow pilot.
- raw substrate micro-opt is not the current lane; park it until method-shaped collection verbs leave Rust ownership.

## Buildability Lock

- any migration slice:
  - Rust からの build/bootstrap route は常に再実行可能であること
  - owner cutover と buildability cutover を同じ slice で壊さないこと
  - `.hako` へ寄せる順番と Rust buildability の保持順番を混ぜないこと

## Owner Growth Rule (array/map)

`array` / `map` の cutover は calendar-based ではなく、owner-shape based で判断する。

Move now と読む理由:
- method-shaped user-visible verbs (`get/set/push/len/has/normalize`) がすでに `.hako` ring1 frontier と Rust plugin の両方に跨っている
- current hot path still lands on Rust method-shaped helpers, which means meaning owner is not cut over yet
- adapter fixtures / provider smokes already express the contract at the `.hako` boundary, so the next clean step is owner growth, not more defer

Keep in Rust:
- allocation / handle cache / downcast / encoded value codec
- slot load/store / growth / probe / rehash / object layout / GC barrier

Move to `.hako`:
- bounds / normalization / visible fallback
- method aliases (`len/length/size`)
- user-visible get/set/push/has contract
- smoke-facing semantics and error contract

## Acceptance (minimum)

- gate:
  - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`
  - `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- quick available now:
  - `bash tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh`
- quick available now:
  - `bash tools/smokes/v2/profiles/quick/core/map/map_basic_get_set_vm.sh`
  - `bash tools/smokes/v2/profiles/quick/core/map/map_len_size_vm.sh`
- integration:
  - `bash tools/smokes/v2/profiles/integration/apps/ring1_array_provider_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/ring1_map_provider_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
- parked pilots:
  - string: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_string_kernel_search_min.sh`
  - numeric: `bash tools/smokes/v2/profiles/integration/apps/phase29ck_numeric_mat_i64_mul_naive_min.sh`

## First Implementation Order

1. `A1: Array semantics lock`
   - make `array_core_box.hako` / `array_state_core_box.hako` the visible owner for `ArrayBox.{get,set,push,len/length/size}`
   - bounds / normalization / visible fallback contract move here first
2. `A2: Array raw substrate contract`
   - `collection-raw-substrate-contract-ssot.md` is the naming/ownership SSOT
   - Rust array helpers must shrink to raw verbs only
3. `A3: Array retarget`
   - `.hako` array owner calls raw substrate verbs
   - method-shaped Rust ownership leaves the daily path
4. `M1: Map repeat`
   - `MapBox` follows the same owner/substrate split
5. `R1: RuntimeData cleanup`
   - `RuntimeDataBox` remains protocol / facade only
6. `P1: Raw substrate perf reopen`
   - only after `array` and `map` method ownership is no longer Rust-owned

## Done Shape (phase closeout)

- `CURRENT_TASK.md` の fixed order が破れず、次の 1 手が常に 1 commit 単位で切れる
- `string`/`numeric` の landed pilots が smoke で固定されている
- `array` の user-visible semantics owner が `.hako` ring1 collection core にある
- `map` の user-visible semantics owner が `.hako` ring1 collection core にある
- `runtime_data` は protocol / facade だけを持ち、array/map semantics owner ではない
- Rust collection code is reduced to raw substrate verbs and no longer owns method-shaped collection semantics

## Backend-Zero Handoff

- この phase は collection owner cutover stop line までを owner にする。
- backend-zero の active order と buildability gate は `docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md` を正本に渡す。
- raw substrate perf wave は、この phase が done shape に届いてから reopen する。
