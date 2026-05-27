---
Status: Current
Date: 2026-05-28
Scope: unify field/property receiver facts after member-call route planning.
Blocker: MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-143-MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT.md
---

# 296x-144 MIR Builder Field/Property Receiver Facts Cleanup

## Purpose

Move field/property receiver fact lookups behind one MIR builder helper surface
so nested receiver lowering remains single-evaluation friendly. This is a
BoxShape cleanup and must not add a new accepted source shape.

## Required Output

```text
output_contract=mir-builder-field-property-receiver-facts-cleanup-v0
input_contract=mir-builder-member-call-route-plan-pilot-v0
fact_owner
field_lowering_uses_fact_owner
property_read_uses_fact_owner
single_eval_surface_ok
summary=ok
```
