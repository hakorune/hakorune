---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-103.
Related:
  - docs/development/current/main/phases/phase-296x/296x-601-MIM-PORT-FMEM-102-PRODUCER-REFRESH-BOOLEAN-IMPORT-CLEANUP.md
  - docs/development/current/main/phases/phase-296x/296x-598-MIM-PORT-FMEM-099-REPORT-CHECK-REFRESH-PROFILE-SSOT-CLEANUP.md
---

# 296x-602 MIM-PORT-FMEM-103 Source-Syntax Refresh Helper Cleanup

## Purpose

Reduce refreshed-profile duplication in `fastmem_source_syntax_smoke.sh` now
that `RefreshProfileSpec` is the report/check SSOT for refreshed terminal
ladder rows.

## Chosen Mode

```text
BoxShape
```

## Required Boundary

```text
do not change source-syntax fixture semantics
do not change emitted KV expectations
do not open product activation, hooks, global allocator claim, or winner behavior
do not mix with new FastMemory feature rows
```

## Acceptance Sketch

```text
source-syntax smoke refresh assertions use a smaller helper surface
refreshed terminal/product/hook/global/winner rows still validate
fastmem_check_smoke stays green
fastmem_source_syntax_smoke stays green
```

## Verification

```bash
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
