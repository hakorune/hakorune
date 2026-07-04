---
Status: SSOT
Date: 2026-07-04
Scope: MirBuilder-only Rust-to-Hako converter task order.
Related:
  - docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md
  - docs/development/current/main/design/rust-lifecycle-projection-ssot.md
  - docs/reference/architecture/rust-to-hako-lifecycle-projection.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
  - docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md
  - docs/development/current/main/design/rust-to-hako-converter-implementation-role-ssot.md
  - docs/development/current/main/design/mirbuilder-authority-based-hako-migration-ssot.md
---

# MirBuilder Rust-to-Hako Converter Task Order

This file is the current task-order entry. It is not a landed-history ledger.
Detailed historical rows live in phase cards and git history.

## Current Target

```text
active blocker:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

current implementation task:
  MIRBUILDER-GENERIC-LOOP-BOOL-LITERAL-CONDITION-CLASSIFIER-HAKO-ADOPTION-DECISION-001.
  The generic_loop_bool_literal_condition_classifier owner is HakoAdopted as
  the one-hundred-twenty-seventh narrow Rust-oracle parity pilot after the
  green 3-row `.hako` EXE parity gate; next is rerun 130.

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
  current pointers: docs/development/current/main/CURRENT_STATE.toml,
    docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-family-guard-manifest-v0.json,
    docs/development/current/main/design/fixtures/rust-lifecycle/source-selfhost-wider-route-selection-resolution-v0.json,
    docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-minimal-path-mainline-readiness-resolution-v0.json
  durable policy: docs/development/current/main/design/mirbuilder-selfhost-checkpoint-roadmap-ssot.md,
    docs/development/current/main/design/derived-to-native-hako-artifact-model-ssot.md,
    docs/development/current/main/design/mirbuilder-authority-based-hako-migration-ssot.md

landed evidence pointer:
  Detailed landed rows live in the route-selection guards, adoption cards, and git history; this task-order keeps the active blocker, fail-fast boundary, and Active Next 3.

selected next task:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-130

normal operating rule:
  One RERUN adopts exactly one owner, advances the pointer, then stops.
  Inventory is recall-only unless explicitly requested; see authority SSOT.
  The compact lane memory aid lives in
  `docs/development/current/main/design/mirbuilder-authority-based-hako-migration-ssot.md`
  under `Working Rules`.

latest design decision:
  basic_block_terminator_classifier is now HakoAdopted as a narrow Rust-oracle
  parity pilot owner after a green 5-row `.hako` EXE parity gate. Basic-block
  terminator classification remains Rust. Backend lowering and MIR mutation
  remain Rust. ArrayStringStoreMicroSeedProofFormatter is now HakoAdopted as a
  narrow Rust-oracle parity pilot owner after a green 1-row `.hako` EXE parity
  gate. Source Selfhost remains unclaimed. ConditionTrueLiteralClassifier is
  now HakoAdopted as a narrow Rust-oracle parity pilot owner after a green
  3-row `.hako` EXE parity gate. Source Selfhost remains unclaimed.
  NormalizedShadowBoolTrueLiteralClassifier is now HakoAdopted as a narrow
  Rust-oracle parity pilot owner after a green 3-row `.hako` EXE parity gate.
  Source Selfhost remains unclaimed.
  GenericLoopBoolLiteralConditionClassifier is now HakoAdopted as a narrow
  Rust-oracle parity pilot owner after a green 3-row `.hako` EXE parity gate.
  Source Selfhost remains unclaimed. The selected next card is
  `MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-130`.

current fail-fast boundary:
  Do not re-enter full converter route selection without new non-self-signed authority or stable input delta. The adopted pilot scopes are narrow pure owners only: classification/formatting vocabulary, tiny mutation-frame leaves, label/tag surfaces, and fixture-backed parity helpers.
  Recent adopted formatters cover Sum variants, MirType, effect/capability plans, route/proof labels, loop deny reasons, byte-boundary proof labels, array-text effect labels, len-window modes, observer region-contract effects, combined-region add-const-one classification, loop-session region-payload add-const-one classification, BasicBlock empty-state classification, BasicBlock terminated-state classification, BasicBlock sealed-state classification, ArrayTextResidenceSession safe-bookkeeping classification, call-lowering constructor-name classification, call-resolution commonly-shadowed-method classification, call-resolution extern-function classification, and call-resolution math-function classification. Metadata refresh, route collection, const emission, full AST traversal, method dispatch, string corridor fact inference, same-module definition closure collection, receiver origin resolution, key route classification, route convergence, closure callsite canonicalization, NewClosure rewrite, lowering execution, Region construction, GC retain/release, loop feature extraction, loop route classification, planner route selection, thin-entry candidate collection/selection, manifest generation, observer route derivation, region matching, combined region planning, and MirBuilder mutation remain Rust.
  Payload-type layout binding, sum placement layout refresh, string-kernel plan construction, legality analysis, publication logic, array/text route matching, exact-shape payload construction, session derivation, executor planning, sum route matching, payload extraction, helper emission, MirType tree traversal, global-call route collection, rune profile expansion, effect/capability plan construction/verification, FastMemory fact construction/region analysis, constructor route collection/callee classification, planner order/selection, legacy observer shadow decisions, loop route candidate collection, runtime route selection, array RMW matching, array receiver proof, array RMW add1 leaf seed matching, array get/set micro seed matching, array string-store micro seed matching, concat const suffix seed matching, array text loopcarry matching, array string length window matching, string direct-set window matching, substring views micro seed matching, UserBox loop micro seed matching, exact seed backend route selection, exact seed payload route migration, UserBox known receiver method seed matching, UserBox local scalar seed matching, seed payload migration, indexOf search micro seed matching, backend action execution, array text edit matching, edit payload migration, string corridor relation detection, sum placement fact collection, objectization policy, string corridor candidate derivation, publication policy, and backend lowering remain Rust.
  Escape use classification, operand-role policy, inline plan construction, inline shape verification, MIR metadata emission, generic-loop shape detection/resolution, loop body analysis, loop-session plan construction, region payload derivation, DirectArray plan construction/proof derivation, DirectArray lowering selection, BoxedSum ABI plan construction/site lookup, BoxedSum lowering, global-call target shape inference/route collection, lowering decision, MIR instruction traversal, receiver-origin classification, publication proof construction, LocalFastPathFact generation, publication-site MIR mutation, Copy instruction emission, dominance checks, test-only copy-emission reasons, CorePlan/Facts flowbox classification, facts-to-feature extraction, tag emission, Freeze contract, stderr write, Callee/ValueId analysis, emit-guard instruction analysis, emit-guard scope validation, index route selection, property registry state, call target resolution, MIR instruction traversal, MIR instruction display formatting, memory operation semantics, FastMem handling, function signature preparation, map lookup fusion route derivation, generic method route analysis, route selection, Freeze construction/message formatting, planner fail-fast policy, global-call target/shape/proof analysis, result-origin mapping, definition-owner inference, route collection, and proof policy remain Rust.

historical design decision:
  array_text_observer_executor_materialization_policy_formatter is now
  HakoAdopted as a narrow Rust-oracle parity pilot owner after a green 1-row
  `.hako` EXE parity gate.
  Array-text route matching, observer contract handling, backend lowering, and
  MIR mutation remain Rust.
  The selected next card is
  `MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-089`.

historical route-selection decision:
  Detailed landed route-selection rows are closed as provenance and live in
  phase cards, fixture guards, and git history. This task-order stays a compact
  pointer to the active pivot and next 3 tasks.

## Converter Completion Task Inventory

```text
source_selfhost_status = Stopped
source_selfhost_blocker = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

latest_diagnostics:
  typed_dependency_root_authority_basis = defined
  latest_evidence_inventory_token = MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-EDGE-EVIDENCE-INVENTORY-001
  latest_derivation_basis_token = MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-REFERENCE-EDGE-DERIVATION-BASIS-001
  latest_taxonomy_inventory_token = MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-INVENTORY-001
  latest_taxonomy_authority_token = MIRBUILDER-DOMAIN-OBJECT-ID-RETURN-TYPE-RESOURCE-TAXONOMY-AUTHORITY-001
  latest_registry_authority_token = MIRBUILDER-DOMAIN-OBJECT-ID-STABLE-TYPE-RESOURCE-REGISTRY-AUTHORITY-001
  latest_explicit_declaration_basis_token = MIRBUILDER-DOMAIN-OBJECT-ID-EXPLICIT-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-BASIS-001
  latest_wider_selector_token = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-008
  latest_remaining_axis_priority_basis_token = MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-BASIS-001
  latest_remaining_axis_priority_rerun_token = MIRBUILDER-CARRIER-TYPE-TRANSPORT-REMAINING-AXIS-PRIORITY-RERUN-001
  latest_wider_selector_basis_010_token = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-010
  latest_stop_token = MIRBUILDER-DOMAIN-OBJECT-ID-SEMANTIC-RESOURCE-DOMAIN-DECLARATION-INVENTORY-001
  selector_basis_token = MIRBUILDER-DOMAIN-OBJECT-ID-TYPED-DEPENDENCY-ROOT-AUTHORITY-BASIS-001
  previous_selector_basis_token = MIRBUILDER-DOMAIN-OBJECT-ID-SUBAXIS-MECHANICAL-SELECTION-BASIS-001
  previous_selector_rerun_token = MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RERUN-002
  previous_ledger_token = MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-RERUN-002
  previous_stop_token = MIRBUILDER-DOMAIN-OBJECT-ID-UNRESOLVED-SUBAXIS-PRIORITY-RESOLUTION-001
  unconverted_surface_count = 1584
  missing_projection_policy_count = 1004
  borrow_surface_needs_policy_count = 112
  type_transport_missing_item_count = 944
  carrier_type_unclassified_count = 130
  DomainObjectOrIdTransportAxis = 116
  legacy_id_scalar_domain_rows = 31
  unresolved_non_id_domain_rows = 85
  unresolved_non_id_subaxis_candidate_count = 5
  accepted_typed_dependency_edge_count = 0
  dependency_root_candidate_count = 0
  return_type_field_reference_candidate_count = 85
  return_type_field_as_edge_by_itself = 0
  distinct_return_type_count = 44
  edge_ready_return_type_count = 0
  return_type_resource_taxonomy_entry_count = 0
  resolved_type_decl_ref_count = 0
  resource_taxonomy_join_ready_count = 0
  accepted_registry_authority_source_count = 0
  registry_ready_row_count = 0
  domain_object_id_lane_parked = 1
  post_domain_object_id_eligible_lane_count = 1
  guard_clean_candidate_count = 5
  proof_tuple_complete_candidate_count = 0
  selection_eligible_subaxis_count = 0
  domain_object_id_subaxis_mechanical_selector_basis = defined

completed:
  VariableContext returned read borrow -> OwnedReadSnapshotProjection
  VariableContext returned mutable borrow -> ExplicitMutationApiOnly
  ReturnEmission / FunctionRegionStackPop / SlotRegistryRelease HakoAdopted
  projection descriptor queue exhausted
  Other shape queue exhausted
  strict-deny near-miss projection clusters exhausted
  returned mutable borrow cluster -> BoundedWithMapOperation policy
  current_bindings bounded mutation-frame descriptor
  multi-axis diagnostic cluster resolution
  carrier/type transport policy inventory
  strict converter emission probe
  native-owner seed capability rerun 003
  strict-emission -> native seed bridge policy
  strict-emission native seed candidate selection
  core_context native source seed
  core_context HakoAdopted decision
  native-owner seed capability rerun 004
  metadata_context native source seed
  metadata_context HakoAdopted decision
  native-owner seed capability rerun 005
  type_context native source seed
  type_context HakoAdopted decision
  native-owner seed capability rerun 006

bridge_progression_summary:
  post_rerun_006_basis -> unconverted_report_rerun_002 -> native_seed_rerun_007
  -> bridge_blocked_reason_axis_resolution -> bridge_blocked_gap_cluster_resolution
  result = selected bridge_gap::carrier_type_transport_only
  carrier_type_transport_only_count = 23
  mixed_borrow_carrier_type_transport_count = 1
  invariant = no manual family/shape/axis/cluster selection, source_selfhost_claim=0

result_carrier_bridge_v2_chain_result:
  1980_to_1985 = carrier/type evidence -> ResultCarrierVerifierProjectionPolicy
  1986_to_1988 = denied-boundary normalization kept Source Selfhost stopped
  1989 = 12 forbidden nonclaim occurrences scoped as wider mention-only
  1990 = BridgePolicyV2 defined; mention-only is neither evidence nor blocker
  1991 = selected hakorune_mir_builder::direct_state_plan_refresh native seed
  invariant = source_selfhost_claim=0, runtime_fallback=0, new backend/ABI=0

bridge_policy_v2_native_owner_result:
  direct_state_plan_refresh / record_packed_layout_refresh / typed_object_plan_refresh
  native seeds materialized and adopted in sequence.
  strict_candidate_selection_rerun_005:
    already_hako_adopted_count = 3
    bridge_eligible_remaining_count = 0
    selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
  invariant = source_selfhost_claim=0, runtime_fallback=0, new backend/ABI=0

post_bridge_policy_v2_basis_003_result:
  token = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-003
  output_contract = rust-lifecycle-source-selfhost-wider-route-selection-basis-003-v0
  basis_kind = PostBridgePolicyV2ExhaustionLaneSelector
  native_owner_adoption_delta_count = 3
  unconverted_surface_report_fresh = 0
  decision = SelectUnconvertedSurfaceReportRerun
  reason_token = SourceSurfaceReportStaleAfterNativeOwnerAdoption
  selected_next_card = MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003
  source_selfhost_claim = 0

unconverted_surface_report_rerun_003_result:
  token = MIRBUILDER-UNCONVERTED-SURFACE-REPORT-RERUN-003
  projection_descriptor_ledger_hash_fresh = 1
  native_owner_adoption_ledger_hash_fresh = 1
  native_owner_adoption_delta_count = 3
  missing_projection_policy_count = 1384
  borrow_policy_needed_count = 112
  decision = KeepStopped
  selected_next_card = SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001
  source_selfhost_claim = 0

native_owner_checkpoint_result:
  token = SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-001
  native_owner_count = 11
  missing_projection_policy_count = 1384
  missing_projection_evidence_quality_count = 1199
  borrow_surface_evidence_quality_count = 0
  decision = SelectMissingProjectionPolicyClusterResolutionV2
  selected_next_card = MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2
  source_selfhost_claim = 0

missing_projection_policy_cluster_resolution_v2_result:
  token = MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V2
  input_candidate_count = 1384
  selection_eligible_cluster_count = 41
  excluded_existing_decision_cluster_count = 41
  selectable_cluster_count = 0
  decision = SelectProjectionDescriptorCoverageReclassification
  selected_next_card = MIRBUILDER-PROJECTION-DESCRIPTOR-COVERAGE-RECLASSIFICATION-001
  source_selfhost_claim = 0

missing_projection_policy_cluster_resolution_v3_result:
  token = MIRBUILDER-MISSING-PROJECTION-POLICY-CLUSTER-RESOLUTION-V3
  input_candidate_count = 1004
  cluster_count = 78
  type_transport_missing_cluster_count = 76
  selected_next_card = MIRBUILDER-CARRIER-TYPE-TRANSPORT-POLICY-INVENTORY-RERUN-002
  source_selfhost_claim = 0

carrier_type_transport_evidence_inventory_rerun_002_result:
  token = MIRBUILDER-CARRIER-TYPE-TRANSPORT-EVIDENCE-INVENTORY-RERUN-002
  input_candidate_count = 944
  unclassified_evidence_count = 130
  selected_next_card = MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001
  source_selfhost_claim = 0

carrier_type_transport_unclassified_evidence_resolution_result:
  token = MIRBUILDER-CARRIER-TYPE-TRANSPORT-UNCLASSIFIED-EVIDENCE-RESOLUTION-001
  unclassified_input_count = 130
  DomainObjectOrIdTransportAxis = 116
  selected_next_card = MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001
  source_selfhost_claim = 0

domain_object_id_transport_policy_inventory_result:
  token = MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001
  domain_object_id_input_count = 116
  IdScalarDomainTransportAxis = 31
  selected_next_card = MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001
  source_selfhost_claim = 0

id_scalar_transport_chain_summary:
  2013_to_2015 = nominal transport policy -> directability rerun -> native seed survey rerun 009
  id_scalar_input_count = 31
  directable_row_count = 19
  directable_owner_edge_count = 4
  selected_next_card = MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001
  source_selfhost_claim = 0

id_scalar_seed_selection_pre_basis_summary:
  2016_to_2018 = equal clusters -> seed readiness -> owner-edge repair
  repaired_row_count = 12
  selected_next_card = MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002
  source_selfhost_claim = 0

id_scalar_domain_seed_readiness_resolution_002_result:
  token = MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002
  readiness_input_owner_edge_count = 10
  seed_materialization_ready_count = 0
  reason_token = NoIdScalarSeedMaterializationReadyOwnerEdgeAfterOwnerEdgeRepair
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
  source_selfhost_claim = 0

id_scalar_seed_evidence_contract_result:
  token = MIRBUILDER-ID-SCALAR-SEED-EVIDENCE-CONTRACT-001
  contract_id = IdScalarSeedEvidencePacketV1
  directability_only_is_seed_evidence = 0
  selected_next_card = MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001
  source_selfhost_claim = 0

id_scalar_source_plan_basis_chain_summary:
  2021_to_2026 = seed packet selection -> derivability -> derivation basis -> source surfaces -> operation vocabulary -> rerun 002
  packet_generation_candidate_count = 10
  ambiguous_candidate_count = 4
  source_plan_derivation_basis_defined = 1
  required_source_surface_count = 102
  operation_vocabulary_token_count = 28
  unknown_operation_count = 0
  source_plan_derivable_count = 0
  reason_token = IdScalarSourcePlanDerivabilityRequiresScopeAndRecipeBasis
  selected_next_card = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-BASIS-COMPONENT-PRIORITY-RESOLUTION-001
  source_selfhost_claim = 0

id_scalar_owner_scope_basis_summary:
  2027_to_2030 = owner-scope priority -> state-target root blocker -> 22 target enumeration
  cross_owner_state_target_count = 4
  selected_next_card = MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002
  source_selfhost_claim = 0

id_scalar_bounded_domain_basis_summary:
  2031_to_2034 = bounded rerun -> seed file boundary -> IdDomainBoundary
  owner_scope_bounded_count = 2
  id_domain_boundary_count = 3
  selected_next_card = MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001
  source_selfhost_claim = 0

id_scalar_state_mutation_frame_basis_result:
  token = MIRBUILDER-ID-SCALAR-STATE-MUTATION-FRAME-BASIS-001
  mutation_frame_count = 3
  selected_next_card = MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001
  source_selfhost_claim = 0

id_scalar_error_and_deterministic_order_basis_result:
  token = MIRBUILDER-ID-SCALAR-ERROR-AND-DETERMINISTIC-ORDER-BASIS-001
  error_semantics_count = 6
  deterministic_order_count = 3
  selected_next_card = MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001
  source_selfhost_claim = 0

id_scalar_behavior_recipe_effect_coverage_basis_result:
  token = MIRBUILDER-ID-SCALAR-BEHAVIOR-RECIPE-EFFECT-COVERAGE-BASIS-001
  effect_class_count = 6
  selected_next_card = MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001
  source_selfhost_claim = 0

id_scalar_verifier_input_contract_basis_result:
  token = MIRBUILDER-ID-SCALAR-VERIFIER-INPUT-CONTRACT-BASIS-001
  input_fact_set_count = 6
  selected_next_card = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003
  source_selfhost_claim = 0

id_scalar_source_plan_derivability_rerun_003_result:
  token = MIRBUILDER-ID-SCALAR-SOURCE-PLAN-AND-RECIPE-DERIVABILITY-RESOLUTION-003
  source_plan_derivable_count = 2
  reason_token = MultipleEqualIdScalarSourcePlanDerivabilityCandidates
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
  source_selfhost_claim = 0

parent_owned_subject_boundary_resolution_result:
  token = MIRBUILDER-ID-SCALAR-PARENT-OWNED-SUBJECT-BOUNDARY-RESOLUTION-001
  reason_token = ContextRegistryRemainsParentOwnedNotSeedEligible
  selected_next_card = SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-007
  source_selfhost_claim = 0

placement_rule:
  scan_unit = rust_function_or_method
  classification_unit = semantic_owner_edge
  native_seed_file_unit = owner_module_bounded_surface_set
  adoption_unit = semantic_owner_or_bounded_surface_set
  authority = derived-to-native-hako-artifact-model-ssot.md
  rust_file_path_as_hako_authority = 0
  one_function_one_hako_file = 0

forbidden:
  strict rule weakening as executable conversion
  manual family / shape / axis selection
  cluster size or coverage percentage as proof
  generated artifact as native edit authority
  runtime fallback, new backend route, new ABI, new Python SemanticProjector
  Source Selfhost claim
```

## Evidence Pointers

Detailed evidence lives in phase cards, fixtures, and git history.

## Active Next 3
```text
1. SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-BASIS-011
   status=closed; boundary=NoMachineDerivedPostMissingProjectionPolicyWiderLane

2. SOURCE-SELFHOST-LOCAL-CANDIDATE-SELECTION-POLICY-001
   status=closed; boundary=SourceSelfhostLocalCandidateSelectionPolicyDefined

3. SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-CLOSEOUT-001
   status=closed; boundary=SourceSelfhostRouteSelectionExhaustedNoMachineDerivedNextLane

next_documented_task =
  stopped at Source Selfhost wider route-selection design stop

next_after_active_3 =
  reentry requires new non-self-signed authority or stable input delta
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

Current status: accepted resolver framework; exact-AOT sweep closed with no
fresh owner; primitive-family remains provisional; backend lowering consumer is
the speed blocker. Parked order when perf/backend reopens: inventory consumers,
implement the first backend fact consumer, measure, then select next owner.

Do not block MirBuilder migration on this backlog. Do not add Hako syntax for
fast paths. Backend facts must be consumed by lowering before claiming speed.

## Completion Boundary

MirBuilder-wide selfhost still has mutable alias, Drop, unsafe/FFI, boxed scalar
payloads, and broader native adoption parked as explicit design stops. Do not
hide them inside the current leaf-projection lane.
