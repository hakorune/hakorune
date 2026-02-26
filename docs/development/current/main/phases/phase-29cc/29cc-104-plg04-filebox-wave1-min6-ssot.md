---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: plugin lane `PLG-04-min6` として wave-1 rollout の最小1件（FileBox）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-100-plg04-intcellbox-reserved-core-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-101-plg04-mapbox-wave1-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-102-plg04-stringbox-wave1-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-103-plg04-consolebox-wave1-min5-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg04_filebox_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_filebox_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-104 PLG-04 FileBox Wave-1 Min6 SSOT

## 0. Goal

`PLG-04` の wave rollout を `FileBox` 1件（min6）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- `PLG-04-min1/min2/min3/min4/min5`（ArrayBox/IntCellBox/MapBox/StringBox/ConsoleBox）との併走で regression を監視

## 1. Boundary (fixed)

In scope:
1. FileBox を plugin config 経由で VM から生成できること
2. 最小動作契約（`file_read=FILEBOX_PILOT_OK`）を fixture/smoke で固定すること
3. 既存導線 `tools/vm_plugin_smoke.sh` で PLG-03/PLG-04-min1/min2/min3/min4/min5/min6 を連続検証すること

Out of scope:
1. wave-2 以降 plugin の同時 rollout
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg04_filebox_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_filebox_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `file_read=FILEBOX_PILOT_OK` を含む
   - `Unknown Box type: FileBox` を含まない

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_filebox_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + ArrayBox + IntCellBox + MapBox + StringBox + ConsoleBox + FileBox）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 4. Decision

Decision: accepted

- `PLG-04-min6`（FileBox）は完了。
- active next は `none`（wave-1 complete）。
