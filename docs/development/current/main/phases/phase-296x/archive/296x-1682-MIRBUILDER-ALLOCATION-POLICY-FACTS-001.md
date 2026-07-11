# 296x-1682: MirBuilder Allocation Policy Facts

Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-ALLOCATION-POLICY-FACTS-001

## Decision

Open the allocation-policy lane with facts first, not executable lowering.

The parent blocker remains:

```text
MIRBUILDER-ALLOCATION-POLICY-SLICE-001
```

The selected child slice is:

```text
MIRBUILDER-ALLOCATION-POLICY-FACTS-001
```

This slice fixes source authority before choosing a Hako representation for
`MirBuilder::next_value_id`.

## Rationale

`CoreContext::next_value` is not the semantic authority for
`MirBuilder::next_value_id`.

The live Rust policy is a composition:

```text
select allocator per candidate attempt
  current_function present -> MirFunction::next_value_id
  current_function absent  -> CoreContext::next_value

consume candidate
check reserved exclusion set membership
  rejected candidate is already consumed
  retry by generating the next candidate
return the first accepted candidate
```

Function-local allocation also differs from module-global allocation:

```text
MirFunction::new:
  parameter ValueIds = [0, param_count)
  next counter       = max(param_count, 1)

CoreContext generator:
  first candidate = 0
```

Therefore the next step is not another CoreContext-style executable smoke. It
is a source-facts and policy-resolution slice.

## Source Authority

Selected source surfaces:

```text
src/mir/builder/utils/id_alloc.rs
src/mir/function/function_impl.rs
src/mir/builder/calls/parameter_setup.rs
src/mir/builder/control_flow/joinir/merge/header_phi_prebuild.rs
src/mir/builder/compilation_context.rs
crates/hakorune_mir_core/src/value_id.rs
```

## Fact Model

Add an allocation-policy-specific facts model outside `CoreContext`,
`VerifiedHakoFamilyIR`, and artifact manifests.

Conceptual shape:

```python
MirBuilderAllocationPolicyFactsV1:
  candidate_selection
  function_allocator
  module_allocator
  exclusion_set
  parameter_initialization
  parameter_binding
  counter_floor_repair
  boundary_facts
```

The next layer resolves source facts into a typed policy:

```python
ResolvedValueAllocationPolicyV1:
  allocator_selector
  candidate_producers
  acceptance_predicate
  rejection_effect
  retry_policy
  result_transport
```

Directability remains a separate decision:

```text
source facts
  -> resolved allocation policy
  -> directability decision
```

## Required Facts

`MirBuilder::next_value_id`:

```text
allocator selection = CurrentFunctionPresent
present source = MirFunctionNextValueId
absent source = CoreContextNextValue
selection frequency = PerCandidateAttempt
acceptance predicate = CandidateNotInReservedSet
rejected_candidate_effect = Consumed
retry = GenerateNextCandidate
```

`MirFunction`:

```text
param IDs = [0, param_count)
counter seed = max(param_count, 1)
next operation = TakeThenIncrement
transport = ValueIdAsI64
```

Parameters:

```text
ParameterIdPrepopulation
FunctionCounterSeed
ParameterBindingReuse
ParameterCounterFloorRepair
compatibility fallback recorded as unselected boundary
```

Reserved exclusion set:

```text
storage_owner = CompilationContext
producer = JoinIrHeaderPhiPrebuild
members = PhiDestinations union JoinIrFunctionParameters
update_kind = ReplaceSnapshot
consumer = MirBuilder::next_value_id
observation = MembershipOnly
lifetime = JoinIrMergeTemporary
```

Sentinel/floor:

```text
function_initial_floor = 1
zero_reserved_by_function_constructor_policy = true
formal_invalid_sentinel = u32::MAX
formal_invalid_exclusion_claim = false
```

## Implementation Boundary

Allowed:

```text
tools/rust_lifecycle/mirbuilder_allocation_policy_facts.py
tools/rust_lifecycle/extract_mirbuilder_allocation_policy_facts.py
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-allocation-policy-facts-v0.json
tools/checks/rust_lifecycle_mirbuilder_allocation_policy_facts_guard.sh
task-order thin pointer update
```

Not allowed:

```text
generated .hako change
CoreContext artifact change
artifact manifest change
VerifiedHakoFamilyIR extension
backend route change
ABI change
runtime fallback
bundle membership change
```

## Directability

The initial directability result is expected to deny executable lowering:

```text
Deny(UnsupportedDirectShape)
  detail=CurrentFunctionAndReservedSetCompositionUnselected
```

Additional explicit unselected boundaries:

```text
CurrentFunctionOptionTransportUnselected
ReservedValueSetTransportUnselected
ParameterSetupCompatibilityFallbackUnselected
ReservedSetLifetimeProofRequired
FormalInvalidSentinelPolicyUnselected
AllocationCounterOverflowPolicyUnselected
```

## Acceptance

```text
live source -> facts fixture deterministic
facts fixture detects drift
resolved policy records consumed rejected candidates
function-local and module-global allocation are distinct
parameter prepopulation / binding reuse / counter floor repair are distinct
reserved exclusion set includes PHI destinations and JoinIR parameters
zero floor and u32::MAX invalid sentinel are distinct
CoreContext reserved-skip claim remains 0
generated Hako byte-identical
executable allocation policy claim = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
task-order remains under 800 lines
```

## Validated

```text
bash tools/checks/rust_lifecycle_mirbuilder_allocation_policy_facts_guard.sh
python3 -m py_compile tools/rust_lifecycle/mirbuilder_allocation_policy_facts.py tools/rust_lifecycle/extract_mirbuilder_allocation_policy_facts.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout Evidence

```text
MirBuilderAllocationPolicyFactsV1=green
ResolvedValueAllocationPolicyV1=green
directability=Deny(UnsupportedDirectShape)
detail=CurrentFunctionAndReservedSetCompositionUnselected
rejected_candidate_effect=Consumed
retry=GenerateNextCandidate
function_allocator=TakeThenIncrement
function_counter_seed=max(param_count, 1)
module_allocator=CoreContextNextValue
reserved_exclusion_members=PhiDestinations union JoinIrFunctionParameters
CoreContext_reserved_skip_claim=0
generated_hako_changed=0
executable_allocation_policy_claim=0
backend_route_changed=0
abi_changed=0
runtime_fallback=0
```

Suggested mutation probes:

```text
reserved contains predicate inverted -> guard fail
max(param_count, 1) changed to param_count -> guard fail
reserved PHI dst union JoinIR params broken -> guard fail
```

## Non-claims

```text
MirBuilder::next_value_id executable conversion = 0
reserved set Hako transport selection = 0
current_function Option transport selection = 0
parameter setup fallback conversion = 0
ValueId::INVALID exclusion = 0
allocation overflow behavior = 0
CoreContext policy expansion = 0
```

## Follow-on Order

```text
1. MIRBUILDER-ALLOCATION-POLICY-FACTS-001 (landed)
2. FUNCTION-LOCAL-VALUE-ID-ALLOCATOR-001 (next)
3. RESERVED-VALUE-EXCLUSION-POLICY-001
4. MIRBUILDER-NEXT-VALUE-ID-COMPOSITION-001
```
