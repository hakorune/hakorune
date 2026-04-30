---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: prune unused shell-only Stage1 contract mode aliases from `tools/selfhost/lib/stage1_contract.sh`.
Related:
  - docs/development/current/main/phases/phase-29ci/P23-PROGRAM-JSON-V0-KEEPER-ROUTE-MAP.md
  - tools/selfhost/lib/stage1_contract.sh
---

# P24 Stage1 Contract Mode Alias Prune

## Goal

Keep Stage1 shell contract modes singular:

- `emit-program`
- `emit-mir`
- `emit-mir-program`
- `run`

The legacy shell-only spellings `emit_program_json`, `emit-program-json`,
`emit_mir_json`, and `emit-mir-json` have no repo caller evidence. Prune them
from `stage1_contract.sh` so future Program(JSON v0) cleanup has fewer public
looking surfaces.

## Non-Goals

- Do not delete `--emit-program-json-v0`.
- Do not change `.hako` Stage1 mode normalization in this slice.
- Do not change `selfhost_build.sh --run` or `--exe` Program(JSON v0) routes.

## Probe Note

Worker inventory found that normal `selfhost_build.sh --exe` might appear
direct-MIR-replaceable, but the quick `static box Main` fixture still fails the
direct source->MIR(JSON)->ny-llvmc route with unsupported pure shape. Keep EXE
on Program(JSON v0) until that backend shape gap is fixed.

P26 fixed that exact `main(args)` entry-args `ArrayBox.birth` pure-first gap and
moved the normal non-diagnostic `--exe` route to direct source->MIR(JSON)->EXE.
This P24 note remains as the historical reason the alias prune did not combine
route movement in the same commit.

## Acceptance

```bash
bash -n tools/selfhost/lib/stage1_contract.sh
source tools/selfhost/lib/stage1_contract.sh
stage1_contract_exec_direct_emit_mode target/release/hakorune emit-program apps/tests/stage1_run_min.hako >/tmp/p24_stage1_program.json
stage1_contract_exec_direct_emit_mode target/release/hakorune emit-mir apps/tests/stage1_run_min.hako >/tmp/p24_stage1_mir.json
bash tools/checks/current_state_pointer_guard.sh
```
