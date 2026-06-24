---
Status: Selected
Date: 2026-06-24
Scope: Promote the first owned read-fold consumer into direct lowering.
---

# BORROW-READ-FOLD-OWNED-MAP-MERGE-001

## Decision

Select the first `borrow.read_fold` direct-shape rule.

The primary live consumer is `MirBuilder::finalize_module` merging
`metadata_ctx.value_origin_callers()` into an owned map. `finalize_function`
has the same source shape but remains parity evidence until the first consumer
is green.

## Source Shape

```text
let mut origin_callers = function.metadata.value_origin_callers.clone();

for (k, v) in self.metadata_ctx.value_origin_callers().iter() {
    origin_callers.insert(*k, v.clone());
}

function.metadata.value_origin_callers = origin_callers;
```

Facts:

```text
source field:
  HashMap<ValueId, String>

destination field:
  BTreeMap<ValueId, String>

primary source:
  module_lifecycle::finalize_module

parity-only source:
  calls::lowering::finalize_function
```

## Normalized Semantics

```text
StorageAccessFacts:
  carrier = SharedHandle
  access = Read
  alias = Shared
  escape = None
  order = Unobserved
  cleanup = Trivial

FoldSemantics:
  input = MapEntries
  key_projection = Copy(ValueIdAsI64)
  value_projection = OwnedImmutableAtom
  base = CloneOwned
  collision = SourceWins
  output = OwnedOrderedMap
  output_order = KeyAscending(ValueIdOrdV1)
```

Physical transport:

```text
source_storage = ValueIdOrderedMapBox
target_storage = ValueIdOrderedMapBox
```

`OrderedMapBox` remains String-key only and is not used for this ValueId-key
fold.

Required proof:

```text
source_destination_alias = false
source_mutated_during_use = false
element_reference_escapes = false
destination_identity_observed = false
collision_policy = SourceWins
```

## Lowering

```text
BorrowUseFacts
  -> StorageAccessFacts
  -> ElideToReadFold
  -> borrow.read_fold
  -> typed operation IR
```

Operation vocabulary:

```text
CloneOwnedMap(target optional)
ForEachMapEntry
MapSet
ReturnSource
```

`ForEachMapEntry` is the only new semantic operation in this slice.

## Acceptance

```text
DIRECT_SHAPE_RULES["borrow.read_fold"] exists
decision = ElideToReadFold
new operation kind = ForEachMapEntry only
family-name branch = 0
Rust spelling branch in emitter = 0
MapReadFoldOwnedCopy main-operation usage = 0
new backend route = 0
new ABI = 0
runtime fallback = 0
full finalize_module conversion claim = 0
```

Oracle:

```text
base:
  1 -> "base"
  7 -> "old"

source:
  2 -> "source"
  7 -> "new"

merged:
  1 -> "base"
  2 -> "source"
  7 -> "new"

original base remains unchanged
source remains unchanged
source mutation after fold does not mutate merged
merged keys iterate as 1, 2, 7
```

Negative acceptance:

```text
source mutated during fold
  -> Deny(ReturnedReadBorrow), detail=OwnerMutationDuringBorrow

element reference escapes
  -> Deny(ReturnedReadBorrow), detail=ElementReferenceEscapes

owned value projection unavailable
  -> Deny(CarrierSensitiveAlias), detail=OwnedValueProjectionUnavailable

source and destination may alias
  -> Deny(CarrierSensitiveAlias), detail=SourceDestinationAlias

collision policy missing
  -> Deny(DefaultSemanticMismatch), detail=CollisionPolicyMissing

ordered output without comparator proof
  -> Deny(UnsupportedOrderCapability)
```

## Gates

```text
metadata value-caller generator --check green
metadata value-caller MIR green
metadata value-caller EXE/AOT green
rust_mirbuilder_converter_matrix_guard green
rust_lifecycle_no_silent_hardcode_guard green
current_state_pointer_guard green
```

## Parked

```text
value_origin_callers().get(...).cloned()
  parked; existing leaf projection rule already covers this class

second live read-fold consumer
  parked until first consumer green

crate-level partial bundle
  parked until semantic shape coverage reaches an adoption checkpoint

returned mutable borrow / Drop / unsafe / FFI
  parked while read-only owned projection lane remains open
```
