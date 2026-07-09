# 3426 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-POLICY-ROW-IDENTITY-TRANSPORT-001
```

## Purpose

Carry the existing `.hako` policy row ID into the generated typed Rust policy
artifacts for MapLoad, String, and Collection. The row ID is identity metadata
for the assertion-only caller consumer; it is not route authority.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Required Delta

1. Add `policy_row_id: &'static str` to the three generated policy structs.
2. Emit the source row ID for the exact 1 + 3 + 4 read rows.
3. Preserve all existing route fields, generated artifact freshness, and Rust
   oracle comparisons.
4. Add a guard for exact row identity, order, uniqueness, and generator parity.

## Ownership Boundary

```text
allowed = policy identity metadata consumed by an assertion-only checker
forbidden = route selection, receiver-domain selection, MIR/backend lowering,
            runtime dispatch, mutation, publication, or Source Selfhost
```

Collection receiver domains remain in the existing Collection policy artifact;
`AnyLength -> Box` remains an explicit policy row and is not copied into the
caller contract.

## Acceptance

```text
read_policy_row_identity_transport = 1
eight_row_identity_exact = 1
mapload_row_count = 1
string_row_count = 3
collection_row_count = 4
route_selection_authority_switch = 0
caller_orientation_runtime_path = 0
source_selfhost_claim = 0
selected_next_card =
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_read_policy_row_identity_transport_guard.sh
```

## Result

```text
status = landed
implementation = complete
read_policy_row_identity_transport = 1
eight_row_identity_exact = 1
selected_next_card =
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-MAPLOAD-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```
