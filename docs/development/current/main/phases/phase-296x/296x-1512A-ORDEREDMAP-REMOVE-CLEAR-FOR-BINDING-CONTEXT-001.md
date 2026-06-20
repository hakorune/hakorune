# 296x-1512A ORDEREDMAP-REMOVE-CLEAR-FOR-BINDING-CONTEXT-001

Status: closed
Date: 2026-06-20

## Purpose

Close the `OrderedMapBox` API gap that blocks the BindingContext behavioral
derived-artifact pilot.

`BindingContext` is backed by Rust `BTreeMap<String, BindingId>` and its target
behavior includes:

```text
remove(name)
clear_for_function_entry()
```

The current `.hako` `OrderedMapBox` v0 surface has deterministic `set/get/has`
and ordered snapshots, but no `remove` or `clear`. Those operations are
collection-library responsibilities, not converter/emitter responsibilities.

## Blocks

```text
296x-1512-BINDING-CONTEXT-DERIVED-HAKO-ARTIFACT-PILOT-001
```

## Scope

Allowed:

```text
OrderedMapBox.remove(key)
OrderedMapBox.clear()
OrderedMapBox reference documentation update
OrderedMapBox focused EXE/AOT smoke coverage
BindingContext task docs prerequisite update
```

Forbidden:

```text
BindingContext generated artifact emission
HakoBehaviorRecipe implementation
converter/emitter workaround for missing collection API
MapBox semantics change
ring0 / provider registration
crate-wide MirBuilder claim
selfhost mainline route selection
```

## API Contract

```text
remove(key):
  returns the removed value when the key exists
  returns null when the key is missing or null
  removes exactly one key/value pair
  preserves deterministic lexical order of remaining keys
  keeps keys and values arrays length-aligned

clear():
  removes all entries
  length becomes 0
  keys() and values() return empty snapshots
  the instance remains reusable by later set/get calls
```

## Acceptance

```text
ordered_map_remove_existing_returns_value=1
ordered_map_remove_missing_returns_null=1
ordered_map_remove_decrements_length=1
ordered_map_remove_preserves_remaining_key_order=1
ordered_map_remove_keeps_values_aligned=1
ordered_map_clear_resets_length=1
ordered_map_clear_resets_keys_and_values=1
ordered_map_reusable_after_clear=1
ordered_map_fresh_per_instance_arrays=1
binding_context_1512_unblocked=1
backend_behavior_changed=0
```

Checks:

```bash
bash apps/lib/collections/smoke_ordered_map.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Result:

```text
ordered_map_remove_existing_returns_value=1
ordered_map_remove_missing_returns_null=1
ordered_map_remove_decrements_length=1
ordered_map_remove_preserves_remaining_key_order=1
ordered_map_remove_keeps_values_aligned=1
ordered_map_clear_resets_length=1
ordered_map_clear_resets_keys_and_values=1
ordered_map_reusable_after_clear=1
ordered_map_fresh_per_instance_arrays=1
binding_context_1512_unblocked=1
summary=ok
```

Executed:

```bash
bash apps/lib/collections/smoke_ordered_map.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Note:

```text
OrderedMapBox.remove/clear are implemented as library-owned behavior. The
implementation rebuilds internal ArrayBox storage instead of depending on
ArrayBox.remove/clear, because those ArrayBox calls are not part of the active
EXE pure-route surface used by this smoke.
```

## Stop Line

```text
do_not_implement_BindingContext_artifact_in_this_row=1
do_not_hide_remove_clear_mapping_inside_emitter=1
do_not_change_MapBox_semantics=1
do_not_promote_OrderedMapBox_to_ring0=1
do_not_add_general_BTreeMap_clone_API=1
do_not_claim_performance_optimization=1
```
