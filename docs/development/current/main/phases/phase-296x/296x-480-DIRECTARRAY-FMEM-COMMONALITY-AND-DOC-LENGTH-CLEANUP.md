---
Status: Active
Date: 2026-06-06
Scope: task incorporation for DirectArray/FastMemory proof commonality and active-doc length cleanup.
Related:
  - docs/development/current/main/design/mir-proof-envelope-v0-ssot.md
  - docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md
  - docs/development/current/main/investigations/current-docs-length-audit-2026-06-06.md
  - docs/development/current/main/DOCS_LAYOUT.md
---

# 296x-480 DirectArray/FastMemory Commonality And Doc Length Cleanup

## Decision

Add the DirectArray/FastMemory commonality work as a task, but keep it out of
the current FastMemory owner-runtime producer implementation.

```text
Accepted:
  DirectArray and FastMemory may share ProofEnvelopeV0 / RangeIndexFact-style
  reporting inputs.

Rejected for current lane:
  DirectArray access does not auto-generate a fastmem region.
  DirectArrayAccessPlan and FastMemAccessPlan are not merged.
  DirectArrayExtentFact is not reused as FastMemory table length proof.
```

This preserves the "envelope is generic, payload is specific" boundary.

## Task Insert

```text
DIRECTARRAY-FMEM-COMMON-001:
  Add a report/check adapter that can emit shared ProofEnvelopeV0-style identity
  for DirectArray and FastMemory proof sites.

Scope:
  proof/report vocabulary only
  no source syntax change
  no DirectArray auto-fastmem region
  no shared access-plan payload
  no LLVM lowering behavior change

Open after:
  MIR-FMEM-008D owner-runtime producer pilot
  or earlier only if 008D reveals direct proof-report duplication.
```

Park any DirectArray-to-fastmem automatic lowering proposal behind a separate
reference decision. It would change source/lowering semantics and must not be
snuck in as a proof-envelope cleanup.

## Long-Doc Cleanup Task Insert

The active docs length audit found active current docs over the 1000-line
maintenance threshold. This card only fixes the task routing; the physical
slimming should happen in dedicated docs-slim rows so it does not mix with
FastMemory producer work.

```text
DOCS-SLIM-296X-001:
  Slim docs/development/current/main/workstreams/mimalloc-current.md.
  Keep current decisions, next tasks, daily commands, and parking lot.
  Move old evidence sections to investigation/archive docs with stubs.

DOCS-SLIM-296X-002:
  Slim docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md.
  Keep current blocker and queue summary.
  Move old row detail and historical restart queues to a phase-local archive.

DOCS-SLIM-FMEM-SSOT-001:
  Split docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md.
  Keep only capability-gap decisions and current implementation order in the
  SSOT; move detailed report-field ledgers to a companion investigation.
```

Archive and investigation docs may remain long. Active entry/workstream/taskboard
docs should get a slimming task once they exceed roughly 1000 lines.

## Acceptance

```text
mir-proof-envelope-v0:
  DIRECTARRAY-FMEM-COMMON-001 is listed with no auto-fastmem lowering.

fastmem-layout-table-contract-v0:
  DirectArray commonality explicitly says proof ingredients only.

workstream/taskboard:
  next implementation remains MIR-FMEM-008D.
  DirectArray/FastMemory commonality and docs-slim rows are queued follow-ups.

docs layout:
  active-doc length hygiene rule is recorded.
```

## Stop Line

```text
do_not_start_MIR_FMEM_008D_in_this_card=1
do_not_move_large_docs_in_this_card=1
do_not_merge_DirectArrayAccessPlan_with_FastMemAccessPlan=1
do_not_auto_generate_fastmem_region_from_DirectArray=1
```
