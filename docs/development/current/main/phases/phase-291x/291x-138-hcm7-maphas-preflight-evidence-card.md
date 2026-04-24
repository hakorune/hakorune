---
Status: Landed
Date: 2026-04-24
Scope: HCM-7 owner-first perf/asm preflight for MapBox get/has hot lowering.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-137-lowering-tier-metadata-card.md
  - benchmarks/bench_kilo_leaf_map_getset_has.hako
---

# 291x-138 HCM-7 MapHas Preflight Evidence Card

## Goal

Run owner-first evidence before adding any `hot_inline` lowering for
CoreMethodOp `MapHas`.

## Evidence

### `method_call_only_small`

```text
c_instr=126891
c_cycles=217110
ny_aot_instr=487292
ny_aot_cycles=747792
ratio_instr=0.26
ratio_cycles=0.29
aot_status=ok
```

The generated `ny_main` for this front is already a simple scalar loop:

```text
call nyash.box.from_i8_string_const
loop:
  add $5
  inc
```

This is not a useful `MapHas` hot-inline target.

### `kilo_leaf_map_getset_has`

```text
c_instr=10122877
c_cycles=2192987
ny_aot_instr=2590470563
ny_aot_cycles=578941277
c_ipc=4.62
ny_aot_ipc=4.47
aot_status=ok
```

Top owners from `bench_micro_aot_asm.sh kilo_leaf_map_getset_has ny_main 10`:

```text
47.86% <i64 as alloc::string::SpecToString>::spec_to_string
34.67% core::hash::BuildHasher::hash_one
 5.30% nyash_kernel::plugin::map_runtime_data::map_runtime_data_has_any_key
 1.99% nyash.runtime_data.get_hh
 1.47% nyash.runtime_data.has_hh
```

The `ny_main` hot loop calls RuntimeData facade helpers:

```text
call nyash.runtime_data.get_hh
call nyash.runtime_data.has_hh
```

The MIR metadata confirms the active route is not the migrated `MapHas`
CoreMethodOp carrier:

```text
route_id = generic_method.has
box_name = RuntimeDataBox
route_kind = runtime_data_contains_any
core_method = null
helper_symbol = nyash.runtime_data.has_hh
```

## Decision

Do not add `hot_inline` lowering for `CoreMethodOp.MapHas` yet.

The active owner for the measured front is:

```text
RuntimeDataBox facade route
  -> any-key map helper
  -> i64 key to_string
  -> hash_one
```

Adding a `MapHas` inline fast path now would target the wrong seam.

## Next

- Add a receiver-origin/CoreMethod route proof so the backend can see a
  manifest-backed `MapHas` carrier for the measured front when it is legal.
- Keep `RuntimeDataBox` compatibility fallback unchanged.
- Re-run HCM-7 perf/asm only after the route proof is visible in MIR metadata.
