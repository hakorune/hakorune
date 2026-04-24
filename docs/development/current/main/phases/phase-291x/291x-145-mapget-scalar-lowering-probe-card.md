---
Status: Rejected
Date: 2026-04-24
Scope: Probe scalar MapGet warm ABI lowering from proven MIR metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-144-mapget-preheader-scalar-proof-card.md
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - crates/nyash_kernel/src/plugin/map_aliases.rs
---

# 291x-145 MapGet Scalar Lowering Probe Card

## Goal

Try the smallest evidence-backed lowering that consumes the H144 MapGet scalar
proof metadata.

This card closed as a rejected probe. The helper route removed
`nyash.runtime_data.get_hh` from the loop, but cycles regressed and IPC
dropped.

## Baseline Evidence

Command:

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_map_getset_has 1 3
bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_map_getset_has ny_main 1
```

Observed baseline:

```text
ny_aot_instr = 2424471098
ny_aot_cycles = 569811600
ny_aot_ms = 107
ny_aot_ipc = 4.25
```

Baseline `ny_main` loop still calls:

```text
nyash.runtime_data.get_hh
nyash.map.has_h
```

Top owners are still dominated by:

```text
<i64 as alloc::string::SpecToString>::spec_to_string
core::hash::BuildHasher::hash_one
malloc / fmt
```

## Design

Consume MIR metadata only when all fields prove the scalar route:

```text
route_id = generic_method.get
box_name = RuntimeDataBox
method = get
receiver_origin_box = MapBox
key_route = i64_const
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
publication_policy = no_publication
proof = map_set_scalar_i64_*_no_escape
core_method.op = MapGet
```

Lowering target:

```text
route_kind = map_load_scalar_i64
helper_symbol = nyash.map.scalar_load_hi
lowering_tier = warm_direct_abi
```

The helper returns RuntimeData-compatible scalar read behavior for proven scalar
values:

```text
IntegerBox value -> immediate i64
BoolBox value    -> immediate i64
missing key      -> 0
```

## Boundary

- Do not use `nyash.map.slot_load_hi` for this route.
- Do not route mixed RuntimeData `get` rows away from
  `nyash.runtime_data.get_hh`.
- Do not add method-name semantic classification in `.inc`.
- Do not add hot inline lowering in this card.
- Do not rewrite Rust storage.
- Do not add benchmark-name or source-name branches.

## Acceptance

- MIR JSON emits `route_kind=map_load_scalar_i64` only for scalar-proof rows.
- `.inc` emits `nyash.map.scalar_load_hi` only from valid MIR metadata.
- Malformed or non-scalar metadata fail-fast instead of silently falling back.
- `bench_kilo_leaf_map_getset_has` asm shows the loop no longer calls
  `nyash.runtime_data.get_hh`.
- Keeper requires cycles improvement without IPC collapse.
- If cycles regress or top owner remains unchanged, close as rejected probe.

## Probe Result

Temporary implementation tested:

```text
route_kind = map_load_scalar_i64
helper_symbol = nyash.map.scalar_load_hi
lowering_tier = warm_direct_abi
```

Observed probe:

```text
ny_aot_instr = 2316470774
ny_aot_cycles = 580288491
ny_aot_ms = 112
ny_aot_ipc = 3.99
```

Asm confirmed the loop changed from:

```text
call nyash.runtime_data.get_hh
```

to:

```text
call nyash.map.scalar_load_hi
```

Reject reason:

```text
instructions improved: 2424471098 -> 2316470774
cycles regressed:     569811600  -> 580288491
IPC regressed:        4.25       -> 3.99
```

The top owners remained the same family:

```text
<i64 as alloc::string::SpecToString>::spec_to_string
core::hash::BuildHasher::hash_one
```

So this helper substitution is not a keeper. It removes one facade call, but
does not remove the real owner: i64 key string conversion plus hash lookup.

## Closeout

- Reverted the probe implementation.
- Kept H144 metadata-only scalar proof as the mainline state.
- Do not retry `nyash.map.scalar_load_hi` without a new owner seam.
- Next work should target an owner-family seam such as native i64-key map lane
  or get/has lookup fusion, not another RuntimeData-to-helper substitution.

## Revert Condition

Revert or reject this card if:

- instructions decrease but cycles increase
- IPC collapses
- semantic tests fail for RuntimeData mixed return behavior
- `.inc` begins rediscovering semantics from method names
- the helper route is not selected from MIR metadata
