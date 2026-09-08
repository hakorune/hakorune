# Native boxes

This directory owns native value/container implementations. Source capability
and lifecycle authority remains in the compiler and language reference.
Array-specific implementation rules are in [array/README.md](array/README.md).

## Map native teardown

`MapBox::insert_key_str` is the shared insertion owner. It commits replacement
under its write lock and detaches the previous Box. The previous Box is dropped
only after unlocking, so native teardown can re-enter the Map and observe the
new value. Missing-key insertion has no displaced value to drop.
`remove_key_str` likewise detaches before dropping and preserves its presence
Bool. `clear_entries` drains native values while locked, preserving capacity,
then drops them after unlocking. Teardown starts after the empty-state commit;
later reentrant mutations are separate operations, not part of that drain.

These shared mutation paths provide native teardown, not source Home transfer
or user `fini`. Ordered terminal end and visible read/clone migration remain
open. See the
[runtime contract](../../docs/reference/runtime/runtime-data-dispatch.md#map-replacement-native-teardown)
and [owned-slot target](../../docs/reference/language/ownership.md#intrinsic-map-slot-destination-target).

## JSON observation

`JSONBox::set` converts native input into an owned JSON tree by borrowing values.
Array/Map traversal does not clone or share stored children; fallback string
conversion observes the stored object itself. The consumed top-level input is
still disposed at the old conversion boundary. Output owns its strings/nodes.
This does not change collection locking, cycle handling, or authorize projection
of future owned native Map residences. See the
[runtime observation contract](../../docs/reference/runtime/runtime-data-dispatch.md#native-json-observation).
