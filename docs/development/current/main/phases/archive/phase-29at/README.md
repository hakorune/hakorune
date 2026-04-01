---
Status: Complete
Scope: BranchN (match/switch) skeleton reservation (docs-first)
Related:
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29at: BranchN skeleton reservation (docs-first)

Goal: make `match` / multi-branch a first-class CorePlan skeleton (`BranchN`)
as the long-term final form, without changing release behavior.

## Gate / Commands (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/match_return_strict_shadow_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/match_return_release_adopt_vm.sh`

## Plan

- P0: docs-first reservation + invariants (done). ✅
- P1: minimal code scaffolding (types/verify only; still unconnected). ✅
- P2: implement CorePlan::BranchN lowering (no new generation yet). ✅
- P3: minimal match subset (Return-only, no join) via CorePlan adopt (strict/dev). ✅
- P4: match_return release adopt (non-strict, no tags). ✅
- P5: closeout (docs-only). ✅

## Instructions

- P0: `docs/development/current/main/phases/phase-29at/P0-BRANCHN-SKELETON-RESERVATION-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29at/P1-BRANCHN-COREPLAN-SCAFFOLD-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29at/P2-BRANCHN-LOWERER-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29at/P3-MATCH-RETURN-MIN-SUBSET-INSTRUCTIONS.md`
- P4: `docs/development/current/main/phases/phase-29at/P4-MATCH-RETURN-RELEASE-ADOPT-INSTRUCTIONS.md`
- P5: `docs/development/current/main/phases/phase-29at/P5-CLOSEOUT-INSTRUCTIONS.md`
