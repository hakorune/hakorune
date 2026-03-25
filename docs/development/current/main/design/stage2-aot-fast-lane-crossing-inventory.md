---
Status: SSOT
Decision: provisional
Date: 2026-03-25
Scope: `stage2` AOT/native fast-lane について、current hot/cold crossing を 3 bucket で棚卸しし、次の exact implementation bucket を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/stage2-aot-native-thin-path-design-note.md
  - docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md
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
  - `.hako semantic owner`
  - `.hako algorithm/capability substrate`
  - `native metal keep`
- したがって、この inventory は「どこを `.hako` に移すか」ではなく、「どこを AOT hot path から外すか」を数える。
- bucket は次の 3 本に固定する。
  1. `collection op`
  2. `allocator/handle op`
  3. `dynamic fallback op`

## Bucket A: Collection Op

### Current chains

| Family | Current chain | Current mainline seam | Remaining layered crossing | Reading |
| --- | --- | --- | --- | --- |
| Array | `MirCallV1HandlerBox -> ArrayCoreBox -> RawArrayCoreBox -> PtrCoreBox -> nyash.array.slot_*` | `nyash.array.slot_load_hi`, `slot_store_hii`, `slot_len_h`, `slot_append_hh` | AOT route tables in `method_call.py` / `collection_method_call.py`, plus `hako_llvmc_ffi.c` dispatch | source owner is already correct; execution route still fans out |
| Map | `MirCallV1HandlerBox -> MapCoreBox -> RawMapCoreBox -> nyash.map.entry_count_h / slot_* / probe_*` | `nyash.map.entry_count_h`, `slot_load_hh`, `slot_store_hhh`, `probe_hh` | AOT route tables in `method_call.py`, `runtime_data_dispatch.py`, `boxcall_runtime_data.py`, plus `hako_llvmc_ffi.c` dispatch | source owner is already correct; runtime-data mono-route is still layered |
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
- [`lang/src/vm/boxes/mir_call_v1_handler.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/vm/boxes/mir_call_v1_handler.hako) still owns the generic VM adapter/router shape and state flags.
- [`src/llvm_py/instructions/mir_call/method_call.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/mir_call/method_call.py) still does layered route choice before reaching collection-specialized lowers.
- [`src/llvm_py/instructions/mir_call/collection_method_call.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/mir_call/collection_method_call.py) and [`src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/mir_call/runtime_data_dispatch.py) still mix specialization and fallback policy.
- [`src/llvm_py/instructions/binop.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/binop.py), [`src/llvm_py/instructions/stringbox.py`](/home/tomoaki/git/hakorune-selfhost/src/llvm_py/instructions/stringbox.py), and [`crates/nyash_kernel/src/exports/string.rs`](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/exports/string.rs) still combine route choice with string leaf execution.
- [`lang/c-abi/shims/hako_llvmc_ffi.c`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi.c) remains a generic ABI dispatch seam even when the target symbol is already monomorphic.

### Next exact implementation buckets

1. `Map hot path collapse`
   - same pattern as Array
2. `String route split`
   - keep `StringCoreBox` observer role
   - thin only the AOT string route tables and fallback bridge
3. `cold dynamic lane split`
   - keep collection hot path away from `HostFacade/provider/plugin loader`

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
- [`lang/c-abi/shims/hako_diag_mem_shared_impl.inc`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_diag_mem_shared_impl.inc) and [`lang/c-abi/shims/hako_kernel.c`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_kernel.c) hold actual mem/TLS/barrier C ABI rows.

### Remaining missing contracts

- There is no single allocator-state manifest/SSOT that ties:
  - `RawBuf`
  - `Layout`
  - `MaybeInit`
  - size/bin/reclaim/locality policy
  into one planning surface.
- `atomic/tls/gc` truthful first rows exist, but not as a unified fast-lane contract.
- current value/ABI manifest does not yet carry fast-lane metadata such as `may_alloc` / `may_barrier`.

### Next exact implementation buckets

1. [`backend-private fast leaf manifest`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/stage2-fast-leaf-manifest-ssot.md)
   - docs + contract first is landed
   - internal metadata rows are fixed there
2. `hako_alloc policy/state contract`
   - fix first-class policy/state rows
   - keep metal body native

## Bucket C: Dynamic Fallback Op

### Current chain split

| Lane | Current chain | Reading |
| --- | --- | --- |
| hot runtime lane | direct runtime/env/console routes | keep as direct as possible |
| cold loader/provider lane | `HostFacadeBox.call(\"loader\", ...) -> extern_provider/plugin loader/hostbridge` | explicit cold dynamic lane |
| plugin route resolution | `plugin_loader_unified` + `plugin_loader_v2` route resolver / metadata / host_bridge | still contains compat fallbacks |

### Current exact truths

- [`lang/src/runtime/host/host_facade_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/host/host_facade_box.hako) is the `.hako` single entry for host calls.
- [`src/backend/mir_interpreter/handlers/extern_provider.rs`](/home/tomoaki/git/hakorune-selfhost/src/backend/mir_interpreter/handlers/extern_provider.rs) is the Rust generic extern dispatcher.
- [`src/runtime/plugin_loader_unified.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_unified.rs) and [`src/runtime/plugin_loader_v2/enabled/route_resolver.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_v2/enabled/route_resolver.rs) still own compat-heavy route resolution.
- [`src/runtime/extern_registry.rs`](/home/tomoaki/git/hakorune-selfhost/src/runtime/extern_registry.rs) already behaves like a manifest-backed registry for extern rows.
- [`src/runner/modes/llvm/method_id_injector.rs`](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/llvm/method_id_injector.rs) shows that AOT/backend lowering already has a method-id injection seam.

### Remaining generic crossings

- any `HostFacadeBox.call("loader", ...)`
- `hostbridge.extern_invoke`
- generic `box.call`
- plugin loader compat fallback in `route_resolver` / `host_bridge`
- generic extern/provider dispatch from interpreter/runtime globals

### Next exact implementation buckets

1. `cold dynamic lane split`
   - keep runtime lane direct
   - fence loader/provider/plugin resolution into an explicit cold lane
2. `plugin route-manifest hardening`
   - unify metadata / resolver / host_bridge contract
   - keep compat fallback off the hot lane

## Fixed Execution Order

1. this crossing inventory
2. backend-private fast leaf manifest contract
3. `Array hot path collapse` (landed)
4. `Map hot path collapse`
5. `String route split`
6. `cold dynamic lane split`
7. `hako_alloc` policy/state contract

## Non-Goals

- relayering `.hako` semantic owners back into Rust
- moving allocator/GC/TLS/atomic metal into `.hako`
- creating a third public ABI
- mixing collection, dynamic route, and allocator buckets in the same implementation slice
