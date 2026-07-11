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
  docs/development/current/main/phases/phase-296x/3504-LANGV1-FAILURE-OUTCOME-DESIGN-STOP-001.md

active_order:
  H0 inventory + warning guard
  H1 phase-296x bounded archive batches
  H2 inactive phase archive
  H3 design/README authority registry
  H4 check-script manifest convergence
  H5 lifecycle enforcement
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

next = H2-TRACE-PARTIAL-PHASE-BATCH-004
maximum_files = 200
```

## Non-Claims

```text
strict_inactive_phase_candidate_drain_complete = 1
all_historical_phase_archive_complete = 0
phase_296x_archive_complete = 0
design_registry_complete = 0
check_script_retirement_complete = 0
docs_private_retention_decided = 0
failure_outcome_design_accepted = 0
selfhost_claim = 0
```
