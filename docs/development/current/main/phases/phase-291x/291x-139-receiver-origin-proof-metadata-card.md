---
Status: Landed
Date: 2026-04-24
Scope: Expose receiver-origin proof for generic-method routes without changing lowering behavior.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-138-hcm7-maphas-preflight-evidence-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-139 Receiver-Origin Proof Metadata Card

## Goal

Expose conservative receiver-origin proof in `generic_method_routes` so later
CoreMethod promotion can distinguish:

```text
callee box = RuntimeDataBox
receiver origin = MapBox
```

from an unknown RuntimeData facade route.

## Implementation

- Added `receiver_origin_box` to `GenericMethodRoute`.
- The route planner resolves copy-chain roots with the MIR value-origin SSOT.
- If the receiver root is `new MapBox`, metadata emits:

```text
receiver_origin_box = MapBox
```

- The JSON emitter includes `receiver_origin_box`.
- The `.inc` `has` consumer now rejects future CoreMethod metadata unless
  `receiver_origin_box=MapBox` is present.

## Boundary

- No helper route is promoted in this card.
- `core_method` remains `null` for the measured RuntimeData facade route.
- `route_kind` remains `runtime_data_contains_any`.
- `helper_symbol` remains `nyash.runtime_data.has_hh`.

Observed metadata for `bench_kilo_leaf_map_getset_has.hako`:

```text
box_name = RuntimeDataBox
receiver_origin_box = MapBox
core_method = null
route_kind = runtime_data_contains_any
helper_symbol = nyash.runtime_data.has_hh
```

## Rejected Promotion

Promoting this route directly to `MapHas` / `nyash.map.has_hh` was tested and
rejected as a non-keeper:

```text
before: ny_aot_instr=2590470563 ny_aot_cycles=578941277
after:  ny_aot_instr=2844470432 ny_aot_cycles=679856467
```

The post-promotion top owners were still key conversion and hashing:

```text
core::hash::BuildHasher::hash_one
nyash_kernel::plugin::map_key_codec::map_key_string_from_any
<i64 as alloc::string::SpecToString>::spec_to_string
```

## Next

- Keep receiver-origin proof as metadata.
- Do not promote to CoreMethodOp until the key-route / value-demand proof can
  avoid the measured i64 key conversion work explosion.
- Re-run HCM-7 perf/asm after the next route proof is visible.
