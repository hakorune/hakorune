# MIR Policies (`src/mir/policies/`)

This subtree holds shared policy SSOT used by builder, canonicalizer, and
router-style consumers.

## Read First

1. [`src/mir/README.md`](../README.md)
2. [`docs/development/current/main/design/mir-crate-split-prep-ssot.md`](../../../docs/development/current/main/design/mir-crate-split-prep-ssot.md)

## Boundaries

- Shared policy lives here instead of being duplicated in local helpers.
- Policy modules should decide shape/acceptance, not generate MIR.
- Do not move this subtree into a separate crate until the `hakorune-mir-core` / `hakorune-mir-builder` / `hakorune-mir-joinir` seams are stable.

## Main Responsibilities

- loop shape policies
- return/early-exit policies
- condition profile and overlap policy
- balanced depth scan policy

## Notes

- This subtree is a shared fence, not a split target for the current P5 step.
- Consumers should import policy types from here rather than re-deriving them locally.
