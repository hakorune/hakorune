# 296x-1323 MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Use OrderedMapBox in a focused BindingContext-style probe before any broad
MirBuilder rewrite.

This row proves the `.hako` OrderedMapBox can model deterministic
String-key binding snapshots of the form `name -> id`.

## Implementation

```text
apps/mirbuilder-binding-context-ordered-map-probe/main.hako
  inserts binding names out of order
  updates an existing binding
  checks deterministic lexical key order through key_at()
  checks lookup/missing behavior

apps/mirbuilder-binding-context-ordered-map-probe/smoke.sh
  builds the probe through EXE/AOT
  diffs a stable expected output
```

## Accepted Scope

```text
binding_context_style_probe_enabled=1
ordered_mapbox_consumer=1
deterministic_string_key_iteration_checked=1
duplicate_binding_update_checked=1
missing_binding_lookup_checked=1
mirbuilder_rewrite_enabled=0
binding_context_replacement_enabled=0
mapbox_semantics_changed=0
ring0_enabled=0
ring1_provider_enabled=0
```

The probe is intentionally app-level. It does not change Rust `BindingContext`,
MirBuilder ownership, `MapBox`, or runtime provider registration.

## Output Contract

```text
binding_count=3
binding[0]=alpha:10
binding[1]=beta:21
binding[2]=gamma:30
lookup.missing=null
summary=ok
```

## Stop Line

```text
do not rewrite MirBuilder
do not replace BindingContext
do not change MapBox
do not add ring0 substrate support
do not add ring1 provider registration
do not claim Rust BTreeMap parity
```

## Evidence

```bash
bash apps/mirbuilder-binding-context-ordered-map-probe/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
CREAT-SUBSET-PILOT-SELECTION-001
```

Return to the RustSubset/creat app-front lane now that the OrderedMapBox
detour has a focused BindingContext-style probe.
