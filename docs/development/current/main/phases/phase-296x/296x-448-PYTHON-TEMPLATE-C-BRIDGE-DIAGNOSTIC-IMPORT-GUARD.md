---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-007C static import guard for the retired Python-template C bridge.
Related:
  - docs/development/current/main/phases/phase-296x/296x-447-PYTHON-TEMPLATE-C-BRIDGE-QUARANTINE-INVENTORY.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# 296x-448 Python Template C Bridge Diagnostic Import Guard

## Decision

Add a lightweight static import guard so normal allocator and hako_check tools
cannot import Python-template C diagnostic payload modules directly.

Allowed imports are limited to:

```text
payload wiring:
  replacement_front_templates.py
  replacement_front_shim_templates.py
  replacement_front_bins_templates.py

diagnostic smoke:
  replacement_front_smokes.py

guarded bridge build helper:
  hakozuna_mixed_ws_build_support.py
```

Everything else must route through the explicit diagnostic baseline boundary:

```text
tools/allocator/python_template_c_bridge.py
tools/allocator/hakozuna_mixed_ws_build_support.py
```

## Guard

```bash
bash tools/checks/python_template_c_bridge_import_guard.sh
```

The guard parses Python AST imports under:

```text
tools/allocator
tools/hako_check
```

and fails if a non-allowlisted file imports any retired diagnostic payload:

```text
replacement_front_templates
replacement_front_shim_templates
replacement_front_bins_templates
replacement_front_smoke_templates
replacement_front_shim_report_source
replacement_front_bins_report_source
```

## Acceptance

```text
python_template_c_bridge_import_guard:
  ok

dev_gate quick:
  includes Python-template C bridge import guard
```

## Next

```text
MIR-FMEM-007D:
  decide whether to keep the diagnostic baseline payload until allocator-owner
  layout/table MemOps are implemented, or archive/delete fixed-slot-only bridge
  payloads first.
```
