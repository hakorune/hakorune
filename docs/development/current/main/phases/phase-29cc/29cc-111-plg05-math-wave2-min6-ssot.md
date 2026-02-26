---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: plugin lane `PLG-05-min6` として wave-2 rollout（Math plugin）を docs+fixture+smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-106-plg05-json-wave2-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-107-plg05-toml-wave2-min2-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-108-plg05-regex-wave2-min3-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-109-plg05-encoding-wave2-min4-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-110-plg05-path-wave2-min5-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - apps/tests/phase29cc_plg05_time_pilot_min.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_time_pilot_vm.sh
  - tools/vm_plugin_smoke.sh
---

# 29cc-111 PLG-05 Math Wave-2 Min6 SSOT

## 0. Goal

`PLG-05`（wave-2）の rollout を `nyash-math-plugin` 1件（min6）で固定する。

- 1 blocker = 1 plugin = 1 commit
- docs-first（契約）+ fixture + smoke を同時に固定
- 既存 wave-1 / PLG-05-min1/min2/min3/min4/min5 を壊さない

## 1. Boundary (fixed)

In scope:
1. Math plugin の `TimeBox` を plugin config 経由で VM から生成できること
2. `now` の最小動作契約（`time_now=<digits>`）を fixture/smoke で固定すること
3. `tools/vm_plugin_smoke.sh` で wave-1 pilots + wave-2 (Json/TOML/Regex/Encoding/Path/Time) を連続検証すること

Out of scope:
1. wave-2 の他 plugin 同時 rollout
2. plugin ABI（PLG-01）や gate pack（PLG-02）の再定義
3. `.hako` plugin 本実装（全面移植）

## 2. Contract Lock

1. fixture: `apps/tests/phase29cc_plg05_time_pilot_min.hako`
2. smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_time_pilot_vm.sh`
3. pass条件:
   - VM実行が `rc=0`
   - 出力に `time_now=<digits>` を含む
   - `Unknown Box type: TimeBox` を含まない

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg05_time_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS（CounterBox + wave-1 pilots + wave-2 Json/TOML/Regex/Encoding/Path/Time pilots）
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 4. Decision

Decision: accepted

- `PLG-05-min6`（wave-2 Math rollout）は完了。
- active next は `PLG-05-min7`（wave-2 rollout を 1 plugin ずつ継続）。
