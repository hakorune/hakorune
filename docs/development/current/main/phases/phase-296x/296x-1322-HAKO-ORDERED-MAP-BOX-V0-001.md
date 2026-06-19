# 296x-1322 HAKO-ORDERED-MAP-BOX-V0-001

Status: closed
Date: 2026-06-20

## Purpose

Implement the first `.hako` OrderedMapBox library after the boundary SSOT.

This row proves a small deterministic String-key map without changing MapBox,
ring0, ring1 provider registration, or MirBuilder.

## Implementation

```text
apps/lib/collections/ordered_map.hako
  OrderedMap.create()
  OrderedMapBox.set/get/has/length/keys/key_at/values
  ArrayBox-backed ordered storage
  String-key-only v0

apps/lib/collections/tests/ordered_map_smoke.hako
  focused EXE/AOT smoke

apps/lib/collections/smoke_ordered_map.sh
  build/run/diff wrapper
```

## Accepted Scope

```text
ordered_map_v0_enabled=1
owner=.hako_library
location=apps/lib/collections/ordered_map.hako
key_domain=String
key_coercion_enabled=0
ordering=deterministic_lexical
mapbox_semantics_changed=0
ring0_enabled=0
ring1_provider_enabled=0
mirbuilder_rewrite_enabled=0
```

`key_at(index): StringBox` is included as a typed observer for EXE/AOT
acceptance. `keys()` remains part of the API, but the smoke uses `key_at`
for key-order assertions because `keys().get(i)` can lose element type
information through the current AOT ArrayBox route.

## Smoke Coverage

```text
insert_b_then_a_sorts_to_a_b=1
insert_a_then_b_then_c_keys_are_a_b_c=1
update_existing_key_no_duplicate=1
get_missing_returns_null=1
has_existing_and_missing=1
values_follow_key_order=1
length_updates_after_insert_only=1
```

## Stop Line

```text
do not change MapBox
do not add ring0 substrate support
do not add ring1 provider registration
do not rewrite MirBuilder
do not claim Rust BTreeMap parity
do not add arbitrary key domains
```

## Evidence

```bash
bash apps/lib/collections/smoke_ordered_map.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001
```

Use OrderedMapBox in a focused BindingContext-style probe without rewriting the
MirBuilder owner.
