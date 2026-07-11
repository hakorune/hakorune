---
Status: Active implementation workstream
Date: 2026-07-11
Owner: current-docs-archive-policy-ssot.md
---

# Repository Artifact Lifecycle Current

## Priority

Repository artifact cleanup temporarily precedes the parked 3504
Failure/Outcome design consultation. No language, parser, runtime, or backend
semantics change in this workstream.

```text
parked_next:
  docs/development/current/main/phases/phase-296x/3505-LANGV1-FAILURE-OUTCOME-RELATION-INVENTORY-001.md

active_order:
  H0 inventory + warning guard
  H1 phase-296x bounded archive batches
  H2 inactive phase archive
  H3 design/README authority registry
  H4 check-script manifest convergence
  H5 lifecycle enforcement
```

## 3504 / 3505 Failure-Outcome Handoff

The 3504 design consultation is accepted as relation/spec plus exhaustive
inventory only. It does not authorize grammar, runtime, VM, cleanup, or
backend behavior changes.

```text
3504_decision = accepted
first_slice = relation_spec_and_exhaustive_inventory
canonical_unit = void
canonical_absence = Option::None
canonical_recoverable_failure = Result::Err
canonical_fault = outcome_not_value
canonical_catchable_fault_count = 0
uninitialized_local = slot_only_state
weak_upgrade_target = Option::Some_or_None
foreign_null = boundary_only
compat_null = Compat2025_only

3505_status = parked
3505_start_condition = repository artifact lifecycle C2 blocker resolved
3505_behavior_change = 0
```

## H0 Inventory Owner

H0 creates one deterministic inventory generator and one manifest. It derives
archive candidates from card status plus tracked references, including links
from other cards in the active phase while excluding only card self-reference. Warning
mode reports drift without blocking ordinary development; archive batches use
strict mode before moving files.

H0 evidence:

```text
inventory generator = tools/docs/repository_artifact_lifecycle_inventory.py
inventory manifest = tools/checks/manifests/repository_artifact_lifecycle_v0.json
current pointer guard = green
docs slim archive policy guard = green
```

## H1 Entry Conditions

```text
inventory strict check = green
current pointer guard = green
phase resolver supports phase-296x = 1
first move batch <= 200
candidate status = closed
tracked external reference = 0
```

The shared `phase_card_path` resolver owns live/archive lookup. Phase 293x
keeps its bucketed archive compatibility wrapper; phase 296x resolves its
existing flat `archive/` layout.

Each batch stops on any unresolved reference, pointer drift, docs-slim failure,
or `dev_gate quick` failure. Only the current batch is reverted.

## H1 Batch Ledger

```text
batch-001:
  moved = 200
  phase_direct = 2123 -> 1923
  phase_archive = 1425 -> 1625
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

batch-002:
  moved = 200
  phase_direct = 1923 -> 1723
  phase_archive = 1625 -> 1825
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

batch-003:
  moved = 200
  phase_direct = 1723 -> 1523
  phase_archive = 1825 -> 2025
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

batch-004:
  moved = 200
  phase_direct = 1523 -> 1323
  phase_archive = 2025 -> 2225
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

batch-005:
  moved = 200
  phase_direct = 1323 -> 1123
  phase_archive = 2225 -> 2425
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

batch-006-final:
  moved = 92
  phase_direct = 1123 -> 1031
  phase_archive = 2425 -> 2517
  archive_candidate_count = 0
  protected_referenced = 851
  protected_needs_review = 177
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

H1 closeout:
  total_moved = 1092
  closed_unreferenced_candidate_drain = complete
  broad_phase_296x_archive_complete = 0
```

## Current Slice

H2 inventories phase directories outside the active phase before any whole
phase move. A phase is eligible only when current pointers, tracked references,
and phase-local status evidence all prove it inactive. Ambiguous phases remain
in place for review.

```text
phase_directories = 392
strict_inactive_candidates = 87
strict_inactive_candidate_files = 238
largest_candidate = phase-292x (31 files)
whole_phase_move_started = 0

excluded_large_phases:
  phase-293x = local status unresolved + 759 external reference files
  phase-291x = active_like + 14 external reference files
  phase-29cv = active_like + 5 external reference files
  phase-294x = local status unresolved + 80 external reference files
  phase-295x = active_like + 137 external reference files
  phase-29cc = closed + 90 external reference files

next = H2-INACTIVE-PHASE-BATCH-001
```

## H2 Closeout

```text
archived_phase_directories = 87
archived_phase_files = 238
current_phase_directories = 305
remaining_strict_inactive_candidates = 0
inventory_strict = green
current_state_pointer_guard = green
docs_slim_archive_policy_guard = green
dev_gate_quick = green

protected_large_or_ambiguous_phases = unchanged
all_historical_phase_archive_complete = 0

next = H3-DESIGN-AUTHORITY-REGISTRY-INVENTORY
```

## H3 Design Stop

The machine inventory is complete, but authority selection is not mechanical.
The active consultation packet is:

`docs/development/current/main/investigations/repository-artifact-lifecycle-h3-design-registry-consultation.md`

```text
design direct files = 848
seed pointer union = 160
unseeded files = 688
usable closed status = 11
usable active-like status = 127
status unresolved = 689

design_registry_decided = 0
design_file_move_started = 0
```

## H3 Accepted Landing

Candidate A is accepted. `design/INDEX.md` now owns typed membership and
precedence rows. The warning rollout starts with five explicit rows and a
no-growth backlog baseline.

```text
registry_mode = warning
registered_rows = 5
owned_sidecars = 0
unregistered_baseline = 844
unregistered_current = 844
precedence_cycle_count = 0
registry_violation_count = 0

README_role = navigation-only
seed_union_is_authority = 0
strict_mode = 0

next = H2-TRACE-ROOT-REACHABILITY-INVENTORY
```

## H2 Tracing Reachability

Reference counts remain diagnostic only. Archive eligibility now uses
reachability from the accepted active root set.

```text
current_documents = 10627
root_documents = 3529
reachable_documents = 4905
unreachable_documents = 5722

whole_phase_unreachable = 198
whole_phase_unreachable_files = 517
phase_scc_clusters = 189
ambiguous_basenames = 26

root_policy:
  CURRENT_STATE/current entry pointers
  phase-296x direct files
  INDEX authority rows
  docs/reference
  AGENTS/CLAUDE/root README/CURRENT_TASK
  src/tools document references

generated_inventory_is_root = 0
design_unregistered_move_allowed = 0
partial_phase_move_allowed = 0

next = H2-TRACE-WHOLE-PHASE-BATCH-001
```

The relocation owner is
`tools/docs/archive_unreachable_phase_clusters.py`. It preserves every local
Markdown link that resolved before the move, rewrites repository-absolute
phase paths, requires a clean worktree, and regenerates the inventory after
application.

## H2 Tracing Whole-Phase Closeout

```text
moved_phases = 198
moved_files = 517
markdown_links_rewritten = 8
repository_paths_rewritten = 644

current_documents = 10627 -> 10110
archived_phase_directories = 87 -> 285
current_phase_directories = 305 -> 107

remaining_whole_phase_unreachable = 0
remaining_whole_phase_unreachable_files = 0
preserved_preexisting_valid_links = green
old_current_phase_path_residue = 0
inventory_strict = green
current_state_pointer_guard = green
docs_slim_archive_policy_guard = green
dev_gate_quick = green

next = H2-TRACE-PARTIAL-PHASE-CLUSTER-INVENTORY
```

## H2 Partial-Phase Cluster Inventory

```text
partial_phases = 41
unreachable_files = 1834
weakly_connected_clusters = 1008
largest_cluster_files = 503
reachable_to_candidate_edges = 0
archive_target_collisions = 0

batch_law:
  never split a weakly connected cluster
  cluster > 200 files -> one dedicated batch
  remaining clusters -> pack up to 200 files

active_phase_296x = excluded
design_unregistered = excluded

next = H2-TRACE-PARTIAL-PHASE-RELOCATOR-001
```

The partial relocation owner is
`tools/docs/archive_unreachable_partial_phase_clusters.py`. It shares the
whole-phase link rewrite implementation and refuses reachable incoming edges,
archive collisions, dirty worktrees, and split weak components.

## H2 Partial-Phase Batch Ledger

```text
batch-001-dedicated-large-cluster:
  moved_files = 503
  markdown_links_rewritten = 0
  repository_paths_rewritten = 669
  current_documents = 10110 -> 9607
  remaining_files = 1331
  remaining_clusters = 1007
  largest_remaining_cluster = 42
  reachable_incoming_edges = 0
  archive_target_collisions = 0
  inventory_strict = green
  current_state_pointer_guard = green
  docs_slim_archive_policy_guard = green
  dev_gate_quick = green

batch-002:
  moved_files = 200
  repository_paths_rewritten = 227
  current_documents = 9607 -> 9407
  remaining_files = 1131
  remaining_clusters = 998
  largest_remaining_cluster = 15
  all_gates = green

batch-003:
  moved_files = 200
  markdown_links_rewritten = 12
  repository_paths_rewritten = 144
  current_documents = 9407 -> 9207
  remaining_files = 931
  remaining_clusters = 928
  largest_remaining_cluster = 2
  all_gates = green

batches-004-through-007:
  moved_files = 800
  batch_size = 200 each
  all_gates = green

batch-008-final:
  moved_files = 131
  repository_paths_rewritten = 14
  current_documents = 8407 -> 8276
  remaining_files = 0
  remaining_clusters = 0
  reachable_incoming_edges = 0
  archive_target_collisions = 0
  all_gates = green

partial_phase_closeout:
  moved_files = 1834
  current_documents = 10110 -> 8276
  candidate_drain = complete

tracing_closeout:
  whole_phase_files = 517
  partial_phase_files = 1834
  total_moved_files = 2351
  current_documents = 10627 -> 8276
  repository_total_files = unchanged_by_design

next = H3-DESIGN-REGISTRY-CLASSIFICATION-DESIGN-STOP
```

## H3 C1 Closeout

C1 classifies only direct design files explicitly named by the checked
README sections. The registry records the evidence section in
`classification_basis`; filename suffixes, status text, and reference
popularity do not assign a role. No design file moved in this slice.

```text
c1_review_basis = explicit README section evidence
c1_review_rows = 112
c1_role_counts:
  authority = 107
  supporting = 2
  status-ledger = 3

registered_rows = 117
owned_sidecars = 0
unregistered_baseline = 732
unregistered_current = 732
precedence_cycle_count = 0
registry_violation_count = 0

current_documents = 8277
whole_phase_unreachable = 0
partial_phase_unreachable = 0
design_file_move_started = 0

inventory_strict = green
current_state_pointer_guard = green
docs_slim_archive_policy_guard = green
dev_gate_quick = green

next = H3-C2-OWNER-FAMILY-REVIEW-DESIGN-STOP
```

## H3 C2 Boundary

C2 reviews the remaining 732 direct design files by explicit owner family.
The queue may be generated deterministically, but a role is not assigned
until an authority spine, precedence parent, and retirement condition are
reviewed for that family. Ambiguous families stop for a focused consultation.

```text
c2_queue_basis = deterministic three-token filename prefix queue only
c2_family_count = 393
c2_multi_file_family_count = 37
c2_singleton_family_count = 356
c2_largest_family = hako-alloc-segment:163
c2_role_assignment = none
owner_family_role_heuristic = forbidden
one_authority_spine_default = 1
multiple_authorities_require_explicit_precedence = 1
physical_move_requires_reference_closure = 1
```

## H3 C2 First Family Stop

The first queued family, `hako-alloc-segment`, is now accepted as four
semantic subfamilies. The queue prefix remains scheduling-only. Individual
roles still require explicit content review, and no physical movement starts
in this slice.

```text
family = hako-alloc-segment
unregistered_files = 163
status_counts = SSOT:64, accepted:21, active:69, mimap_active:9
external_incoming_references = 0
internal_family_references = present
authority_spine = existing allocator authority chain
role_assignment = none
physical_move = forbidden
next = S2 explicit content review projection
```

## H3 C2 Accepted Family Design

```text
semantic_subfamilies:
  segment-lifecycle-and-membership
  segment-allocation-and-local-reuse
  segment-arena-backing-and-residence
  segment-map-and-release

new_family_authority_document = 0
historical_ssot_suffix_implies_authority = 0
authority_spine = existing allocator authority chain
lifecycle_blueprint_registration = prerequisite
role_assignment = explicit content review only
superseded_assignment = 0
physical_move = 0
```

## H3 C2 Family Classification Task

```text
task = H3-C2-HAKO-ALLOC-SEGMENT-FAMILY-CLASSIFICATION
scope = all 163 hako-alloc-segment rows

S1 = establish authority/precedence chain
S2 = generate subfamily and proposed-role review projection
S3 = land reviewed registry rows and unique sidecars
S4 = lower baseline only after full batch is green
S5 = run cycle/orphan/reference/pointer/docs-slim/dev-gate guards

superseded_rows = 0
physical_moves = 0
```

S1 closeout:

```text
registered_rows = 118
unregistered_current = 731
unregistered_baseline = 732
precedence_cycle_count = 0
lifecycle_blueprint_registered = 1
new_authority_documents = 0
next = S2 explicit content review projection
```

S2 projection closeout:

```text
projection_manifest = tools/checks/manifests/hako_alloc_segment_family_projection_v0.json
projection_rows = 163
review_status = pending:163
role_assignment = none
owner_fields_set = 0
precedence_parent_fields_set = 0
sidecar_owner_fields_set = 0
next = explicit content review before S3 registry landing
```

`supporting`, `status-ledger`, and `sidecar` remain review outcomes, not
filename-derived assignments. Bridge rows require individual review.

S3 lifecycle/membership review closeout:

```text
reviewed_base_rows = 2
supporting_rows = 2
owned_sidecars = 2
registered_rows = 120
unregistered_current = 727
unregistered_baseline = 732
precedence_cycle_count = 0
authority_rows_added = 0
superseded_assignment = 0
physical_move = 0
projection_rows_remaining = 159
next = allocation/local-reuse explicit content review
```

The reviewed base rows are the proof-only lifecycle scalar and page-membership
scalar contracts. Their closeout documents are guard-only and are owned as
sidecars by the corresponding base row. No allocator behavior, arena backing,
segment-map mutation, or backend capability is activated by this review.

S3 allocation-readiness review closeout:

```text
reviewed_base_rows = 3
supporting_rows = 3
owned_sidecars = 3
registered_rows = 121
unregistered_current = 725
unregistered_baseline = 732
precedence_cycle_count = 0
authority_rows_added = 0
superseded_assignment = 0
physical_move = 0
projection_rows_remaining = 157
next = modeled-consume/local-free explicit content review
```

The allocation-readiness scalar is proof-only and keeps real allocation/free,
arena backing, raw pointer residence, segment-map lookup, bitmap/OSVM,
threads, providers, and backend matchers inactive. Its guard-only closeout is
the third owned sidecar; modeled consume and local-free rows remain unclassified.

S3 modeled-consume/ledger review closeout:

```text
reviewed_base_rows = 5
supporting_rows = 4
status_ledger_rows = 1
owned_sidecars = 5
registered_rows = 123
unregistered_current = 721
unregistered_baseline = 732
precedence_cycle_count = 0
authority_rows_added = 0
superseded_assignment = 0
physical_move = 0
projection_rows_remaining = 153
next = local-free explicit content review
```

Modeled consume remains a scalar proof route, and modeled ledger remains a
deterministic inventory of modeled tokens. Neither row opens real allocation,
arena residence, raw pointers, segment maps, bitmap/OSVM, threads, providers,
or backend matchers.

S3 local-free chain review closeout:

```text
reviewed_base_rows = 9
status_ledger_rows = 3
supporting_rows = 6
owned_sidecars = 7
registered_rows = 127
unregistered_current = 715
unregistered_baseline = 732
precedence_cycle_count = 0
authority_rows_added = 0
superseded_assignment = 0
physical_move = 0
projection_rows_remaining = 147
next = local-free/reuse residual review
```

The candidate ledger and apply plan are deterministic status ledgers. The
page-model apply and integration rows are bounded supporting composition
surfaces, with their closeout documents owned as sidecars. The reuse closeout
without a current direct base row remains unclassified rather than becoming an
orphan sidecar.

## Non-Claims

```text
strict_inactive_phase_candidate_drain_complete = 1
all_historical_phase_archive_complete = 0
phase_296x_archive_complete = 0
design_registry_complete = 0
design_registry_decided = 1
heuristic_role_assignment = 0
design_file_move_started = 0
check_script_retirement_complete = 0
docs_private_retention_decided = 0
failure_outcome_design_accepted = 1
selfhost_claim = 0
```
