# 293x-390 MIMAP-SUBSTRATE-CONC-002 Route Inventory Guard

Status: landed
Date: 2026-05-15

## Decision

Before adding new allocator concurrency substrate behavior, inventory and guard
the existing narrow route facts for:

```text
hako.atomic
hako.tls
hako.osvm
hako.mem
```

This row should prove that backend lowering uses MIR-owned route facts through
`extern_call_routes` / `lowering_plan`, not raw helper-name rediscovery.

## Scope

- Inventory existing route rows and proof apps for atomic/TLS/OSVM/hako.mem.
- Add or update a focused guard only if the current route inventory is not
  already protected by an indexed check.
- Do not add new substrate behavior.
- Do not widen language-level concurrency.

## Route Inventory

The current allocator substrate route inventory is already covered by existing
route rows and proof guards. This card adds a thin inventory guard so future
readers can verify the whole route set from one current entry.

| Family | Live route symbols | Runtime-decl-only / notes | Primary proof guards |
| --- | --- | --- | --- |
| `hako.mem` | `hako_mem_alloc`, `hako_mem_free` | `hako_mem_realloc` is runtime-decl native leaf only for this inventory; do not treat runtime-decl presence as an `extern_call_routes` row. | `tools/checks/k2_wide_hako_mem_extern_pure_first_guard.sh`, `tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh` |
| `hako.osvm` | `hako_osvm_reserve_bytes_i64`, `hako_osvm_commit_bytes_i64`, `hako_osvm_decommit_bytes_i64` | `hako_osvm_page_size_i64` is runtime-decl/page-size leaf; this card does not open unreserve/release rows. | `tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh` |
| `hako.tls` | `hako_tls_cache_slot_get_i64`, `hako_tls_cache_slot_set_i64` | allocator cache-slot leaves only; no generic TLS cells and no source-level `worker_local`. | `tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh` |
| `hako.atomic` fixed-slot | `hako_atomic_slot_cas_i64`, `hako_atomic_slot_load_i64`, `hako_atomic_slot_store_i64`, `hako_atomic_slot_fetch_add_i64` | default-order narrow fixed-slot rows only; ordered fixed-slot rows remain inactive. | `tools/checks/k2_wide_mimalloc_atomic_cas_exe_guard.sh`, `tools/checks/k2_wide_mimalloc_atomic_load_exe_guard.sh`, `tools/checks/k2_wide_mimalloc_atomic_store_exe_guard.sh`, `tools/checks/k2_wide_mimalloc_atomic_fetch_add_exe_guard.sh` |
| `hako.atomic` native-pointer | `hako_atomic_ptr_store_ordered`, `hako_atomic_ptr_load_ordered`, `hako_atomic_ptr_cas_ordered` | direct native-pointer extern routes only; pointer fetch_add remains inactive. | `tools/checks/k2_wide_mimalloc_ptr_atomic_store_exe_guard.sh`, `tools/checks/k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh`, `tools/checks/k2_wide_mimalloc_ptr_atomic_cas_exe_guard.sh` |

Closeout coverage:

```text
tools/checks/k2_wide_mimalloc_allocator_substrate_closeout_guard.sh
tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh
```

The inventory guard pins the route list above and checks that backend lowering
uses MIR-owned route facts through `extern_call_routes` / `lowering_plan`
coverage, not raw helper-name rediscovery.

No new extern route row is introduced by this card.

## Stop Lines

- No new extern route row unless an inventory gap is proven.
- No source-level `worker_local`.
- No generic TLS cells.
- No generic atomic surface beyond existing rows.
- No provider hook or host allocator replacement.

## Required Evidence

```text
git diff --check
bash tools/checks/k2_wide_mimalloc_substrate_route_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
