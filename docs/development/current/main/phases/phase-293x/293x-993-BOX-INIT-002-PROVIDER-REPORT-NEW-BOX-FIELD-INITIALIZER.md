# 293x-993 BOX-INIT-002 Provider Report New-Box Field Initializer

Status: landed
Date: 2026-05-21

## Purpose

Apply the landed `new Box { field: expr }` surface to the current provider
report-copy lane, without opening shorthand copy, wildcard copy, provider
activation, host allocator replacement, hooks, `#[global_allocator]`, or backend
matchers.

## Scope

Convert ReportFields-to-Report helper methods in the provider ladder:

```text
HakoAllocProviderBoundaryDiagnosticVocabulary.makeProviderBoundaryDiagnosticVocabularyReport
HakoAllocProviderReadinessPreflight.makeProviderReadinessPreflightReport
HakoAllocProviderSelectionInventory.makeProviderSelectionInventoryReport
HakoAllocProviderActivationUnsupportedOutcomeLedger.makeProviderActivationUnsupportedOutcomeLedgerReport
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
- No constructor named arguments.
- No provider activation, provider call, host replacement, hook install, backend
  matcher, worker/TLS, or source concurrency behavior.
- No change to report schema or proof app expected output.

## Evidence

```bash
bash tools/checks/k2_wide_box_new_field_initializer_provider_reports_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

BOX-INIT-003 decides whether to add same-name initializer shorthand or return
to the parked MIMAP-375A provider activation follow-up.
