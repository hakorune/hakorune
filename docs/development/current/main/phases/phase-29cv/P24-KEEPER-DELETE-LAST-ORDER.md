---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: historical delete-last order, refreshed after the P37 compat capsule split.
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P37-PROGRAM-JSON-V0-COMPAT-CAPSULE-SSOT.md
  - docs/development/current/main/phases/phase-29ci/P24-STAGE1-CONTRACT-MODE-ALIAS-PRUNE.md
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost/compat/run_stage1_cli.sh
  - tools/lib/program_json_v0_compat.sh
---

# P24 Keeper Delete-Last Order

## Goal

Keep the remaining `Program(JSON v0)` cleanup map short and current.

After the P18-P23 archive work and the P37 compat capsule split, the next risk
is not broad deletion but accidentally treating explicit capsule routes as if
they were still public or mainline.

## Current Order

| Order | Bucket | Current owner | Posture |
| --- | --- | --- | --- |
| 1 | retired wrapper-local dead seams | `tools/selfhost/compat/run_stage1_cli.sh`, old root helpers | done through P32 |
| 2 | Stage-B artifact diagnostic capsule | `tools/dev/phase29cv_stageb_artifact_probe.sh`, `tools/lib/program_json_v0_compat.sh` | explicit capsule / keep |
| 3 | Program(JSON)->MIR bridge capsule | `tools/selfhost/lib/program_json_mir_bridge.sh`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, `tools/selfhost_exe_stageb.sh` (`stageb-delegate`) | explicit capsule / keep |
| 4 | Stage1 contract capsule | `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost/compat/run_stage1_cli.sh` | explicit contract / keep |
| 5 | JoinIR / MirBuilder fixture capsule | `tools/smokes/v2/lib/stageb_helpers.sh`, `phase29bq_hako_program_json_contract_pin_vm.sh`, `phase29bq_hako_mirbuilder_*` | true keeper |
| 6 | Rust/public compat surface | `src/runtime/deprecations.rs`, `src/stage1/program_json_v0*`, `src/runner/stage1_bridge/**` | final delete-last |

## Notes

- `phase-29ci` P24 already pruned the shell-only Stage1 contract mode aliases
  from `tools/selfhost/lib/stage1_contract.sh`. Do not reopen that slice here.
- `tools/selfhost/compat/run_stage1_cli.sh emit program-json` is intentionally
  retired. It remains only as a redirect + contract smoke surface.
- `tools/dev/phase29cv_stageb_artifact_probe.sh` and
  `tools/lib/program_json_v0_compat.sh` are one artifact diagnostic capsule,
  not two independent delete-last buckets.
- `docs/reference/environment-variables.md` may still mention raw
  `--emit-program-json-v0`; treat that as compat reference, not implementation
  direction.

## Remaining Slice Shape

The wrapper-local thin slice named here originally landed in later P25-P32
cleanup cards. From P37 onward, the next slice must choose one compat capsule
and either define a MIR-first replacement or move it to an archive owner.

Recommended order:

1. archive or replace standalone diagnostic canaries that are not in the P37
   capsule table
2. replace/archive the Stage-B artifact diagnostic capsule
3. replace/archive the bridge capsule
4. replace/archive the fixture capsule
5. only then remove Rust/public compat surface

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
