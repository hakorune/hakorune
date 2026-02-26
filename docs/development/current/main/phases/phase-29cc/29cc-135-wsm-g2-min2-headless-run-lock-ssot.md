---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G2-min2 headless browser run automation を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
  - docs/development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_browser_run_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-135 WSM-G2-min2 Headless Run Lock

## Purpose
`nyash_playground.html` の Run ボタン経路を headless browser で自動検証し、`WSM-02d` の demo-min marker と browser 実行結果を同じ契約で固定する。

## Decision
1. `nyash_playground.html` に `?autorun=1` モードを追加し、初期化完了後に `runNyash()` を自動実行する。
2. `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_browser_run_vm.sh` を追加し、以下を一気通貫で実行する。
   - `projects/nyash-wasm/build.sh`
   - `python3 -m http.server` 起動
   - `chromium-browser --headless --dump-dom` で `autorun=1` 実行
   - `wsm02d_demo_min_*` と `[autorun] done` marker を fail-fast 検証
3. `tools/checks/dev_gate.sh wasm-demo-g2` を追加し、`WSM-G2-min1` + `WSM-G2-min2` を日常実行可能にする。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g2_browser_run_vm.sh`
- `tools/checks/dev_gate.sh wasm-demo-g2`

## Next
- `WSM-G2-min3`（WASM guide alignment）: contributor 向け手順と gate を docs 側へ同期する。
