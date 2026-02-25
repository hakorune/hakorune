# Pattern2 Promotion API Contract (SSOT)

This directory is the single entry point for Pattern2 LoopBodyLocal promotion.
All callers must go through `try_promote()` and must honor the decision contract
below.

## PromoteDecision Contract

- `Promoted`
  - All contract checks satisfied.
  - Pattern2 continues in the JoinIR path.

- `NotApplicable`
  - Promotion not applicable (no LoopBodyLocal in conditions).
  - The caller continues Pattern2 with unchanged inputs.
  - Example causes:
    - No LoopBodyLocal variables in the break condition.

- `Freeze`
  - Contract violation or unimplemented behavior.
  - Fail-fast with a clear error tag, no fallback.
  - Example causes:
    - Read-only contract broken (assignment detected).
    - Missing required metadata (loop scope/break guard).

## Reject Mapping Rules (PolicyDecision::Reject -> PromoteDecision)

The mapping lives in `promote_runner.rs` and must remain stable:

- Any `PolicyDecision::Reject` becomes `Freeze`
- Promotion not applicable (no LoopBodyLocal vars) uses `NotApplicable`
