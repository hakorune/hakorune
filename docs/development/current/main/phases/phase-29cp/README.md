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

## Current Blocker

- `build_stage1.sh --artifact-kind stage1-cli --force-rebuild` gets past the C ABI rebuild seam and bridge-first MIR build, but the reduced artifact itself is runnable bootstrap output rather than the payload-emitting contract
- the payload proof is now anchored on the stage0 bootstrap route, not on direct artifact emission
- `stage1-contract` / `identity_routes` need the same bootstrap-route wording so the probe and the build route read the same contract

## Acceptance

- `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --force-rebuild` PASS
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` remains green
- stage0 bootstrap route emits the same Program/MIR identities and the reduced artifact remains runnable

## Non-Goals

- changing `stage1_cli_env.hako` authority semantics
- reopening `phase-29cn`
- reworking the C ABI shim repair body
- mixing this bootstrap env-contract fix with backend-zero or `.hako` authoring
