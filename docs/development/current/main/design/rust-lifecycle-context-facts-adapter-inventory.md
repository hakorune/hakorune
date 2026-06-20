# Rust Lifecycle Context Facts Adapter Inventory

Status: Inventory
Scope: RustLifecycleFacts requirements for MirBuilder context migration.

## Purpose

Define what an external rustc semantic adapter must report before the
lifecycle-aware converter route can claim ownership / borrow / Drop parity for
the first MirBuilder context families:

```text
BindingContext
VariableContext
```

This document is not an adapter implementation plan. It is the fact boundary
the adapter must satisfy.

## Owner Split

```text
rustc_adapter_fact:
  Rust semantic evidence only

hako_lifecycle_plan:
  Hako representation / borrow / cleanup choice

verifier_check:
  condition that must be proven before emission

converter_rendering:
  rendering requirement only
```

The adapter must not choose Hako policy.

## BindingContext

### Source Shape

```text
owned struct:
  BindingContext

map field:
  BTreeMap<String, BindingId>

methods:
  new/default
  is_empty / lookup / contains / len
  insert / remove / clear
```

### Required Facts

```text
field=binding_map
rust_type=BTreeMap<String, BindingId>
deterministic_order_required=true
identity_observed=false
layout_observed=false
thread_atomic_observed=false
drop_class=TrivialMemory
```

Method receiver facts:

```text
&self read methods:
  borrow_kind=SharedRead
  borrow_escape=CallOnly
  mutation=false

&mut self mutation methods:
  borrow_kind=UniqueWrite
  borrow_escape=CallOnly
  mutation=true
```

Value facts:

```text
BindingId:
  copy_class=ImmediateValue
  drop_class=TrivialMemory
```

### Projection Ownership

```text
rustc_adapter_fact:
  deterministic_order_required=true
  borrow kind / escape
  BindingId copy/drop class
  map Drop=TrivialMemory

hako_lifecycle_plan:
  binding_map -> OrderedMapBox
  &self -> direct read or BorrowView(read)
  &mut self -> owner method mutation
  memory Drop -> erase

verifier_check:
  borrow does not escape method call
  no identity/layout/thread observation
  deterministic order is preserved by selected plan
  Drop erase is backed by TrivialMemory fact

converter_rendering:
  render selected OrderedMapBox operations only after verified plan
```

## VariableContext

### Source Shape

```text
owned struct:
  VariableContext

map field:
  BTreeMap<String, ValueId>
```

Already separated slices:

```text
simple map:
  lookup / contains / len / is_empty / insert / remove

immutable returned map:
  variable_map() -> &BTreeMap<String, ValueId>

mutable returned map:
  variable_map_mut() -> &mut BTreeMap<String, ValueId>

snapshot / restore:
  snapshot() -> owned BTreeMap
  restore(snapshot: BTreeMap)

carrier consumers:
  CarrierInfo::from_variable_map
  CarrierInfo::with_explicit_carriers
```

### Required Facts: Simple Map

```text
field=variable_map
rust_type=BTreeMap<String, ValueId>
deterministic_order_required=true
identity_observed=false
layout_observed=false
thread_atomic_observed=false
drop_class=TrivialMemory
```

Method receiver facts:

```text
read methods:
  borrow_kind=SharedRead
  borrow_escape=CallOnly

mutation methods:
  borrow_kind=UniqueWrite
  borrow_escape=CallOnly
```

Value facts:

```text
ValueId:
  copy_class=ImmediateValue
  drop_class=TrivialMemory
```

### Required Facts: Immutable Map Borrow

```text
method=variable_map
return_type=&BTreeMap<String, ValueId>
borrow_kind=SharedRead
borrow_escape=Returned
owner=VariableContext
mutation_allowed=false
```

This fact does not allow a naked alias. It only allows an owner-carrying
BorrowView candidate.

### Required Facts: Mutable Map Borrow

```text
method=variable_map_mut
return_type=&mut BTreeMap<String, ValueId>
borrow_kind=UniqueWrite
borrow_escape=Returned
```

Current policy:

```text
Hako plan:
  Deny(ReturnedMutableBorrow)
```

The adapter reports the fact. It does not choose a workaround.

### Required Facts: Snapshot / Restore

Snapshot:

```text
method=snapshot
return_type=BTreeMap<String, ValueId>
ownership=CloneOwnedMap
drop_class=TrivialMemory for values
deterministic_order_required=true
```

Restore:

```text
method=restore
argument_type=BTreeMap<String, ValueId>
ownership=ReplaceOwned
old_field_drop=TrivialMemory
new_field_owner=VariableContext
```

### Required Facts: Carrier Consumers

```text
consumer=CarrierInfo::from_variable_map
input=BorrowView(read)
mutation=false
requires_deterministic_order=true
```

```text
consumer=CarrierInfo::with_explicit_carriers
input=explicit carrier values
missing_carrier_policy=fail-fast
```

### Projection Ownership

```text
rustc_adapter_fact:
  BTreeMap deterministic order
  read/write receiver borrow escape
  returned immutable/mutable borrow escape
  snapshot CloneOwnedMap
  restore ReplaceOwned
  carrier consumer read-only requirements

hako_lifecycle_plan:
  simple map -> OrderedMapBox
  immutable returned map -> owner-carrying BorrowView(read)
  mutable returned map -> Deny(ReturnedMutableBorrow)
  snapshot -> CloneOwnedMap
  restore -> ReplaceOwned
  carrier snapshot -> CarrierSnapshotFromBorrowView

verifier_check:
  returned immutable borrow keeps owner identity
  returned mutable borrow is not emitted as naked alias
  clone/replace ownership is explicit
  carrier consumer receives read-only view or explicit carriers

converter_rendering:
  render only plan-approved surfaces
  emit deny/TODO on skeleton route without lifecycle parity claim
```

## Adapter Output Minimum

The first adapter fixture should be able to emit a compact fact bundle like:

```text
context_family=BindingContext|VariableContext
field_map_kind=BTreeMap
key_type=String
value_type=BindingId|ValueId
deterministic_order_required=1
borrow_escape=CallOnly|Returned
borrow_kind=SharedRead|UniqueWrite
ownership_effect=None|CloneOwnedMap|ReplaceOwned
drop_class=TrivialMemory
identity_observed=0
thread_atomic_observed=0
```

This is still target-neutral. It contains no `OrderedMapBox` spelling.

## Stop Lines

```text
do not let adapter output say OrderedMapBox
do not infer Drop erase from missing Drop data
do not emit returned &mut map as a naked alias
do not make snapshot/restore ordinary assignment
do not claim full VariableContext parity from simple map facts
do not change converter core in an inventory row
do not start rustc toolchain integration in this row
```

## Next Candidate Tasks

```text
A. RUST-LIFECYCLE-FACTS-ADAPTER-BINDING-CONTEXT-FIXTURE-001
   first compact facts fixture for BindingContext only

B. HAKO-LIFECYCLE-VERIFIER-CONTEXT-FACTS-FIXTURE-001
   consume checked-in facts/plan fixtures and report Allow/Deny

C. RUST-LIFECYCLE-FACTS-ADAPTER-VARIABLE-CONTEXT-FIXTURE-001
   VariableContext simple map and returned borrow facts
```
