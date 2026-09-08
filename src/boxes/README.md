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

## Checked Map storage

`map_box_table.rs` owns the common key/payload table. Native MapBox stores only
NyashBox values; `map_box_checked.rs` has a separate non-NyashBox, non-Clone
facade for source-authorized canonical residences. No native-visible Map can
be promoted to contain those residences. Each facade has one payload table.

Checked install reserves capacity before commit and returns the original
candidate on refusal. Replacement returns a detached outcome that must be ended,
including the no-old case. Terminal end rejects new admission before callbacks,
then consumes live entries in reverse successful-install order outside locks.
An empty end buffer reserves capacity during install, holds no duplicate payload,
and receives the drained entries at end so teardown can sort without allocation.
Finite first/suppressed failures preserve best-effort cleanup; suppressed_count
is the total count and may exceed the eight stored diagnostic slots.

The runtime placement caller must end acquired Maps and detached outcomes;
ordinary Rust Drop does not perform source finalization. `require_disposable`
checks Map state before destruction. The opaque ABI must enforce this protocol
and reject Ready-outcome disposal; kernel opaque wrappers now enforce it.
Native projection of a present owned entry explicitly refuses, distinct from a
missing key. MapKeyDomain has fallible native text preparation for the key-before-child ABI;
its canonical numeric classifier allocates no temporary String. Source read/escape
authority and C Map operation emission/source cutover remain open. See the
[checked storage contract](../../docs/reference/runtime/runtime-data-dispatch.md#checked-map-storage-and-indexed-residence).

## JSON observation

GC uses `MapBox::native_trace_children` rather than accessing the raw table.
The owner retains native child clone semantics and rejects storage failure;
callbacks run after child projection and unlocking. The controller and kernel
metrics preserve incomplete observation. JSON and GC use scoped Map-owner
access; the public raw-table accessor is removed. Neither path authorizes
owned-entry intake.

`JSONBox::set` converts native input into an owned JSON tree by borrowing values.
Array/Map traversal does not clone or share stored children; fallback string
conversion observes the stored object itself. The consumed top-level input is
disposed before destination locking and commit. The existing public Rust setter
returns `Result<Box<dyn NyashBox>, JsonSetError>`: source Map storage failure,
destination storage failure and non-object destinations have distinct errors.
Success retains the `ok` Box; source failure takes priority over destination
failure. Output owns its strings/nodes. This does not change collection traversal
locking, cycle handling, or authorize projection
of future owned native Map residences. See the
[runtime observation contract](../../docs/reference/runtime/runtime-data-dispatch.md#native-json-observation).
