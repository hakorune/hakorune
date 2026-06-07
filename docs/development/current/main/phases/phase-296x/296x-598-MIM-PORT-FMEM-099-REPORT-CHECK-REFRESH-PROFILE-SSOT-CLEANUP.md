---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-099.
Related:
  - docs/development/current/main/phases/phase-296x/296x-597-MIM-PORT-FMEM-098-POST-REFRESH-CLEANUP-PLANNING.md
  - docs/development/current/main/phases/phase-296x/296x-596-MIM-PORT-FMEM-097-REFRESHED-WINNER-CLOSEOUT-AUDIT.md
---

# 296x-598 MIM-PORT-FMEM-099 Report/Check Refresh-Profile SSOT Cleanup

## Purpose

Reduce the duplicated refreshed terminal-ladder profile metadata in
`tools/hako_check` without changing report/check behavior.

## Chosen Mode

```text
BoxShape
```

This row is a structure cleanup. It must not introduce a new accepted route,
new MemOp vocabulary, or product behavior.

## Target Shape

Introduce one shared refresh-profile table or helper seam that can describe:

```text
profile name
report flag
selected route
selected memop family
selected memop kinds
next producer slice
deferred memop kinds
expected zero fields
expected positive fields
```

The first implementation slice should keep scope narrow. It may start with the
588..596 refresh profiles only, leaving older non-refresh pilot rows unchanged.

## Required Boundaries

```text
emitted KV compatibility preserved
fastmem-check behavior preserved
source-syntax smoke behavior preserved
no product activation side effect
no hook installation side effect
global_allocator_product_claim remains 0
type_abi_hot_lookup_count remains 0
provider_abi_hot_dispatch_count remains 0
```

## Acceptance Sketch

```text
refresh profile metadata has one SSOT table/helper
terminal rule checks consume the shared metadata for refresh profiles
producer route row selection consumes the shared metadata for refresh profiles
all existing refresh-profile smoke expectations stay green
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_terminal_rules.py tools/hako_check/fastmem_mir_to_llvm_producer_report_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_body.py tools/hako_check/fastmem_mir_to_llvm_producer_report_tail_rows.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```
