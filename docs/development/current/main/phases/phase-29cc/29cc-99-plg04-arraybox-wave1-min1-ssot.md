---
Status: Done
Decision: accepted
Date: 2026-02-25
Scope: plugin lane `PLG-04-min1` として wave-1 rollout の最小1件（ArrayBox）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-98-plg03-counterbox-wave1-pilot-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg04_arraybox_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_arraybox_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-99 PLG-04 ArrayBox Wave-1 Min1 SSOT

## 0. Goal

`PLG-04` の wave rollout を `ArrayBox` 1件（min1）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- `PLG-03`（CounterBox pilot）との併走で regression を監視

## 1. Boundary (fixed)

In scope:
1. ArrayBox を plugin config 経由で VM から生成できること
2. `push/get/size` の最小動作契約（`array_size=2`, `array_g1=4`）を fixture/smoke で固定すること
3. 既存導線 `tools/vm_plugin_smoke.sh` で PLG-03/PLG-04-min1 を連続検証すること

Out of scope:
1. wave-1 の他 plugin（String/Map/Console/FileBox）同時 rollout
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg04_arraybox_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_arraybox_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `array_size=2` と `array_g1=4` を含む
   - `Unknown Box type: ArrayBox` を含まない

## 3. Evidence (2026-02-25)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_arraybox_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + ArrayBox pilot）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `cargo check --bin hakorune` -> PASS
5. `bash tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh` -> PASS

## 4. Decision

Decision: accepted

- `PLG-04-min1`（ArrayBox）は完了。
- active next は `PLG-04-min2`（wave-1 rollout を 1 plugin ずつ継続）。

## 5. Rollback

1. `tools/vm_plugin_smoke.sh` の ArrayBox 追加導線を戻す
2. `phase29cc_plg04_arraybox_pilot_vm.sh` と fixture を削除
3. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc` pointer を `PLG-04-min1` 未完了状態へ戻す
