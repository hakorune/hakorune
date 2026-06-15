---
Status: SSOT
Decision: accepted
Date: 2026-06-15
Scope: User-facing `record` / `box` distinction and shared aggregate/object
  optimization substrate.
Related:
  - docs/development/current/main/design/record-and-packed-array-lowering-ssot.md
  - docs/development/current/main/design/record-local-scalarization-ssot.md
  - docs/development/current/main/design/record-construction-ergonomics-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/design/box-callable-registry-ssot.md
---

# Record / Box: Two Surfaces, One Substrate

## Decision

Keep `record` and `box` separate in the user-facing language.

Unify their optimization path internally through aggregate/object planning
where the proofs allow it.

```text
User-facing surface:
  record = identity-free value aggregate
  box    = identity object / behavior / lifecycle boundary

Compiler/runtime substrate:
  record / simple box / enum payload / closure env can share the same
  AggregateStoragePlan / ObjectStoragePlan optimization axis
```

Short form:

```text
Users see two surfaces.
The compiler uses one optimization substrate where proofs permit.
```

## User Model

The source-level rule should stay simple:

```text
data/value:
  use record

thing/owner/behavior/lifecycle:
  use box
```

Examples:

```hako
record Point {
    x: i64
    y: i64
}

box Counter {
    value: i64 = 0

    inc(delta: i64): void {
        me.value = me.value + delta
    }

    get(): i64 {
        return me.value
    }
}
```

Recommended vocabulary:

```text
record:
  Point, Size, Range, Token, Span, Header, ConfigRow, metadata rows

box:
  Parser, Counter, Arena, File, Channel, Future, Socket, Allocator
```

Do not describe `record` as a fast `box`.

```text
wrong:
  record is a faster box
  box is a slower record
  record exists only for exact-AOT

right:
  record is identity-free named data
  box is identity / ownership / behavior boundary
  speed is a compiler responsibility
```

## Record Semantics

`record` is a product semantics surface, not just an optimization hint.

```text
record:
  identity-free named data
  typed fixed fields
  value replacement via `with`
  no lifecycle
  no weak fields in MVP
  no dynamic Box API requirement
  no field mutation semantics
```

The `with` form remains record-only:

```hako
local p2 = p1 with { x: p1.x + 1 }
```

This creates a new value. It does not mutate `p1`.

Ordinary boxes must not gain `with` as a shallow/deep copy shortcut.

```text
ordinary_box_with_enabled=0
automatic_record_to_box_copy=0
```

## Box Semantics

`box` remains the identity / behavior / lifecycle boundary.

```text
box:
  identity-capable object
  mutable fields
  methods
  birth / fini
  weak fields
  delegate / interface
  sync box
  plugin / dynamic dispatch
  resource lifetime
```

`box` optimization is still allowed, but it must be proof-driven and must not
erase source semantics.

```text
simple_box_exact_aot_optimization_allowed=1
simple_box_semantics_erased_by_mirbuilder=0
```

## No Record Methods In V0

Do not add methods to records in v0.

```text
record_methods_enabled=0
record_fini_enabled=0
record_dynamic_dispatch_enabled=0
```

Reason:

```text
record = data only
box    = behavior
```

If behavior is needed, use free functions or a later explicitly designed
extension surface. Do not blur the first model.

## Internal Substrate

The implementation may converge internally.

```text
AggregateStoragePlan / ObjectStoragePlan candidates:
  record
  enum payload
  tuple payload
  closure environment
  simple local user box
```

This does not mean the source surfaces merge.

```text
source_surface_count=2
optimization_substrate_count=1
```

Planned internal split:

```text
Record:
  language semantics are already identity-free
  AggregateStoragePlan can usually decide earlier

Box:
  language semantics include identity/lifecycle/behavior
  ObjectStoragePlan needs escape, route, layout, and lifecycle proof
```

## Relationship To ObjectStoragePlan

`record` does not replace ObjectStoragePlan.

```text
record:
  source semantics

AggregateStoragePlan / ObjectStoragePlan:
  execution representation decision

exact-AOT backend:
  emits scalar/native/stack/generic fallback according to plans
```

The same backend substrate may lower:

```text
record Point:
  ExactNativeStruct / Scalarized / StackAggregate

simple non-escaping box Counter:
  ExactNativeStruct / ExactStackObject / Scalarized
```

But the proofs differ:

```text
record:
  field types known
  no identity
  no lifecycle
  no methods

box:
  receiver type known
  constructor / birth route known
  method routes known
  escape closed
  fini/drop semantics closed
  dynamic Box API not required
```

## Stop Lines

```text
do not remove record from the user-facing language
do not collapse record and box into one source model
do not teach users that record is a performance escape hatch
do not add record methods in v0
do not add ordinary-box `with`
do not add automatic record-to-box copy
do not let MIRBuilder choose object representation
do not make Type ABI / hako_check execution truth
```

## Report Vocabulary

Rows that touch this boundary should report:

```text
record_box_surface_model=two_surface_one_substrate
record_identity_free_value_surface=1
box_identity_behavior_lifecycle_surface=1
source_surface_collapsed_to_box=0
record_methods_enabled=0
ordinary_box_with_enabled=0
automatic_record_to_box_copy=0
aggregate_storage_plan_shared_substrate=1
object_storage_plan_shared_substrate=1
mirbuilder_representation_owner=0
```

## Task Order

Use this order after this SSOT is accepted.

```text
RECORD-BOX-SURFACE-000:
  Land this decision.
  Keep record and box separate for users.
  Name the internal model two-surface / one-substrate.

RECORD-BOX-DOCS-001:
  Update user-facing reference docs:
    data/value -> record
    thing/owner/behavior/lifecycle -> box
  Avoid performance-first wording.

AGG-STORAGE-PLAN-000:
  Introduce AggregateStoragePlan vocabulary as the record/enum/tuple/closure
  side of the shared substrate.
  Keep it planning-only.

AGG-OBJECT-STORAGE-BRIDGE-001:
  Document how AggregateStoragePlan and ObjectStoragePlan share backend
  lowering concepts without merging source semantics.

RECORD-METHODS-GATE-000:
  Add a guard/report rule that record methods remain disabled in v0.

RECORD-WITH-BOX-GATE-000:
  Add a guard/report rule that `with` remains record-only and ordinary box
  update/copy syntax stays rejected.

SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001:
  Only after fresh owner evidence, allow simple non-escaping boxes onto the
  same backend substrate through ObjectStoragePlan.
  Do not use record semantics as proof for boxes.
```

## Current Status

This sequence is landed through the first simple-box candidate gate.

```text
RECORD-BOX-SURFACE-000:
  landed
  row=296x-732

RECORD-BOX-DOCS-001:
  landed
  row=296x-733

AGG-STORAGE-PLAN-000:
  landed
  row=296x-734

AGG-OBJECT-STORAGE-BRIDGE-001:
  landed
  row=296x-735

RECORD-METHODS-GATE-000:
  landed
  row=296x-736

RECORD-WITH-BOX-GATE-000:
  landed
  row=296x-737

SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001:
  landed/parked
  row=296x-738
  reason=fresh_high_confidence_owner_evidence=0
```

The parked candidate row is intentional. Simple boxes may later enter the
shared backend substrate, but only through ObjectStoragePlan / RoutePlan after
a fresh owner row proves that the exact-AOT object boundary is again the right
implementation target.
