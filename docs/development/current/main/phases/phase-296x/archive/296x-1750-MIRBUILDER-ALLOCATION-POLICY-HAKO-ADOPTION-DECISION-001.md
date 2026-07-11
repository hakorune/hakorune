---
Status: Landed
Date: 2026-06-26
Card: MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001
---

# MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001

## Summary

Record the first narrow Hako adoption decision for the prepared-state
allocation-policy kernel. The decision is deferred: the family is already a
derived mainline artifact, but no native `.hako` source owner exists yet, so
edit/semantic authority cannot be handed off.

## Authority

Decision inputs:

```text
derived-to-native Hako artifact model
rust-to-hako converter implementation role SSOT
prepared-state next_value_id kernel artifact manifest
allocation-policy mainline selection plan
selected route closure evidence
```

The decision is family-scoped:

```text
family:
  hakorune_mir_builder::next_value_id_prepared_state_kernel

scope:
  PreparedStateMirBuilderNextValueIdKernel
```

## Decision

```text
Defer(
  native `.hako` source owner does not yet exist for
  hakorune_mir_builder::next_value_id_prepared_state_kernel
)
```

The family remains a selected generated artifact. The adoption handoff is
deferred until a native `.hako` source file exists and can become the edit and
semantic authority for this narrow family.

## Acceptance

```text
source family is already DerivedMainline
native .hako adoption candidate is identified
generator write to adopted source is explicitly deferred
Rust bootstrap/oracle role remains available
fallback_policy = Forbidden
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

Recheck only after a native `.hako` source owner exists for this family and
the overwrite boundary can be enforced on that native source.

## Non-Claims

```text
full minimal-path mainline = 0
full MirBuilder object method = 0
source selfhost = 0
Python converter retirement = 0
RecordAndPackedLayoutRefresh implementation = 0
refresh decomposition = 0
new ABI = 0
new backend route = 0
runtime fallback = 0
```

## Parked Follow-Up

After this decision, continue with:

```text
MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-EXECUTION-DECOMPOSITION-001
```

The refresh edge is a composite owner. Do not materialize it as one large
artifact before decomposition.

## Closeout

```text
output_contract=mirbuilder-allocation-policy-hako-adoption-decision-v0
family_id=hakorune_mir_builder::next_value_id_prepared_state_kernel
decision=Defer
defer_reason=native .hako source owner does not yet exist for this family
rust_bootstrap_retained=1
runtime_fallback=0
new_backend_route=0
new_abi=0
source_selfhost_claim=0
summary=ok
```
