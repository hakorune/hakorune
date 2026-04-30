---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: let `tools/selfhost/selfhost_build.sh --mir <file>` use its direct MIR owner without first materializing Program(JSON v0) when no Stage-B artifact/debug output is requested.
Related:
  - docs/development/current/main/phases/phase-29ci/P21-PROGRAM-JSON-V0-INVENTORY-WORDING-SYNC.md
  - docs/development/current/main/phases/phase-39x/39x-90-stage0-vm-gate-thinning-ssot.md
  - tools/selfhost/README.md
  - tools/selfhost/selfhost_build.sh
---

# P22 Selfhost Build MIR-Only Direct Route

## Goal

`selfhost_build.sh --mir <file>` is already documented as a direct MIR
consumer, but the route main still produced a temporary Program(JSON v0)
before calling the direct MIR helper.

Make the MIR-only case actually direct:

- `--mir <file>` with no `--run` and no `--exe` emits MIR(JSON) from source
  and exits before Stage-B Program(JSON v0) production.
- stdout prints the MIR output path for the file-producing result.
- diagnostic Stage-B routes stay unchanged when `--keep-tmp` or
  `NYASH_SELFHOST_KEEP_RAW=1` is set.

## Non-Goals

- Do not change `--run`; it remains the Core-Direct execution path for the
  materialized Program(JSON v0) route.
- Do not change `--exe`; it still owns the explicit
  Program(JSON v0) -> MIR(JSON) -> EXE lane.
- Do not remove the raw Program(JSON v0) compatibility flag; shell spelling
  remains owned by `tools/lib/program_json_v0_compat.sh`.

## Caller Inventory

Current active script callers do not use `selfhost_build.sh --mir`; quick
selfhost smokes use `--run` or `--exe`. Existing `--mir` references are docs
and user-facing examples, so this can be tightened without changing a smoke
caller contract.

## Acceptance

```bash
bash -n tools/selfhost/selfhost_build.sh tools/selfhost/lib/selfhost_build_route.sh tools/selfhost/lib/selfhost_build_direct.sh
bash tools/selfhost/selfhost_build.sh --in apps/tests/stage1_run_min.hako --mir /tmp/hako_selfhost_build_mir_only.json
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_return_vm.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_binop_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```
