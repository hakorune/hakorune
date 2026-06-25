Status: Complete
Date: 2026-06-25
Token: MIRBUILDER-PREPARED-STATE-RESERVED-MEMBERSHIP-TRANSPORT-ALIGNMENT-001

# Prepared-State Reserved Membership Transport Alignment

## Scope

Align the prepared-state `next_value_id` kernel's reserved membership transport
with its declared execution projection.

The projection and verifier already named `ValueIdOrderedMapBox`, but the
generated field was still rendered as `OrderedMapBox`. This card fixes the
actual generated substrate and strengthens the smoke to cover multi-entry
reserved membership.

## Landed Changes

```text
ReservedValueIdMembershipView.storage: ValueIdOrderedMapBox
ReservedValueIdMembershipView.birth:
  me.storage = ValueIdOrderedMap.create()

reserved present smoke:
  reserved = [2, 4]
  outputs = [1, 3, 5]
  final function counter = 6
```

The shared `NewValueIdOrderedMap` renderer now emits the ValueId-specific
constructor. Existing ValueIdOrderedMap consumers were regenerated and guarded.

## Evidence

```text
reserved_membership_transport=ValueIdOrderedMapBox
reserved_membership_initializer=ValueIdOrderedMap.create
metadata_context_value_caller green
ordered_map_crate_bundle green
```

## Non-Claims

```text
new semantic capability = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
```

## Next

Continue with:

```text
MIRBUILDER-MINIMAL-EXECUTION-PATH-SELECTION-001
```
