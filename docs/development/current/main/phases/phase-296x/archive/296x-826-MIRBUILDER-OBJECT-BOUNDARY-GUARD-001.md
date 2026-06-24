---
Status: Landed
Date: 2026-06-16
Task: MIRBUILDER-OBJECT-BOUNDARY-GUARD-001
Scope: Guard MIRBuilder against object representation / publication truth.
Related:
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-825-COMPILER-OBJECT-FINAL-SHAPE-001.md
---

# MIRBUILDER-OBJECT-BOUNDARY-GUARD-001

## Purpose

`COMPILER-OBJECT-FINAL-SHAPE-001` fixed the boundary:

```text
MIRBuilder:
  meaning only

ObjectPlan:
  representation + publication-site truth
```

This row adds a lightweight repository guard so future MIRBuilder work cannot
quietly add object representation truth in `src/mir/builder`.

## Result

```text
output_contract=hako-mirbuilder-object-boundary-guard-v0
source_evidence=296x-825
guard_scope=src/mir/builder
mirbuilder_object_management_enabled=0
mirbuilder_object_storage_plan_reference_count=0
mirbuilder_local_first_object_plan_reference_count=0
mirbuilder_object_publication_reference_count=0
mirbuilder_hosthandle_bypass_reference_count=0
mirbuilder_arc_retirement_reference_count=0
mirbuilder_arcdynbox_reference_count=0
mirbuilder_helper_symbol_inference_reference_count=0
mirbuilder_method_name_special_case_reference_count=0
mirbuilder_variable_name_special_case_reference_count=0
product_default_changed=0
implementation_started=0
selected_next=SELFHOST-MIR-OBJECT-METADATA-001
summary=ok
```

## Guarded Forbidden Terms

The guard rejects these representation/publication-owner terms under
`src/mir/builder`:

```text
ObjectStoragePlan
LocalFirstObjectPlan
ObjectPublication
HostHandleEscaped
ArcDynBox
hosthandle_bypass
arc_retirement
helper_symbol_inference
method_name_special_case
variable_name_special_case
```

The list is intentionally narrow.  It does not reject unrelated words like
`bypass`, because existing control-flow and PHI docs use that term for other
layers.

## Stop Line

```text
do not add ObjectPlan / ObjectStoragePlan construction to MIRBuilder
do not decide publication in MIRBuilder
do not decide HostHandle bypass in MIRBuilder
do not decide Arc retirement in MIRBuilder
do not add helper/method/variable-name direct lowering in MIRBuilder
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_mirbuilder_object_boundary_guard.sh
```
