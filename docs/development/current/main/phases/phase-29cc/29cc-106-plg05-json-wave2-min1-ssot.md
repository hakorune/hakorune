---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: plugin lane `PLG-05-min1` として wave-2 entry（Json plugin）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg05_json_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_json_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-106 PLG-05 Json Wave-2 Min1 SSOT

## 0. Goal

`PLG-05`（wave-2）の entry lock を `nyash-json-plugin` 1件（min1）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- wave-1 完了状態を壊さずに wave-2 へ進む

## 1. Boundary (fixed)

In scope:
1. Json plugin（`JsonDocBox`/`JsonNodeBox`）を plugin config 経由で VM から生成できること
2. `parse/root/get/str/size` の最小動作契約（`json_kind=Program`, `json_stmt_size=0`）を fixture/smoke で固定すること
3. `tools/vm_plugin_smoke.sh` で wave-1 pilots + wave-2 Json pilot を連続検証すること

Out of scope:
1. wave-2 の他 plugin 同時 rollout
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg05_json_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_json_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `json_kind=Program` と `json_stmt_size=0` を含む
   - `Unknown Box type: JsonDocBox/JsonNodeBox` を含まない

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_json_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + wave-1 pilots + wave-2 Json pilot）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 4. Decision

Decision: accepted

- `PLG-05-min1`（wave-2 Json entry lock）は完了。
- active next は `PLG-05-min2`（wave-2 rollout を 1 plugin ずつ継続）。
