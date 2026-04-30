---
Status: Superseded
Decision: accepted
Date: 2026-04-30
Scope: delete the dead `stageb_emit_program_json_v0_fixture()` wrapper after its live callers were retired or moved away.
Related:
  - docs/development/current/main/phases/phase-29cv/P24-KEEPER-DELETE-LAST-ORDER.md
  - docs/development/current/main/phases/phase-29cv/P25-STAGE1-CONTRACT-COMPAT-HELPER-NARROW.md
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/lib/program_json_v0_compat.sh
---

# P26 Delete Dead StageB Fixture Emit Wrapper

> Superseded by P36. Later validation proved the phase29bq fixture/contract-pin
> caller set was still live, so the shared helper was restored as a thin wrapper
> over `program_json_v0_compat_emit_to_file()`.

## Goal

Remove dead shell scaffolding inside the probe bucket without touching the live
probe or fixture owners.

`stageb_emit_program_json_v0_fixture()` was treated here as a thin shared
wrapper for the `phase29bq` fixture producers whose caller set had disappeared.
Later validation showed those callers were still live, while the actual shell
compat owner remained `tools/lib/program_json_v0_compat.sh`.

## Decision

- delete `stageb_emit_program_json_v0_fixture()` from
  `tools/smokes/v2/lib/stageb_helpers.sh`
- keep `tools/smokes/v2/lib/stageb_helpers.sh` itself; its Stage-B compile /
  capture helpers are still live
- keep `tools/lib/program_json_v0_compat.sh` as the raw shell spelling SSOT
- sync the stale historical note in `phase-29ci/P7`

## Non-goals

- do not touch `program_json_v0_compat_emit_to_file()`
- do not touch `program_json_mir_bridge_emit()`
- do not touch `phase29bq` fixture semantics
- do not touch Rust/public delete-last compat surfaces

## Acceptance

```bash
bash -n tools/smokes/v2/lib/stageb_helpers.sh \
  tools/dev/phase29cv_stageb_artifact_probe.sh \
  tools/selfhost/lib/stage1_contract.sh
rg -n 'stageb_emit_program_json_v0_fixture\(|program_json_v0_compat_emit_to_file\(' tools docs
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
