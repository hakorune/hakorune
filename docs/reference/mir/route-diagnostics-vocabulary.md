# MIR Route Diagnostics Vocabulary

Status: accepted
Date: 2026-05-18

## Purpose

This file is the SSOT for route diagnostic reason tokens used by pure-first
MIR/EXE preflight. It separates three vocabularies that are easy to confuse:

```text
proof:
  backend-facing route contract selected by MIR route planners

planner reason:
  route-family-local explanation stored in route metadata when a proof is not
  accepted

preflight reason:
  stable diagnostic token printed before backend emission
```

Backends consume `functions[].metadata.lowering_plan` and proof-bearing route
fields. They must not infer route support from raw helper names, raw app names,
or source strings.

## Producers And Consumers

| Layer | Owner | Role |
| --- | --- | --- |
| MIR route planners | `src/mir/*_route_plan*` | Produce route facts, proof names, planner-local `reason` fields, return shape, value demand, and target facts. |
| MIR JSON emit | `src/runner/mir_json_emit/route_json.rs` | Publishes route facts into `functions[].metadata.lowering_plan`. |
| Pure-first preflight | `tools/checks/pure_first_route_preflight.py` | Reads `lowering_plan` and emits stable preflight reasons before ny-llvmc/C shim. |
| C backend shim | `lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc` | Consumes accepted proofs and route fields for emission. It is the final route guard, not the first diagnostic owner. |
| Docs | this file | Owns reason/proof boundaries and token spelling. |

## Output Contract

Pure-first route preflight failures print one compact diagnostic block:

```text
[pure-first-route][fail]
layer=<source/parser|mir-emit|semantic-route|route-preflight|backend|mir-schema>
function=<function name>
site=<block.instruction or metadata/schema>
callee=<callee or ?>
reason=<stable preflight reason>
owner=<metadata owner>
contract=<missing or violated contract>
suggestion=<next structural action>
```

`reason` is the stable token for automation. `owner` and `contract` point to
the route metadata family that should be fixed.

## Preflight Reasons

| Reason | Layer | Owner | Source condition | Next action |
| --- | --- | --- | --- | --- |
| `lowering_plan_missing` | `route-preflight` or `mir-schema` | `metadata.lowering_plan` / route family | Reachable extern call has no plan row, `functions[]` / `metadata.lowering_plan` is absent or invalid, or duplicate site rows violate schema uniqueness. | Publish or repair the `lowering_plan` row before pure-first EXE build. |
| `unsupported_tier` | `route-preflight` | plan `source` family | `tier == "Unsupported"` or `emit_kind == "unsupported"` on a reachable plan row. | Select a supported tier / emit kind, or keep the call out of the pure-first EXE route. |
| `target_exists=false` | `semantic-route` | `global_call_routes` / `user_box_method_routes` | Same-module target publication failed. | Fix resolver/module target publication before backend emission. |
| `arity_mismatch` | `semantic-route` | `global_call_routes` / `user_box_method_routes` | `arity_matches == false` or the target arity does not match the call. | Fix source call arity or route arity contract. |
| `target_body_supported=false` | `semantic-route` | `user_box_method_routes` | Target exists, but the method body shape is not in the accepted route profile. | Split a narrow user-box method body acceptance row. |
| `typed_user_box_method_contract_missing` | `semantic-route` | `user_box_method_routes` | User-box route exists but its planner-local `reason` / proof is not an accepted direct contract. | Publish a supported typed user-box method route contract. |
| `typed_global_call_contract_missing` | `semantic-route` | `global_call_routes` | Global-call route exists but its planner-local `reason` / proof is not an accepted direct contract. | Publish a supported typed global-call route contract. |
| `return_shape_missing` | `semantic-route` | route family | Route has a result and supported emit kind, but `return_shape` is absent. | Publish `return_shape` or make the route diagnostics-only. |
| `value_demand_mismatch` | `semantic-route` | route family | Route has `return_shape`, but `value_demand` is absent or incompatible. | Publish value demand compatible with the selected return shape. |
| `object_return_target_box_missing` | `semantic-route` | `global_call_routes` / `user_box_method_routes` | `return_shape == "object_handle"` but `target_result_box_name` is absent. | Publish the result box name for object-handle route results. |
| `random_capability_route_unsupported` | `route-preflight` | `capability_plans` | `--reject-unsupported-random` is active and reachable metadata allows `hako.random` without a supported route. | Select a random substrate route row or keep the caller on proof-only deterministic keys. |
| `reclaim_execution_route_unsupported` | `route-preflight` | `capability_plans` | `--reject-unsupported-reclaim-execution` is active and reachable metadata allows `hako.alloc.reclaim` without a supported route. | Select a reclaim execution row or keep the caller on read-only reclaim inventory. |

Schema failures currently reuse `lowering_plan_missing` so existing tools get a
single fail-fast route-preflight class. Split schema-specific tokens only in a
future diagnostics row.

## Planner-Local Reasons

Planner-local `reason` fields are route-family evidence. They are not the
stable preflight vocabulary unless the preflight tool maps them to a reason
above.

| Family | Planner-local reason | Meaning |
| --- | --- | --- |
| `global_call_routes` | `missing_multi_function_emitter` | Target shape is known enough for a future uniform multi-function emitter, but this row is still diagnostics-only. |
| `global_call_routes` | `global_call_arity_mismatch` | Global target exists but arity does not match. |
| `global_call_routes` | `unknown_global_callee` | Resolver/module facts did not publish a known target. |
| `user_box_method_routes` | `typed_object_plan_missing` | Receiver type id / typed object plan is absent. |
| `user_box_method_routes` | `user_box_method_target_missing` | Method target was not found. |
| `user_box_method_routes` | `user_box_method_arity_mismatch` | Method target arity does not match receiver+arguments. |
| `user_box_method_routes` | `user_box_birth_body_unsupported` | `birth` target body shape is not accepted. |
| `user_box_method_routes` | `user_box_method_body_unsupported` | Non-birth method body shape is not accepted. |
| `user_box_method_routes` | `user_box_method_return_type_unsupported` | Target return type cannot be lowered by the current route profile. |
| `user_box_method_routes` | `user_box_method_contract_missing` | Target facts are present but no direct contract was selected. |

Pure-first preflight may report a broader stable token such as
`typed_global_call_contract_missing`, `typed_user_box_method_contract_missing`,
`arity_mismatch`, or `target_body_supported=false` for these route-local
reasons.

## Proof Vocabulary Boundary

Proof names are backend-facing route contracts. They are not diagnostic
categories, even when a transitional missing-contract proof shares a similar
string.

| Source | Proof examples | Consumer | Boundary |
| --- | --- | --- | --- |
| `extern_call_routes` | `extern_registry` and substrate-specific extern proofs | C backend shim / `lowering_plan` consumers | Extern proof selects an explicit runtime/direct extern route. |
| `global_call_routes` | `typed_global_call_leaf_numeric_i64`, `typed_global_call_generic_pure_string`, `typed_global_call_stage1_emit_program_json`, `typed_global_call_same_module_object_handle`, `typed_global_call_same_module_scalar_i64`, `typed_global_call_same_module_void_sentinel`, `typed_global_call_void_side_effect` | Global-call route emitter | Accepted proof permits the backend to emit from route fields. |
| `global_call_routes` | `typed_global_call_contract_missing` | Diagnostics / preflight | Missing contract proof must not be treated as backend permission. |
| `user_box_method_routes` | `typed_user_box_birth_same_module`, `typed_user_box_method_same_module` | User-box method route emitter | Accepted proof permits same-module direct call emission. |
| `user_box_method_routes` | `typed_user_box_method_contract_missing` | Diagnostics / preflight | Missing contract proof must not be treated as backend permission. |

When adding a new proof, update the route producer, MIR JSON emission, backend
consumer, and this table in the same row. When adding a new preflight reason,
update `tools/checks/pure_first_route_preflight.py`, this file, and the
corresponding guard/docs row in the same row.

## Stop Lines

- Do not add route acceptance shapes in a diagnostics-vocabulary row.
- Do not widen backend proof allowlists without a route implementation row.
- Do not add `.inc` app-name or helper-name matchers to repair diagnostics.
- Do not make preflight a backend verifier; backend SSA/PHI failures remain
  backend-layer failures.
