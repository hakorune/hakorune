# 296x-879 MIMALLOC-MAP-KEY-DOMAIN-STORAGE-VALIDATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-storage-validation-v0
source_evidence=296x-878
row_kind=validation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implementation_guard_passed=1
cargo_fmt_check_passed=1
cargo_check_release_bin_hakorune_passed=1
current_state_pointer_guard_passed=1
git_diff_check_passed=1
map_key_domain_unit_tests_passed=1
mapbox_i64_text_alias_tests_passed=1
mapbox_public_key_text_test_passed=1

scalar_load_hi_consumes_map_key_domain=0
slot_load_hi_consumes_map_key_domain=0
slot_load_hh_consumes_map_key_domain=0
kernel_scalar_helper_route_changed=0
product_default_changed=0
winner_claim=0
summary=ok
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_map_key_domain_storage_implementation_guard.sh
cargo fmt --check
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
cargo test --lib test_key_domain -- --nocapture
cargo test --lib test_keys_public_text_after_key_domain_storage -- --nocapture
cargo test --lib map_key_domain -- --nocapture
```

## Result

The public MapBox semantics are green after the storage shift:

```text
integer key 1 aliases canonical text key "1"
non-canonical text key "01" remains a separate text domain
keys() exposes public text keys
JSON object conversion exposes public text keys
```

## Not Claimed

This row does not claim performance improvement. It also does not connect
kernel scalar helper routes directly to `MapKeyDomain`; those routes still pass
through the existing string-key helper surface.

## Next

```text
selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-MEASUREMENT-001
```

Measure before selecting any scalar-helper consumer or route implementation
row.
