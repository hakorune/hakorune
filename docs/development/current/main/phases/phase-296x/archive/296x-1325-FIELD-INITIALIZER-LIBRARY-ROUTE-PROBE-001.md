# 296x-1325 FIELD-INITIALIZER-LIBRARY-ROUTE-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Pin down where declaration-site stored field initializers currently diverge
from the language reference.

The language contract remains:

```text
stored field initializer:
  per-instance
  evaluated on every new
  runs before birth
```

This row is an observation row. It does not fix the backend/compiler route.

## Implementation

```text
apps/field_initializer_route_probe/library.hako
  imported library boxes with field initializers

apps/field_initializer_route_probe/*_*.hako
  one source file per route
  same-file direct new
  same-file static factory new
  same-file birth(value)
  imported static factory new
  imported birth(value)
  imported ordered-map-like two-array default

apps/field_initializer_route_probe/smoke.sh
  EXE/AOT observation ledger
```

## Observation

Current EXE/AOT observation:

```text
same_file_direct_default=ok
same_file_factory_default=ok
same_file_birth=ok
imported_factory_default=unsupported_pure_shape
imported_factory_birth=unsupported_pure_shape
imported_factory_ordered_like=unsupported_pure_shape
```

Interpretation:

```text
same-file direct default-only field initializer:
  works

same-file factory synthetic birth/0:
  works

same-file explicit birth(value):
  works

imported static factory routes:
  not yet observable

imported factory compile failure:
  unsupported pure shape
  route metadata reason=unknown_global_callee
```

The first failing seam is not field initializer evaluation itself:

```text
imported static factory routes:
  unsupported_pure_shape
  route metadata reason=unknown_global_callee
```

Until the imported global/static callee route is accepted, imported synthetic
`birth/0` and imported explicit `birth(value)` cannot be observed.

## Accepted Scope

```text
field_initializer_route_probe_enabled=1
same_file_direct_default_only_field_initializer_checked=1
same_file_direct_default_only_field_initializer_green=1
same_file_factory_synthetic_birth0_checked=1
same_file_explicit_birth_checked=1
imported_static_factory_route_checked=1
imported_static_factory_route_unknown_global_callee=1
imported_field_initializer_runtime_observed=0
fix_implemented=0
ordered_map_api_changed=0
mirbuilder_changed=0
```

## Stop Line

```text
do not rewrite OrderedMapBox in this row
do not move meaningful birth logic into field initializers
do not change MapBox
do not change ring0/ring1 provider registration
do not claim imported synthetic birth/0 is broken or fixed until the imported
  factory route reaches runtime
```

## Next

```text
IMPORTED-STATIC-GLOBAL-CALLEE-ROUTE-PROBE-001
```

Determine why imported static factory routes stop before runtime with
`unknown_global_callee`, while same-file construction routes are accepted.

## Evidence

Run EXE smoke commands sequentially. They share `tmp/nyash_cli_emit.json`, so
parallel execution can produce false `unsupported_pure_shape` failures.

```bash
bash apps/field_initializer_route_probe/smoke.sh
bash apps/lib/collections/smoke_ordered_map.sh
bash apps/constructor-lifecycle-probe/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
