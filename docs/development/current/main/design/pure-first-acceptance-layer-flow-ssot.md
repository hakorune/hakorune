# Pure-First Acceptance Layer Flow SSOT

Status: accepted
Date: 2026-05-16

## Purpose

Pure-first EXE failures must identify which compiler layer could not satisfy
the contract. A user should not need to infer whether a failure is slow build,
semantic route metadata, route preflight, or backend lowering.

## Layer Flow

```text
source/parser:
  accepts syntax and source surface

mir-emit:
  emits canonical MIR JSON

semantic-route:
  refreshes value facts, receiver facts, return_shape, value_demand,
  target_result_box_name, and lowering_plan rows

route-preflight:
  reads MIR JSON metadata before backend emission and fails fast on route
  contracts that are already known missing or unsupported

backend:
  lowers accepted routes to LLVM/EXE and owns SSA / PHI / IR verifier failures
```

## Diagnostic Contract

Preflight failures must print these stable fields:

```text
[pure-first-route][fail]
layer=<source/parser|mir-emit|semantic-route|route-preflight|backend|mir-schema>
function=<function name>
site=<block.instruction or metadata/schema>
callee=<callee or ?>
reason=<stable reason token>
owner=<metadata owner>
contract=<missing or violated contract>
suggestion=<next structural action>
```

The `layer` field names where the contract should have been satisfied. The
`owner` field names the metadata family or artifact owner. The `contract` field
names the precise data that is absent or inconsistent.

## Reason Tokens

The stable reason vocabulary is owned by:

```text
docs/reference/mir/route-diagnostics-vocabulary.md
```

This section is a compact operational mirror for the pure-first layer flow.

```text
lowering_plan_missing:
  route-preflight could not find a required metadata.lowering_plan row

unsupported_tier:
  lowering_plan row exists but cannot be emitted by pure-first

typed_user_box_method_contract_missing:
  user_box_method_routes row exists but reason/proof is not accepted

typed_global_call_contract_missing:
  global_call_routes row exists but reason/proof is not accepted

target_exists=false:
  semantic-route did not publish a valid same-module target

arity_mismatch:
  semantic-route target arity contract does not match the call

target_body_supported=false:
  semantic-route found the target but not a supported body shape

return_shape_missing:
  route has a result but no return_shape

value_demand_mismatch:
  route has return_shape but no compatible value_demand

object_return_target_box_missing:
  route has return_shape=object_handle but no target_result_box_name

random_capability_route_unsupported:
  route-preflight was asked to reject reachable hako.random capability metadata

reclaim_execution_route_unsupported:
  route-preflight was asked to reject reachable hako.alloc.reclaim capability metadata
```

## Boundary

Route preflight is not a backend verifier. If LLVM emission creates invalid SSA
or PHI values, that is a backend-layer failure and should be fixed or classified
by a backend invariant guard. Do not hide backend failures by adding semantic
fallbacks in preflight.
