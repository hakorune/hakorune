---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-101.
Related:
  - docs/development/current/main/phases/phase-296x/296x-599-MIM-PORT-FMEM-100-REMOVE-DORMANT-REFRESH-TERMINAL-BRANCHES.md
  - docs/development/current/main/phases/phase-296x/296x-598-MIM-PORT-FMEM-099-REPORT-CHECK-REFRESH-PROFILE-SSOT-CLEANUP.md
---

# 296x-600 MIM-PORT-FMEM-101 Producer Refresh Flag-Row Cleanup

## Purpose

Move refreshed profile report-flag row emission in
`fastmem_mir_to_llvm_producer_report_route_rows.py` behind
`REFRESH_PROFILE_SPECS`, preserving emitted KV rows.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
do not change emitted KV rows
do not change profile choices
do not touch non-refresh pilot rows
do not add product behavior
```

## Acceptance Sketch

```text
refresh report flags are emitted from REFRESH_PROFILE_SPECS
explicit per-refresh flag row list is removed or reduced
fastmem_check_smoke stays green
fastmem_source_syntax_smoke stays green
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
