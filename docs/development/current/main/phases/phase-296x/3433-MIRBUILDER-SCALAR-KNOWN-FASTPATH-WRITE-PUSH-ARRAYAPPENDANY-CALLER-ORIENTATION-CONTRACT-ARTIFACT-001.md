# 3433 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-PUSH-ARRAYAPPENDANY-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-WRITE-PUSH-ARRAYAPPENDANY-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001
```

## Purpose

Materialize a metadata-only caller-orientation contract for the existing
`array_append_any_push_surface` policy row and generate its checked-in typed
Rust artifact.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Required Delta

1. Add a hand-authored `.hako` contract containing only the existing policy
   row ID and the standard assertion-only metadata flags.
2. Add a deterministic generator and checked-in typed Rust artifact.
3. Add freshness, exact row identity, no-live-consumer, and no-mutation-copy
   guards.

## Boundary

The contract must not copy `effect_class`, `mutation_class`, value boundary,
route kind, core operation, or lowering tier. Those remain owned by the
existing policy/oracle.

## Non-Claims

```text
caller_orientation_live_consumer = 0
caller_orientation_runtime_path = 0
route_selection_authority_switch = 0
write_mutation_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
delete_hako_route_decision_authority_pilot = 0
write_wide_authority = 0
scalar_known_wide_authority = 0
backend_lowering_authority = 0
source_selfhost_claim = 0
```

## Stop Conditions

Stop and consult Pro before adding any authority-bearing caller result,
runtime/backend/mutation/publication consumer, Delete row, or ScalarKnown-wide
claim. The next packet item is 3434 only after this artifact and its guard are
green.
