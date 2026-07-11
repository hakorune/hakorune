# 296x-1685: MirBuilder next_value_id Composition

Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-NEXT-VALUE-ID-COMPOSITION-001

## Decision

Close the full `MirBuilder::next_value_id` policy as a plan/oracle
composition, then stop before choosing the executable surface.

This slice consumes:

```text
ResolvedValueAllocationPolicyV1
FunctionLocalValueIdAllocatorPlanV1
ReservedValueExclusionPolicyPlanV1
```

and projects:

```text
MirBuilderNextValueIdCompositionPlanV1
```

## Source Authority

The composition does not rescan or reinterpret source syntax. It consumes prior
facts and subplans:

```text
allocator_selector = CurrentFunctionPresent
present producer   = MirFunctionNextValueId
absent producer    = CoreContextNextValue
acceptance         = CandidateNotInReservedSet
rejection.effect   = Consumed
retry              = GenerateNextCandidate
result_transport   = ValueIdAsI64
```

## Added Contract

```text
MirBuilderNextValueIdCompositionPlanV1:
  subplans:
    function_local = FunctionLocalValueIdAllocatorPlanV1
    reserved_exclusion = ReservedValueExclusionPolicyPlanV1
  allocator_selector = CurrentFunctionPresent
  candidate_producers = ResolvedValueAllocationPolicyV1.candidate_producers
  acceptance_predicate = CandidateNotInReservedSet
  rejection = Consumed + GenerateNextCandidate
  result_transport = ValueIdAsI64
```

Focused oracle vectors:

```text
current_function=Present
  candidates=[1,2,3,4], reserved=[2]
  accepted=[1,3,4], rejected_consumed=[2]

current_function=Absent
  candidates=[0,1,2], reserved=[1]
  accepted=[0,2], rejected_consumed=[1]
```

## Implementation

Added:

```text
tools/rust_lifecycle/mirbuilder_next_value_id_composition.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-next-value-id-composition-plan-v0.json
tools/checks/rust_lifecycle_mirbuilder_next_value_id_composition_guard.sh
```

The guard verifies the plan fixture and drift probes for:

```text
allocator selector drift
function-local subplan drift
retry drift
absent branch oracle drift
```

## Non-Claims

```text
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
formal_invalid_sentinel_exclusion = 0
overflow_policy = 0
silent_fallback = 0
runtime_fallback = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_next_value_id_composition_guard.sh
bash tools/checks/rust_lifecycle_function_local_value_id_allocator_guard.sh
bash tools/checks/rust_lifecycle_reserved_value_exclusion_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_next_value_id_composition.py
cargo check --release
```

## Design Consultation Stop

The next blocker is:

```text
MIRBUILDER-ALLOCATION-POLICY-EXECUTION-SURFACE-CONSULTATION-001
```

Question to resolve:

```text
Should the resolved allocation policy become executable first as:
  A. a generated Hako artifact using the composition plan,
  B. a backend/interpreter consumer of the composition metadata,
  C. another smaller proof surface?
```

Do not implement the execution surface until this boundary is selected.
