# 296x-1687: MirBuilder next_value_id Prepared-State Hako Kernel

Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-NEXT-VALUE-ID-PREPARED-STATE-HAKO-KERNEL-001

## Decision

Implement the prepared-state generated Hako policy kernel for
`MirBuilder::next_value_id`.

The implemented lowering is:

```text
MirBuilderNextValueIdCompositionPlanV1
  -> MirBuilderNextValueIdExecutionProjectionV1
  -> VerifiedHakoFamilyIR
  -> generated Hako policy kernel
```

This is not a full `MirBuilder` object method.

## Added Artifacts

```text
tools/rust_lifecycle/mirbuilder_next_value_id_prepared_state_kernel_artifacts.py
tools/checks/rust_lifecycle_mirbuilder_next_value_id_prepared_state_kernel_guard.sh
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-next-value-id-execution-projection-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-next-value-id-prepared-state-oracle-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-next-value-id-prepared-state-recipe-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-next-value-id-prepared-state-verifier-result-v0.json
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.hako
lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.artifact.json
```

The family is also registered in:

```text
tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py
```

## Execution Projection

```text
MirBuilderNextValueIdExecutionProjectionV1:
  execution_scope = PreparedStatePolicyKernel
  selector_transport = CurrentFunctionPresenceI64BoolV0
  selector_evaluation = PerCandidateAttempt
  function_state_transport = FunctionValueIdCounterState.next_value_id
  module_state_transport = CoreContext VerifiedFamilyArtifactContractV1
  exclusion_transport = ReservedValueIdMembershipOnly
  result_transport = ValueIdAsI64
```

Prepared-state directability:

```text
PreparedStatePolicyKernel = Allow
FullMirBuilderObjectMethod = Deny
```

## Generated Kernel

The generated kernel uses:

```text
FunctionValueIdCounterStateApi.next
CoreContextApi.next_value
ReservedValueIdMembershipViewApi.has
MirBuilderAllocationPolicyApi.next_value_id
```

The selector remains inside the retry loop. The retry loop is bounded by the
overflow boundary; total termination and overflow parity remain non-claims.

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_next_value_id_prepared_state_kernel_guard.sh
python3 tools/rust_lifecycle/convert_mirbuilder_lightweight_facts.py --all --check
python3 -m py_compile tools/rust_lifecycle/mirbuilder_next_value_id_prepared_state_kernel_artifacts.py
```

Guard evidence:

```text
generated_hako_parse=green
generated_hako_mir_emit=green
policy_route=generic_i64_or_leaf_direct
generated_hako_exe_aot=green
```

## Non-Claims

```text
full_mirbuilder_object_method = 0
full Option<MirFunction> transport = 0
full MirFunction conversion = 0
ScopeContext conversion = 0
CompilationContext conversion = 0
parameter compatibility fallback = 0
formal INVALID sentinel exclusion = 0
overflow parity = 0
total termination = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
mainline_selected = 0
```

The next blocker is:

```text
MIRBUILDER-ALLOCATION-POLICY-BUNDLE-ADOPTION-001
```
