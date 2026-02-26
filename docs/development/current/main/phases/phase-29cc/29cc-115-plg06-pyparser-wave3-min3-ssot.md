---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: plugin lane `PLG-06-min3` として wave-3 rollout（PythonParser plugin）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-114-plg06-python-wave3-min2-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg06_pyparser_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pyparser_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-115 PLG-06 PythonParser Wave-3 Min3 SSOT

## 0. Goal

`PLG-06`（wave-3）の rollout を `nyash-python-parser-plugin` 1件（min3）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- 既存 wave-1 / wave-2 / wave-3-min1..min2 を壊さない

## 1. Boundary (fixed)

In scope:
1. PythonParser plugin（`PythonParserBox`）を plugin config 経由で VM から生成できること
2. `parse` の最小動作契約（`pyparser_out=void`、戻り値bridgeは既知の後続課題）を fixture/smoke で固定すること
3. `tools/vm_plugin_smoke.sh` で wave-1 pilots + wave-2 pilots + wave-3 pilots を連続検証すること

Out of scope:
1. wave-3 の他 plugin 同時 rollout（Egui）
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg06_pyparser_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pyparser_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `pyparser_out=void` を含む
   - `Unknown Box type: PythonParserBox` を含まない

## 2.1 Known Debt (explicit)

- debt tag: `[plg06/pyparser:return-bridge]`
- 現状: `parse` 呼び出しは成功するが戻り値は `void` で観測される。
- 昇格条件（PLG-06-min4 以降）:
  1. `parse` 戻り値（JSON文字列）が VM caller へ橋渡しされる
  2. この文書と smoke を non-void payload 契約へ置換する

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pyparser_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + wave-1 pilots + wave-2 pilots + wave-3 pilots）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 4. Decision

Decision: accepted

- `PLG-06-min3`（wave-3 rollout, PythonParser plugin）は完了。
- active next は `PLG-06-min4`（wave-3 rollout を 1 plugin ずつ継続）。
