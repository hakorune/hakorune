# 3328 - MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001

## Token

```text
MIRBUILDER-POST-HARD-AUTHORITY-PILOT-NEXT-SEAM-SELECTION-001
```

## Purpose

Consume the first hard-authority pilot evidence and select the next
consultation-gated hard-authority seam without opening runtime route authority.

The selected next seam is:

```text
CompareRhsMaterializationIntentSnapshotBox
```

This selection is deterministic from the post-3327 owner graph: it is the
direct downstream read-only `.hako` semantic DTO owner after
`CompareLoweringSymbolicCommandSnapshotV1`.

## Output Contract

```text
rust-lifecycle-mirbuilder-post-hard-authority-pilot-next-seam-selection-v0
```

## Selected Next Seam

```text
candidate:
  CompareRhsMaterializationIntentBoundary

owner:
  CompareRhsMaterializationIntentSnapshotBox

input_surface:
  CompareLoweringSymbolicCommandSnapshotV1

output_surface:
  CompareRhsMaterializationIntentSnapshotV1

downstream_consumer:
  CompareRhsValueIdResolutionPlanSnapshotBox
```

## Rejected Automatic Follow-ons

```text
CompareLoweringSymbolicCommandSnapshotBox:
  already part of the 3327 pilot

RecipeMatcherObserveOnlyBoundary:
  too close to route authority before a separate consultation

actual ValueId / LocalSSA / MIR Compare / Branch bridges:
  cross mutation or allocation boundaries
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-post-hard-authority-pilot-next-seam-selection-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_post_hard_authority_pilot_next_seam_selection_guard.sh
```

## Claims

```text
post_hard_authority_pilot_next_seam_selected = 1
selected_next_seam = CompareRhsMaterializationIntentBoundary
selected_next_card = MIRBUILDER-HARD-AUTHORITY-PILOT-COMPARE-RHS-MATERIALIZATION-INTENT-001
```

## Non-Claims

```text
next_seam_implemented = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
route_selection = 0
runtime_route_switch = 0
programjson_runtime_route_authority = 0
runtime_fallback = 0
mir_mutation = 0
id_allocation = 0
new_backend_route = 0
new_abi = 0
```
