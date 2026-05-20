# 293x-995 BOX-INIT-004 Support/Provider Report New-Box Field Initializer

Status: landed
Date: 2026-05-21

## Purpose

Continue the explicit `new Box { field: expr }` construction-site initializer
sweep after BOX-INIT-003, without adding shorthand or wildcard copy surface.

## Scope

Convert the next low-risk cluster of ReportFields-to-Report helpers whose copy
body is a straight `result.field = fields.field` sequence followed by
`return result`:

```text
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixInventory.makeExecutionSupportRequirementMatrixReport
HakoAllocSegmentArenaBackingModeledAllocationLedgerReleaseRecycleExecutionSupportRequirementMatrixDiagnostic.makeExecutionSupportRequirementMatrixDiagnosticReport
HakoAllocProviderInactiveBoundaryInventory.makeProviderInactiveBoundaryInventoryReport
HakoAllocOSVMPageSourcePilot.makeOSVMPageSourcePilotReport
HakoAllocAtomicBitmapPilot.makeAtomicBitmapPilotReport
```

Each helper now constructs the report with explicit field initializer entries:

```hako
local result = new SomeReport {
    accepted: fields.accepted,
    reason: fields.reason
}
```

## Stop Lines

- No same-name shorthand (`fields.accepted` as a standalone initializer).
- No wildcard copy.
- No spread copy.
- No constructor named arguments.
- No report schema or expected-output changes.
- No raw pointer dereference, arena release/recycle execution, segment-map
  mutation execution, real atomic primitive, OSVM execution, worker scheduling,
  provider activation, host allocator replacement, hooks, `#[global_allocator]`,
  or backend matcher additions.

## Evidence

```bash
bash tools/checks/k2_wide_box_new_field_initializer_support_provider_reports_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

BOX-INIT-005 decides whether to continue the explicit initializer sweep for the
next low-risk report helper cluster or park the syntax cleanup and return to the
allocator/provider lane.
