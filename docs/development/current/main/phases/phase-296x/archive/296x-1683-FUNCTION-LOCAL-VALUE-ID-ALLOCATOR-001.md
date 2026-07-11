# 296x-1683: Function-Local ValueId Allocator

Status: Complete
Date: 2026-06-25
Token: FUNCTION-LOCAL-VALUE-ID-ALLOCATOR-001

## Decision

Close only the `MirFunction`-local ValueId allocator boundary.

This slice consumes the already-verified allocation-policy facts and projects:

```text
MirBuilderAllocationPolicyFactsV1.function_allocator
  -> FunctionLocalValueIdAllocatorPlanV1
```

It does not claim full `MirBuilder::next_value_id` execution.

## Source Authority

Selected live source semantics:

```text
MirFunction::new:
  parameter ValueIds = [0, param_count)
  next counter       = max(param_count, 1)

MirFunction::next_value_id:
  result = next_value_id
  next_value_id += 1
```

The projection is derived from:

```text
MirBuilderAllocationPolicyFactsV1.function_allocator
```

No second copy of the source policy is introduced.

## Added Contract

```text
FunctionLocalValueIdAllocatorPlanV1:
  state = MirFunction.next_value_id
  parameter_prepopulation = [0, param_count)
  counter_seed = max(param_count, 1)
  next_operation = TakeThenIncrement
  result_transport = ValueIdAsI64
```

Focused oracle vectors:

```text
param_count=0 -> params=[],      initial=1, next=[1,2,3]
param_count=1 -> params=[0],     initial=1, next=[1,2,3]
param_count=3 -> params=[0,1,2], initial=3, next=[3,4,5]
```

## Implementation

Added:

```text
tools/rust_lifecycle/mirbuilder_function_local_value_id_allocator.py
docs/development/current/main/design/fixtures/rust-lifecycle/function-local-value-id-allocator-plan-v0.json
tools/checks/rust_lifecycle_function_local_value_id_allocator_guard.sh
```

The guard verifies the plan fixture and drift probes for:

```text
counter seed drift
ValueIdAsI64 transport drift
zero-param next sequence drift
```

## Non-Claims

```text
reserved_exclusion_set_retry = 0
current_function_composition = 0
module_global_fallback = 0
formal_invalid_sentinel_exclusion = 0
overflow_policy = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_function_local_value_id_allocator_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_allocation_policy_facts_guard.sh
bash tools/checks/current_state_pointer_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_function_local_value_id_allocator.py
cargo check --release
```

The next blocker is:

```text
RESERVED-VALUE-EXCLUSION-POLICY-001
```
