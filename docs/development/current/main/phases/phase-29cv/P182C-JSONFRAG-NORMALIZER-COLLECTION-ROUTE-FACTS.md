---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P182c, JsonFrag normalizer collection route facts
Related:
  - docs/development/current/main/phases/phase-29cv/P182A-GENERIC-STRING-CLASSIFIER-BOUNDARY.md
  - docs/development/current/main/phases/phase-29cv/P182B-JSONFRAG-INSTRUCTION-ARRAY-NORMALIZER-SHAPE.md
  - src/mir/generic_method_route_facts.rs
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_plan/tests/map_set_routes.rs
---

# P182c: JsonFrag Normalizer Collection Route Facts

## Problem

`JsonFragNormalizerBox._normalize_instructions_array/1` is now a known
`jsonfrag_instruction_array_normalizer_body` shape, but its internal collection
method calls were not fully represented in `generic_method_routes` /
`lowering_plan`.

The missing sites were `RuntimeDataBox.get/set` calls whose receivers flow
through typed PHI/copy values:

```text
MapBox  PHI -> RuntimeDataBox.get/set
ArrayBox PHI -> RuntimeDataBox.get
```

Without these facts, the C backend would have to rediscover collection receiver
semantics from raw MIR in the planned module emitter. That would violate the
P182a boundary.

## Decision

Keep the meaning in MIR route metadata:

- `receiver_origin_box_name()` may use typed PHI / `value_types` metadata as
  collection-origin evidence after copy-origin resolution.
- `RuntimeDataBox.set` is accepted when the receiver origin is `MapBox` or
  `ArrayBox`.
- `RuntimeDataBox.get` keeps using the same route-policy path and now sees
  PHI-typed `ArrayBox` / `MapBox` receivers.

This is not a JsonFrag body rule. It is a reusable generic-method route fact.

## Result

The real `stage1_cli_env.hako` MIR now emits lowering-plan rows inside
`_normalize_instructions_array/1` for:

```text
RuntimeDataBox.get -> MapGet / nyash.runtime_data.get_hh
RuntimeDataBox.set -> MapSet / nyash.map.slot_store_hhh
RuntimeDataBox.get -> ArrayGet / nyash.array.slot_load_hi
```

The remaining blocker moves to the scalar/string sentinel child path:

```text
JsonFragBox.get_int/2
  -> JsonFragBox.read_int_after/2
  -> JsonFragBox.read_int_from/2
```

## Acceptance

```bash
cargo test -q generic_method_route --lib
cargo fmt --check
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p182c_fact_probe.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
