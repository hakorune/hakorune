---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: plugin lane `PLG-06-min1` として wave-3 entry lock（PythonCompiler plugin）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-112-plg05-net-wave2-min7-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg06_pycompiler_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pycompiler_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-113 PLG-06 PythonCompiler Wave-3 Min1 SSOT

## 0. Goal

`PLG-06`（wave-3）の entry lock を `nyash-python-compiler-plugin` 1件（min1）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- 既存 wave-1 / wave-2（PLG-05-min1..min7）を壊さない

## 1. Boundary (fixed)

In scope:
1. PythonCompiler plugin（`PythonCompilerBox`）を plugin config 経由で VM から生成できること
2. `compile` の最小動作契約（`pyc_out=void`、戻り値bridgeは既知の後続課題）を fixture/smoke で固定すること
3. `tools/vm_plugin_smoke.sh` で wave-1 pilots + wave-2 pilots + wave-3 PythonCompiler pilot を連続検証すること

Out of scope:
1. wave-3 の他 plugin 同時 rollout（Python/PythonParser/Egui）
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg06_pycompiler_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pycompiler_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `pyc_out=void` を含む
   - `Unknown Box type: PythonCompilerBox` を含まない

## 2.1 Known Debt (explicit)

- debt tag: `[plg06/pycompiler:return-bridge]`
- 現状: `compile` 呼び出しは成功するが戻り値は `void` で観測される。
- 昇格条件（PLG-06-min2 以降）:
  1. `compile` 戻り値が VM caller へ string として橋渡しされる
  2. この文書と smoke を `pyc_out=static box Generated...` 契約へ置換する

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg06_pycompiler_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + wave-1 pilots + wave-2 pilots + wave-3 PythonCompiler pilot）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 4. Decision

Decision: accepted

- `PLG-06-min1`（wave-3 entry lock, PythonCompiler）は完了。
- active next は `PLG-06-min2`（wave-3 rollout を 1 plugin ずつ継続）。
