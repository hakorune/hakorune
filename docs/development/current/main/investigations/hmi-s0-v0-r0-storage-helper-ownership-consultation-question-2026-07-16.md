---
Status: Resolved; superseded by DELTA0/TYPE0 proof chain
Date: 2026-07-16
Decision needed: select the storage-helper ownership contract before HMI-S0-V0-R0 resumes
Baseline: eff120b77c
Parent: hmi-s0-v0-r0-map-field-owner-proof-task-2026-07-16.md
Scope: one minimized generic MapBox-formal mutation/return boundary
---

# HMI-S0-V0-R0 storage-helper ownership consultation

## Question

Which contract should own mutation of a field-held `MapBox` through a static
helper?

Recommended answer:

```text
select A-prime

static helper:
  borrowed/noescape MapBox formal
  mutate in place
  no MapBox return

caller:
  keeps the field owner
  does not reassign the field

first code-facing row:
  R0-DELTA0 generic MapBox-formal mutation proof
```

Do not select implicit MapBox owner roundtrip through an untyped helper return.

## Closed evidence

`R0-STOP0` is closed with:

```text
runtime cases:
  10 / 10 pass in debug and release

MapBox / Known calls:
  22

RuntimeDataBox / Union calls:
  2 control-merge set sites

receiver PHI root:
  param:0

CopyOwned / DestroyOwned:
  0

exclusive selection:
  NONE-HMI-DELTA0
```

Therefore these compiler tasks are not authorized:

```text
KEY0
RECV0
```

The generic proof is registered as:

```bash
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-STOP0
```

## Minimized missing shape

The stashed register prototype is evidence only. It contains this helper:

```hako
static box HmiScalarPayloadStorageV1 {
    put_payload(storage, value_id, payload) {
        storage.set("" + value_id, payload)
        return storage
    }
}
```

The register owner consumes it as:

```hako
me.register_map = HmiScalarPayloadStorageV1.put_payload(
    me.register_map,
    value_id,
    payload
)
return me.has(value_id)
```

The failing probe reported:

```text
static helper set was reached
immediate owner observation was false
```

This shape differs from the green generic matrix in one ownership chain:

```text
field-held MapBox
  -> untyped static helper formal
  -> in-place mutation
  -> same MapBox returned
  -> caller field reassignment
  -> observation through another helper
```

The generic matrix already proves:

```text
field-held MapBox direct mutation
method-formal String keys
same-method observation
same-receiver helper observation
fallthrough receiver PHIs
receiver aliases
instance isolation
```

It does not prove a MapBox object itself crossing a static helper formal and
return boundary.

## Why this is an ownership decision

The sparse ownership design says:

```text
ordinary parameter:
  borrowed/noescape by default

ordinary return:
  Owned by default

same identity independent owner:
  explicit share

owner transfer:
  explicit move
```

Under that model:

```hako
put_payload(storage, ...) {
    storage.set(...)
    return storage
}
```

cannot quietly mean both:

```text
borrowed formal during mutation
and
Owned same-identity return after mutation
```

The helper either performs a borrowed mutation command or explicitly transfers
or shares ownership. Register storage needs only the mutation command.

## Candidate A-prime — borrowed mutation command

```hako
static box ScalarPayloadStorageV1 {
    put_payload(storage, value_id, payload) {
        storage.set("" + value_id, payload)
    }
}

box ScalarRegisterFileV1 {
    store_proven_payload(value_id, payload) {
        ScalarPayloadStorageV1.put_payload(
            me.register_map,
            value_id,
            payload
        )
        me.register_order.push(value_id)
        return me.has(value_id)
    }
}
```

Semantics:

```text
storage formal:
  borrowed/noescape alias

MapBox identity:
  caller-owned field remains owner

runtime ownership operation:
  0

field reassignment:
  0
```

Advantages:

```text
matches future parameter ownership law
keeps mutation policy in one helper
does not invent an owner return
does not rehome or replace the field
```

Risk:

```text
current compiler may still lose MapBox identity through a static helper formal
```

Therefore A-prime requires one generic proof before HMI code:

```text
R0-DELTA0
```

## Candidate B — owner roundtrip helper

Preserve the current shape and require the compiler to prove:

```text
field MapBox
  -> helper formal
  -> mutation
  -> helper return
  -> field reassignment

same identity:
  guaranteed
```

This would require an explicit return ownership answer:

```text
move:
  caller temporarily transfers field ownership into helper

view:
  helper returns an alias that cannot replace an Owned field

share:
  creates an unnecessary independent owner and RC cost
```

Problems:

```text
widens the compiler before the source ownership grammar is active
conflates mutation command and owner transport
encourages redundant field reassignment
risks hidden share/rehome semantics
```

Recommendation:

```text
reject
```

## Candidate C — direct register-owner mutation

```hako
store_proven_payload(value_id, payload) {
    me.register_map.set("" + value_id, payload)
    me.register_order.push(value_id)
    return me.has(value_id)
}
```

This is likely supported by the green generic matrix.

Advantages:

```text
smallest source shape
field owner and mutation are co-located
```

Problem:

```text
collapses the storage helper boundary instead of proving reusable borrowed
MapBox formal mutation
```

Recommendation:

```text
park as fallback only if A-prime's generic proof fails for a reason that
requires a wider unrelated compiler feature
```

## Recommended first proof: R0-DELTA0

Use one generic, application-independent fixture:

```hako
static box MapFormalMutationProbeV1 {
    put(storage, key, value) {
        storage.set(key, value)
    }

    has(storage, key) {
        return storage.has(key)
    }

    load_present(storage, key) {
        if storage.has(key) {
            return storage.get(key)
        }
        return -1
    }
}
```

Exact matrix:

```text
1. local MapBox -> static put -> caller direct has/get
2. local MapBox -> static put -> static has/load
3. field MapBox -> static put -> owner direct has/get
4. field MapBox -> static put -> static has/load
5. two field owners remain isolated
6. helper returns no MapBox
7. field replacement after birth = 0
```

Selection:

```text
all pass:
  A-prime is authorized
  reimplement HMI register storage without return/reassignment

local passes, field fails:
  select a narrow field-to-static-formal identity compiler prerequisite

local and field fail:
  return to compiler formal-object identity consultation
```

## Authority split

```text
source owner:
  register_map field

mutation helper:
  borrowed/noescape command

Binding SSA:
  value/receiver reaching identity

ownership authority:
  no new owner token in first proof

runtime:
  ordinary MapBox mutation only

non-authorities:
  method name
  HMI type name
  RuntimeDataBox fallback
  raw MapBox return
  field reassignment
```

## Stop conditions

Stop if any of these is required:

1. Return the MapBox from the helper to make mutation visible.
2. Reassign the owner field after an in-place mutation.
3. Add `share`, RC, rehome, CopyOwned, DestroyOwned, or ReleaseStrong policy.
4. Add an HMI/type-name compiler branch.
5. Change MapBox runtime semantics.
6. Use ArrayBox or a public raw storage carrier.
7. Restore the stashed register implementation before the generic proof.
8. Add fallback, retry, environment selection, or backend widening.
9. Activate ownership grammar in the same row.
10. Touch a source/check file at or above 800 lines.

## Decision request

```text
select:
  A-prime

first implementation:
  R0-DELTA0 generic borrowed MapBox-formal mutation proof

park:
  B owner roundtrip
  C direct field mutation fallback
```

The central rule is:

> A helper that only mutates caller-owned storage should be a borrowed
> mutation command, not an implicit same-identity owner-return function.
