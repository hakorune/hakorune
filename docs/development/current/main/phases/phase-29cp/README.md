---
Status: Active
Decision: provisional
Date: 2026-03-20
Scope: `stage1-cli` bootstrap rebuild の payload proof を stage0 bootstrap route 側に寄せ、reduced `stage1-cli` artifact は runnable bootstrap output として扱う専用 phase。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-evidence-and-legacy-lanes.md
  - docs/development/current/main/phases/phase-29co/README.md
  - tools/selfhost/lib/stage1_contract.sh
  - tools/selfhost/lib/identity_routes.sh
  - tools/selfhost/build_stage1.sh
  - src/runner/stage1_bridge/env.rs
  - src/runner/stage1_bridge/env/stage1_aliases.rs
  - src/runner/stage1_bridge/stub_child.rs
---

# Phase 29cp: Stage1 Bootstrap Child Entry Contract Repair

## Goal

- `stage1-cli` bootstrap rebuild の payload proof を stage0 bootstrap route に一本化する。
- 具体的には `target/release/hakorune` 側で Program(JSON v0) / MIR(JSON v0) payload proof を維持し、reduced `stage1-cli` artifact は runnable bootstrap output として扱う。
- `STAGE1_CLI_ENTRY` / `HAKORUNE_STAGE1_ENTRY` は child contract の SSOT だが、この phase の本当の目的は artifact payload ではなく bootstrap route proof の整理である。

## Boundary

- in scope:
  - `tools/selfhost/lib/stage1_contract.sh` の child env shaping
  - `tools/selfhost/lib/identity_routes.sh` の exact probe contract wording
  - `tools/selfhost/build_stage1.sh` の stage1-cli bootstrap handoff wording
  - `src/runner/stage1_bridge/env.rs` / `src/runner/stage1_bridge/env/stage1_aliases.rs` / `src/runner/stage1_bridge/stub_child.rs`
- out of scope:
  - `lang/c-abi/shims/hako_json_v1.c` の schema repair body work
  - backend-zero / `.hako` authoring wave
  - Program(JSON v0) retirement body work
  - kernel migration refactors

## Fixed Order

1. stage0 bootstrap route の payload proof を inventory 化する
2. reduced `stage1-cli` artifact を runnable bootstrap output として確認する
3. `build_stage1.sh --artifact-kind stage1-cli --force-rebuild` を green に戻す
4. stage0 wrapper / bootstrap route の wording を docs と helper で揃える

## Repair Checklist

- [x] stage0 bootstrap route proof inventory
- [x] reduced `stage1-cli` artifact is runnable bootstrap output
- [x] legacy `stage1-cli` env route is diagnostics-only and no longer the acceptance path
- [x] `build_stage1.sh --artifact-kind stage1-cli --force-rebuild` returns green
- [x] helper/docs wording is aligned across `stage1_contract.sh`, `identity_routes.sh`, and `build_stage1.sh`

## Current Blocker

- no active blocker remains for the bootstrap route repair; the reduced `stage1-cli` artifact is a runnable bootstrap output and payload proof stays on the stage0 bootstrap route
- the legacy `NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_PROGRAM_JSON=1 ... target/selfhost/hakorune.stage1_cli` probe still returns `Result: 0`, but that probe is diagnostics-only and no longer blocks acceptance
- `stage1-contract` / `identity_routes` / `build_stage1.sh` now read the same bootstrap-route wording

## Acceptance

- `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS
- `target/release/hakorune` on the stage0 bootstrap route emits Program(JSON v0) / MIR(JSON v0)
- `target/selfhost/hakorune.stage1_cli` remains runnable bootstrap output
- stage0 bootstrap route emits the same Program/MIR identities and the reduced artifact remains runnable
- full identity compare remains a separate diagnostics lane, not this phase's acceptance gate

## Non-Goals

- changing `stage1_cli_env.hako` authority semantics
- reopening `phase-29cn`
- reworking the C ABI shim repair body
- mixing this bootstrap env-contract fix with backend-zero or `.hako` authoring
