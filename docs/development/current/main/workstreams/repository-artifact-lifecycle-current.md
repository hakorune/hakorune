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

## Current Slice

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

Each batch stops on any unresolved reference, pointer drift, docs-slim failure,
or `dev_gate quick` failure. Only the current batch is reverted.

## Non-Claims

```text
inactive_phase_archive_complete = 0
phase_296x_archive_complete = 0
design_registry_complete = 0
check_script_retirement_complete = 0
docs_private_retention_decided = 0
failure_outcome_design_accepted = 0
selfhost_claim = 0
```
