---
Status: Active
Date: 2026-05-30
Scope: task roadmap for extending Array residence lanes without turning ArrayBox into the performance substrate.
Related:
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md
  - docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md
  - docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md
  - docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md
  - crates/nyash_kernel/src/plugin/value_demand.rs
  - crates/nyash_kernel/src/plugin/value_lane.rs
---

# Array Lane Extension Roadmap SSOT

## Purpose

Keep future Array expansion ordered and narrow.

Future Array work must first decide which layer owns the behavior:

```text
Layer 1: public ArrayBox identity
Layer 2: internal residence lane
Layer 3: ABI / plugin / type-specific transport
```

## Decision

The proposed direction is correct with one current-state correction:

- `TextLane / ArrayStorage::Text` is not future-only; the minimal
  runtime-private text lane has already landed in phase-137x.
- the runtime-private `DemandSet -> ValueLanePlan -> executor action` bridge
  has also landed for the array text residence route.

Therefore the next Array work is not "open TextLane from scratch". It is:

```text
1. keep ArrayBox as public identity
2. keep current scalar/text residences internal
3. add future lanes only through demand/fact rows
4. keep plugin objects boxed/handled first
5. defer record/union inline layout until explicit evidence exists
```

## Layer 1: Public ArrayBox Identity

`ArrayBox` remains the public semantic container.

Responsibilities:

- public identity
- public `get` / `set` / `push` / `len` behavior
- mixed and dynamic values
- fallback, debug, materialization, observer boundaries
- public ABI compatibility

Non-responsibilities:

- long-term performance substrate
- C-like direct array storage ABI
- plugin private cache ABI
- direct pointer interpretation of public handles

Rules:

```text
nyash.array.birth_h remains public ArrayBox birth.
Public ArrayBox handles must not be reinterpreted as DirectArray handles.
Public ABI widening requires a separate explicit row.
```

## Layer 2: Internal Residence Lane

Internal residence may keep values unboxed while public behavior stays
unchanged.

Current truth:

```text
ArrayStorage::Boxed
ArrayStorage::InlineI64
ArrayStorage::InlineBool
ArrayStorage::InlineF64
ArrayStorage::Text
DirectArrayI64BufferV0 as separate DirectArray-family substrate
```

Boundary rule:

```text
internal residence:
  immediate/cell/direct storage

public boundary:
  materialize or promote to public object form

public world:
  NyashBox / ArrayBox-visible values
```

Scalar lane readback truth:

```text
InlineI64:
  has direct typed encoded-load readback

InlineBool / InlineF64:
  existing residence exists
  typed readback route must be selected by an explicit future row
  do not imply public ABI widening
```

Text lane truth:

```text
ArrayStorage::Text:
  landed as runtime-private text residence
  storage/residence only
  not semantic truth
  generic/mixed routes degrade to Boxed
```

DirectArray truth:

```text
DirectArrayI64BufferV0:
  exact i64 hot substrate
  separate from public ArrayBox identity
  selected only from proven DirectArray facts
```

## Layer 3: ABI / Plugin / Type-Specific Transport

Plugin and type ABI values must enter Array through the safest transport first.

Allowed first:

```text
plugin object:
  Boxed / handle

public boxes:
  IntegerBox / BoolBox / FloatBox / StringBox

typed scalar plugin value:
  InlineI64 / InlineBool / InlineF64 only after explicit lossless scalar facts
```

Deferred:

```text
plugin record direct lane
InlineRecord production auto-use
DirectRecord
heterogeneous / union inline layout
public ABI widening
plugin ABI value direct lane
```

Plugin scalar inline eligibility requires all of:

```text
plugin ABI value kind is i64 / f64 / bool
conversion is lossless
ownership is immediate
no plugin-side destructor is required
public readback boxes correctly
no silent fallback
```

Plugin object/record eligibility starts as:

```text
object/record plugin value:
  Boxed / handle first

inline record:
  later, only after layout / drop / ownership / ABI version / visibility facts
```

## Ordered Task Backlog

These are ordered backlog rows. Do not implement a later item before its guard
surface exists.

### ARR-001: Scalar Residence Truth Refresh

Goal:
- refresh docs and tests for current `InlineI64` / `InlineBool` / `InlineF64`
  residence.

Tasks:
- confirm public ArrayBox identity remains unchanged
- confirm mixed or unsupported writes promote/degrade to `Boxed`
- confirm public readback materializes the correct box type
- decide whether Bool/F64 typed-load routes have enough evidence

Output:

```text
selected_next=inline_bool_f64_typed_readback_selection|no_implementation_owner_refresh
```

### ARR-002: InlineBool / InlineF64 Typed Readback Selection

Goal:
- decide whether internal typed readback should be added for Bool/F64.

Rules:
- this is not public ABI widening
- keep encoded-any/public handle readback unless a positive internal route is
  proven
- no helper micro-lane without perf or route evidence

Output:

```text
selected_owner=inline_bool_typed_load|inline_f64_typed_load|closed
```

### ARR-003: TextLane Current-State Inventory

Goal:
- inventory the landed `ArrayStorage::Text` and `ValueLanePlan` bridge.

Tasks:
- confirm `TextLane` remains storage/residence only
- confirm `String = value` remains semantic truth
- confirm generic/mixed operations degrade to `Boxed`
- list any remaining TextLane cleanup or deletion gates

Output:

```text
selected_next=textlane_cleanup_selection|no_implementation_owner_refresh
```

### ARR-004: Value Lane Bridge Extension Selection

Goal:
- decide whether to extend the runtime-private demand bridge beyond landed text
  residence.

Rules:
- do not introduce `ArrayStorage::ValueLane` directly
- first add demand/fact vocabulary if needed
- executor actions must stay runtime-private

Output:

```text
selected_owner=value_lane_bridge_extension|closed
```

### ARR-005: Plugin Object Array Bridge

Goal:
- formalize plugin object values entering Array as Boxed/handle values.

Rules:
- no direct plugin-value lane
- preserve clone/share/destructor/lifetime boundaries
- no plugin unload or thread-safety assumptions hidden in Array storage

Output:

```text
selected_owner=plugin_object_boxed_handle_bridge
```

### ARR-006: Plugin Scalar Inline Eligibility

Goal:
- allow plugin typed scalar values to map to scalar lanes only when ABI facts
  prove immediate, lossless, destructor-free values.

Rules:
- i64/f64/bool only
- no record
- no heterogeneous inline storage
- no silent fallback

Output:

```text
selected_owner=plugin_scalar_inline_eligibility_guard|closed
```

### ARR-007: Record / Union Inline Layout Inventory

Goal:
- keep DirectRecord / InlineRecord / heterogeneous layout out of production
  until an explicit evidence row exists.

Tasks:
- inventory existing test-only InlineRecord probes
- list production blockers: layout, alignment, drop, ownership, visibility,
  ABI versioning, materialization, public readback
- decide whether a new inventory row is justified

Output:

```text
selected_owner=record_inline_layout_inventory|closed
```

## Non-Goals

```text
ArrayStorage enum growth without demand/fact row
public ABI widening
public ArrayBox behavior change
public ArrayBox handle reinterpretation
plugin private layout exposure
generic heterogeneous array layout
DirectRecord as the next Array step
provider activation
allocator replacement
hooks
global allocator
```

## Reopen Rules

Open a new Array lane only if all are true:

```text
selected_callsite_or_family=1
positive_net_helper_delta=1 or source-level owner evidence=1
materialization_policy_known=1
public ArrayBox identity preserved=1
silent_fallback_allowed=0
recent_nonkeeper_repeat=0
```

Otherwise keep the work as backlog and continue the active mimalloc
source-level owner refresh.
