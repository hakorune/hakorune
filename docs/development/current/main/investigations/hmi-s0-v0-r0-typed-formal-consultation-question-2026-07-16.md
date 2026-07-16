---
Status: Candidate A accepted; implementation task issued
Date: 2026-07-16
Decision: explicit `storage: MapBox` value type with unchanged A-prime ownership
Baseline: 178776e410
Parent: hmi-s0-v0-r0-clean-register-storage-task-2026-07-16.md
Scope: one field-held MapBox -> no-result static mutation command boundary
---

# HMI-S0-V0-R0 typed-formal consultation

## Resolution

Candidate A is accepted. Implementation is owned by:

```text
hmi-s0-v0-r0-type0-typed-formal-mutation-task-2026-07-16.md
```

The first code-facing row is `R0-TYPE0-S0`. Candidate B remains parked unless
the exact V0 classifier selects `FIELD-PROPAGATION-REQUIRED`.

## Question

How should HMI declare the value type of the A-prime storage command?

Recommended:

```text
select A

explicit value type:
  storage: MapBox

ownership:
  ordinary implicit noescape source shape

first code-facing row:
  R0-TYPE0 generic typed-formal mutation proof
```

```hako
put_proven(storage: MapBox, value_id, payload) {
    storage.set("" + value_id, payload)
    return
}
```

`MapBox` declares the value type only. It adds no `share`, `move`, `clone`, or
new ownership syntax.

## Closed prerequisites

```text
MAPFIELD-R0-DELTA0:
  A-PRIME-AUTHORIZED

HMI-S0-V0-R0-S0:
  closed at 178776e410

storage helper:
  no MapBox return

result vocabulary:
  no raw storage exposure

production callers / opcode handlers / ownership operations:
  0
```

## I0 failure

The clean I0 prototype is stashed by label:

```text
wip/hmi-s0-v0-r0-i0 cross-file static formal loses MapBox type
```

The older owner-roundtrip prototype is:

```text
wip/hmi-s0-v0-r0 register storage fails field mutation
```

Both remain evidence only. Stash ordinals are not stable authority.

Runtime:

```text
producer-backed seal:
  succeeds

define(2, i64(10)):
  fails

post-mutation has(2):
  false

error:
  invalid-transition
  storage mutation postcondition failed
```

## Exact MIR split

The same declared field behaves differently at two sites.

### Simple `has()` method

```text
field_get storage:
  declared_type = handle:MapBox
  result type = handle:MapBox

global contains formal 0:
  handle:MapBox

contains internal call:
  RuntimeDataBox.has / Union
```

### Complex `define()` method

```text
field_get storage:
  declared_type = absent
  result type = Unknown

global put_proven formal 0:
  Unknown

put_proven internal call:
  RuntimeDataBox.set / Union

postcondition:
  register.has(value_id) = false
```

The field is already declared:

```hako
storage: MapBox
```

Therefore the failure is not missing source field annotation, MapBox runtime
set/has behavior, key conversion, scalar payload shape, or ownership opcode.

The boundary is:

```text
late declared-field type publication
and/or
static callable parameter type sealing for a no-result helper
```

## Why DELTA0 was not enough

DELTA0 proved a same-source generic shape with an untyped formal. HMI adds:

```text
using/text-merged helper boundary
typed field declaration
validation and early returns
late field_get
void/no-result mutation call
post-call observation through another method
```

Do not generalize DELTA0 into a completed borrowed MapBox ABI.

## Candidate A — explicit value type

```hako
put_proven(storage: MapBox, value_id, payload)
```

Meaning:

```text
MapBox:
  value type

ordinary parameter:
  implicit noescape ownership intent

ownership spelling:
  0

MapBox return:
  0
```

Advantages:

```text
signature owns the value-type contract
no method-name/runtime inference
matches the typed field declaration
avoids relying on cross-function type inference
keeps the ownership surface unchanged
```

Risk:

```text
current call lowering may still fail to transport the typed field argument
```

Therefore A requires a generic `R0-TYPE0` proof before HMI edits.

### R0-TYPE0 proof

Use an HMI-independent two-file fixture:

```text
main.hako
storage_command.hako
```

Matrix:

```text
local MapBox -> typed static put -> direct observation
field MapBox -> typed static put -> direct observation
field MapBox -> typed static put -> helper observation
validation/early-return method -> late field_get -> typed put
repeated mutation
instance isolation
```

Source law:

```text
storage: MapBox formal
put returns no-value
MapBox return = 0
field reassignment = 0
ownership spelling = 0
```

MIR law:

```text
helper formal 0:
  handle:MapBox

caller field argument:
  exact typed callable contract

helper set:
  verified typed-formal route

CopyOwned / DestroyOwned:
  0
```

If green, update the clean HMI task to use `storage: MapBox` and reimplement
I0 cleanly. Do not restore the WIP.

## Candidate B — compiler field propagation repair

Keep the helper formal untyped and repair declared field typing through:

```text
early-return control flow
late expression lowering
global-call argument materialization
callable parameter convergence
```

This is stronger but larger. It risks mixing field publication and callable
ABI inference. Select B only if TYPE0 shows that explicit parameter typing
cannot close the boundary.

## Candidate C — HMI-local workaround

Rejected:

```text
direct me.storage.set
cache storage in a statement-order-dependent local
move helper into the owner method
return storage and reassign the field
runtime MapBox special case
```

These make HMI source layout an accidental compiler authority.

## Recommended task order

```text
R0-TYPE0-S0
  disconnected two-file typed-formal fixture

R0-TYPE0-M0
  debug/release runtime and normalized MIR

R0-TYPE0-V0
  select TYPED-FORMAL-AUTHORIZED
  or FIELD-PROPAGATION-REQUIRED

R0-TYPE0-G0
  existing compiler MapFieldOwner proof family
```

The consultation card owns the decision. Do not add a separate docs-only
current row.

## Counters

```text
HMI I0 WIP restores = 0
old owner-roundtrip restores = 0
HMI source delta during TYPE0 = 0
MapBox runtime delta = 0
ownership syntax/opcode delta = 0
backend/fallback delta = 0
new proof manifest family = 0
source/check files >= 800 = 0
```

## Stop conditions

Stop if TYPE0 requires:

1. MapBox return or field reassignment.
2. share, move, clone, CopyOwned, or DestroyOwned.
3. Runtime type-name or HMI-specific compiler logic.
4. Direct field mutation as fallback.
5. Method-name inference.
6. A second field/callable type authority.
7. HMI imports or HMI fixture dependence.
8. Backend widening, retry, or fallback.
9. Restoring either HMI stash.
10. A source/check file reaching 800 lines.

## Final question

```text
A:
  explicit storage: MapBox value type
  implicit noescape ownership
  generic R0-TYPE0 proof first

B:
  untyped formal
  compiler field/callable propagation repair first
```

Recommended:

```text
A
```

> Type and ownership are separate. `storage: MapBox` states what crosses the
> callable boundary; the absence of `share` or `move` keeps the A-prime
> noescape intent. Neither fact may be recovered from a method name or runtime
> value.
