---
Status: SSOT
Date: 2026-06-27
Scope: MirBuilder-only Rust-to-Hako converter task order.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-lifecycle-projection-ssot.md
  - docs/reference/architecture/rust-to-hako-lifecycle-projection.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md
---

# MirBuilder Rust-to-Hako Converter Task Order

This file is the current task-order entry. It is not a landed-history ledger.
Detailed historical rows live in phase cards and git history.

## Current Target

```text
active blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

current implementation task:
  MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-001.
  Select the first small native source owner seed pilot target. ReturnEmission
  is selected by stable priority among small HakoMainline support-lane leaves.
  This does not materialize native source, run HakoAdopted, or claim Source
  Selfhost.
  output_contract = rust-lifecycle-mirbuilder-native-owner-seed-pilot-target-selection-v0

selected decision slice:
  source_selfhost.adoption_plan
    -> SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
    -> SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001
    -> SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-RESOLUTION-001
    -> SOURCE-SELFHOST-DOCS-GUARD-MAINTENANCE-REDUCTION-001
    -> MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-GUARD-REALIGNMENT-001
    -> GUARD-SOURCE-SELFHOST-CURRENT-POINTER-DECOUPLE-001
    -> GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001
    -> DOCS-SOURCE-SELFHOST-COMPACT-CURRENT-STATE-001
    -> DOCS-SOURCE-SELFHOST-TASK-ORDER-THINNING-001
    -> DOCS-CHECK-INDEX-FAMILY-VIEW-001
    -> SOURCE-SELFHOST-POST-MAINTENANCE-TASK-INVENTORY-001
    -> MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
    -> MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-OWNER-SEED-INVENTORY-001
    -> MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001
    -> MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-001

selected evidence:
  current pointers:
    -> docs/development/current/main/CURRENT_STATE.toml
    -> docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json
    -> docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-resolution-v0.json
    -> docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json
  durable policy:
    -> docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
    -> docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md

landed evidence pointer:
  Detailed landed rows live in the route-selection guards, adoption cards, and
  git history. This task-order only keeps the active blocker, fail-fast
  boundary, and Active Next 3.

selected next owner:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

current fail-fast boundary:
  Do not reopen `variable_map()` as a raw borrowed alias. The selected
  projection is adopted only as a bounded native surface. Route-matrix rerun
  002 has already derived the candidate-eligible explicit-mutation surface;
  full VariableContext, returned mutable borrow, MutLease, Rust deletion, and
  source selfhost claims remain parked. Selected surface id
  `VariableContextNativeSurfaceExplicitMutationApiOnlyV1` is the bounded
  explicit-mutation adoption target. Returned mutable borrow is replaced by
  explicit mutation APIs, and `replace_owned_map` is the owned-map replacement
  hook. The reference projection contract is semantic one-to-one verified
  projection, not Rust syntax one-to-one translation. The current readiness
  decision is bounded-consumer readiness only; `entries_snapshot` is not
  required for current bounded consumers, `snapshot_owned` and `restore_owned`
  remain naming cleanup only, and MutLease remains a follow-up lane. The next
  policy does not select a family by hand: it classifies route-family rows and
  blocks with recovery guidance when no machine-derived native adoption
  candidate remains. The wider route-selection resolution row keeps the lane
  stopped until consultation-gated widening or machine-derived route repair
  resumes it. The current maintenance phase does not change that stop line; it
  fixes stale guard vocabulary and reduces row-update cost before semantic
  native-slice decomposition resumes. Native-slice decomposition now consumes
  the composed-closure evidence and derives no eligible native adoption slice
  and no concrete route repair. Source Selfhost stays stopped at the wider
  route-selection design stop.

latest design decision:
  Pointer realignment, VariableContext closeout, the blocked candidate
  selection, the artifact-selfhost checkpoint, and the minimal-path mainline
  pilot are closed as provenance. The ReturnEmission, FunctionRegionStackPop,
  and SlotRegistryRelease promotion decisions are closed as provenance. The
  VariableContext native surface that excludes returned borrow routes is
  adopted. The returned read borrow repair lane now has an owned read snapshot
  projection, the route matrix rerun derives a bounded owned-read snapshot
  native surface candidate, that surface is adopted, and the queue stops at
  returned mutable borrow policy consultation. ExplicitMutationApiOnly is now
  selected as the replacement policy, and the explicit mutation API
  projection materializes replace_owned_map as the bounded owned-map
  replacement hook before route-matrix rerun 002 derives the repaired
  explicit-mutation surface, which is then adopted as native Hako authority.
  VariableContext reference projection is now fixed as a semantic one-to-one
  verified projection contract: `variable_map()` remains an owned read
  snapshot projection, `variable_map_mut()` remains explicit mutation APIs, and
  Rust lifetime syntax / raw borrow alias transport are not target authority.
  The adopted explicit-mutation surface is now resolved as ready for bounded
  native consumers, while full VariableContext and Source Selfhost remain
  closed. `entries_snapshot` is not required for the current bounded
  consumers, so the next machine-derived step is NextRouteFamilySelectionPolicy,
  not a new projection lane. That policy now classifies the current rows and
  keeps Source Selfhost stopped because no eligible native adoption candidate
  remains after excluding already adopted, bounded-only, support-lane, and
  consultation-gated rows. That policy's recovery line now lands on the wider
  route-selection design stop, which keeps the lane stopped until consultation-
  gated widening or machine-derived route repair resumes it. Reason token:
  `NoEligibleNativeAdoptionCandidate`. Runner roles are now separated from
  semantic ownership: EXE/AOT is a validation gate, current `vm-hako` is an
  internal witness/debug/bootstrap-proof lane, and a future interpreter is not
  required for Python-to-Hako projector migration. The next task pack freezes
  the consultation-gated wider route-selection resolution rather than opening
  route repair or family adoption. The next selected task is a maintenance
  phase: realign stale guard vocabulary, decouple historical row guards from
  live CURRENT_STATE pointer churn, and prepare reusable Source Selfhost family
  guards before native-slice decomposition. The stale readiness guard
  vocabulary is now represented as structured evidence fields rather than a
  prose task-order needle. Historical 1801/1802 row guards no longer pin a
  live `CURRENT_STATE.latest_card` allowlist; `current_state_pointer_guard`
  owns live pointer validation. Source Selfhost row boilerplate now has a
  reusable family manifest guard for card / fixture / non-claim existence.
  `CURRENT_STATE.toml` is now compact pointer state, not a landed-history
  ledger. This task-order is also compact pointer state; detailed landed rows
  live in phase cards, git history, and the Source Selfhost family manifest.
  The check-scripts index now treats Source Selfhost row-specific guards as
  legacy traceability and keeps the family guard as the stable public entry.
  The post-maintenance queue is now fixed: next semantic work is native-slice
  decomposition, and Source Selfhost remains stopped at the wider route
  selection design stop. The native-slice decomposition result is now fixed:
  `candidate_eligible_count = 0`, `repairable_inconsistency_count = 0`, and
  selected next card remains `SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001`.
  Native owner seed inventory now confirms that the composed closure is not a
  semantic family owner, the generated artifact is not edit authority, and no
  leaf native owner seed is machine-derived. The GeneratedArtifactOnly to
  NativeOwnerSeedCandidate policy now defines the missing promotion conditions
  without selecting a family or opening Source Selfhost. The native source seed
  pilot target selection now narrows progress to ReturnEmission, treating
  support-lane projectors as seed pilot targets but not HakoAdopted candidates.

## Source Selfhost Adoption Plan Evidence

```text
candidate_pool_state = Blocked
native_surface_candidate_state = CandidateEligible
native_surface_id = VariableContextNativeSurfaceNoReturnedBorrowV1
native_surface_adoption_decision = Adopt
post_variable_context_surface_resolution = DesignConsultationRequired
post_variable_context_surface_reason = NoRemainingMachineDerivedNativeSurfaceCandidate
returned_read_repair_route = OwnedReadSnapshotProjection
owned_read_snapshot_projection = OwnedReadSnapshotProjection
returned_read_route_candidate_state = CandidateEligible
route_matrix_rerun_surface = VariableContextNativeSurfaceOwnedReadSnapshotV1
owned_read_snapshot_surface_adoption_decision = Adopt
decision=Adopt
post_owned_snapshot_resolution = DesignConsultationRequired
post_owned_snapshot_resolution_reason = ReturnedMutableBorrowPolicyRequired
returned_mutable_borrow_policy = ExplicitMutationApiOnly
returned_mutable_borrow_owner_kind = VariableContextReturnedMutableBorrowPolicyDecision
reference_projection_contract = MIRBUILDER-VARIABLE-CONTEXT-REFERENCE-PROJECTION-CONTRACT-001
reference_projection_contract_output = rust-lifecycle-variable-context-reference-projection-contract-v0
projection_model = SemanticOneToOneVerifiedProjection
syntax_one_to_one_required = 0
variable_map_projection = OwnedReadSnapshotProjection
variable_map_mut_projection = ExplicitMutationApiOnly
bounded_native_surface_readiness = ReadyForBoundedVariableContextNativeSurfaceConsumer
  bounded_native_surface_readiness_contract = rust-lifecycle-mirbuilder-variable-context-bounded-native-surface-readiness-resolution-v0
  bounded_native_surface_selected = VariableContextNativeSurfaceExplicitMutationApiOnlyV1
entries_snapshot_state = NotNeededForBoundedNativeSurface
next_route_family_selection_policy = SOURCE-SELFHOST-NEXT-ROUTE-FAMILY-SELECTION-POLICY-001
next_route_family_selection_decision = KeepSourceSelfhostStopped
next_route_family_selection_reason = NoEligibleNativeAdoptionCandidate
next_route_family_selection_recovery = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
next_route_family_selection_recovery_contract = rust-lifecycle-source-selfhost-wider-route-selection-design-stop-v0
consultation_gated_wider_route_selection = 1
manual_family_selection = 0
machine_derived_route_repair_allowed = 1
classification_partition_complete = 1
support_lane_projector_as_adoption_candidate = 0
mut_lease_state = DeferredUntilLiveNeed
mainline_readiness = Ready
mainline_readiness_decision = ReadyForMinimalPathMainlinePilot
mainline_next_unconsumed_edge = Closed
mainline_generated_hako_executable_closure = Closed
mainline_same_state_handoff_observed = 1
mainline_pilot_next_slice = MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001
artifact_selfhost_checkpoint = ARTIFACT-SELFHOST-CHECKPOINT-001
mainline_pilot = MIRBUILDER-MINIMAL-PATH-MAINLINE-PILOT-001
source_selfhost_adoption_plan = SOURCE-SELFHOST-ADOPTION-PLAN-001
blocked_recovery_diagnostic = SOURCE-SELFHOST-BLOCKED-RECOVERY-DIAGNOSTIC-001
native_surface_selection = VARIABLE-CONTEXT-NATIVE-SURFACE-ADOPTION-SELECTION-001
native_surface_adoption = VARIABLE-CONTEXT-NATIVE-SURFACE-HAKO-ADOPTION-DECISION-001
post_surface_resolution = SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-SURFACE-RESOLUTION-001
returned_read_snapshot_route = MIRBUILDER-VARIABLE-CONTEXT-RETURNED-READ-SNAPSHOT-ROUTE-001
owned_read_snapshot_projection_card = MIRBUILDER-VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-PROJECTION-001
route_matrix_rerun = MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-001
owned_read_snapshot_surface_adoption = VARIABLE-CONTEXT-OWNED-READ-SNAPSHOT-HAKO-ADOPTION-DECISION-001
post_owned_snapshot_resolution_card = SOURCE-SELFHOST-POST-VARIABLE-CONTEXT-OWNED-SNAPSHOT-RESOLUTION-001
explicit_mutation_surface_selection = MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-SURFACE-SELECTION-001
explicit_mutation_projection = MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001
route_matrix_rerun_002 = MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002
replace_owned_map_native_api = 1
explicit_mutation_surface_state = Adopt
post_explicit_mutation_resolution_contract = rust-lifecycle-source-selfhost-post-variable-context-explicit-mutation-resolution-v0
next_action = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
resume_condition = ConsultationGatedWiderRouteSelectionOrMachineDerivedRouteRepair
runner_task_breakdown = SOURCE-SELFHOST-RUNNER-AND-ROUTE-TASK-BREAKDOWN-001
runner_task_breakdown_output = rust-lifecycle-source-selfhost-runner-and-route-task-breakdown-v0
single_hako_meaning_source = 1
runner_semantic_owner = 0
future_interpreter_required_for_projector_migration = 0
next_task_pack = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-RESOLUTION-001
route_selection_basis = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-001
route_selection_basis_output_contract = rust-lifecycle-source-selfhost-wider-route-selection-basis-v0
route_selection_resolution = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-RESOLUTION-001
route_selection_resolution_output_contract = rust-lifecycle-source-selfhost-wider-route-selection-resolution-v0
docs_guard_maintenance_reduction = SOURCE-SELFHOST-DOCS-GUARD-MAINTENANCE-REDUCTION-001
docs_guard_maintenance_output = rust-lifecycle-source-selfhost-docs-guard-maintenance-reduction-v0
readiness_guard_realignment = MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-GUARD-REALIGNMENT-001
current_pointer_decouple = GUARD-SOURCE-SELFHOST-CURRENT-POINTER-DECOUPLE-001
source_selfhost_family_guard = GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001
source_selfhost_family_guard_output = rust-lifecycle-source-selfhost-family-guard-manifest-v0
compact_current_state = DOCS-SOURCE-SELFHOST-COMPACT-CURRENT-STATE-001
task_order_thinning = DOCS-SOURCE-SELFHOST-TASK-ORDER-THINNING-001
check_index_family_view = DOCS-CHECK-INDEX-FAMILY-VIEW-001
post_maintenance_task_inventory = SOURCE-SELFHOST-POST-MAINTENANCE-TASK-INVENTORY-001
maintenance_sequence_next =
  MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
deferred_semantic_next =
  MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
native_slice_decomposition =
  MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-SLICE-DECOMPOSITION-001
native_slice_decomposition_output =
  rust-lifecycle-mirbuilder-minimal-path-composed-closure-native-slice-decomposition-v0
native_slice_decomposition_decision = KeepStopped
native_slice_decomposition_reason = NoCandidateAfterNativeSliceDecomposition
native_slice_decomposition_candidate_eligible_count = 0
native_slice_decomposition_repairable_inconsistency_count = 0
native_slice_decomposition_selected_next_card =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
native_slice_decomposition_manual_family_selection = 0
native_slice_decomposition_source_selfhost_claim = 0
native_owner_seed_inventory =
  MIRBUILDER-MINIMAL-PATH-COMPOSED-CLOSURE-NATIVE-OWNER-SEED-INVENTORY-001
native_owner_seed_inventory_output =
  rust-lifecycle-mirbuilder-minimal-path-composed-closure-native-owner-seed-inventory-v0
native_owner_seed_inventory_decision = KeepStopped
native_owner_seed_inventory_reason = NoNativeOwnerSeedCandidate
native_owner_seed_candidate_count = 0
composition_owner_as_semantic_owner = 0
generated_artifact_as_edit_authority = 0
native_owner_seed_selected_next_card =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
generated_artifact_to_native_owner_seed_policy =
  MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001
generated_artifact_to_native_owner_seed_policy_output =
  rust-lifecycle-mirbuilder-generated-artifact-to-native-owner-seed-policy-v0
generated_artifact_to_native_owner_seed_policy_decision = PolicyDefined
generated_artifact_to_native_owner_seed_next_resolution =
  MIRBUILDER-GENERATED-ARTIFACT-NATIVE-OWNER-SEED-CANDIDATE-RESOLUTION-001
seed_candidate_requires_leaf_semantic_owner = 1
seed_candidate_requires_verified_artifact = 1
seed_candidate_requires_deterministic_regeneration = 1
seed_materialization_is_separate_card = 1
hako_adoption_decision_is_separate_card = 1
native_owner_seed_pilot_target_selection =
  MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-001
native_owner_seed_pilot_target_selection_output =
  rust-lifecycle-mirbuilder-native-owner-seed-pilot-target-selection-v0
native_owner_seed_pilot_selected_target = ReturnEmission
native_owner_seed_pilot_next_card =
  MIRBUILDER-RETURN-EMISSION-HAKO-NATIVE-SOURCE-SEED-001
return_emission_native_source_seed =
  MIRBUILDER-RETURN-EMISSION-HAKO-NATIVE-SOURCE-SEED-001
return_emission_native_source_seed_output =
  rust-lifecycle-mirbuilder-return-emission-hako-native-source-seed-v0
return_emission_native_source_path =
  lang/src/compiler/lib/return_emission_native_seed.hako
return_emission_generator_overwrite_guard = 1
return_emission_hako_adoption_decision =
  MIRBUILDER-RETURN-EMISSION-HAKO-ADOPTION-DECISION-001
return_emission_hako_adoption_decision_output =
  rust-lifecycle-mirbuilder-return-emission-hako-adoption-decision-v0
return_emission_hako_adopted = 1
return_emission_source_selfhost_claim = 0
native_owner_seed_pilot_target_selection_002 =
  MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-002
native_owner_seed_pilot_002_selected_target = FunctionRegionStackPop
native_owner_seed_pilot_002_next_card =
  MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001
function_region_stack_pop_native_source_seed =
  MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-NATIVE-SOURCE-SEED-001
function_region_stack_pop_native_source_path =
  lang/src/compiler/lib/function_region_stack_pop_native_seed.hako
function_region_stack_pop_generator_overwrite_guard = 1
function_region_stack_pop_hako_adoption_decision =
  MIRBUILDER-FUNCTION-REGION-STACK-POP-HAKO-ADOPTION-DECISION-001
function_region_stack_pop_hako_adopted = 1
function_region_stack_pop_source_selfhost_claim = 0
native_owner_seed_pilot_target_selection_003 =
  MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-003
native_owner_seed_pilot_003_selected_target = SlotRegistryRelease
native_owner_seed_pilot_003_next_card =
  MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001
slot_registry_release_native_source_seed =
  MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-NATIVE-SOURCE-SEED-001
slot_registry_release_hako_adoption_decision =
  MIRBUILDER-SLOT-REGISTRY-RELEASE-HAKO-ADOPTION-DECISION-001
slot_registry_release_hako_adopted = 1
current_seed_pilot_targets_all_adopted = 1
support_lane_projector_as_hako_adoption_candidate = 0
support_lane_projector_as_seed_pilot_target = 1
conditional_route_matrix_repair =
  <ROUTE-FAMILY>-ROUTE-MATRIX-REPAIR-001
  only_after = ConcreteRouteMatrixInconsistency
old_1650_design_stop = provenance_only
```

forbidden:
  callee-name branches; C-side ArrayBox inference; scalar fail-code
  reinterpretation; mixed-runtime promotion; extern fallback; new route kind;
  new canonical MIR instruction; runtime fallback
```

## Evidence Pointers

```text
semantic_closure_report =
  docs/development/current/main/design/fixtures/rust-lifecycle/
  minimal-mirbuilder-execution-path-semantic-closure-report-v0.json

latest_frontier_card =
  docs/development/current/main/phases/phase-296x/
  1840-MIRBUILDER-LOOP-COND-BC-CLEANUP-EXIT-PREDICATE-PROJECTION-POLICY-001.md

latest_integration_card =
  docs/development/current/main/phases/phase-296x/
  296x-1763-MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-RESOLVER-001.md

latest_guard_repair_card =
  docs/development/current/main/phases/phase-296x/
  1772-MIRBUILDER-COMPOSED-PREFIX-GUARD-DRIFT-REPAIR-001.md

latest_adoption_card =
  docs/development/current/main/phases/phase-296x/
  1771-MIRBUILDER-CONTEXT-HAKO-ADOPTION-DECISION-001.md

latest_native_owner_card =
  docs/development/current/main/phases/phase-296x/
  1770-MIRBUILDER-CONTEXT-HAKO-NATIVE-SOURCE-OWNER-001.md
```

## Active Next 3

Keep this section short. Detailed landed rows belong in phase cards and git
history, not in this task-order SSOT.

```text
1. SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
   status=stop-line
   boundary=current seed pilot targets are adopted, but Source Selfhost remains unclaimed
   semantic_authority=design-stop card, ReturnEmission/FunctionRegionStackPop/SlotRegistryRelease adoption fixtures
   non_authority=manual family selection, runtime fallback, Source Selfhost claim

2. MIRBUILDER-RETURN-EMISSION-HAKO-ADOPTION-DECISION-001
   status=green
   boundary=narrow ReturnEmission leaf HakoAdopted decision
   semantic_authority=native source seed fixture, Derived-to-Native artifact model
   non_authority=Source Selfhost claim, Rust deletion

3. MIRBUILDER-NATIVE-OWNER-SEED-PILOT-TARGET-SELECTION-002
   status=green
   boundary=select next seed pilot by stable priority after ReturnEmission adoption
   semantic_authority=seed target selection fixture, adoption decision fixture
   non_authority=manual family selection, Source Selfhost claim

next_after_active_3 =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Landed Converter Capability Summary

```text
ordered-map contexts, snapshots, carrier projection, scalar counters
TypeContext value-kind / origin-map / value-type / snapshot-restore
MetadataContext scalar/source-file, value-caller
MetadataContext region-parent EXE/AOT
structured loop, scalar loop carrier, explicit PHI, multi-exit PHI
RegionObserver source-ordered read-fold and SlotMetadata output
boxed native enum ABI and boxed enum container round trip
mixed runtime value carrier for MapBox and ArrayBox
generic method route descriptor SSOT and mismatch diagnostics
generic read-fold operation decomposition
TypeContext string literal leaf projection
boxed-sum const payload definition index
boxed-sum lowering facade
C shim variant binding fact owner drain
same-module typed-field RMW fusion plan
same-module result-capsule reset batch fusion plan
same-module boxed-sum handle fact owner selected
same-module explicit boxed-sum value fact
generic-method explicit boxed-sum result fact
```

## Direct-Lowering Policy

The converter is direct-first:

```text
Rust source
  -> lightweight body/signature facts
  -> directability check
  -> typed VerifiedHakoFamilyIR
  -> shared emitter
  -> runnable native-shaped .hako
```

The older lifecycle vocabulary may remain as provenance/guard input for
families that already use it, but it is not the standard path for simple
mechanical shapes.

Use direct shape lowering when all are true:

```text
source body has a bounded shape
all calls are in the allowed vocabulary
field ownership is local to the translated box
no returned mutable alias escapes
no Drop / unsafe / FFI is required
control-flow and PHI facts are explicit when present
generated operation IR is typed before emission
```

When directness cannot be proved, emit a stable `Deny(reason)`. Do not emit
fallback Hako, TODO bodies, null placeholder bodies, or try-Hako-then-Rust
runtime routes.

## Shape Rule Table

The active rule table uses shape names, not family names.

| Shape | Operation family | Status |
| --- | --- | --- |
| `single_ordered_map_context` | `NewOrderedMap`, `MapGetCopied`, `MapHas`, `MapLength`, `MapIsEmpty`, `MapSet`, `MapRemove`, `MapClear` | landed |
| `owned_ordered_map_snapshot` | `CloneOwnedMap`, `ReplaceOwnedMap` | landed |
| `multi_ordered_map_context` | `NewOrderedMap`, `AllMapsEmpty` | landed |
| `scalar_counter_context` | `InitFieldConst`, `TakeThenSaturatingIncrementU32`, `ReturnI64` | landed |
| `owned_map_carrier_projection` | `CarrierSnapshotFromOwnedMap`, `ExplicitCarrierSnapshotFromOwnedMap` | landed |
| `map.optional_copy_default` | `NewMap`, `MapGetOption`, `MapSet`, `ReturnDefaultIfMissing` | landed |
| `map.optional_immutable_atom` | `MapGetOption`, `MapSet`, `MapClear` | landed |
| `aggregate.take_restore_with_defaults` | `MoveFieldAndResetSource`, `AssertNotConsumed`, `MarkConsumed` | landed |
| `control.structured_loop_without_carried_state` | `StructuredLoop`, `ArrayPush`, `Assign`, `ReturnI64` | landed |
| `control.single_scalar_loop_carrier` | `StructuredLoop`, `Assign`, `ReturnSource` | landed |
| `control.canonical_explicit_phi` | `ExplicitPhiI64`, `ReturnSource` | landed |
| `control.multi_carrier_exit_phi` | `ExplicitMultiExitPhiI64Array`, `ReturnSource` | landed |
| `map.immutable_leaf_projection` | `MapGetOption` | active |
| `borrow_use.sequence_last_copy` | `SequenceLastOption` | landed |
| `borrow.read_fold` | map/sequence fold into owned output | landed |

Do not create rules like `type_context.value_kind_map_context`; that is a
family-specific hardcode table under another name.

## Storage Access Facts

`BorrowUseFacts` is Rust-specific adapter input. It is not the universal model
for all source languages. Normalize source-specific references into
`StorageAccessFacts` before Hako lowering:

```text
source-specific facts
  Rust borrow / Go pointer-slice-map / C pointer
        ↓
StorageAccessFacts
        ↓
lowering decision
```

Use small orthogonal facts:

```text
carrier:
  Value | Place | SharedHandle | Span | RawAddress

access:
  Read | ReadWrite | Atomic

alias:
  Unique | Shared | Unknown

lifetime:
  Lexical | OwnerBound | Managed | Foreign | Untracked

escape:
  None | Return | Store | ForeignRetained

order:
  Unobserved | Unspecified | SourceOrdered

cleanup:
  Trivial | Managed | ExplicitRelease | CustomDrop
```

Lowering decisions:

```text
ElideToLeafProjection
ElideToReadFold
FreezeOwned
KeepSharedHandle
MaterializeSharedCell
MaterializeSpan
RequireUnsafeCapability
Deny
```

Current Rust borrow path:

```text
Rust lightweight facts
  -> BorrowUseFacts
  -> StorageAccessFacts
  -> BorrowLoweringDecision
```

Future language adapters can enter directly at `StorageAccessFacts`:

```text
Go map:
  SharedHandle(Map)

Go slice:
  Span or SliceDescriptor(backing=SharedHandle(Buffer), offset, len, cap)

Go address-taken scalar:
  SharedCell only when shared addressable mutation is required

C / unsafe Rust pointer:
  RawAddress, then RequireUnsafeCapability or Deny
```

## Hako Syntax Boundary

Do not add source-language pointer syntax for this lane:

```text
no general &
no general *
no arrow / ->
no general borrow lifetime syntax
no raw pointer syntax in safe Hako
```

If a future source needs shared mutable or span semantics frequently, add a
capability type first:

```text
SharedCell<T>
Span<T>
Slice<T>
ValidatedHandle<T>
RawPtr<T> only inside an unsafe capability boundary
```

Unsafe and foreign are separate axes:

```text
unsafe:
  memory-safety obligation is not compiler-proved

foreign:
  ABI / external symbol / layout boundary
```

Use the stable top-level deny reason and detail fields:

```text
Deny(UnsafeOrFFI)
  detail=RequireUnsafeCapabilityBoundary
  detail=RawAddressRequired
  detail=PointerArithmeticRequired
  detail=UntrackedAliasRequired
  detail=ForeignCallRequired
  detail=LayoutDependentCastRequired
  detail=ManualLifetimeRequired
```

Output from such a boundary may only be a safe value, owned aggregate, owned
buffer, validated opaque handle, or verified box.

## Stable Deny Reasons

Use medium-grained reasons:

```text
UnsupportedResolvedCallTarget
UnsupportedDirectShape
UnsupportedTypeTransport
UnsupportedKeyTransport
UnsupportedOrderCapability
NullableMapValue
DefaultSemanticMismatch
UnstructuredControlFlow
LoopCarriedStateRequired
PhiJoinRequired
ReturnedReadBorrow
ReturnedMutableBorrow
CarrierSensitiveAlias
NonTrivialDrop
UnsafeOrFFI
ConstructorLifecycleMismatch
```

Do not encode family names in Deny reasons.

## Parked Backlog

These are intentionally not part of the current task:

```text
full MirBuilder crate claim
crate-wide generated-to-native authority cutover
variable_map_mut raw alias
live read-view / lease framework
general Drop / RAII lowering
general Option payload support
InlineRecord / packed / SoA SlotMetadata transport
nightly rustc adapter for easy-tier families
runtime try-Hako-then-Rust fallback
new Hako pointer syntax
```

## Task Hygiene Backlog

Keep this lane separate from semantic converter slices:

```text
landed: guard表示の false-green 修正
landed: current docs を thin pointer 化
landed: task-order SSOT を active next 3 + parked index へ圧縮
  boundary=keep task-order as pointer; detailed artifact/evidence rows belong
  to semantic closure reports, phase cards, and git history
landed: compiler projector helper support box v0
  boundary=small lang/src/compiler/lib helper only (_tag/fail/require/copy);
  first users are ReturnEmission, FunctionRegionStackPop, and
  SlotRegistryRelease projectors; no projector framework or semantic DSL
landed: Python semantic projector freeze reverse coverage hardening
  boundary=reverse-enumerate tools/rust_lifecycle/*.py roles and require
  exception tokens for new SemanticProjector files
landed: mirbuilder_family_artifacts.py 分割
  boundary=behavior_preserving_split_only
landed: leaf projection validator 二重化を整理
  boundary=one validator owns map.immutable_leaf_projection acceptance
```

## C ABI Shim Responsibility Cleanup Backlog

See [c-abi-shim-responsibility-cleanup-backlog-ssot.md](./c-abi-shim-responsibility-cleanup-backlog-ssot.md) for the full P0/P1/P2 cleanup inventory.

## MIR Instruction SSOT Cleanup Backlog

This is a cleanup lane, not the active boxed enum ABI blocker.

Accepted finding:

```text
instruction enum / backend ledger / INSTRUCTION_SET.md counts are partially
sync-tested, but docs/reference/mir/json_v0.schema.json is not part of that
sync contract.

src/mir/contracts/backend_core_ops.rs also mixes:
  instruction tag/cohort classification
  per-backend support policy
  ledger constants
  sync tests

docs/reference/mir/INSTRUCTION_SET.md and docs/reference/mir/json_v0.schema.json
are independently maintained outputs today. They are not generated from
src/mir/instruction.rs.
```

Task order:

```text
P1. Add JSON schema to MIR instruction SSOT sync coverage
    - extend the existing backend_core_ops doc-sync tests or add a small
      adjacent test module
    - assert doc <-> ledger <-> json_v0.schema.json agree on kept JSON ops
    - ensure VariantMake/VariantTag/VariantProject and MemOp stay schema-visible
    - no generator
    - no backend behavior change

P2. Generate doc machine-readable rows and JSON schema from enum metadata
    - only if instruction vocabulary starts changing frequently
    - doc / JSON become derived outputs, not independent sources

P3. Split backend_core_ops.rs owners
    - structural instruction classification near the enum/introspection layer
    - per-backend support policy in a policy module
    - tests outside the mixed owner file where practical
```

Immediate recommendation:

```text
implement P1 when the current boxed enum ABI slice has a clean stopping point.
park P2/P3 until churn justifies the extra generator/refactor machinery.
do not block boxed enum ABI work on this cleanup backlog.
```

## Fast-Path Lowering Reminder

Fast-path lowering is important for the long-term speed goal, but it is a
backend/perf lane, not the active Rust-to-Hako converter task.

Tracker:

```text
docs/development/current/main/design/perf-owner-first-optimization-ssot.md
```

Current status:

```text
fastpath_analysis_ready=partial
fastpath_backend_consumption=parked
speed_goal_blocker=backend_lowering_consumer_missing
optimization_open=0
```

Parked task order when perf/backend work reopens:

```text
1. Inventory fast-path fact consumers.
2. Implement the first backend fact consumer.
   preferred first target: DirectArrayI64 / ArrayBox i64 lowering
3. Measure exact / meso / whole fronts.
4. Select the next owner only after measurement.
```

Do not block MirBuilder migration on this backlog. Do not add Hako syntax for
fast paths. Backend facts must be consumed by lowering before claiming speed.

## Completion Boundary

MirBuilder-wide selfhost still has mutable alias, Drop, unsafe/FFI, boxed scalar
payloads, and broader native adoption parked as explicit design stops. Do not
hide them inside the current leaf-projection lane.
