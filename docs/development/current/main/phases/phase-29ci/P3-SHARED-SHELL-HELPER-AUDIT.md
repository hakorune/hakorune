---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` の shared shell helper keep 3本を exact role 付きで audit し、smoke tail / probe keep と分離した delete-order を固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - CURRENT_TASK.md
---

# P3 Shared Shell Helper Audit

## Goal

shared shell helper keep として残っている 3 file について、

- 何の contract を持っているか
- どれが最初に audit しやすいか
- smoke tail / diagnostics とどう分離するか

を delete-order の SSOT として固定する。

## Exact Helper Roles

### `tools/hakorune_emit_mir.sh`

- role:
  - explicit dev/helper entry for `.hako -> Program(JSON v0) -> MIR(JSON)` emission
  - direct `MirBuilderBox.emit_from_program_json_v0(...)` helper call
- contract shape:
  - helper-local pipeline script
  - not the shared selfhost build contract
- audit priority:
  - highest in the helper trio
  - owner is narrow and caller surface is explicit

### `tools/selfhost/selfhost_build.sh`

- role:
  - shared selfhost build contract
  - optional `HAKO_USE_BUILDBOX=1` lane still uses `BuildBox.emit_program_json_v0(...)`
- contract shape:
  - build pipeline helper
  - touches build output / stageb command description / raw capture
- audit priority:
  - second
  - broader than `hakorune_emit_mir.sh`, so keep separate

### `tools/smokes/v2/lib/test_runner.sh`

- role:
  - shared smoke/runtime helper
  - fallback/full MirBuilder lane still uses `MirBuilderBox.emit_from_program_json_v0(...)`
- contract shape:
  - common test harness
  - tightly connected to smoke tail callers
- audit priority:
  - last in the helper trio
  - must not be mixed with the helper-local slices above

## Fixed Delete Order

1. audit `tools/hakorune_emit_mir.sh`
2. audit `tools/selfhost/selfhost_build.sh`
3. audit `tools/smokes/v2/lib/test_runner.sh`
4. only then collapse the 43-file smoke tail that depends on the test runner
5. diagnostics/probe keep remains after live/helper caller audit

## Guardrails

- do not audit `tools/smokes/v2/lib/test_runner.sh` in the same patch as `tools/hakorune_emit_mir.sh`
- do not fold the 43-file smoke tail into the helper-trio patch
- keep `tools/selfhost/selfhost_build.sh` separate from `tools/hakorune_emit_mir.sh`; both are shared helpers but have different contracts
- current authority and `.hako` live/bootstrap owners are out of scope here

## Retreat Finding

- helper trio is not homogeneous:
  - `hakorune_emit_mir.sh` is a narrow helper-local pipeline
  - `selfhost_build.sh` is a build contract helper
  - `test_runner.sh` is a shared smoke harness tied to the 43-file tail
- therefore, the safest next helper audit is `tools/hakorune_emit_mir.sh`
- `tools/hakorune_emit_mir.sh` can keep shrinking by localizing its embedded selfhost/provider runner generation; this is helper-local structure work and does not require touching the shared build/test contracts
- `tools/hakorune_emit_mir.sh` still owns `Stage-B Program(JSON) production + imports normalize + Program→MIR fallback funnel`, so the next safe helper-local slice is the Stage-B Program(JSON) production block itself; do not mix that with direct-emit fallback or legacy delegate retirement in the same patch
- `tools/hakorune_emit_mir.sh` now keeps Stage-B Program(JSON) production + imports normalize behind `emit_stageb_program_json_v0()`, so the remaining helper-local funnel is narrower and the next slice can focus on direct-emit fallback or delegate tail in isolation
- `tools/smokes/v2/lib/test_runner.sh` should be treated as the bridge between helper keep and smoke tail, not as “just another helper script”

## Immediate Next

1. isolate the exact JSON v0 contract still needed by `tools/hakorune_emit_mir.sh`
2. record whether `tools/selfhost/selfhost_build.sh` still needs the `HAKO_USE_BUILDBOX=1` keep as a real live contract
3. defer `tools/smokes/v2/lib/test_runner.sh` until the smoke tail audit is ready
