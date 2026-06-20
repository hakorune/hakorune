# VariableContext Lifecycle Gap Inventory

Status: SSOT
Scope: MirBuilder `VariableContext` lifecycle migration gap inventory.

## Source

```text
crates/hakorune_mir_builder/src/variable_context.rs
```

## Safe BindingContext-Like Slice

These shapes are close to the closed BindingContext lifecycle pilot:

```text
variable_map: BTreeMap<String, ValueId>
  deterministic_order_required=true
  candidate plan=OrderedMapBox

new/default:
  LocalBox with OrderedMapBox field

lookup / contains / len / is_empty:
  SharedRead, escapes=false

insert / remove:
  UniqueWrite, escapes=false

ValueId:
  immediate/scalar-like ID value

memory Drop:
  TrivialMemory candidate
```

This slice can be the next facts/plan pilot only if returned map references and
snapshot/restore are excluded.

## Gaps Before Full VariableContext Projection

### Returned Immutable Map

```rust
pub fn variable_map(&self) -> &BTreeMap<String, ValueId>
```

Lifecycle issue:

```text
returned shared borrow
scope is not CallOnly
consumer may iterate carrier-sensitive map state
```

Initial policy:

```text
Deny or model as owner-carrying BorrowView
```

### Returned Mutable Map

```rust
pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>
```

Lifecycle issue:

```text
returned UniqueWrite borrow
caller can mutate map outside method boundary
direct owner mutation proof is not local to VariableContext
```

Initial policy:

```text
Deny(ReturnedMutableBorrow) until API-specific replacement is selected
```

### Snapshot Clone

```rust
pub fn snapshot(&self) -> BTreeMap<String, ValueId>
```

Lifecycle issue:

```text
owned collection clone
deterministic order must be preserved
clone ownership and Drop must be explicit
```

Initial policy:

```text
separate SnapshotOwnedMap plan or Deny in v0
```

### Restore Transfer

```rust
pub fn restore(&mut self, snapshot: BTreeMap<String, ValueId>)
```

Lifecycle issue:

```text
ReplaceOwned map transfer
old map cleanup and new owner transfer must be explicit
```

Initial policy:

```text
TransferOwned / ReplaceOwned plan required
```

### SSA Rename Overwrite

```rust
ctx.insert("x".to_string(), ValueId::new(2))
```

Lifecycle issue:

```text
same key overwrite
previous ValueId is TrivialMemory but overwrite semantics should be explicit
```

Initial policy:

```text
allowed in simple slice if ValueId DropFact=TrivialMemory
```

### Carrier-Sensitive Consumers

`VariableContext` is used by JoinIR carrier extraction and PHI planning.

Lifecycle issue:

```text
iteration order is semantically relevant
map state may be observed across loop/branch planning boundaries
```

Initial policy:

```text
inventory-only until VariableContext simple slice is green
```

## Selected Next Slice

```text
VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-PILOT-001
```

Included:

```text
new/default
lookup
contains
len
is_empty
insert
remove
deterministic iteration expectation
SSA overwrite with TrivialMemory ValueId
memory-only Drop erase with TrivialMemory
```

Excluded:

```text
variable_map()
variable_map_mut()
snapshot()
restore()
carrier extraction consumers
PHI planner integration
```

## Stop Lines

```text
do not model returned &mut BTreeMap as direct mutation
do not clone/restore maps without explicit ownership plan
do not claim carrier/PHI lifecycle parity from simple map slice
do not generalize BindingContext facts blindly
```
