# 296x-870 MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-VALIDATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-scalar-load-i64-borrowed-lookup-validation-v0
source_evidence=296x-869
row_kind=validation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implementation_guard_green=1
design_guard_green=1
cargo_fmt_check_green=1
cargo_check_release_bin_hakorune_green=1
current_state_pointer_guard_green=1
git_diff_check_green=1

validated_shape=scalar_helper_borrowed_lookup
validated_helper=nyash.map.scalar_load_hi
mapbox_storage_change_enabled=0
i64_sidecar_storage_enabled=0
mapbox_public_get_contract_changed=0
mapbox_public_set_contract_changed=0
slot_load_hi_changed=0
slot_load_hh_changed=0
runtime_data_get_route_change_enabled=0
product_default_changed=0
winner_claim=0
selected_next=MIMALLOC-MAP-SCALAR-LOAD-I64-BORROWED-LOOKUP-MEASUREMENT-001
summary=ok
```

## Commands

```bash
bash tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_implementation_guard.sh
bash tools/checks/k2_wide_phase296x_map_scalar_load_i64_borrowed_lookup_design_guard.sh
cargo fmt --check
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Decision

The narrow implementation is structurally valid. The next row must measure the
same Hako-only front and compare hot symbols before any keeper claim.

## Stop Lines

- do not claim a winner from validation
- do not change storage or public semantics in validation
- do not continue route proof work without fresh route evidence

