---
Status: Active
Decision: provisional
Date: 2026-03-20
Scope: `kernel-mainline`（`.hako` kernel）authority migration の fixed order と collection owner growth rule を 1 枚で固定する（中途半端な境界いじりを止める）。
Related:
  - CURRENT_TASK.md
  - lang/src/runtime/kernel/README.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
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

## Axis Lock

- `stage0/stage1/stage2+` と `owner/substrate` は別軸で読む。
- current matrix SSOT:
  - `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`
- this phase owns only the kernel owner axis.
- `done-enough stop line` here means phase-local owner progress, not end-state completion for stage2+ mainline.

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
- [x] `array` owner cutover reached the current stop line
  - first slice landed: `ArrayCoreBox` owns ArrayBox push/get/set/size fallback routing and `mir_call_v1_handler.hako` no longer carries Array-specific size/push branches
  - second slice landed: Rust `array` helper ownership is split into raw `slot_load` / `slot_store` modules while legacy method-shaped helper names remain thin wrappers
  - third slice landed: `ArrayCoreBox.get_i64/set_i64` retarget to raw `slot_load/slot_store` exports while legacy `get_hi/set_hii` stay compat-only
- [x] `map` owner cutover reached the current stop line
  - first slice landed: `MapCoreBox` is now the single visible owner frontier for handler-side `MapBox.{set,get,has,size/len/length}` routing and `mir_call_v1_handler.hako` no longer carries inline MapBox set fallback logic
  - second slice landed: Rust `map` helper ownership is split into raw `slot_load` / `slot_store` / `probe` modules while legacy `nyash.map.{get,set,has}_*` exports remain thin wrappers
  - third slice landed: `map_state_core_box.hako` now owns vm-hako-visible `MapBox.{set,get,has,getField,setField,delete,keys,clear}` stateful routing and `mir_vm_s0_boxcall_builtin.hako` only delegates
  - adjacent lane C blocker sweep is now fully ported through `RVP-C28`; no current `.hako VM` blocker remains for MapBox bad-key field routes
- [x] `runtime_data` cleanup keeps protocol/facade-only shape
  - first slice landed: `crates/nyash_kernel/src/plugin/runtime_data.rs` is now a dispatch shell over `runtime_data_array_route.rs` / `runtime_data_map_route.rs`
  - second slice landed: `runtime_data_core_box.hako` owns arg-decode/ABI-dispatch helpers and `mir_call_v1_handler.hako` now sees `RuntimeDataBox` as one delegated branch
  - `RuntimeDataBox` still does not own array/map semantics; it only routes to them
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
  - `map_state_core_box.hako` is now the vm-hako-visible stateful helper frontier for `set/get/has/getField/setField/delete/keys/clear`.
  - `MapBox` follows `array`; do not treat it as forever-defer just because it lives under `collections/`.
  - Rust `map` plugin/helpers should end at raw hash-table substrate, not method-shaped semantics.
- `runtime_data`
  - `runtime_data_core_box.hako` is narrow dispatch today and should stay that way.
  - first Rust-side cleanup landed: `runtime_data.rs` is now a dispatch shell over dedicated array/map route modules.
  - do not move array/map semantics into `RuntimeDataBox`; use it as protocol / facade / routing only.
- `numeric`
  - `lang/src/runtime/kernel/numeric/matrix_i64.hako` plus `lang/src/runtime/numeric/{mat_i64_box.hako,intarray_core_box.hako}` are already thin enough.
  - `MatI64.mul_naive` remains the only credible kernel pilot; no second narrow op is justified by the current inventory.
  - stop here until a new exact blocker appears after collection owner cutover settles.

## Current Reading

- collection owner shift is done-enough for this phase, not end-state complete.
- `.hako` ring1 owns visible collection semantics, while Rust still owns the raw substrate/plugin ABI path.
- `array -> map -> runtime_data cleanup` is parked unless a new exact collection blocker appears; `string` is parked at stop line and `numeric` is parked as a narrow pilot.
- raw substrate perf reopen (`P1`) is parked again until the boundary is deeper.
- do not read the green acceptance set as “kernel migration finished”; it only proves the current owner frontier is stable enough to move to `B1`.
- the last accepted `P1` keep is the `array` read-seam slice at `ny_aot_ms=43`.
- immediate rejected probes (reverted):
  - dedicated i64 write helper (`43 -> 47 ms`)
  - `ArrayBox::try_set_index_i64_integer()` cold-split (`43 -> 48 ms`)
- `B1a` landed: the daily `.hako` array observer path now uses `nyash.array.slot_len_h`, while `nyash.array.len_h` remains compat-only.
- `B1b` landed: the daily `.hako` array append path and arrayish runtime-data mono-route now use `nyash.array.slot_append_hh`, while `nyash.array.push_hh` remains compat-only.
- `B1c` landed: the daily `.hako` map observer path now uses `nyash.map.entry_count_h`, while `nyash.map.size_h` remains compat-only.
- `B1d1` landed: `nyash.array.slot_append_hh` now executes through `ArrayBox.slot_append_box_raw(...)`, and compat append routes no longer call the visible `push()` method below the raw name.
- `B1d2` landed: `nyash.array.slot_store_hii` and runtime-data array set now execute through `ArrayBox.slot_store_*_raw(...)`, while preserving the current append-at-end/rebox behavior.
- next exact boundary-deepen task is to demote the remaining transitional method-shaped Rust exports still used by `.hako` owners:
  1. hidden map residue under `nyash.map.slot_* / probe_*`
- after those explicit exports, deepen the hidden raw-named residue:
  - `nyash.map.slot_* / probe_*` still execute through `MapBox.get_opt/set/has`
- build-freshness note:
  - new kernel exports on the AOT boundary path require fresh release artifacts before link/pure smokes
  - stale pure-link failures must fail fast on missing staticlib symbols instead of relying on manual rebuild memory
- `RuntimeDataBox` remains facade-only while the boundary deepens, and it has no active code task now.
- `crates/nyash_kernel/src/plugin/array_index_helpers.rs` / `array_route_helpers.rs` are now thin wrappers and should not be treated as the primary boundary owner.

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
  - `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_array_provider_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/ring1_providers/ring1_map_provider_vm.sh`
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
   - first landed slice: `runtime_data.rs` delegates array/map behavior to dedicated Rust route helpers instead of owning inline collection logic
   - second landed slice: `runtime_data_core_box.hako` centralizes unary/binary arg decode + ABI dispatch helpers and `mir_call_v1_handler.hako` only delegates
6. `B1: Deeper collection boundary before perf`
   - `B1a`: landed; daily `.hako` array observer path now uses `nyash.array.slot_len_h`
   - `B1b`: landed; daily `.hako` array append path and arrayish runtime-data mono-route now use `nyash.array.slot_append_hh`
   - `B1c`: landed; daily `.hako` map observer path now uses `nyash.map.entry_count_h`
   - `B1d`: deepen hidden array write residue under `nyash.array.slot_append_hh` / `nyash.array.slot_store_hii`
   - `B1e`: deepen hidden map residue under `nyash.map.slot_* / probe_*`
   - `B1r`: keep `RuntimeDataBox` facade-only; docs/task lock only unless an exact protocol/dispatch bug appears
   - do not describe this phase as finished until these transitional exports are either removed from the daily path or explicitly accepted as the long-term substrate boundary
7. `P1: Raw substrate perf reopen`
   - only after the deeper collection boundary is fixed
   - the last accepted keep is the `array` read-seam slice (`ny_aot_ms=43`)
   - write/TLS probes stay parked until `B1` is done

## Done Shape (phase closeout)

- `CURRENT_TASK.md` の fixed order が破れず、次の 1 手が常に 1 commit 単位で切れる
- `string`/`numeric` の landed pilots が smoke で固定されている
- `array` の user-visible semantics owner frontier が `.hako` ring1 collection core にある
- `map` の user-visible semantics owner frontier が `.hako` ring1 collection core にある
- `runtime_data` は protocol / facade だけを持ち、array/map semantics owner ではない
- Rust collection code is reduced to raw substrate verbs, but raw substrate ownership still remains in Rust until the deeper boundary is complete

## Stop-Line Confirmation (2026-03-21)

- minimum acceptance is green:
  - `phase29cc_runtime_v0_adapter_fixtures_vm`
  - `phase29cc_runtime_v0_abi_slice_guard`
  - `array_length_vm`
  - `map_basic_get_set_vm`
  - `map_len_size_vm`
  - `ring1_array_provider_vm`
  - `ring1_map_provider_vm`
  - `phase29x_runtime_data_dispatch_llvm_e2e_vm`
- current reading:
  - `array`, `map`, and `runtime_data` are done-enough for this phase
  - this is not the same as end-state completion
  - reopen only on a new exact collection blocker or boundary-deepen slice
  - next fixed order is `B1: deeper collection boundary before perf`

## Backend-Zero Handoff

- この phase は collection owner cutover stop line までを owner にする。
- backend-zero の active order と buildability gate は `docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md` を正本に渡す。
- raw substrate perf wave は、この phase が done shape に届いてから reopen する。
