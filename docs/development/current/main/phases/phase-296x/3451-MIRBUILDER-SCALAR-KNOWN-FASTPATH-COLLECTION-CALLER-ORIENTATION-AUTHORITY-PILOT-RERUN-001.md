# 3451 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001

## Status

Queued after 3450. Do not mark green from task definition alone.

## Required Rerun

```text
cargo test -q caller_orientation
cargo test -q scalar_known_hako_shadow
python3 tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_collection_caller_orientation_authority_pilot.py --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```

The rerun must prove exact four-row freshness, explicit receiver-domain checks,
the `AnyLength -> Box` non-wildcard boundary, policy-row-ID-only Unit
consumption, Rust oracle veto, and zero runtime/backend/mutation/publication/
fallback authority.

On green, select 3452. Do not jump directly to Write, Delete, wide, or Source
Selfhost authority.
