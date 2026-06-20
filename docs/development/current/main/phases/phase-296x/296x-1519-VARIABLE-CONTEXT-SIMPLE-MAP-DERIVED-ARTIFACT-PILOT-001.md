# 296x-1519 VARIABLE-CONTEXT-SIMPLE-MAP-DERIVED-ARTIFACT-PILOT-001

Status: closed
Date: 2026-06-20

## Purpose

Generate a VariableContext simple-map-only derived `.hako` artifact from the
existing facts / plan / oracle fixtures.

The artifact must not claim full VariableContext coverage.

## Selected By

```text
296x-1518-VARIABLE-CONTEXT-DERIVED-ARTIFACT-PILOT-SELECTION-001
```

## Scope

Allowed:

```text
VariableContext simple-map behavior recipe
VariableContext simple-map generated artifact
artifact manifest
deterministic regeneration guard
parser/MIR gate if supported
```

Forbidden:

```text
returned variable_map / variable_map_mut behavior
snapshot / restore behavior
carrier-sensitive behavior
full VariableContext parity claim
native Hako adoption
Rust bootstrap removal
runtime fallback from Hako to Rust
```

## Acceptance Draft

```text
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_simple_map_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
full_variable_context_claim=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-simple-map-derived-artifact-v0
family_id=hakorune_mir_builder::variable_context
pilot_scope=VariableContext_simple_map_only
generated_hako_checked_in=1
artifact_manifest_checked_in=1
deterministic_regeneration=green
generated_hako_parse=green
generated_hako_mir_emit=green
full_variable_context_claim=0
returned_borrow_methods_generated=0
snapshot_restore_methods_generated=0
rust_bootstrap_retained=1
runtime_try_hako_then_rust_fallback=0
backend_behavior_changed=0
summary=ok
```

Evidence:

```text
tools/rust_lifecycle/generate_variable_context_simple_map_artifact.py
tools/checks/rust_lifecycle_variable_context_simple_map_derived_artifact_guard.sh
lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.hako
lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json
```

Boundary:

```text
This artifact covers VariableContext simple-map methods only. It does not
generate returned map borrow, mutable map borrow, snapshot/restore, or
carrier-sensitive behavior.
```

Next:

```text
296x-1520-VARIABLE-CONTEXT-SIMPLE-MAP-DERIVED-ROUTE-SELECTION-001
```
