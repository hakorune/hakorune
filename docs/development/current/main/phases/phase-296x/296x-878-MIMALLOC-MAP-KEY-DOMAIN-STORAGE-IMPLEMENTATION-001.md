# 296x-878 MIMALLOC-MAP-KEY-DOMAIN-STORAGE-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-mimalloc-map-key-domain-storage-implementation-v0
source_evidence=296x-877
row_kind=implementation
target_front=kilo_leaf_map_get_dynamic_covered_i64

implemented_storage_shape=HashMap<MapKeyDomain, Box<dyn NyashBox>>
mapbox_set_normalizes_key_domain=1
mapbox_get_normalizes_key_domain=1
mapbox_has_normalizes_key_domain=1
mapbox_delete_normalizes_key_domain=1
mapbox_keys_uses_public_text=1
mapbox_values_order_uses_public_text_sort=1
mapbox_json_uses_public_text=1

i64_text_alias_test_present=1
noncanonical_text_preservation_test_present=1
public_keys_text_output_test_present=1

scalar_load_hi_consumes_map_key_domain=0
slot_load_hi_consumes_map_key_domain=0
slot_load_hh_consumes_map_key_domain=0
kernel_scalar_helper_route_changed=0
i64_sidecar_storage_enabled=0
hashmap_hasher_swap_enabled=0
product_default_changed=0
winner_claim=0
summary=ok
```

## Implementation

`MapBox` storage now uses the core key-domain owner:

```rust
HashMap<MapKeyDomain, Box<dyn NyashBox>>
```

Public and raw string-key entry points normalize through
`MapKeyDomain::from_text`. Public key output uses `public_text()`:

```text
set/get/has/delete/raw helpers:
  MapKeyDomain::from_text

keys/values/json/debug:
  MapKeyDomain::public_text
```

This preserves the public alias between integer key `1` and text key `"1"`,
while keeping non-canonical numeric-looking text such as `"01"` in the text
domain.

## Tests

The implementation adds semantic fixtures in `src/boxes/map_box.rs`:

```text
test_key_domain_i64_text_alias
test_key_domain_noncanonical_text_preserved
test_keys_public_text_after_key_domain_storage
```

## Not Included

The generated scalar helper routes still do not consume `MapKeyDomain`
directly:

```text
nyash.map.scalar_load_hi
nyash.map.slot_load_hi
nyash.map.slot_load_hh
```

Those remain separate consumer rows. This row does not add sidecar storage,
does not change the hasher, and does not change MIRBuilder / route proof / C
shim routing.

## Validation

```bash
bash tools/checks/k2_wide_phase296x_map_key_domain_storage_implementation_guard.sh
cargo fmt --check
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
selected_next=MIMALLOC-MAP-KEY-DOMAIN-STORAGE-VALIDATION-001
```

The next row should validate public semantics and then decide whether a scalar
helper consumer row is still useful after storage normalization.
