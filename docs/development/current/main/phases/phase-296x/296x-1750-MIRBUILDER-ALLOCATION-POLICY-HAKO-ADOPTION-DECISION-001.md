---
Status: Selected
Date: 2026-06-26
Card: MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001
---

# MIRBUILDER-ALLOCATION-POLICY-HAKO-ADOPTION-DECISION-001

## Summary

Record the first narrow Hako adoption decision for the prepared-state
allocation-policy kernel.

This card does not implement a new artifact. It decides whether the existing
DerivedMainline family can move to HakoAdopted, should defer with a named
reason, or should be rejected for this stage.

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

## Required Decision

The card must produce exactly one result:

```text
Adopt
Defer(reason)
Reject(reason)
```

`Adopt` is allowed only if the native `.hako` source can become the edit and
meaning authority for this narrow family, with generator overwrite forbidden.

`Defer` is valid if the artifact is executable but still lacks a required
native-source contract, review boundary, or library dependency.

`Reject` is valid if this family is the wrong adoption pilot.

## Acceptance

```text
source family is already DerivedMainline
native .hako adoption candidate is identified
generator write to adopted source is forbidden or explicitly deferred
Rust bootstrap/oracle role remains available
fallback_policy = Forbidden
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

If adopted, update the artifact model state and route docs for this family
only. If deferred or rejected, record the concrete reason and the next condition
needed before rechecking.

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
