---
Status: Active
Date: 2026-06-09
Scope: active mimalloc migration pointers and thin restart surface.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/phases/phase-296x/README.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# Mimalloc Current Workstream

This is a thin pointer surface. Keep the current migration lane first and keep
the long history in the inventory note.

## Current Lane

- active lane: `MIM-PORT-FMEM-001 PageMeta scalar pilot`
- blocker: read `CURRENT_STATE.toml`
- inventory note: `docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md`
- fastmem layout/table SSOT: `docs/development/current/main/design/fastmem-layout-table-contract-v0-ssot.md`

## Thin Mirrors

```text
active_lane=MIM-PORT-FMEM-001 PageMeta scalar pilot
current_state_pointer_guard=pass
restart_surface_thin=1
long_history_in_current_mirrors=0
implementation_gap_count=0
mim_port_fmem_001_done=1
next_row=MIM-PORT-FMEM-002
```

## Archived Lane Notes

- docs pointer cleanup is landed and stays in the archive note
- route taxonomy and provider-package evidence stay in the design/docs archive

## Stop Line

- no provider activation claim
- no hook installation claim
- no winner claim
- no global allocator claim
- no source-history copyback into the restart mirrors
