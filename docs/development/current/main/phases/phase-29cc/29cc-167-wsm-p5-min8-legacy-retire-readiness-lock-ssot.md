---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min8（.hako-only roadmap P5）として legacy-wasm-rust lane の retire readiness 判定基準を docs-first で固定し、route trace 証跡と lightweight gate の接続を SSOT 化する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-166-wsm-p5-min7-shape-trace-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-167 WSM-P5-min8 Legacy Retire Readiness Lock

## Purpose
`legacy-wasm-rust` lane を即削除せず、先に「どの条件が揃えば retire 実行へ進めるか」を固定する。  
min7 で導入した route trace 契約を使い、判定根拠を lightweight gate で再現可能にする。

## Decision
1. `WSM-P5-min8` は docs-first の readiness lock とし、code behavior は変えない。
2. retire readiness 判定は次の 3 点を満たすこと:
   - `NYASH_WASM_ROUTE_TRACE=1` の trace 契約が維持される。
   - `default` policy で `native-shape-table` 証跡を観測できる。
   - `legacy-wasm-rust` policy で `legacy-rust` 証跡を観測できる。
3. 受け入れ gate は `phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh` を `wasm-boundary-lite` に接続して固定する。
4. legacy lane の実際の retire（挙動変更）は `WSM-P5-min9` 以降で実施する。

## Implemented
1. smoke 追加:
   - `tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh`
   - lock doc keyword と route-trace 契約テストを検証。
2. gate 接続:
   - `tools/checks/dev_gate.sh` の `wasm-boundary-lite` に `WSM-P5-min8` step を追加。
3. pointer sync:
   - `CURRENT_TASK.md` / `phase-29cc/README.md` / `10-Now.md` を `next=WSM-P5-min9` へ同期。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p5/phase29cc_wsm_p5_min8_legacy_retire_readiness_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min9`: legacy lane retire execution lock（readiness 判定済み条件のもとで縮退実行境界を固定）。
