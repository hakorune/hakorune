# 3451 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001

## Result

The Collection four-row caller-orientation authority pilot reran green with
the exact policy-row-ID-only Unit boundary and explicit mixed-domain checks.

## Required Rerun

```text
cargo test -q caller_orientation       # 23 passed
cargo test -q scalar_known_hako_shadow  # 19 passed
python3 tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_collection_caller_orientation_authority_pilot.py --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```

The rerun must prove exact four-row freshness, explicit receiver-domain checks,
the `AnyLength -> Box` non-wildcard boundary, policy-row-ID-only Unit
consumption, Rust oracle veto, and zero runtime/backend/mutation/publication/
fallback authority.

Do not promote Write, Delete, wide, runtime/backend, or Source Selfhost
authority from this rerun. 3452 is now the next design consultation stop.
