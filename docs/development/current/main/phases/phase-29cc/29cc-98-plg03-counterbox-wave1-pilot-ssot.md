---
Status: Done
Decision: accepted
Date: 2026-02-25
Scope: plugin lane `PLG-03` として wave-1 pilot（CounterBox 1件）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg03_counterbox_pilot_min.hako
  - apps/tests/vm-plugin-smoke-counter/main.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg03_counterbox_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-98 PLG-03 CounterBox Wave-1 Pilot SSOT

## 0. Goal

`PLG-03` の wave-1 pilot を `CounterBox` 1件で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- non-plugin done 契約（29cc-94）に影響させない

## 1. Boundary (fixed)

In scope:
1. CounterBox を plugin config 経由で VM から生成できること
2. `inc/get` の最小動作契約（`counter=1`）を fixture/smoke で固定すること
3. 既存導線 `tools/vm_plugin_smoke.sh` を pilot smoke へ委譲すること

Out of scope:
1. wave-1 の他 plugin（String/Integer/Array/Map/Console/FileBox）同時対応
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg03_counterbox_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg03_counterbox_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `counter=1` を含む
   - `Unknown Box type: CounterBox` を含まない

## 3. Evidence (2026-02-25)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg03_counterbox_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（pilot smoke へ委譲）
3. `cargo check --bin hakorune` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS

## 4. Decision

Decision: accepted

- `PLG-03`（wave-1 pilot）は CounterBox 1件で完了。
- 次の active は `PLG-04`（wave rollout）。

## 5. Rollback

1. `tools/vm_plugin_smoke.sh` を旧導線へ戻す
2. `phase29cc_plg03_counterbox_pilot_vm.sh` を削除
3. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc` pointer を `PLG-03` 未完了状態へ戻す
