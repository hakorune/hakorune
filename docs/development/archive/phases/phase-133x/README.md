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
  - `micro kilo` is now closed with `substring_concat` parity lock, `indexof_line` faster-than-C freeze, and `array_getset` parity lock
  - successor is `phase-134x nyash_kernel layer recut selection`

## Decision Now

- `vm` default removal is landed
- explicit `vm` / `vm-hako` proof-debug lanes remain frozen keep
- caller-zero remains parked debt, not current work
- fixed order remains `leaf-proof micro -> micro kilo -> main kilo`
- micro reopen is now closed:
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3 / ratio_ms=1.00`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4 / ratio_ms=1.00`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3 / ratio_ms=1.33`
- before main kilo, one structural kernel lane is now fixed
- successor lane is `phase-134x nyash_kernel layer recut selection`

## Fresh Read

- `bash tools/perf/run_kilo_micro_machine_ladder.sh 1 7`
  - `kilo_micro_indexof_line`: `c_ms=4 / ny_aot_ms=3 / ratio_ms=1.33`
  - `kilo_micro_substring_concat`: `c_ms=3 / ny_aot_ms=3 / ratio_ms=1.00`
  - `kilo_micro_array_getset`: `c_ms=4 / ny_aot_ms=4 / ratio_ms=1.00`
- `bash tools/perf/report_mir_hotops.sh kilo_micro_substring_concat --top 20`
  - `RuntimeDataBox.substring` x3
  - `StringBox.length` x2
  - `RuntimeDataBox.length` x1
- `bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat ny_main 20`
  - hot tier stays concentrated in `ny_main`
  - next cut remains owner-lock / bundle-first, not broad substrate retune

## Next

1. keep `phase-132x` closed
2. keep `kilo_micro_substring_concat` parity-locked
3. keep `kilo_micro_indexof_line` frozen faster-than-C
4. carry `kilo_kernel_small_hk` as the next post-recut main-kilo front
