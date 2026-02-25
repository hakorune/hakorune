---
Status: SSOT
Scope: Reserve `ExitKind::Unwind` in CorePlan/FlowBox contracts (docs-first)
Related:
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# ExitKind::Unwind reservation (SSOT)

## Goal

Reserve `Unwind` as an `ExitKind` in the CorePlan/FlowBox model so cleanup and
observability contracts do not need redesign when exceptions/unwind is added.

This is **docs-first**. It does not imply unwind is implemented.

## Decision

- `ExitKind` conceptually includes `Unwind` as a first-class exit channel.
- Cleanup rules are defined over **all** exit kinds (Normal/Return/Break/Continue/Unwind).
- Until implemented, any real `Unwind` emission is:
  - either impossible (not generated), or
  - fail-fast in strict/dev if it somehow appears.

## Non-goals

- No runtime exception system.
- No new control-flow constructs.
- No behavior changes in release.

