---
Status: Complete
Scope: pattern_scan_with_init / pattern_split_scan decomposition (Skeleton + FeatureSet)
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# Phase 29bv: scan_with_init / split_scan decomposition

Goal: pattern_scan_with_init / pattern_split_scan を `LoopSkeleton + FeatureSet` 合成へ寄せる。
release 既定は不変、strict/dev + planner_required のみで表現力を広げる。

Non-goals:
- AST rewrite
- by-name 特例
- 既定挙動変更

Entry (gates):
- `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh`
- `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`

Acceptance:
- 上記 2 gate が PASS

## Plan (P0-P3)

- P0: 対象ファイルを SSOT で固定（この README と `src/mir/builder/control_flow/plan/REGISTRY.md`）
- P1: pattern_scan_with_init / pattern_split_scan を skeleton + features に切り出し（normalizer は pipeline 呼び出しのみ）
- P2: 重複 feature の統合（exit-map / carrier / if-join は既存 feature を使い回す）
- P3: legacy 経路を deprecated→削除（入口一本化）し、gate green を証跡として closeout

## Targets (SSOT)

- `src/mir/builder/control_flow/plan/normalizer/pattern_scan_with_init.rs`
- `src/mir/builder/control_flow/plan/normalizer/pattern_split_scan.rs`
- `src/mir/builder/control_flow/plan/extractors/pattern6_scan_with_init.rs`
- `src/mir/builder/control_flow/plan/extractors/pattern7_split_scan.rs`

## Notes

- selfhost canary は主目的にしない（secondary）。

## Progress

- P0: targets fixed in this README + REGISTRY.
- P1: scan_with_init / split_scan を skeleton + pipeline (ops) に移行済み。
- P2: ops の合流/exit/phi を feature helper 経由に寄せ、split_emit を唯一出口に固定。
- P3: normalizer 入口を pipeline のみへ一本化（legacy 分岐なし）。
- post-change green: `cargo build --release` / `./tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh` / `./tools/smokes/v2/profiles/integration/joinir/phase29bp_planner_required_dev_gate_v4_vm.sh`
