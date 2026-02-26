---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G3-min1（canvas/enhanced demo gap inventory）を docs-first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
  - docs/development/current/main/phases/phase-29cc/29cc-136-wsm-g2-min3-guide-alignment-lock-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - projects/nyash-wasm/canvas_playground.html
  - projects/nyash-wasm/enhanced_playground.html
---

# 29cc-137 WSM-G3-min1 Gap Inventory Lock

## Purpose
G2 完了後に G3 を実装へ進める前提として、`canvas_playground` / `enhanced_playground` の API 要求と現行 wasm backend の対応差分を優先順位つきで固定する。

## Decision
1. 差分の主軸を Canvas drawing core とし、`fillRect/fillText` 以外を未対応ギャップとして明文化した。
2. `console.group/groupEnd/separator` は引き続き scope-out（fail-fast維持）として扱う。
3. DOM/event bridge は G3後段の語彙設計対象（`env.dom.*` / `env.anim.*`）とし、先に Canvas 1語彙ずつ進める。

## Acceptance
- `docs/guides/wasm-guide/planning/unsupported_features.md` に G3-min1 節があり、対象デモ・優先順・次の1語彙が記載されていること。

## Next
- `WSM-G3-min2`: Canvas 1語彙（`strokeRect` or `clear`）の fixture/gate 追加。
