# Trim Route Lowering Decision Probe

Status: fixture probe
Scope: read-only trim route lowering decision over existing lifecycle facts.

## Purpose

The trim route lowering inventory established that `trim_helper` is metadata,
not an executable route proof.

This probe adds one read-only decision surface:

```text
metadata candidate:
  trim_helper present and structurally valid

executable route lowering:
  denied until promoted carrier identity / join_id proof exists
```

No backend lowering is implemented here.

## Inputs

The decision reads already-inventoried facts:

```text
TrimLoopHelper:
  original_var
  carrier_name
  whitespace_chars
  carrier_type() == "Bool"
  initial_value() == true
  has_valid_structure()

CarrierInfo:
  trim_helper()
  promoted_body_locals
  resolve_promoted_join_id()
```

## Decision Shape

The probe deliberately separates metadata readiness from executable route
permission:

```text
TrimRouteLoweringDecision:
  candidate_kind=TrimRouteMetadataCandidate
  metadata_decision=AllowMetadataCandidate
  executable_decision=DenyMissingPromotedCarrierIdentity
```

This prevents a common bug:

```text
trim_helper is present
  -> accidentally emit executable route lowering
```

## Positive Metadata Conditions

The metadata candidate is valid when:

```text
trim_helper is present
original_var is not empty
carrier_name is not empty
whitespace_chars is not empty
carrier_type is Bool
initial_value is true
```

This is only a metadata Allow.

## Executable Deny Condition

Executable lowering requires a stable identity for the promoted carrier:

```text
promoted_body_locals contains original_var
resolve_promoted_join_id(original_var) returns Some(ValueId)
```

Current production code has no non-test `Some(ValueId)` join_id producer.
Therefore the executable route decision is:

```text
DenyMissingPromotedCarrierIdentity
```

## Output Fixtures

```text
trim-route-lowering-decision-facts-v0.json:
  records the candidate input facts and denied dependency

trim-route-lowering-decision-plan-v0.json:
  records metadata Allow plus executable Deny

trim-route-lowering-decision-oracle-vectors-v0.json:
  records valid metadata, missing join_id, and invalid metadata vectors
```

## Decision

```text
trim_route_decision_probe=1
metadata_candidate_allow=1
executable_lowering_allow=0
deny_reason=MissingPromotedCarrierIdentity
join_id_producer=0
backend_behavior_changed=0
generated_program_execution_claim=0
```

## Stop Lines

```text
do not emit trim route lowering
do not infer executable permission from trim_helper presence
do not fabricate join_id
do not change backend behavior
do not claim generated program execution
do not start rustc adapter work in this row
```
