---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X25 route orchestration 入口一本化（`vm` / `vm-hako` / selfhost stage-a）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - src/runner/route_orchestrator.rs
  - src/runner/dispatch.rs
  - src/runner/selfhost.rs
---

# Phase 29x X25: Route Orchestration Single Entry SSOT

## 0. Goal

`vm` / `vm-hako` / selfhost stage-a の route 判定入口を一本化し、
直配線の増殖を防ぐ。

## 1. Single Entry Contract

1. VM lane 選択は `src/runner/route_orchestrator.rs` の
   `execute_vm_route()` / `decide_vm_route_plan()` を唯一の判定入口にする。
2. selfhost stage-a の compat 境界判定は
   `enforce_stage_a_compat_policy_or_exit()` を唯一の入口にする。
3. selfhost stage-a の Program(JSON) strict reject 判定は
   `enforce_stage_a_program_payload_policy_or_exit()` を唯一の入口にする。
4. `dispatch.rs` / `selfhost.rs` 側に env 条件の直接分岐を再導入しない。

## 2. Route Tag Compatibility

既存の route 契約タグは維持する:

- `[vm-route/select] backend=vm lane=vm reason=default`
- `[vm-route/select] backend=vm lane=vm-hako reason=strict-dev-prefer`
- `[vm-route/select] backend=vm lane=compat-fallback reason=env:NYASH_VM_USE_FALLBACK=1`
- `[vm-route/select] backend=vm-hako lane=vm-hako reason=backend:vm-hako`
- `[contract][runtime-route][expected=mir-json] ... non_strict_compat=disabled ...`
- `[contract][runtime-route][expected=mir-json] ... strict_planner_required=1`

## 3. Evidence (X25)

1. `cargo check -q --bin hakorune`
2. `cargo test -q route_orchestrator -- --nocapture`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh`
5. `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## 4. Next Step

X26 では route observability 契約（タグ語彙/理由語彙）の固定を進める。
