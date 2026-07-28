---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-007B remaining Python-template C bridge quarantine/delete inventory.
Related:
  - docs/development/current/main/phases/phase-296x/296x-446-PYTHON-TEMPLATE-C-BRIDGE-RETIREMENT-FIRST-SLICE.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/hako-alloc-mimalloc-port-identity-boundary-ssot.md
---

# 296x-447 Python Template C Bridge Quarantine Inventory

## Decision

Keep the Python-template C replacement-front bridge available only as an
explicit diagnostic baseline, and move the retirement guard into a shared
allocator-tool module used by both CLI entrypoints and bridge build helpers.

This closes the remaining easy leak where a future tool could import a
replacement-front builder directly and generate the retired bridge without
passing the diagnostic baseline flag through the public compare CLI.

## Implementation

```text
tools/allocator/python_template_c_bridge.py:
  ALLOW_FLAG
  PRODUCER
  RETIREMENT_MESSAGE
  add_baseline_flag(parser)
  require_explicit_baseline(allowed)

tools/allocator/hakozuna_mixed_ws_build_support.py:
  build_python_template_c_bridge_slot_baseline(..., allow_python_template_c_bridge_baseline)
  build_python_template_c_bridge_bins_baseline(..., allow_python_template_c_bridge_baseline)
```

Both build helpers fail-fast unless the caller explicitly passes the baseline
allowance. CLI tools still fail earlier through argument validation, but the
helper-level guard is the durable quarantine boundary.

## Inventory Result

Remaining Python-template C files are classified as diagnostic baseline
implementation until the MIR-to-LLVM/object producer covers the replacement
front behavior:

```text
diagnostic implementation:
  tools/allocator/replacement_front_templates.py
  tools/allocator/replacement_front_shim_templates.py
  tools/allocator/replacement_front_bins_templates.py
  tools/allocator/replacement_front_shim_report_source.py
  tools/allocator/replacement_front_bins_report_source.py
  tools/allocator/replacement_front_smoke_templates.py
  tools/allocator/replacement_front_smokes.py

guarded build entry:
  tools/allocator/hakozuna_mixed_ws_build_support.py

normal public CLI gate:
  tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py
  tools/allocator/hakmem_fixture_ldpreload_compare.py
  tools/allocator/hakozuna_mixed_ws_gap_ladder.py
```

## Rejected

```text
deleting all Python-template C payloads before the MIR-to-LLVM replacement-front
producer can emit equivalent report.kv evidence

leaving build helpers callable without the explicit diagnostic baseline flag

using replacement_front_c_shim as an implicit synonym for
python_template_c_bridge in report/check normalization
```

## Next

```text
MIR-FMEM-007C:
  add a lightweight static guard that inventories diagnostic bridge imports and
  fails if a normal allocator tool imports template payloads without routing
  through python_template_c_bridge.py / hakozuna_mixed_ws_build_support.py.
```
