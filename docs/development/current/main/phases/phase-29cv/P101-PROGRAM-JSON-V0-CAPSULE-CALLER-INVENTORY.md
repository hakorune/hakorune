---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: refresh live `Program(JSON v0)` caller ownership after HAKO_CAPI_PURE retirement.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - docs/development/current/main/phases/phase-29cv/P37-PROGRAM-JSON-V0-COMPAT-CAPSULE-SSOT.md
  - docs/reference/environment-variables.md
  - tools/selfhost_exe_stageb.sh
  - tools/lib/program_json_v0_compat.sh
  - tools/selfhost/lib/program_json_mir_bridge.sh
  - tools/selfhost/lib/stageb_program_json_capture.sh
---

# P101 Program(JSON v0) Capsule Caller Inventory

## Goal

Refresh the remaining `Program(JSON v0)` caller map before replacing or
archiving capsules. This is a BoxShape inventory card, not a delete card.

Primary rule stays unchanged:

```text
selfhost_build.sh mainline = source/direct MIR(JSON), not Program(JSON v0)
```

## Current Caller Buckets

| Bucket | Live owner | Posture | Next action |
| --- | --- | --- | --- |
| Stage-B artifact diagnostic | `tools/dev/program_json_v0/stageb_artifact_probe.sh`, `tools/lib/program_json_v0_compat.sh` | explicit diagnostic capsule | keep until replaced by MIR-first artifact diagnostics or archived |
| Program(JSON)->MIR bridge | `tools/selfhost/lib/stageb_program_json_capture.sh`, `tools/selfhost/lib/program_json_mir_bridge.sh`, `tools/selfhost_exe_stageb.sh`, `tools/dev/phase29ci_selfhost_build_exe_consumer_probe.sh`, `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` | bridge compat capsule | test whether `tools/selfhost_exe_stageb.sh` can default to `direct` |
| Stage1 contract | `tools/selfhost/lib/stage1_contract.sh`, `tools/selfhost/compat/run_stage1_cli.sh`, `src/runner/stage1_bridge/**` | contract/probe capsule | keep until exact Stage1 CLI contract is proven through MIR-first wrappers only |
| JoinIR / MirBuilder fixtures | `tools/smokes/v2/lib/stageb_helpers.sh`, `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*`, `lang/src/compiler/mirbuilder/emit_mir_json_v0_from_program_json_v0.hako`, `lang/src/compiler/mirbuilder/program_json_v0_*` | true fixture keeper | keep while .hako MirBuilder parity is still Program(JSON)-fixture based |
| Rust/public delete-last surface | `src/cli/args.rs`, `src/runtime/deprecations.rs`, `src/stage1/program_json_v0*`, `src/runner/stage1_bridge/**` | final delete-last | delete only after shell/test keeper inventory reaches zero or archive-only |

## Current Read

- Broad deletion is not ready: fixture keepers and Stage1 bridge surfaces still
  actively pin compatibility contracts.
- The next useful code card is narrow: evaluate
  `tools/selfhost_exe_stageb.sh` `stageb-delegate` default retirement.
- `tools/lib/program_json_v0_compat.sh` remains the only shell owner for the raw
  `--emit-program-json-v0` spelling.
- `tools/selfhost/lib/program_json_mir_bridge.sh` remains the shell owner for
  non-raw Program(JSON)->MIR conversion.
- Any new caller must be assigned to one bucket above; do not add a free-floating
  Program(JSON v0) route.

## Non-goals

- no `--emit-program-json-v0` deletion
- no Stage1 bridge deletion
- no fixture archive movement
- no `stageb-delegate` default change in this card

## Acceptance

```bash
rg -l "emit-program-json-v0|program_json_v0_compat_emit_to_file|stageb_emit_program_json_v0_fixture|program_json_mir_bridge_emit|HAKORUNE_STAGE1_EMIT_ROUTE|stageb-delegate|--emit-program-json-v0|emit_from_program_json_v0|ProgramJson" tools src lang -g '!target'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
