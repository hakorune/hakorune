---
Status: SSOT
Decision: provisional
Date: 2026-04-03
Scope: Stage-A payload resolution / compat fallback ownership を `selfhost.rs` から thin owner へ寄せる順番を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-35x/README.md
  - docs/development/current/main/phases/phase-35x/35x-91-task-board.md
  - docs/development/current/main/phases/phase-34x/34x-90-stage0-shell-residue-split-ssot.md
---

# 35x-90 Stage-A Compat Route Thinning

## Goal

- Stage-A child capture 後の payload-family branching を `selfhost.rs` から薄くし、captured payload resolution を `stage_a_compat_bridge.rs` に集約する。
- direct `MIR(JSON)` lane と Program(JSON v0) compat lane の ownership を route sequencing から分離する。

## Fixed Rules

- keep `selfhost.rs` focused on Stage-A spawn / route sequencing / terminal accept only
- keep captured payload-family resolution under `stage_a_compat_bridge.rs`
- keep Program(JSON v0) compat fallback explicit-only via `stage_a_policy.rs`
- keep `LANE_DIRECT` for direct MIR payloads; do not widen Stage-A compat semantics
- raw backend default/token truthification remains deferred

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `35xA payload owner split` | active | captured payload resolution を `stage_a_compat_bridge.rs` へ寄せる | `selfhost.rs` が orchestration-only に近づく |
| `35xB orchestration-only lock` | queued | `selfhost.rs` の Stage-A sequencing を exact に固定する | Stage-A compat lane に新機能が増えない |
| `35xC proof/closeout` | queued | slimmed Stage-A route を proof と docs に落とす | next phase へ handoff できる |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `35xA1` | landed | captured payload resolver rehome | `resolve_captured_payload_to_mir(...)` が `stage_a_compat_bridge.rs` にあり、`selfhost.rs` はそれを呼ぶ |
| `35xA2` | active | `selfhost.rs` orchestration-only lock | Stage-A child spawn / accept / fallback owner reading が exact に読める |
| `35xB1` | queued | Stage-A compat keep/no-widen lock | Program(JSON v0) compat lane が widen しない |
| `35xC1` | queued | proof/closeout | direct-vs-compat Stage-A route が evidence command まで固定される |

## Current Focus

- active macro wave: `35xA payload owner split`
- active micro task: `35xA2 selfhost orchestration-only lock`
- next queued micro task: `35xB1 Stage-A compat keep/no-widen lock`
- current blocker: `none`
- exact reading:
  - `child.rs` keeps shell/process residue and public capture facade
  - `stage_a_compat_bridge.rs` now owns captured payload-family resolution through `resolve_captured_payload_to_mir(...)`
  - direct MIR payload stays `LANE_DIRECT`
  - Program(JSON v0) payload still flows through explicit compat fallback ownership in the same bridge
  - `selfhost.rs` now delegates captured payload resolution and focuses on Stage-A spawn / route sequencing / terminal accept
