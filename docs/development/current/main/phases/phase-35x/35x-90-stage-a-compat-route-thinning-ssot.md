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
- Stage-A child spawn/setup と captured payload handoff は `stage_a_route.rs` の thin owner に寄せる。
- direct `MIR(JSON)` lane と Program(JSON v0) compat lane の ownership を route sequencing から分離する。

## Fixed Rules

- keep `selfhost.rs` focused on Stage-A route sequencing / terminal accept only
- keep Stage-A child spawn/setup and captured payload handoff under `stage_a_route.rs`
- keep captured payload-family resolution under `stage_a_compat_bridge.rs`
- keep Program(JSON v0) compat fallback explicit-only via `stage_a_policy.rs`
- keep `LANE_DIRECT` for direct MIR payloads; do not widen Stage-A compat semantics
- raw backend default/token truthification remains deferred

## Macro Tasks

| Wave | Status | Goal | Acceptance |
| --- | --- | --- | --- |
| `35xA payload owner split` | landed | captured payload resolution を `stage_a_compat_bridge.rs` へ寄せる | `selfhost.rs` が orchestration-only に近づく |
| `35xB orchestration-only lock` | landed | `selfhost.rs` の Stage-A sequencing を exact に固定する | Stage-A compat lane に新機能が増えない |
| `35xC proof/closeout` | landed | slimmed Stage-A route を proof と docs に落とす | next phase へ handoff できる |

## Micro Tasks

| ID | Status | Task | Acceptance |
| --- | --- | --- | --- |
| `35xA1` | landed | captured payload resolver rehome | `resolve_captured_payload_to_mir(...)` が `stage_a_compat_bridge.rs` にあり、`selfhost.rs` はそれを呼ぶ |
| `35xA2` | landed | `selfhost.rs` orchestration-only lock | `stage_a_route.rs` が child spawn/setup を持ち、`selfhost.rs` は route sequencing / terminal accept だけを読む |
| `35xB1` | landed | Stage-A compat keep/no-widen lock | Program(JSON v0) compat lane が widen しない |
| `35xC1` | landed | proof/closeout | direct-vs-compat Stage-A route が evidence command まで固定される |

## Current Focus

- current phase state: `closeout review`
- current blocker: `none`
- exact reading:
  - `child.rs` keeps shell/process residue and public capture facade
  - `stage_a_route.rs` owns Stage-A child spawn/setup and captured payload handoff
  - `stage_a_compat_bridge.rs` owns captured payload-family resolution through `resolve_captured_payload_to_mir(...)`
  - direct MIR payload stays `LANE_DIRECT`
  - Program(JSON v0) payload still flows through explicit compat fallback ownership in the same bridge
  - `selfhost.rs` delegates Stage-A route setup and focuses on route sequencing / terminal accept
