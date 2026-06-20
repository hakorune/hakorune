# VariableContext Returned Borrow Boundary Inventory

Status: SSOT
Scope: lifecycle boundary for `VariableContext::variable_map()` and
`VariableContext::variable_map_mut()`.

## Source

```text
crates/hakorune_mir_builder/src/variable_context.rs
```

## Boundary

The simple-map lifecycle pilot covers owner-local operations:

```text
new/default
lookup
contains
len
is_empty
insert
remove
```

It explicitly excludes returned map borrows:

```rust
pub fn variable_map(&self) -> &BTreeMap<String, ValueId>
pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>
```

These methods return aliases to the internal map. They are not equivalent to
local read/write methods and must not be projected as ordinary direct owner
access.

## Immutable Returned Map

```rust
pub fn variable_map(&self) -> &BTreeMap<String, ValueId>
```

Lifecycle issue:

```text
returned shared borrow
scope is caller-owned, not CallOnly
carrier-sensitive code may iterate deterministic map state
```

Current consumer families:

```text
read-only tests:
  crates/hakorune_mir_builder/src/variable_context.rs
    test_variable_map_access
    test_btree_deterministic_iteration

observation:
  src/mir/region/observer.rs
    classify_slots_from_variable_map(builder)

carrier-sensitive:
  src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
    CarrierInfo::from_variable_map(...)
    CarrierInfo::with_explicit_carriers(...)
```

Initial policy:

```text
do not emit naked borrowed alias
future candidate=OwnerCarryingBorrowView(read)
requires owner identity and non-escape proof
read-only tests and observation may be first probe candidates
carrier-sensitive consumers require a separate consumer contract
```

## Mutable Returned Map

```rust
pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>
```

Lifecycle issue:

```text
returned UniqueWrite borrow
caller can mutate the map outside the method boundary
mutation proof is not local to VariableContext
```

Initial policy:

```text
Deny(ReturnedMutableBorrow)
```

Current consumer status:

```text
external_callsite_count=0
reason_to_keep_boundary=public API still exposes caller-owned mutable alias
```

This remains denied until one of these replacement plans is selected:

```text
API-specific methods:
  add explicit VariableContext methods for the needed mutation

with-map operation:
  owner-controlled callback/bulk operation with bounded scope

ReplaceOwned:
  snapshot/restore-style owned transfer plan
```

## Why Converter/Emitter Cannot Decide This

The Rust-to-Hako converter/emitter renders verified lifecycle plans only. It
must not choose whether a returned `&BTreeMap` becomes a `BorrowView`, an
`OrderedMapBox`, a copied aggregate, or a denied projection.

Correct ownership split:

```text
rustc semantic adapter:
  emits Rust lifecycle facts

Hako lifecycle resolver:
  chooses lifecycle plan or Deny

verifier:
  checks escape, alias, cleanup, publication, and identity constraints

converter/emitter:
  renders verified HakoLifecyclePlan only
```

## Follow-Up Rows

```text
VARIABLE-CONTEXT-IMMUTABLE-MAP-BORROWVIEW-PROBE-001:
  read-only owner-carrying BorrowView candidate for test/observation style
  consumers, no carrier/PHI claim

VARIABLE-CONTEXT-MUTABLE-MAP-API-REPLACEMENT-SELECTION-001:
  choose explicit mutation APIs vs with-map operation vs Deny-only closeout

VARIABLE-CONTEXT-SNAPSHOT-RESTORE-OWNERSHIP-001:
  clone / ReplaceOwned / old-map cleanup plan

VARIABLE-CONTEXT-CARRIER-PHI-LIFECYCLE-INVENTORY-001:
  carrier-sensitive consumers after returned borrow boundary is named
```

## Stop Lines

```text
do not model returned &mut BTreeMap as direct owner mutation
do not create naked borrow aliases in generated Hako
do not infer carrier/PHI safety from deterministic iteration alone
do not let converter/emitter choose lifecycle representation
do not claim full VariableContext parity from simple-map plus borrow inventory
```
