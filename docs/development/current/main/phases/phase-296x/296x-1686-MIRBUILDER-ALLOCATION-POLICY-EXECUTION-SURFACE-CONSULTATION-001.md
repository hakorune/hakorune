# 296x-1686: MirBuilder Allocation Policy Execution Surface Consultation

Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-ALLOCATION-POLICY-EXECUTION-SURFACE-CONSULTATION-001

## Decision

Choose generated Hako as the first executable surface, but only as a
prepared-state policy kernel.

Selected next owner:

```text
MIRBUILDER-NEXT-VALUE-ID-PREPARED-STATE-HAKO-KERNEL-001
```

The selected shape is:

```text
MirBuilderNextValueIdCompositionPlanV1
  -> MirBuilderNextValueIdExecutionProjectionV1
  -> VerifiedHakoFamilyIR
  -> generated Hako policy kernel
  -> focused EXE/AOT smoke
```

This is "A", scoped to "C-size".

## Rejected Surfaces

Rejected:

```text
BackendDirectMetadataConsumer
AdditionalOracleOnlyRunner
```

Reason:

```text
Backend direct interpretation would create a second allocation-policy
interpreter. Additional oracle-only proof would add little after facts,
subplans, and composition are already green.
```

## Prepared-State Boundary

The first artifact is not a full `MirBuilder` object method.

Allowed prepared state:

```text
current_function_present:
  semantic = CurrentFunctionPresence
  physical = typed bool or named i64 bool transport
  evaluation = PerCandidateAttempt

function_state:
  semantic = MirFunction.next_value_id
  physical = FunctionValueIdCounterState.next_value_id
  result = ValueIdAsI64

core_context:
  existing generated CoreContext artifact / contract

reserved_membership:
  semantic = ReservedValueIdMembership
  access = ReadOnly
  physical pilot = membership-only view

result:
  ValueIdAsI64
```

Non-claims:

```text
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

## Execution Projection Requirements

Add:

```text
MirBuilderNextValueIdExecutionProjectionV1
```

It must consume:

```text
MirBuilderNextValueIdCompositionPlanV1
FunctionLocalValueIdAllocatorPlanV1
ReservedValueExclusionPolicyPlanV1
CoreContext VerifiedFamilyArtifactContractV1
```

It must not copy or re-author:

```text
allocator selector
reserved rejection
retry policy
function-local counter seed
oracle expected values
```

## Kernel Semantics

The generated kernel should be equivalent to:

```text
loop forever:
  if current_function_present:
    candidate = FunctionLocalAllocatorApi.next(function_state)
  else:
    candidate = CoreContextApi.next_value(core_context)

  if !ReservedValueIdMembershipView.has(reserved_membership, candidate):
    return candidate
```

The selector stays inside the retry loop because source facts require
`selection_frequency = PerCandidateAttempt`.

## Minimal Acceptance For Next Slice

```text
authority:
  MirBuilderNextValueIdCompositionPlanV1 is consumed
  source syntax is not rescanned
  function-local / reserved subplans are not re-decided

projection:
  MirBuilderNextValueIdExecutionProjectionV1 is generated
  PreparedStatePolicyKernel directability = Allow
  FullMirBuilderObjectMethod directability = Deny

artifact:
  generated Hako + manifest + verifier
  deterministic regeneration
  VerifiedFamilyArtifactContractV1 projects manifest/verifier expectation

present smoke:
  initial function counter = 1
  initial CoreContext value counter = 100
  reserved = [2]
  outputs = [1,3,4]
  final function counter = 5
  final CoreContext value counter = 100

absent smoke:
  initial function counter = 100
  initial CoreContext value counter = 0
  reserved = [1]
  outputs = [0,2]
  final function counter = 100
  final CoreContext value counter = 3

structural verifier:
  selector remains inside retry loop
  only selected allocator mutates
  policy method uses reserved view.has only
  candidate/result remain ValueIdAsI64
  emitter has no allocation-family name branch
```

## Guardrails

Forbidden:

```text
backend direct metadata interpretation
ad hoc Hako shape
runtime fallback
family-name branch in shared emitter
raw i64 truthiness for current_function presence
full context transport claims
```
