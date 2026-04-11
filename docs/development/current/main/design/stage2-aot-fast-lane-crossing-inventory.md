---
Status: SSOT
Decision: provisional
Date: 2026-03-25
Scope: `stage2` AOT/native fast-lane について、current hot/cold crossing を 3 bucket で棚卸しし、次の exact implementation bucket を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
  - docs/development/current/main/design/stage2-string-route-split-plan.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/perf-optimization-method-ssot.md
  - lang/src/runtime/collections/README.md
  - lang/src/hako_alloc/README.md
  - lang/src/runtime/host/host_facade_box.hako
---

# Stage2 AOT Fast-Lane Crossing Inventory

## Goal

- `stage2` の source layering を作り直さず、`AOT/native` で collapse すべき execution crossing だけを固定する。
- 次の implementation bucket を、owner 混線なしに 1 本ずつ切れる粒度まで落とす。
- hot lane / cold lane / metal keep を docs だけで判定できるようにする。

## Fixed Reading

- current repo の source layering は正しい前提で読む。
  - `hako_kernel`
  - `hako_substrate` + capability floor
  - `native metal keep`
- したがって、この inventory は「どこを `.hako` に移すか」ではなく、「どこを AOT hot path から外すか」を数える。
- backend lane は mainline / keep / replay の 3 lane で読む。
  - `ny-llvm` / `ny-llvmc` = daily/mainline AOT lane
  - `llvmlite` = stage0/compat/probe keep
  - `native` = explicit replay/canary lane
- runtime bridge ownership (`driver_spawn.rs`, `extern_provider/lane.rs`, `mir_call_v1_handler.hako`, `mir_vm_s0_call_exec.hako`) is tracked on a separate lane from smoke-retirement work.
- implementation bucket は `ny-llvm first` で切り、`llvmlite` は shared contract keep としてだけ追従確認する。
- bucket は次の 3 本に固定する。
  1. `collection op`
  2. `allocator/handle op`
  3. `dynamic fallback op`

## Bucket A: Collection Op

### Current chains

| Family | Current chain | Current mainline seam | Remaining layered crossing | Reading |
| --- | --- | --- | --- | --- |
| Array | `MirCallV1HandlerBox -> ArrayCoreBox -> RawArrayCoreBox -> PtrCoreBox -> nyash.array.slot_*` | `nyash.array.slot_load_hi`, `slot_store_hii`, `slot_len_h`, `slot_append_hh` | AOT route tables in `method_call.py` / `collection_method_call.py`, plus `hako_llvmc_ffi.c` dispatch | source owner is already correct; execution route still fans out |
| Map | `MirCallV1HandlerBox -> MapCoreBox -> RawMapCoreBox -> nyash.map.entry_count_i64 / slot_* / probe_*` | `nyash.map.entry_count_i64`, `slot_load_hh`, `slot_store_hhh`, `probe_hh` | AOT route tables in `method_call.py`, `runtime_data_dispatch.py`, `boxcall_runtime_data.py`, plus `hako_llvmc_ffi.c` dispatch | source owner is already correct; runtime-data mono-route is still layered |
| String | `MirCallV1HandlerBox -> StringCoreBox -> nyash.string.len_h` for observer; AOT routes `concat/substring/indexOf` through LLVM route tables into `exports/string.rs` / `plugin/string.rs` | `nyash.string.len_h`, `concat_hh`, `concat3_hhh`, `substring_hii`, `indexOf_hh`, `lastIndexOf_hh`, `concat_ss`, `substring_sii` | `binop.py`, `stringbox.py`, `string_console_method_call.py`, `hako_forward_bridge.rs`, `hako_llvmc_ffi.c` | owner split is mixed by design: observer in `.hako`, bulk/string leafs still backend-driven |

### Current exact truths

- Array owner frontier is [`lang/src/runtime/collections/array_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/collections/array_core_box.hako).
- Map owner frontier is [`lang/src/runtime/collections/map_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/collections/map_core_box.hako).
- String observer frontier is [`lang/src/runtime/collections/string_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/collections/string_core_box.hako).
- Raw substrate frontiers are:
  - [`lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_array/raw_array_core_box.hako)
  - [`lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_map/raw_map_core_box.hako)
- ABI inventory truth is [`docs/development/current/main/design/abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md), where `slot_*` / `probe_*` rows are already mainline.

### Remaining layered crossings

- landed Array slice:
  - explicit `ArrayBox.{len,get,set,push}` AOT route selection now lowers to `nyash.array.slot_len_h` / `slot_load_hi` / `set_hih|set_hii` / `slot_append_hh`
  - non-i64 array `get/set/has` stays on `nyash.runtime_data.*` facade as the current cold/compat contract
- landed Map slice:
  - explicit `MapBox.{size,len,length}` observer route now lowers to `nyash.map.entry_count_i64`
  - direct `MapBox.{get,set,has}` raw routes stay on `slot_load_hh` / `slot_store_hhh` / `probe_hh`
- [`lang/src/vm/boxes/mir_call_v1_handler.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/vm/boxes/mir_call_v1_handler.hako) still owns the generic VM adapter/router shape and state flags.
- [`src/llvm_py/instructions/mir_call/method_call.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/mir_call/method_call.py) still does layered route choice before reaching collection-specialized lowers.
- [`src/llvm_py/instructions/mir_call/collection_method_call.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/mir_call/collection_method_call.py) and [`src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/mir_call/runtime_data_dispatch.py) still mix specialization and fallback policy.
- [`src/llvm_py/instructions/binop.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/binop.py), [`src/llvm_py/instructions/stringbox.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/stringbox.py), and [`crates/nyash_kernel/src/exports/string.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string.rs) still combine route choice with string leaf execution.
- [`lang/c-abi/shims/hako_llvmc_ffi.c`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi.c) remains a generic ABI dispatch seam even when the target symbol is already monomorphic.

### Next exact implementation buckets

1. `String search/slice route split` (landed)
   - `substring/indexOf/lastIndexOf` AOT route tables are now split out of the generic method-call owner
   - boundary-default pure-first repair is landed, so `phase29ck_boundary/string` search/slice seeds no longer depend on a broken generic-symbol default
   - `llvmlite` keep lane remains shared-contract-only
2. `String concat route split` (landed)
   - `binop.py` string `+` concat path is now owned by a dedicated lowering helper instead of inline orchestration in `lower_binop(...)`
   - `nyash.string.concat_hh` / `concat3_hhh` exports now delegate route/fallback ownership into concat-specific helpers
   - `llvmlite` keep acceptance remains shared-contract-only
3. `cold dynamic lane split`
   - keep collection hot path away from `HostFacade/provider/plugin loader`
   - `ny-llvm` mainline acceptance:
     - hot collection/runtime paths do not enter loader/provider dispatch
   - `llvmlite` keep acceptance:
     - explicit compat/probe route still replays the cold lane when selected
4. `hako_alloc` policy/state contract
  - keep allocator metal in native keep and narrow the `.hako` policy rows
  - `ny-llvm` mainline acceptance:
    - policy/state rows are fixed without widening the metal boundary
   - `llvmlite` keep acceptance:
     - allocator/handle shared contract remains stable

## Bucket B: Allocator / Handle Op

### Current chain split

| Layer | Current owner | Current truth |
| --- | --- | --- |
| policy/state | `lang/src/hako_alloc/**` | policy-plane root only; current live modules are `memory.arc_box` and `memory.refcell_box` |
| capability substrate | `lang/src/runtime/substrate/{mem,buf,ptr,atomic,tls,gc}/**` | helper-shaped substrate rows and truthful seams |
| metal keep | C ABI shims + Rust runtime/kernel | actual alloc/free/realloc, handle registry, GC hooks, TLS/atomic body |

### Current exact truths

- [`lang/src/hako_alloc/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/hako_alloc/README.md) fixes `hako_alloc` as alloc/policy anchor, not allocator metal.
- [`src/runtime/host_handles.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/host_handles.rs) is the current handle registry body.
- [`src/runtime/gc.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/gc.rs) and [`src/runtime/gc_controller.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/gc_controller.rs) are current GC metal owners.
- [`hako-alloc-policy-state-contract-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md) now fixes the first concrete allocator policy/state rows.
- [`lang/c-abi/shims/hako_diag_mem_shared_impl.inc`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_diag_mem_shared_impl.inc) and [`lang/c-abi/shims/hako_kernel.c`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_kernel.c) hold actual mem/TLS/barrier C ABI rows.

### Remaining missing contracts

- Reserved-only future rows are still not live:
  - `RawBuf`
  - `Layout`
  - `MaybeInit`
  - size/bin/reclaim/locality policy
- `atomic/tls/gc` truthful first rows exist, but allocator/state migration stops before moving their metal body.

### Next exact implementation buckets

1. [`backend-private fast leaf manifest`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md)
   - docs + contract first is landed
   - internal metadata rows are fixed there
2. `hako_alloc policy/state contract`
   - landed stop-line:
     - handle reuse policy vs slot-table body
     - GC trigger threshold policy vs root-trace/metrics body
   - keep metal body native

## Bucket C: Dynamic Fallback Op

### Current chain split

| Lane | Current chain | Reading |
| --- | --- | --- |
| hot runtime lane | direct runtime/env/console routes | keep as direct as possible |
| cold loader/provider lane | `HostFacadeBox.call(\"loader\", ...) -> extern_provider/plugin loader/hostbridge` | explicit cold dynamic lane |
| plugin route resolution | `plugin_loader_unified` + `plugin_loader_v2` route resolver / metadata / host_bridge | manifest-first route, compat shim kept explicit and cold |

### Current exact truths

- [`lang/src/runtime/host/host_facade_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/host/host_facade_box.hako) is the `.hako` single entry for host calls.
- [`src/backend/mir_interpreter/handlers/extern_provider.rs`](/home/tomoaki/git/hakorune-selfhost/src/backend/mir_interpreter/handlers/extern_provider.rs) is the Rust generic extern dispatcher.
- [`src/runtime/plugin_loader_unified.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_unified.rs) and [`src/runtime/plugin_loader_v2/enabled/route_resolver.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_v2/enabled/route_resolver.rs) now treat compat fallback as an explicit cold-lane policy, not as the default manifest route.
- [`src/runtime/extern_registry.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/extern_registry.rs) already behaves like a manifest-backed registry for extern rows.
- [`src/runner/modes/llvm/method_id_injector.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/llvm/method_id_injector.rs) shows that AOT/backend lowering already has a method-id injection seam.

### Remaining generic crossings

- any `HostFacadeBox.call("loader", ...)`
- `hostbridge.extern_invoke`
- generic `box.call`
- plugin loader compat fallback behind the explicit `vm_compat_fallback_allowed` cold lane
- generic extern/provider dispatch from interpreter/runtime globals

### Next exact implementation buckets

1. `cold dynamic lane split` (landed)
   - runtime direct env/console routes are now classified separately from loader/provider cold routes
   - `HostFacade(loader)` is fixed as an explicit cold dynamic lane, not a generic host hot path
2. `plugin route-manifest hardening` (landed)
   - metadata / resolver / host_bridge contract is manifest-first
   - compat fallback is explicit and off the mainline route
3. `hako_alloc policy/state contract`
   - landed stop-line for allocator series
   - do not reopen it in the plugin wave unless a fresh blocker appears
4. `FastLeafManifest widen judgment` (landed)
   - result: keep V0 narrow
   - there is no active widen code wave until a concrete consumer patch appears

## Fixed Execution Order

1. this crossing inventory
2. backend-private fast leaf manifest contract
3. `Array hot path collapse` (landed)
4. `Map hot path collapse` (landed)
5. `String search/slice route split` (landed)
6. `String concat route split` (landed)
7. `cold dynamic lane split` (landed)
8. `hako_alloc` policy/state contract (landed stop-line)
9. `plugin route-manifest hardening` (landed)
10. `FastLeafManifest widen judgment` (landed / no widen now)

## Lane Rule

- `ny-llvm` is the only hot-path judge for stage2 implementation slices.
- `llvmlite` stays maintained, but only as:
  - stage0/bootstrap preservation lane
  - explicit compat lane
  - probe/canary lane
- do not widen a code slice just to preserve `llvmlite` execution layering; if shared MIR / ABI / observer contract survives, the keep lane is considered intact.

## Current Stop Line

- there is no active stage2 code wave after the widen judgment
- `phase-29ck` `P7` runway is now closed through `W4`
- `phase-29ck` `P8` reopen judgment is landed with `no reopen now`
- the next `ny-llvm` code-facing front is `P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md`, not a new stage2 fast-lane slice and not immediate `kilo` retune
- reopen only when a concrete `ny-llvm` / `ny-llvmc` consumer patch needs new rows
- `RawBuf / Layout / MaybeInit` stay reserved-only after the allocator stop-line

## Non-Goals

- relayering `.hako` semantic owners back into Rust
- moving allocator/GC/TLS/atomic metal into `.hako`
- creating a third public ABI
- mixing collection, dynamic route, and allocator buckets in the same implementation slice
