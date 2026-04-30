---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: fix the current delete-last order for the remaining `Program(JSON v0)` keepers after the P18-P23 archive/route cleanup.
Related:
  - docs/development/current/main/phases/phase-29cv/P23-PROGRAM-JSON-V0-KEEPER-ROUTE-MAP.md
  - docs/development/current/main/phases/phase-29ci/P24-STAGE1-CONTRACT-MODE-ALIAS-PRUNE.md
  - docs/development/current/main/phases/phase-29cv/P0-POST-EXE-DIRECT-KEEPER-INVENTORY.md
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost/compat/run_stage1_cli.sh
  - tools/lib/program_json_v0_compat.sh
---

# P24 Keeper Delete-Last Order

## Goal

Keep the remaining `Program(JSON v0)` cleanup map short and current.

After the P18-P23 archive work, the next risk is not broad deletion but
accidentally treating explicit keeper routes as if they were still public or
mainline. This card fixes the delete-last order and pins the next thin slice.

## Current Order

| Order | Bucket | Current owner | Posture |
| --- | --- | --- | --- |
| 1 | retired wrapper-local dead seams | `tools/selfhost/compat/run_stage1_cli.sh` | thin now |
| 2 | explicit Stage-B artifact diagnostic probe | `tools/dev/phase29cv_stageb_artifact_probe.sh`, `tools/lib/program_json_v0_compat.sh` | explicit probe / keep |
| 3 | explicit Program(JSON)->MIR bridge probe | `tools/selfhost/lib/program_json_mir_bridge.sh`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, `tools/selfhost_exe_stageb.sh` | explicit probe / keep |
| 4 | Stage1 contract keepers | `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost/compat/run_stage1_cli.sh` | explicit contract / keep |
| 5 | JoinIR / MirBuilder fixture keepers | `tools/smokes/v2/lib/stageb_helpers.sh`, `phase29bq_hako_program_json_contract_pin_vm.sh`, `phase29bq_hako_mirbuilder_*` | true keeper |
| 6 | raw shell spelling SSOT | `tools/lib/program_json_v0_compat.sh` | shell delete-last |
| 7 | Rust/public compat surface | `src/runtime/deprecations.rs`, `src/stage1/program_json_v0*`, `src/runner/stage1_bridge/**` | final delete-last |

## Notes

- `phase-29ci` P24 already pruned the shell-only Stage1 contract mode aliases
  from `tools/selfhost/lib/stage1_contract.sh`. Do not reopen that slice here.
- `tools/selfhost/compat/run_stage1_cli.sh emit program-json` is intentionally
  retired. It remains only as a redirect + contract smoke surface.
- `tools/dev/phase29cv_stageb_artifact_probe.sh` is a deliberate diagnostic
  entry, not a mainline build route.
- `docs/reference/environment-variables.md` may still mention raw
  `--emit-program-json-v0`; treat that as compat reference, not implementation
  direction.

## Next Slice

Thin the retired wrapper keeper without changing behavior:

1. remove wrapper-local dead seams from `tools/selfhost/compat/run_stage1_cli.sh`
2. keep the explicit retired redirect for `emit program-json`
3. keep `emit mir-json` behavior unchanged
4. prove with shell syntax + current-state pointer guard + the exact Stage1
   contract smoke when the stage1 artifact is present

## Acceptance

```bash
bash -n tools/selfhost/compat/run_stage1_cli.sh
if [ -x target/selfhost/hakorune.stage1_cli.stage2 ]; then
  ./tools/smokes/v2/run.sh --profile integration --filter 'phase29ci_stage1_cli_exact_emit_contract_vm.sh'
fi
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
