# VariableContext Carrier/PHI Lifecycle Inventory

Status: SSOT
Scope: carrier-sensitive consumers of `VariableContext.variable_map`.

## Source Consumers

```text
src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
src/mir/region/observer.rs
```

## Existing Proven Boundaries

Already green:

```text
VariableContext simple map:
  OrderedMapBox-owned local map operations

VariableContext immutable map borrow:
  owner-carrying read BorrowView for test/observation consumers

VariableContext snapshot/restore:
  CloneOwnedMap and ReplaceOwned transfer

VariableContext mutable map:
  Deny(ReturnedMutableBorrow)
```

## CarrierInfo::from_variable_map

```rust
pub fn from_variable_map(
    loop_var_name: String,
    variable_map: &BTreeMap<String, ValueId>,
) -> Result<Self, String>
```

Lifecycle role:

```text
read deterministic map
find loop_var ValueId
derive all non-loop variables as carriers
copy ValueId into CarrierVar.host_id
clone carrier names into CarrierVar.name
sort carriers deterministically
```

Boundary:

```text
candidate future plan=CarrierSnapshotFromBorrowView
requires owner-carrying read BorrowView
requires deterministic order
requires ValueId TrivialMemory
does not mutate VariableContext
does not publish VariableContext map
```

## CarrierInfo::with_explicit_carriers

```rust
pub fn with_explicit_carriers(
    loop_var_name: String,
    loop_var_id: ValueId,
    carrier_names: Vec<String>,
    variable_map: &BTreeMap<String, ValueId>,
) -> Result<Self, String>
```

Lifecycle role:

```text
read deterministic map by requested carrier names
copy ValueId into CarrierVar.host_id
clone carrier names into CarrierVar.name
sort carriers deterministically
```

Boundary:

```text
candidate future plan=ExplicitCarrierSnapshotFromBorrowView
requires owner-carrying read BorrowView
requires requested names are owned inputs
missing carrier remains fail-fast
```

## Region Observer

```rust
fn classify_slots_from_variable_map(builder: &MirBuilder) -> Vec<SlotMetadata>
```

Lifecycle role:

```text
read deterministic map for observation
copy slot names into SlotMetadata
copy ValueId for type lookup
does not mutate VariableContext
does not publish VariableContext map
```

Boundary:

```text
already closest to immutable BorrowView observation
no carrier/PHI semantics claimed
```

## PHI Lifecycle Questions

Still open:

```text
Does CarrierInfo output own all copied carrier names?
Does CarrierInfo output copy ValueId as ImmediateValue only?
Where is join_id assigned, and is that assignment a separate lifecycle owner?
Do promoted_body_locals and trim_helper introduce additional owned data?
Which PHI planner consumers mutate carrier state after extraction?
```

Initial policy:

```text
inventory-only
no HakoLifecyclePlan for carrier/PHI yet
no resolver consumption yet
```

## Follow-Up Rows

```text
VARIABLE-CONTEXT-CARRIER-SNAPSHOT-PLAN-PROBE-001:
  fixture CarrierInfo::from_variable_map as a snapshot from BorrowView

VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-PROBE-001:
  fixture with_explicit_carriers separately

PHI-CARRIER-LIFECYCLE-CONSUMER-INVENTORY-001:
  inspect join_id / promoted body local / trim helper consumers

HAKO-LIFECYCLE-RESOLVER-READONLY-SKELETON-001:
  only after carrier-sensitive contracts are named
```

## Stop Lines

```text
do not treat deterministic iteration as PHI lifecycle safety
do not mutate VariableContext through carrier extraction
do not start a general resolver from carrier inventory
do not claim full VariableContext parity
```
