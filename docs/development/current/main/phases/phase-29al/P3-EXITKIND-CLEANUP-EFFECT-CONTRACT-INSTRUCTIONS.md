---
Status: Active
Scope: docs-first（仕様不変）
Related:
- docs/development/current/main/phases/phase-29al/README.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
---

# Phase 29al P3: ExitKind/Cleanup/Effect contract SSOT（docs-first）

Date: 2025-12-29  
Status: Ready for execution  
Scope: cleanup を ExitKind と effect の契約として固定する（仕様不変）

## Objective

- cleanup（RC release 等）が、最適化/再順序で壊れないための **最小契約**を SSOT 化する
- “pattern個別cleanup” の増殖を防ぎ、ExitKind の語彙へ収束させる

## Non-goals

- 新しい cleanup 機能追加
- RC insertion の仕様拡張（実装変更）
- 新 env var / 恒常ログ追加

## Deliverables

- SSOT: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- 参照導線:
  - `docs/development/current/main/design/planfrag-ssot-registry.md`
  - `docs/development/current/main/design/joinir-plan-frag-ssot.md`（必要なら）

## Acceptance

- docs-only（コード変更なし）
- “cleanup は ExitKind 語彙” と “cleanup=Mut/Control跨ぎ禁止” が明文化されている

## Commit

- `git add -A && git commit -m "docs(phase29al): exitkind cleanup effect contract ssot"`

