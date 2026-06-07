---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-098.
Related:
  - docs/development/current/main/phases/phase-296x/296x-596-MIM-PORT-FMEM-097-REFRESHED-WINNER-CLOSEOUT-AUDIT.md
  - docs/development/current/main/phases/phase-296x/296x-588-596-MIM-PORT-FMEM-REFRESH-LADDER-TASK-ORDER.md
---

# 296x-597 MIM-PORT-FMEM-098 Post-Refresh Cleanup Planning

## Purpose

Select the next narrow cleanup or implementation row after the refreshed winner
claim closeout. This row is planning-only until a single cleanup/implementation
slice is chosen.

## Candidate Work

```text
report/check duplication cleanup
refresh reference docs after the refreshed terminal ladder
Python-template C bridge retirement/delete decision
real activation ladder planning
post-refresh source/docs length cleanup
```

## Required Boundary

```text
do not mix cleanup with real product activation
do not reopen Python-template C semantics
do not add a new MemOp kind in this planning row
```

## Acceptance Sketch

```text
one next row selected
BoxCount vs BoxShape choice recorded
verification command set for the selected row recorded
```

## Decision

Choose **BoxShape** next.

The refreshed terminal ladder is behaviorally closed through 296x-596, but it
left repeated route/profile facts across the report/check layer. The next row
should reduce that duplication before reopening real activation or bridge
retirement work.

## Inventory

```text
tools/hako_check/fastmem_route_profiles.py: 720 lines
tools/hako_check/fastmem_check_terminal_rules.py: 1389 lines
tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py: 1143 lines
tools/hako_check/fastmem_source_syntax_smoke.sh: 4269 lines
```

Repeated refresh ladder decisions currently appear in multiple places:

```text
profile flag
selected route
selected memop family
selected memop kinds
next producer slice
deferred memop kinds
expected zero/positive fields
source syntax smoke report/check block
```

## Selected Next Row

```text
296x-598 MIM-PORT-FMEM-099 report/check refresh-profile SSOT cleanup
```

## BoxShape Boundary

```text
do not add a new route profile
do not add a new MemOp kind
do not reopen product activation, hook install, global allocator product claim,
  or winner/perf validation
do not change emitted KV rows except by preserving them through a shared table
```

## Verification For 296x-598

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_terminal_rules.py tools/hako_check/fastmem_mir_to_llvm_producer_report_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_body.py tools/hako_check/fastmem_mir_to_llvm_producer_report_tail_rows.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Next

```text
296x-598 MIM-PORT-FMEM-099 report/check refresh-profile SSOT cleanup.
```
