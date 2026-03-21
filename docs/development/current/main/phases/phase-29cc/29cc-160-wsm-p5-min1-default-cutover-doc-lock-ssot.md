---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P5-min1（.hako-only roadmap P5）として default cutover の境界と gate を docs-first で固定する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-159-wsm-p4-min6-shape-table-lock-ssot.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-160 WSM-P5-min1 Default Cutover Docs Lock

## Purpose
P5 の default cutover（既定経路切替）に入る前に、切替境界・後方互換・受け入れ gate を docs-first で固定する。  
実装時の silent fallback や曖昧な二重既定を禁止する。

## Decision
1. default route は「`.hako` emitter/binary writer を優先」へ切替する。
2. Rust WASM backend は `--legacy-wasm-rust` 相当の明示 opt-in 互換 lane とする。
3. 切替中の二重既定（auto fallback）は禁止し、未対応は fail-fast で露出する。
4. P5 は docs-first で進め、先に gate を固定してから routing 実装へ入る。

## P5-min2 Entry Contract
1. route policy の判断源は 1 箇所（SSOT）に集約する。
2. default route と legacy route は同時有効にしない（明示フラグのみ legacy）。
3. 受け入れは既存 `wasm-boundary-lite` 緑を前提に、cutover 専用 smoke を追加して固定する。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min1_default_cutover_docs_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min2` は `29cc-161` で完了。
- `WSM-P5-min3` は `29cc-162` で完了。
- `WSM-P5-min4` は `29cc-163` で完了。
- `WSM-P5-min5` は `29cc-164` で完了。
- 次は `WSM-P5-min6`: pilot 以外の 1 shape を `.hako` 実体路へ拡張し、fallback 範囲をさらに縮退。
