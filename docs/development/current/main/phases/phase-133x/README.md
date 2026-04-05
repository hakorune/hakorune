# Phase 133x: micro kilo reopen selection

- 目的: `phase-132x` closeout 後に、vm keep の parked debt を凍結したまま perf/mainline を `micro kilo` から再開する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/perf-optimization-method-ssot.md`
  - `docs/development/current/main/phases/phase-29ck/P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
- success:
  - `phase-132x` is landed
  - current no longer reads like vm retirement work
  - `micro kilo` is fixed as the active optimization reopening point
  - first owner target is `kilo_micro_substring_concat`, not broad array retune

## Decision Now

- `vm` default removal is landed
- explicit `vm` / `vm-hako` proof-debug lanes remain frozen keep
- caller-zero remains parked debt, not current work
- fixed order remains `leaf-proof micro -> micro kilo -> main kilo`
- current `micro kilo` ranking is:
  - `kilo_micro_substring_concat` first
  - `kilo_micro_array_getset` second
  - `kilo_micro_indexof_line` third
- `array_getset` is recheck-only for now because its same-artifact direct-route proof is already strong

## Next

1. keep `phase-132x` closed
2. lock `kilo_micro_substring_concat` as the first owner slice
3. recheck `kilo_micro_array_getset`
4. keep vm-family retirement work parked unless a new exact blocker appears
