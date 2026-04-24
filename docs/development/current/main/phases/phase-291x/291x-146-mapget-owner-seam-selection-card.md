---
Status: Landed
Date: 2026-04-24
Scope: Select the next owner seam after the scalar MapGet helper probe was rejected.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-145-mapget-scalar-lowering-probe-card.md
  - benchmarks/bench_kilo_leaf_map_getset_has.hako
  - src/boxes/map_box.rs
  - crates/nyash_kernel/src/plugin/map_slot_load.rs
  - crates/nyash_kernel/src/plugin/map_probe.rs
---

# 291x-146 MapGet Owner Seam Selection Card

## Goal

Pick the next MapGet optimization seam without repeating the rejected helper
substitution.

## Evidence

`291x-145` proved that replacing:

```text
nyash.runtime_data.get_hh
```

with:

```text
nyash.map.scalar_load_hi
```

is not a keeper.

The loop still pays the dominant owner family:

```text
i64 key -> String
HashMap<String, _> hash lookup
```

Current storage confirms the owner:

```text
MapBox.data = HashMap<String, Box<dyn NyashBox>>
map_slot_load_i64 -> key_i64.to_string() -> get_opt_key_str
map_probe_contains_i64 -> key_i64.to_string() -> contains_key_str
```

## Decision

Do not try another single-call MapGet helper substitution.

Next keeper attempt must first target duplicated lookup work:

```text
MapGet(i64_const key)
MapHas(i64_const same key)
```

Preferred next seam:

```text
H147: same-key MapGet/MapHas fusion metadata preflight
```

The first H147 slice should be metadata-only:

```text
fusion_op = MapLookupSameKey
receiver_origin_box = MapBox
key_route = i64_const
get_return_shape = scalar_i64_or_missing_zero
has_result = presence_bool
stored_value_proof = scalar_i64_const | scalar_i64_nonzero | unknown_scalar
lowering_tier = cold_fallback
```

No codegen changes in H147.

## Why Not Native I64-Key Lane First

A native i64-key map lane attacks the strongest owner directly, but it touches
storage semantics:

```text
MapBox.set(-1, v)
MapBox.get("-1")
MapBox.keys()
MapBox.delete("-1")
MapBox.clear()
```

To preserve string-key compatibility, a native lane would need either dual
publication or a carefully proven private lane. That is larger than the next
compiler fact and should not be the immediate card.

## Why Fusion First

The measured loop performs get and has for the same receiver/key in the same
iteration:

```text
local v = map.get(key)
if (map.has(key)) {
  sum = sum + v
}
```

A fusion preflight can stay compiler-owned and metadata-only first. It can
prove whether a later lowering may remove the second key conversion/hash
without changing MapBox storage.

## Boundary

- Do not add benchmark-name branches.
- Do not add native i64 storage in H147.
- Do not lower get/has fusion until metadata proves the exact pair.
- Do not use `slot_load_hi` for RuntimeData mixed return semantics.
- Do not add `.inc` method-name semantic classification.
- Keep H144 scalar MapGet proof as the input fact.

## Next

Implement H147:

```text
same-key MapGet/MapHas fusion metadata preflight
```

Acceptance for H147:

- MIR JSON marks only same receiver / same i64 const key get+has pairs.
- Metadata records get result value and has result value.
- Metadata records whether stored scalar value is known nonzero.
- Codegen remains unchanged.
- `bench_kilo_leaf_map_getset_has.hako` carries the fusion metadata.
