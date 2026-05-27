---
Status: Landed
Date: 2026-05-27
Scope: add the first Python MIR method shape adapter outside hako_check core.
Blocker: HAKO-MIR-METHOD-SHAPE-PYTHON-ADAPTER-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-84-HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER.md
---

# 296x-85 Hako MIR Method Shape Python Adapter

## Purpose

Add the first MIR-level observation app as Python, consuming selected MIR JSON
and producing method shape counts. This remains outside hako_check core.

## Required Output

```text
output_contract=hako-mir-method-shape-v0
input_kind=mir_json
selected_method
mir_instruction_count
call_count
field_get_count
field_set_count
array_get_call_count
array_length_call_count
phi_count
copy_count
branch_count
return_count
summary=ok
```

## Stop Line

Do not port this to `.hako` in this row. `.hako` migration selection is later.

## Landed Evidence

```text
output_contract=hako-mir-method-shape-v0
input_kind=mir_json
selected_method=HakoAllocObjectLifecyclePageQueue.selectPage/0
mir_instruction_count
call_count
field_get_count
field_set_count
array_get_call_count
array_length_call_count
phi_count
copy_count
branch_count
return_count
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mir_method_shape_python_adapter_guard.sh
```
