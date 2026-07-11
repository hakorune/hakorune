# Failure and Outcome Relations

Status: Current reference.
Decision: accepted (3504 relation/spec first slice).

This document fixes the semantic vocabulary for the Language v1
Failure/Outcome lane. It is a relation specification, not a runtime migration
permission. Existing parser, MIR, VM, provider, and backend behavior remains
unchanged until a later activation design stop accepts one boundary.

## Canonical Vocabulary

```text
Unit:
  successful evaluation with no useful result value

Option::None:
  ordinary value-level absence

Option::Some(T):
  ordinary value-level presence

Result::Ok(T):
  ordinary successful value-level result

Result::Err(E):
  ordinary recoverable failure represented as a value

Fault:
  unrecoverable violated contract represented as an evaluation outcome,
  not as a language value
```

The canonical source spelling for `Unit` is `void`. `Option::None` and
`Result::Err` are distinct enum values; neither is an alias for `void`, `null`,
or the other enum.

## Internal and Boundary Carriers

These carriers are not ordinary language values:

```text
UninitializedSlot:
  a local binding slot before its first successful publication

ForeignNull:
  a native/FFI boundary observation before the declared boundary policy maps it

CompatNull:
  a Compat2025-only compatibility carrier during null migration

InternalMissing:
  parser, builder, or compiler-helper not-found state

CompatFailure:
  legacy throw/catch control carrier at an explicit compatibility boundary
```

An internal carrier cannot be stored, compared, returned, or passed as an
ordinary language value. A foreign carrier must be converted by its declared
boundary policy; a backend must not infer meaning from a zero handle, null
pointer, or missing payload buffer.

## Relation Table

| Meaning | Canonical carrier | Not interchangeable with |
| --- | --- | --- |
| successful no-result | `Unit` / `void` | `Option::None`, `Result::Err`, `Fault` |
| optional absence | `Option::None` | `Unit`, `Result::Err`, `Fault` |
| optional presence | `Option::Some(T)` | `Unit`, `Option::None` |
| recoverable failure | `Result::Err(E)` | `Option::None`, `Fault`, `Unit` |
| successful value result | `Result::Ok(T)` | `Unit`, `Result::Err` |
| violated contract | `Fault` outcome | every ordinary value carrier |
| uninitialized local | `UninitializedSlot` | every language value |
| foreign null observation | `ForeignNull` | `Unit`, `Option::None`, `CompatNull` |
| compatibility null | `CompatNull` | canonical `Unit`, `Option::None` |

## Forbidden Implicit Conversions

The following conversions require an explicit, owned boundary adapter and are
otherwise rejected by the inventory checker:

```text
Fault -> Result::Err
Fault -> Option::None
Fault -> Unit
Result::Err -> Fault
Result::Err -> Option::None
Option::None -> Unit
Unit -> Option::None
CompatNull -> Unit
backend zero/null/missing payload -> Option::None
```

Allowed boundary adapters are explicit and policy-owned:

```text
foreign nullable success + native null -> Option::None
foreign error status + declared result mapping -> Result::Err
foreign void success + declared void return -> Unit
```

## Control Outcomes and Cleanup

Canonical evaluation outcomes are:

```text
Normal(value_or_unit)
Return(value_or_unit)
Break
Continue
Fault(fault_record)
```

`Fault` is not catchable in Canonical v1. `Result::Err` and `Option::None`
must be handled as values with `match` or another explicit enum operation.
Cleanup runs after the body outcome becomes pending; the first cleanup Fault
becomes the final Fault and later cleanup faults are suppressed diagnostics.

## Uninitialized Locals

An uninitialized local is a slot state, not an implicit `Unit`, `None`, or
`null` value:

```text
local x       -> UninitializedSlot
x = value     -> Initialized(value)
read before assignment -> Fault(Contract, uninitialized_local)
```

This is a target relation only in the first inventory slice. Existing default
behavior remains an inventory finding until an activation card accepts it.

## Weak Upgrade

The target relation for weak upgrade is:

```text
alive target       -> Option::Some(BoxRef)
dead/freed target  -> Option::None
empty weak slot    -> Option::None
invalid receiver   -> Fault(Contract, type_mismatch)
```

The reference VM's current `Void` result is recorded by the inventory; this
document does not change it.

## Profiles and Activation

`null`, postfix `catch`, and compatibility throw/catch paths remain governed
by their current grammar/profile registries in this slice. The relation target
is not permission to flip those rows. `CompatNull` is permitted only under the
named Compat2025 boundary after a future migration decision.

```text
runtime_activation = 0
canonical_null_migration = 0
weak_upgrade_option_activation = 0
uninitialized_local_activation = 0
catch_profile_change = 0
```
