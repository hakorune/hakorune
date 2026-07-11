---
Status: Ready
Scope: Phase 29ar closeout (docs-only)
Related:
- docs/development/current/main/phases/phase-29ar/README.md
- docs/development/current/main/phases/phase-29ae/README.md
- docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
---

# Phase 29ar P3: Closeout (docs-only)

Goal: close Phase 29ar after `is_integer` strict/dev fail-fast reject and release adopt
are both green in the JoinIR regression gate.

## Checklist

1. Mark `docs/development/current/main/phases/phase-29ar/README.md` as `Status: Complete`.
2. Ensure Phase README includes:
   - Gate commands (quick + phase29ae pack)
   - Fixture/smoke paths (strict + release)
   - Summary of what was added (`ExitIfReturn`, strict fail-fast reject, release adopt)
3. Update pointers:
   - `docs/development/current/main/10-Now.md` and `docs/development/current/main/30-Backlog.md`
     move next focus off Phase 29ar.
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` reflects the next phase.
4. Update done criteria SSOT:
   - `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
     explicitly includes return-in-loop minimal coverage as satisfied.

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
