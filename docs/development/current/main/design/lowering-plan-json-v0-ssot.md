---
Status: SSOT
Decision: accepted
Date: 2026-05-01
Scope: backend-facing LoweringPlan JSON v0 contract for pure-first ny-llvmc.
Related:
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - src/mir/core_method_op.rs
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# LoweringPlan JSON v0 SSOT

## Purpose

Move ny-llvmc toward emit-only behavior without a wide rewrite.

The backend must not keep discovering semantic shape from raw MIR. The
backend-facing contract is:

```text
CoreOp is semantics.
LoweringPlan is backend contract.
ny-llvmc is an emitter.
ColdRuntime is explicit unoptimized lowering, not compat replay.
HotInline is proof-only optimization.
```

## Stop Line

- ny-llvmc may consume `metadata.lowering_plan`.
- ny-llvmc must not invent a `LoweringPlan` entry from raw MIR.
- if a site has a valid `LoweringPlan` entry, ny-llvmc should prefer that entry
  over legacy route metadata for the same site.
- `Unsupported` belongs to the plan-builder side. The backend may report it, but
  it must not lower it.
- `ColdRuntime` is allowed only when the plan names an explicit runtime ABI
  symbol. It is not `HAKO_BACKEND_COMPAT_REPLAY=harness`.

## JSON v0 Shape

`metadata.lowering_plan` is an array of flat entries. The flat form is
intentional: C consumers can read it without nested shape interpretation.

Required fields:

| field | meaning |
| --- | --- |
| `site` | stable display id such as `b0.i2` |
| `block` | MIR block id |
| `instruction_index` | MIR instruction index within the block |
| `source` | builder source family, initially `generic_method_routes` |
| `source_route_id` | original source route id, for example `generic_method.get` |
| `core_op` | resolved semantic op, for example `MapGet` |
| `tier` | `HotInline`, `DirectAbi`, `ColdRuntime`, or `Unsupported` |
| `emit_kind` | `inline_ir`, `direct_abi_call`, `runtime_call`, or `unsupported` |
| `symbol` | ABI/helper symbol for call-based entries, or `null` |
| `proof` | semantic proof, initially `core_method_contract_manifest` |
| `route_proof` | source route proof, for example `get_surface_policy` |
| `route_kind` | source route kind needed by current emitters during migration |
| `perf_proof` | true only for keeper hot-path proof |

Optional fields may carry operands and result values:

| field | meaning |
| --- | --- |
| `receiver_value` | receiver value id |
| `receiver_origin_box` | receiver origin family when known |
| `arity` | method arity for method-derived plan entries |
| `key_route` | key/index route, or `null` |
| `key_value` | first key/index value id, or `null` |
| `result_value` | result value id, or `null` |
| `return_shape` | semantic result shape, or `null` |
| `value_demand` | value demand expected by emitter/runtime |
| `publication_policy` | publication/objectization policy, or `null` |
| `effects` | stable effect tags |

Example:

```json
{
  "site": "b0.i2",
  "block": 0,
  "instruction_index": 2,
  "source": "generic_method_routes",
  "source_route_id": "generic_method.get",
  "core_op": "MapGet",
  "tier": "ColdRuntime",
  "emit_kind": "runtime_call",
  "symbol": "nyash.runtime_data.get_hh",
  "proof": "core_method_contract_manifest",
  "route_proof": "get_surface_policy",
  "route_kind": "runtime_data_load_any",
  "perf_proof": false,
  "receiver_value": 1,
  "key_value": 2,
  "result_value": 3,
  "return_shape": "mixed_runtime_i64_or_handle",
  "value_demand": "runtime_i64_or_handle",
  "publication_policy": "runtime_data_facade",
  "effects": ["read.key"]
}
```

## Tier Mapping

v0 bridges existing CoreMethodContract vocabulary into backend-facing tiers:

| source tier | plan tier | meaning |
| --- | --- | --- |
| `warm_direct_abi` | `DirectAbi` | direct helper ABI call is explicit and accepted |
| `cold_fallback` | `ColdRuntime` | runtime ABI call is explicit and not perf proof |
| future hot proof | `HotInline` | inline IR with keeper proof |
| no plan | absent / `Unsupported` | plan builder owns diagnosis |

## Migration Rule

v0 starts by deriving plan entries from `generic_method_routes`.

That is not the final architecture, but it creates the right boundary:

```text
generic_method_routes
  -> lowering_plan v0
  -> ny-llvmc reads plan first
  -> legacy route readers stay as fallback only during migration
```

New backend work should add a `LoweringPlan` entry before adding a new raw
`.inc` matcher. Existing route metadata may stay until the matching plan
consumer is proven.

## Consumer Rule

`.inc` consumers must read the common generic-method plan fields through the
shared LoweringPlan metadata view before applying family-specific legality.
Consumers may validate operands, proofs, effects, and helper symbols for their
own family, but they should not duplicate the generic source/tier/proof/site
field parsing.

Need-kind declaration rules should be table rows keyed by LoweringPlan view
fields. Do not add one-off `strcmp` ladders for every new proven plan slice.

## Proven v0 Slices

| slice | tier | symbol | proof |
| --- | --- | --- | --- |
| `MapGet` | `ColdRuntime` | `nyash.runtime_data.get_hh` | P70 plan-only fixture |
| `MapGet` | `DirectAbi` | `nyash.map.slot_load_hh` | P80 plan-only fixture |
| `MapHas` | `DirectAbi` | `nyash.map.probe_hi` | P72 plan-only fixture |
| `ArrayHas` | `DirectAbi` | `nyash.array.has_hh` | P82 plan-only fixture |
| `MapLen` | `DirectAbi` | `nyash.map.entry_count_i64` | P75 plan-only fixture |
| `ArrayLen` | `DirectAbi` | `nyash.array.slot_len_h` | P76 plan-only fixture |
| `StringLen` | `DirectAbi` | `nyash.string.len_h` | P77 plan-only fixture |
| `ArrayGet` | `DirectAbi` | `nyash.array.slot_load_hi` | P78 plan-only fixture |
| `ArrayPush` | `ColdRuntime` | `nyash.array.slot_append_hh` | P83 plan-only fixture |

## Non-goals

- no broad `CoreOp` expansion in this card
- no hidden compat replay
- no new environment variable
- no promise that `ColdRuntime` is a perf keeper
