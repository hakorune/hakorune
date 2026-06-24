---
Status: Landed
Date: 2026-05-28
Scope: define CFG-aware typed-field residence ownership before any transform.
Blocker: CFG-AWARE-TYPED-FIELD-RESIDENCE-SSOT-296X-001
Related:
  - docs/development/current/main/design/cfg-aware-typed-field-residence-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-198-CFG-RESIDENCE-OR-RUNTIME-OWNER-SELECTION.md
---

# 296x-199 CFG-Aware Typed Field Residence SSOT

## Purpose

Accept the CFG-aware typed-field residence design boundary selected by row198.
This row is docs-only: it defines ownership, barriers, fallback behavior, and
the next inventory contract. It does not implement a compiler transform.

## Decision

```text
Decision: accepted

owner=cfg_aware_typed_field_residence
design_ssot=docs/development/current/main/design/cfg-aware-typed-field-residence-ssot.md
runtime_helper_abi=fallback
block_local_retry=0
transform_open=0
by_name_special_case=0
generic_cse=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Required Next Row

```text
row200:
  cfg_aware_typed_field_residence_plan_inventory

Goal:
  produce a selected-method CFG-aware plan with net_helper_call_delta before
  any compiler/runtime code is changed.
```

Acceptance:

```text
cfg_aware_typed_field_residence_ssot=accepted
next_inventory_required=1
transform_open=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_cfg_aware_typed_field_residence_ssot_guard.sh
```
