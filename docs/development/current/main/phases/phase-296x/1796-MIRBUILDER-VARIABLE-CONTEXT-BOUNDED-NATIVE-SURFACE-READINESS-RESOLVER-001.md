---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Resolve whether the adopted VariableContext native surface is ready for bounded consumers.
Related:
  - docs/development/current/main/phases/phase-296x/1795-MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-bounded-native-surface-readiness-resolution-v0.json
  - tools/checks/rust_lifecycle_variable_context_bounded_native_surface_readiness_guard.sh
---

# MIRBUILDER-VARIABLE-CONTEXT-BOUNDED-NATIVE-SURFACE-READINESS-RESOLVER-001

## Goal

Resolve the current VariableContext state at the correct granularity.

The adopted surface is ready as a bounded native consumer surface, but this is
not a full Rust `VariableContext` parity claim and not a Source Selfhost claim.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Decision

```text
decision:
  ReadyForBoundedVariableContextNativeSurfaceConsumer

readiness_state:
  Ready

reason_token:
  ExplicitMutationSurfaceAdoptedAndReferenceProjectionContractClosed

selected_surface_id:
  VariableContextNativeSurfaceExplicitMutationApiOnlyV1
```

## Consumed Evidence

```text
reference_projection_contract:
  MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001

read surface:
  OwnedReadSnapshotProjection

mutable surface:
  ExplicitMutationApiOnly

native source:
  apps/lib/hakorune_mir_builder/variable_context.hako

bounded API:
  lookup
  contains
  len
  is_empty
  snapshot
  restore
  replace_owned_map
  insert
  remove
```

## Explicit Non-Requirements

These are not required for bounded readiness and remain separate follow-up
lanes.

```text
entries_snapshot:
  FutureConsumerNeedOnly

snapshot_owned / restore_owned:
  NamingCompatibilityCleanupOnly

MutLease:
  DeferredUntilLiveNeed
```

## Next Split

```text
next_action:
  MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001

if entries_snapshot needed:
  MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-PROJECTION-001

if entries_snapshot not needed:
  NextRouteFamilySelectionPolicy

if MutLease needed:
  SOURCE-SELFHOST-RUST-MUTLEASE-SEMANTICS-DESIGN-STOP-001
```

## Acceptance

```text
bounded_readiness = Ready
selected_surface_id = VariableContextNativeSurfaceExplicitMutationApiOnlyV1
projection_model = SemanticOneToOneVerifiedProjection
variable_map_projection = OwnedReadSnapshotProjection
variable_map_mut_projection = ExplicitMutationApiOnly
native_hako_source_owner_present = 1
native_behavior_guard_green = 1
owned_read_snapshot_projection_green = 1
explicit_mutation_api_projection_green = 1
reference_projection_contract_green = 1
full_variable_context_claim = 0
source_selfhost_claim = 0
entries_snapshot_implemented = 0
snapshot_owned_implemented = 0
restore_owned_implemented = 0
raw_variable_map_alias_selected = 0
raw_variable_map_mut_alias_selected = 0
mut_lease_selected = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
manual_family_selection = 0
```

## Closeout

```text
output_contract=rust-lifecycle-mirbuilder-variable-context-bounded-native-surface-readiness-resolution-v0
decision=ReadyForBoundedVariableContextNativeSurfaceConsumer
readiness_state=Ready
reason_token=ExplicitMutationSurfaceAdoptedAndReferenceProjectionContractClosed
selected_surface_id=VariableContextNativeSurfaceExplicitMutationApiOnlyV1
next_action=MIRBUILDER-VARIABLE-CONTEXT-ENTRIES-SNAPSHOT-NEED-RESOLVER-001
summary=ok
```
