---
Status: SSOT
Scope: JoinIR strict-mode StepTree “nested loop depth” guard（max_loop_depth）
Related:
- docs/development/current/main/phases/phase-29bq/README.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs
---

# ControlTree nested-loop depth guard (SSOT)

## 現状

`src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs` は strict mode で StepTree を検査し、
`tree.features.max_loop_depth > 2` の場合に fail-fast する。

エラー例:
- `[joinir/control_tree/nested_loop/depth_exceeded] Nesting depth 3 exceeds limit (max=2)`

## なぜ “cap” があるか

この guard は、JoinIR/Normalized の未受理形を **silent fallback せずに早期に止める**ためのもの。
ただし「depth=2 固定」は Phase 29bq の selfhost canary 進行を阻害し、レゴ化（feature/pipeline）で吸収する前に止まってしまう。

## 方針（Phase 29bq）

目的は “深いネストを通すための workaround” ではなく、
**後段の plan/feature が判断できるように、入口の cap を保守的に緩める**こと。

- strict/dev + `HAKO_JOINIR_PLANNER_REQUIRED=1` のとき:
  - `max_loop_depth` の上限を段階的に引き上げて、planner/feature 側で fail-fast できる状態にする
- strict のみ（planner_required OFF）のとき:
  - 既存の開発体験を崩さない範囲で維持（必要なら同じ上限でも可だが、SSOT で明記する）

## 受け入れ（green）

- selfhost canary が “depth guard で止まらない” ところまで進む（次の blocker は planner/feature 側で確定できる）
- 既存 gate の緑維持:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`（=29ae 含む）

