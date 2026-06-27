---
Status: Active
Decision: accepted
Date: 2026-06-28
Scope: Materialize the native `.hako` source owner for the BoxCompilationContext
  family selected by the current route manifest.
Related:
  - docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
---

# MIRBUILDER-CONTEXT-HAKO-NATIVE-SOURCE-OWNER-001

## Summary

Materialize the native `.hako` source owner for the `context` route-manifest
entry, which is currently selected on mainline as `BoxCompilationContext` but
still exists only as a generated derived artifact. This is intentionally
narrow. It does not claim Source Selfhost, does not remove Rust bootstrap, and
does not widen the route beyond the `BoxCompilationContext` family.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Required Delta

At least one code-facing artifact must land:

```text
native Hako source owner file
module export wiring
machine-checkable native guard
fixture-backed native result
```

## Expected Shape

The native owner should cover the bounded `BoxCompilationContext` ctor +
`is_empty` slice, matching the already generated derived artifact, while keeping
the rest of the family parked.

```text
native source owner path:
  apps/lib/hakorune_mir_builder/box_compilation_context.hako

native smoke source:
  apps/tests/phase296x_box_compilation_context_native_min.hako

native guard:
  tools/checks/rust_mirbuilder_box_compilation_context_native_guard.sh
```

## Acceptance

```text
native_hako_source_owner_present = 1
generator_overwrite_guard = 1
decision = Adopt
target_family_is_derived_mainline = 1
target_scope_is_narrow = 1
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
manual_next_owner_selection = 0
```

## Non-Claims

```text
full MirBuilder object adoption = 0
all generated artifacts HakoAdopted = 0
Rust bootstrap removal = 0
Source Selfhost = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Next

Follow the decision result:

```text
Adopt:
  enforce native source authority and generator overwrite guard

Defer:
  park the family on the named missing requirement

Reject:
  keep the family generated and record why it should remain derived
```

