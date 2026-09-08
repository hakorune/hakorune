# Native boxes

This directory owns native value/container implementations. Source capability
and lifecycle authority remains in the compiler and language reference.
Array-specific implementation rules are in [array/README.md](array/README.md).

## Map replacement teardown

`MapBox::insert_key_str` is the shared insertion owner. It commits replacement
under its write lock and detaches the previous Box. The previous Box is dropped
only after unlocking, so native teardown can re-enter the Map and observe the
new value. Missing-key insertion has no displaced value to drop.

This is native replacement behavior, not source Home transfer or user `fini`.
Remove/clear and visible read/clone have separate migration work; this fix does
not certify all Map teardown paths. See the
[runtime contract](../../docs/reference/runtime/runtime-data-dispatch.md#map-replacement-native-teardown)
and [owned-slot target](../../docs/reference/language/ownership.md#intrinsic-map-slot-destination-target).
