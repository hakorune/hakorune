---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G2-min3 guide alignment（contributor入口の一本化）を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
  - docs/development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-135-wsm-g2-min2-headless-run-lock-ssot.md
  - docs/guides/wasm-guide/README.md
  - docs/guides/wasm-guide/wasm_quick_start.md
---

# 29cc-136 WSM-G2-min3 Guide Alignment Lock

## Purpose
WASM browser demo の運用入口を docs 上で一本化し、実行順が迷子にならない状態を固定する。

## Decision
1. `docs/guides/wasm-guide/README.md` の先頭に「現行の運用入口（G2固定）」を追加し、build/min1/min2/dev-gate の実行順を明示した。
2. `docs/guides/wasm-guide/wasm_quick_start.md` の先頭に「現行の最短手順（WSM-G2）」を追加し、旧プロトタイプ手順は「履歴資料」として分離した。
3. `29cc-133` task plan の状態を `WSM-G2-min1/min2/min3 done` へ更新し、次タスクを `WSM-G3-min1` に進めた。

## Acceptance
- `tools/checks/dev_gate.sh wasm-demo-g2`
- ガイド上の手順が `build -> min1 smoke -> min2 smoke -> dev-gate` の順で一致していること

## Next
- `WSM-G3-min1`: `canvas_playground.html` / `enhanced_playground.html` の API ギャップ棚卸し（fixtures/gates化の優先順位付け）。
