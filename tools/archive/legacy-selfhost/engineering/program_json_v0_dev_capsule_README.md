# Program(JSON v0) Dev Capsules

This directory is the retired active home for manual Stage-A/Stage-B
Program(JSON v0) dev probes.

Rules:

- Do not add active Program(JSON v0) dev entries here without a keeper card.
- Archived helpers are explicit compatibility diagnostics, not daily compiler
  proof routes.
- Do not source archived helpers from `tools/selfhost/selfhost_build.sh`.
- Prefer MIR-first routes for normal `--mir`, `--run`, and `--exe` work.
- Keep Stage-A/Stage-B fallback behavior visible; do not add silent fallback
  paths outside these probes.
- `tools/lib/program_json_v0_compat.sh` remains the shared raw emit spelling
  SSOT while Stage1 and fixture keepers still source it.
- Stage-B artifact capture and the old Stage-A/Stage-B manual dev runners are
  archived under `tools/archive/legacy-selfhost/engineering/` and must not be
  reintroduced as active dev entries without a new keeper card.

Entries:

- none

Archived entries:

- `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stagea.sh`
- `tools/archive/legacy-selfhost/engineering/program_json_v0_dev_stageb.sh`
