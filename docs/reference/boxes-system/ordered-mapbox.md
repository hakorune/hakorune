# OrderedMapBox Reference

Status: live reference for v0.

`OrderedMapBox` is a deterministic ordered key/value collection for `.hako`
apps and compiler-construction support code. It is not a replacement for
`MapBox`, and it is not part of the ring0 substrate in v0.

## Purpose

Use `OrderedMapBox` when output stability depends on key order.

Accepted first users:

```text
compiler app utilities
MirBuilder migration probes
BindingContext-like maps
stable golden-output helpers
```

The primary use case is the Rust `BTreeMap<String, T>` style requirement:

```text
same input
  -> same key order
  -> same emitted skeleton / MIR / diagnostics
```

## Placement

v0 implementation location:

```text
apps/lib/collections/ordered_map.hako
```

v0 is a normal `.hako` library box. It is intentionally not ring0.

Promotion gates:

```text
ring1:
  allowed only after at least two real users need the shared provider shape
  and the `.hako` library API is stable

ring0:
  not allowed for v0
  requires a separate substrate capability card
```

## Key Domain

v0 key domain:

```text
String keys only
```

The collection does not inherit `MapBox` key canonicalization. In particular,
it does not claim dynamic `1 == "1"` behavior.

Non-string keys are outside v0. Callers must normalize keys before storing.

## Ordering

Keys are ordered by deterministic byte/string lexical order.

No locale, Unicode collation, or normalization is applied in v0.

## API

The v0 API is intentionally small.

```hako
box OrderedMapBox {
    set(key: StringBox, value)
    get(key: StringBox)
    has(key: StringBox)
    remove(key: StringBox)
    clear()
    length()
    keys()
    key_at(index: i64): StringBox
    values()
}
```

Semantics:

```text
set:
  insert if absent
  update if present
  must not create duplicate keys

get:
  returns the stored value
  returns null when missing

has:
  returns true when the key exists
  returns false when missing

remove:
  returns the removed value when the key exists
  returns null when missing or when the key is null
  removes exactly one key/value pair
  preserves deterministic order and key/value alignment for remaining entries

clear:
  removes all entries
  keeps the instance reusable by later set/get calls

length:
  returns the number of entries

keys:
  returns a snapshot ArrayBox of keys in deterministic order

key_at:
  returns the key at a deterministic ordered index
  returns "" when out of range
  exists so EXE/AOT tests can verify key order without relying on ArrayBox
  element-type recovery

values:
  returns a snapshot ArrayBox of values in key order
```

The v0 API does not include range queries, mutable iterators, or lower-bound
search. Add those only when a real caller needs them.

## Implementation Shape

The v0 implementation may be simple.

```text
storage:
  ArrayBox keys
  ArrayBox values

lookup:
  linear search

insert:
  find sorted insertion position
  insert key/value at that position

update:
  replace value at existing key index
```

This is acceptable because v0 optimizes determinism and simple semantics, not
large-map performance.

Do not route this through `MapBox` and then sort on every read unless a later
card proves that shape is simpler and still deterministic. The reference
contract is ordered storage, not dynamic-map behavior.

v0 does not coerce keys. Callers pass String keys directly. This avoids mixing
OrderedMapBox with MapBox key-publication/canonicalization semantics.

## Construction State

`OrderedMapBox` owns two per-instance ArrayBox slots:

```hako
keys_value: ArrayBox
values_value: ArrayBox
```

The canonical language contract permits these defaults to be expressed as
declaration-site stored field initializers:

```hako
keys_value: ArrayBox = new ArrayBox()
values_value: ArrayBox = new ArrayBox()
```

Stored field initializers are evaluated for each `new`, before `birth`, and
must not create shared mutable defaults between instances.

Current v0 implementation keeps initialization inside `OrderedMap.create()`:

```hako
local map = new OrderedMapBox()
map.keys_value = new ArrayBox()
map.values_value = new ArrayBox()
return map
```

This is a route-compatibility choice, not a different semantic contract.
Do not mechanically move meaningful `birth(args...)` logic into field
initializers. Use field initializers only for simple per-instance defaults that
do not depend on constructor arguments and have no external side effects.

## Non-Goals

```text
not a MapBox replacement
not a generic ordered container for arbitrary key types
not a Rust BTreeMap API clone
not a ring0 primitive
not a runtime performance optimization
not a crate-wide symbol resolver
not a route for changing MapBox semantics
```

## Required Tests

The first implementation must include a small smoke or fixture that proves:

```text
insert_b_sorts_after_a=1
insert_a_then_b_keys_are_a_b=1
insert_b_then_a_keys_are_a_b=1
update_existing_key_no_duplicate=1
get_missing_returns_null=1
remove_existing_returns_value=1
remove_missing_returns_null=1
remove_preserves_order_and_value_alignment=1
clear_resets_entries=1
reusable_after_clear=1
values_follow_key_order=1
length_updates_after_insert_only=1
key_at_reports_ordered_string_keys=1
fresh_per_instance_arrays=1
```

The constructor lifecycle itself is guarded outside OrderedMapBox by a focused
probe that checks:

```text
field_initializer_runs_before_birth=1
birth_runs_once=1
constructor_args_reach_birth=1
birth_arrays_are_fresh_per_instance=1
```

## Related Design

- `docs/development/current/main/design/ordered-map-box-boundary-ssot.md`
- `docs/development/current/main/design/ring1-core-provider-scope-ssot.md`
- `docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md`
