Status: Active
Date: 2026-08-26
Scope: restart in 2-5 minutes with a thin pointer surface.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/RULES.md

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

重いgateはactive cardが要求したときだけ実行する。Cargoのjob数・同時実行・
profileの規則は[開発ルール](../../RULES.md#6-テストと計算資源)に従う。

中断後は`cargo`/`rustc` processが終了したことを確かめてから再開する。
focused testが0件ならfilterを見直す。資源と検証の詳細は
[開発ルール](../../RULES.md#6-テストと計算資源)に集約している。

## Current Lane

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- MirBuilder north star: read `mirbuilder_north_star` in `CURRENT_STATE.toml`
- workstream card: read `latest_workstream_card` in `CURRENT_STATE.toml`
- method anchor: read `method_anchor` in `CURRENT_STATE.toml`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- work mode: read `work_mode` in `CURRENT_STATE.toml`; do not infer it from the blocker text
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- current scope: read `latest_card_summary`; parked resume and exact task order
  belong to the file named by `latest_workstream_card`

## Restart Notes

- handoff frontier: read `current_blocker_token` in `CURRENT_STATE.toml`
- when `work_mode = "design_stop"`, pause implementation and resolve the named design dependency in the frontier card; goal status follows the session contract
- read `latest_card_path` before editing
- apply the phase-specific proof timing in
  [開発ルール](../../RULES.md#4-建設と旧経路退役の証明タイミング): start
  construction after the bounded mapping is fixed; require caller-zero and
  acceptance before old-edge removal
- continue only the exact `current_blocker_token` and `latest_card_path` from
  `CURRENT_STATE.toml`; this mirror does not select or rename executable rows
- read `method_anchor` for the in-place production replacement law
- read `mirbuilder_north_star` before selecting a replacement cell; cell and
  LOC counters are migration metrics, not the architecture goal
- an in-place production-replacement `I0` requires a real production caller
  switch; a disconnected candidate is S0/PROBE0, not I0. Bounded
  parser/resolver/contract I0 rows use their own card acceptance and do not
  claim a production switch
- the same cell deletes the selected old branch in I0/R0 before unrelated work
- Stage-B, Ownership, Language v1, and `.hako` selfhost lanes are parked
  unless `CURRENT_STATE.toml` explicitly selects one of them
- do not paste landed chronology into restart docs
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- the current lane is the `active_lane` in `CURRENT_STATE.toml`
- all other rows remain parked unless `CURRENT_STATE.toml` explicitly selects
  them
- product/app validation now uses EXE/AOT as the primary route; VM work is a
  small semantic-reference subset only
