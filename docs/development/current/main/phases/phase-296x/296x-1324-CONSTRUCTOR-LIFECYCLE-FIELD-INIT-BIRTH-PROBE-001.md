# 296x-1324 CONSTRUCTOR-LIFECYCLE-FIELD-INIT-BIRTH-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Separate OrderedMapBox construction cleanup from the language-level
constructor lifecycle contract.

This row documents the correct boundary:

```text
stored field initializers:
  simple per-instance defaults

birth(args...):
  constructor-argument work and meaningful initialization
```

## Implementation

```text
apps/constructor-lifecycle-probe/main.hako
  BirthProbeBox with ArrayBox field initializer and birth(value)
  checks field initializer state is available to birth
  checks constructor arg reaches birth
  checks per-instance ArrayBox freshness

apps/constructor-lifecycle-probe/smoke.sh
  EXE/AOT smoke for the lifecycle probe

apps/lib/collections/tests/ordered_map_smoke.hako
  adds fresh-per-instance OrderedMapBox behavior check
```

`OrderedMapBox` keeps its explicit `OrderedMap.create()` initialization in v0.
That is a route-compatibility choice. It does not change the language contract
that declaration-site field initializers are per-instance values evaluated
before `birth`.

## Accepted Scope

```text
birth_probe_enabled=1
field_initializer_before_birth_checked=1
constructor_arg_reaches_birth_checked=1
fresh_per_instance_array_checked=1
ordered_map_fresh_instance_smoke_checked=1
ordered_map_api_changed=0
ordered_map_birth_added=0
mechanical_birth_to_field_initializer_migration=0
```

## Result

```text
OrderedMapBox design:
  keeper

OrderedMapBox initialization:
  explicit create() initialization remains v0 implementation

birth route:
  EXE/AOT probe green for same-module user-box birth with constructor arg

field initializer route:
  language contract documented as existing SSOT
```

## Stop Line

```text
do not mechanically move birth logic into field initializers
do not claim all EXE/AOT field-initializer routes from this probe alone
do not change MapBox
do not change ring0/ring1 provider registration
do not rewrite MirBuilder
```

## Evidence

```bash
bash apps/lib/collections/smoke_ordered_map.sh
bash apps/constructor-lifecycle-probe/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
CREAT-SUBSET-PILOT-SELECTION-001
```

Return to the RustSubset/creat app-front lane.
