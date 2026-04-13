Status: SSOT
Date: 2026-04-13
Scope: explicit `TaskGroupBox` aggregate failure reporting only.

# 254x Aggregate / Multi-Failure Reporting

Current aggregate failure reporting is pinned as:

1. `TaskGroupBox` preserves the first failed child payload as the main failure.
2. Later failed child payloads may be preserved as additional failures in observation order.
3. `TaskGroupBox.failureReport()` returns `ArrayBox`.
4. `failureReport()` is `[]` when no child failure has been observed.
5. Non-empty `failureReport()` is `[first_failure, additional_failures...]`.
6. `joinAll(timeout_ms)` and explicit scope exit still surface only the first failure.
7. Sibling cancellations (`Cancelled: sibling-failed`) are cancellation side-effects, not aggregate failure causes.

Out of scope for this cut:

- implicit root-scope aggregate reporting
- scope-exit aggregate payload surface
- timeout payload widening
