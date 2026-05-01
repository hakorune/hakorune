# Program(JSON v0) Dev Capsules

This directory owns manual Stage-A/Stage-B Program(JSON v0) dev probes.

Rules:

- These helpers are explicit compatibility diagnostics, not daily compiler
  proof routes.
- Do not source these helpers from `tools/selfhost/selfhost_build.sh`.
- Prefer MIR-first routes for normal `--mir`, `--run`, and `--exe` work.
- Keep Stage-A/Stage-B fallback behavior visible; do not add silent fallback
  paths outside these probes.

Entries:

- `tools/dev/program_json_v0/dev_stagea.sh`
- `tools/dev/program_json_v0/dev_stageb.sh`
