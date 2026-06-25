# 296x-1684: Reserved ValueId Exclusion Policy

Status: Complete
Date: 2026-06-25
Token: RESERVED-VALUE-EXCLUSION-POLICY-001

## Decision

Close only the reserved ValueId exclusion policy boundary.

This slice consumes the already-verified allocation-policy facts and projects:

```text
MirBuilderAllocationPolicyFactsV1.exclusion_set
  -> ReservedValueExclusionPolicyPlanV1
```

It does not claim full `MirBuilder::next_value_id` execution or any concrete
set/map representation.

## Source Authority

Selected live source semantics:

```text
CompilationContext:
  reserved_value_ids storage owner
  membership observation only

JoinIR header PHI prebuild:
  members = PhiDestinations union JoinIrFunctionParameters
  update_kind = ReplaceSnapshot

MirBuilder::next_value_id:
  predicate = CandidateNotInReservedSet
  rejected candidate is already consumed
  retry = GenerateNextCandidate
```

The projection is derived from:

```text
MirBuilderAllocationPolicyFactsV1.exclusion_set
```

## Added Contract

```text
ReservedValueExclusionPolicyPlanV1:
  storage.owner = CompilationContext
  storage.concrete_representation = Unselected
  producer.source = JoinIrHeaderPhiPrebuild
  producer.members = PhiDestinations + JoinIrFunctionParameters
  producer.update_kind = ReplaceSnapshot
  consumer.source = MirBuilder::next_value_id
  consumer.predicate = CandidateNotInReservedSet
  consumer.observation = MembershipOnly
  rejection.effect = Consumed
  rejection.retry = GenerateNextCandidate
```

Focused oracle vectors:

```text
reserved=[2,4], candidates=[1,2,3,4,5]
  accepted=[1,3,5]
  rejected_consumed=[2,4]

reserved=[], candidates=[1,2,3]
  accepted=[1,2,3]
  rejected_consumed=[]
```

## Implementation

Added:

```text
tools/rust_lifecycle/mirbuilder_reserved_value_exclusion_policy.py
docs/development/current/main/design/fixtures/rust-lifecycle/reserved-value-exclusion-policy-plan-v0.json
tools/checks/rust_lifecycle_reserved_value_exclusion_policy_guard.sh
```

The guard verifies the plan fixture and drift probes for:

```text
member union drift
predicate inversion
rejection effect drift
oracle drift
```

## Non-Claims

```text
function_allocator = 0
current_function_composition = 0
module_global_fallback = 0
formal_invalid_sentinel_exclusion = 0
concrete_ordered_map_or_set_representation = 0
phi_dst_only_naming = 0
generated_hako_artifact = 0
backend_route_changed = 0
abi_changed = 0
runtime_fallback = 0
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_reserved_value_exclusion_policy_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_allocation_policy_facts_guard.sh
bash tools/checks/current_state_pointer_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_reserved_value_exclusion_policy.py
cargo check --release
```

The next blocker is:

```text
MIRBUILDER-NEXT-VALUE-ID-COMPOSITION-001
```
