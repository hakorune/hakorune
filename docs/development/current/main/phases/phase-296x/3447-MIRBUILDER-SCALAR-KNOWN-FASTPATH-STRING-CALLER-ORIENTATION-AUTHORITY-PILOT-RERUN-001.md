# 3447 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-AUTHORITY-PILOT-RERUN-001

## Result

The String caller-orientation authority pilot reran after implementation with
the exact three generated rows and the policy-row-ID-only Unit boundary.

```text
string_caller_orientation_authority_pilot = 1
string_caller_orientation_authority_scope = policy_row_id_contract_only
string_caller_orientation_consumer_unit_only = 1
string_exact_three_row_scope = 1
string_hako_route_decision_authority_retained = 1
string_rust_oracle_compat_checker_retained = 1
string_mismatch_fail_fast = 1
no_new_route_authority = 1
```

The live route consumer still performs the existing `.hako` route decision and
Rust-oracle comparison. Caller orientation only validates generated contract
metadata and cannot select a route or enter runtime/backend/mutation/
publication paths.

## Verification

```text
cargo test -q caller_orientation       # 22 passed
cargo test -q scalar_known_hako_shadow  # 19 passed
python3 tools/rust_lifecycle/mirbuilder_scalar_known_fastpath_string_caller_orientation_authority_pilot.py --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```

## Next Stop

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-POST-STRING-CALLER-ORIENTATION-PILOT-DESIGN-CONSULTATION-001
```

Do not promote Collection, Write, Delete, ScalarKnown-wide, runtime/backend,
or Source Selfhost authority from this rerun.
