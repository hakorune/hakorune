---
Status: Open design consultation
Date: 2026-07-16
Decision: pending
Baseline: 1248fc0fcd
Parent: hmi-s0-v0-r0-clean-register-storage-task-2026-07-16.md
Scope: current-method receiver -> declared ArrayBox field after control flow
---

# HMI R0-I0 declared ArrayBox field consultation

## Question

HMI register file の `definition_order: ArrayBox` を、validation/early-return
後に直接 `push()` すると、declared field type が消えて実行に失敗します。

どの境界を次の durable compiler row にすべきでしょうか。

推奨:

```text
Candidate B′

current-method receiver declared-field fact
through a neutral Copy-only chain
```

推奨順:

```text
R0-DECLFIELD0-S0
  generic HMI-free late-field fixture

R0-DECLFIELD0-M0
  normalized missing-fact proof

R0-DECLFIELD0-I0
  narrow Builder field-fact connection

R0-DECLFIELD0-G0
  guard / regression / HMI resume authorization

then:
  clean HMI-S0-V0-R0-I0 rewrite
```

## Closed prerequisites

```text
MAPFIELD-R0-DELTA0:
  A-PRIME-AUTHORIZED

MAPFIELD-R0-TYPE0:
  TYPED-FORMAL-AUTHORIZED

HMI-S0-V0-R0-S0:
  closed

HMI production callers:
  0

ownership syntax/opcodes:
  0
```

TYPE0 proves that an explicit `storage: MapBox` formal seals the callee-side
value type. It does not claim that all declared fields retain their type after
arbitrary control flow.

## New I0 failure

The clean I0 WIP is stored only as evidence:

```text
wip/hmi-s0-v0-r0-i0 ArrayBox late field receiver is Void
```

Do not apply or restore it.

Runtime:

```text
whole-document seal:
  succeeds

register construction/birth:
  succeeds

first typed define:
  reaches definition-order publication

failure:
  Unknown method `length` on Void
```

The source declares:

```hako
box HmiScalarRegisterFileV1 {
    storage: MapBox
    definition_order: ArrayBox
    function_view
}
```

Birth publishes:

```text
storage:
  new MapBox

definition_order:
  new ArrayBox

function_view:
  null
```

The emitted birth MIR contains exact `MapBox` and `ArrayBox` declared field
types. No field replacement occurs after birth.

## Exact MIR split

### Simple `size()` access

```text
receiver:
  handle:HmiScalarRegisterFileV1

field_get definition_order:
  declared_type = handle:ArrayBox
  result type = handle:ArrayBox

call:
  ArrayBox.length / Known
```

### Validation-heavy `define()` access

After null checks, sealed-view checks, kind checks, duplicate checks, MapBox
publication, and postcondition checks:

```text
receiver root:
  current method receiver through Copy/PHI carrier

receiver type:
  handle:HmiScalarRegisterFileV1

field_get definition_order:
  declared_type = absent
  result type = Unknown

call:
  RuntimeDataBox.push / Union
```

The declaration and receiver identity are present. Only the field-result type
publication is missing.

## Current compiler seam

The current Builder lookup is narrower than the observed source authority.

```text
declared_field_type_for_value:
  value_origin_newbox[object_value]
  -> declared_field_type_name(box, field)
```

It does not currently prove:

```text
current instance method receiver parameter
  -> neutral Copy-only chain
  -> exact current callable user-box owner
  -> explicit declared field
```

Later MIR products can already recognize the same field site as a typed-object
exact slot. That downstream product must not become a backfeed authority for
Builder field typing.

## Candidate A — typed ArrayBox helper

Example:

```hako
append_proven(order: ArrayBox, value_id) {
    order.push(value_id)
    return
}
```

Pros:

```text
explicit callee type
small source change
likely reuses the TYPE0 signature path
```

Why it is not recommended:

```text
direct declared-field method access remains broken
HMI source avoids a compiler expressivity gap
helper-shaped debt spreads to length/get/snapshot construction
compiler-expressivity-first policy is violated
```

Candidate A may be used only as a disconnected comparison control. It must not
be the production HMI fix.

## Candidate B′ — narrow current-receiver field fact

Admission:

```text
field base:
  resolves through Copy-only chain

root:
  current instance-method receiver parameter

callable owner:
  one exact user-box declaration

field:
  explicit declaration on that exact box

first fixture type:
  ArrayBox

regression type:
  MapBox
```

Materialization:

```text
existing FieldGet.declared_type:
  Some(Box(ArrayBox))

existing value type publication:
  handle:ArrayBox

following calls:
  ArrayBox.push / Known
  ArrayBox.length / Known
```

Non-authorities:

```text
method name
field name
runtime class tag
HMI source path
TypedObjectPlan
MIR symbol parsing
dynamic receiver inference
```

This is the recommended candidate.

## Candidate C — representation/source restructuring

Examples:

```text
replace definition order with MapBox
store a separate size field
move mutation before validation
expose raw arrays
reassign the field after helper calls
```

Reject. These change the source/representation to route around the compiler
boundary and do not repair declared-field semantics.

## Proposed generic proof

Use one HMI-independent app:

```text
owner:
  items: ArrayBox
  map: MapBox

cases:
  direct push/length
  one fallthrough merge then push/length
  nested validation + early returns then push/length
  receiver Copy alias
  rejected branch changes state = 0
  repeated mutation
  two instances remain isolated
  MapBox declared-field regression
```

Before the compiler change, the proof must record the exact split:

```text
direct ArrayBox:
  Known

late ArrayBox:
  declared_type absent
  RuntimeDataBox / Union

receiver root/type:
  still exact current owner
```

After the change:

```text
all accepted ArrayBox field sites:
  declared_type handle:ArrayBox
  ArrayBox / Known

RuntimeDataBox/Union for selected sites:
  0

field replacement:
  0

CopyOwned / DestroyOwned / selected ReleaseStrong:
  0
```

## Exact authority split

| Concern | Authority |
| --- | --- |
| current callable owner | existing function/method lowering context |
| current receiver identity | existing receiver parameter fact |
| Copy-chain resolution | one neutral Builder provenance helper |
| source field declaration | existing user-box declaration registry |
| FieldGet declared type | existing `MirInstruction::FieldGet` field |
| result ValueId type | existing Builder type publication |
| method route | existing generic method route planner |
| runtime object | existing field_get/runtime storage |

No second `ValueId -> type`, field registry, or runtime inference table is
allowed.

## Stop conditions

Stop and request a broader field-provenance design if implementation requires:

1. Arbitrary PHI or mixed-root alias joining.
2. Dynamic receiver class inference.
3. Method-name, field-name, or HMI-path special casing.
4. Runtime type tags.
5. TypedObjectPlan or another downstream MIR product feeding Builder.
6. A second field/type/origin authority.
7. AST rewriting.
8. HMI source restructuring as the compiler fix.
9. Ownership grammar/opcodes, receiver ABI, or backend widening.
10. Fallback, retry, or legacy route probing.
11. More than one new accepted compiler shape in the row.
12. A source/check file reaching 800 lines.

## Consultation request

Please decide:

```text
1. Candidate A, B′, or C?
2. Is current receiver + Copy-only root the correct first admission boundary?
3. Which existing receiver/provenance owner should the neutral lookup reuse?
4. Should ArrayBox and MapBox be co-fixtures in the first row?
5. Is `R0-DECLFIELD0 S0 -> M0 -> I0 -> G0` the correct task order?
6. What exact stop condition would require a broader PHI/provenance design?
```

## Recommended decision lock

> Select Candidate B′. First prove the gap in an HMI-independent declared-field
> fixture. Extend only the Builder field-fact lookup so a Copy-only chain rooted
> at the current instance-method receiver can recover an explicit field
> declaration from the exact current user-box owner. Publish that type through
> the existing FieldGet and value-type channels before method routing. Do not
> use typed ArrayBox helpers, source restructuring, runtime tags, downstream
> plan backfeed, name heuristics, fallback, or ownership widening to hide the
> missing declared-field fact.
