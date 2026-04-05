# Phase 159x: observe trace split

- Status: Active
- 目的: exact counter lane と heavy trace lane を分け、observe plane を `release / observe-release / trace-debug` に整理する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `crates/nyash_kernel/src/observe/**`

## Goal

- exact counter lane は `perf-observe` のまま narrow probe に固定する
- heavy trace / sampled probe / scoped timing は別 feature lane に分ける
- exact counter sink と trace sink を混ぜない

## Current Shape

- `perf-observe`
  - canonical contract identity
  - TLS exact counters
  - stderr summary
- next split target
  - sampled trace
  - scoped duration probe
  - expensive debug-only observer

## Next

1. `phase-137x main kilo reopen selection`
2. `phase-kx vm-hako small reference interpreter recut`

## Exit

- exact counter lane が `perf-observe` として固定される
- trace/debug-only observer lane の置き場所が source-backed に読める
- perf reopen 前に observer plane が `exact` と `trace` で混ざらない
