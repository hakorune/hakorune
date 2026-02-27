---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min9（.hako-only roadmap P5）として legacy-wasm-rust lane を accepted-but-blocked（実行境界でretire）へ固定し、縮退実行の fail-fast 契約を SSOT 化する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-167-wsm-p5-min8-legacy-retire-readiness-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min9_legacy_retire_execution_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-168 WSM-P5-min9 Legacy Retire Execution Lock

## Purpose
min8 で固定した readiness 判定を、実行境界の契約へ昇格する。  
`legacy-wasm-rust` は値として受理しつつ、WASM compile 実行時に fail-fast で停止する。

## Decision
0. policy 判断源は `NYASH_WASM_ROUTE_POLICY` を継続利用する（新しい route env は増やさない）。
1. `legacy-wasm-rust` は parse 受理を維持する（互換入口は保持）。
2. compile route が `legacy-rust` へ到達した時点で即 fail-fast する。
3. fail-fast タグは固定:
   - `[freeze:contract][wasm/legacy-route-retired] policy=legacy-wasm-rust lane=legacy-rust status=retired`
4. route trace（`NYASH_WASM_ROUTE_TRACE=1`）は維持し、legacy 分岐でも trace 行を先に出す。
5. hard remove（enum/parseから削除）は `WSM-P5-min10` で行う。

## Implemented
1. runner route boundary:
   - `src/runner/modes/wasm.rs`
   - `LegacyRust` 分岐を `compile_module` 実行から fail-fast へ変更。
2. contract tests:
   - `tests/wasm_demo_min_fixture.rs`
   - legacy success/parity 前提を retire fail-fast 契約へ更新。
3. smoke/gate:
   - `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min9_legacy_retire_execution_lock_vm.sh`
   - `tools/checks/dev_gate.sh` の `wasm-boundary-lite` へ統合。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min9_legacy_retire_execution_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min10`: legacy route hard remove lock（`legacy*` 受理値削除 + route enum/parse/test/docs 同期）。
