Status: Active
Date: 2026-07-28
Scope: restart in 2-5 minutes with a thin pointer surface.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
bash tools/checks/current_state_pointer_guard.sh
```

Run heavier gates only when the next slice is ready:

```bash
bash tools/checks/dev_gate.sh quick
# standalone check only when the gate did not already cover it
CARGO_BUILD_JOBS=4 cargo check
```

同じcheckoutの別terminalやbackground terminalでCargoを重ねて起動しない。
開始前に既存の `cargo`/`rustc` が終わっていることを確認し、Cargoは常に1本ずつ
実行する。`dev_gate.sh` も子Cargoを最大4 jobへ制限する。`--release`、
`--nocapture`、`RUSTFLAGS`切替の扱いとOOM停止線は
[`agent-current-entry-contract-ssot.md`](design/agent-current-entry-contract-ssot.md#local-cargo-resource-safety-contract)
に従う。

`Waiting for background terminal` は完了ではない。再起動・強制終了後は、まず
次を実行して残存プロセスが空になるまで新しい Cargo を起動しない。

```bash
git status -sb
ps -eo pid,ppid,stat,etime,pcpu,pmem,args | rg '[c]argo|[r]ustc|[s]ccache|[r]ustdoc' || true
```

focused test の `0 passed` / `0 tests` は green の証拠ではなく、filter の誤りとして
完全な test path を `--exact` で選び直す。warning を隠すための
`RUSTFLAGS=-Awarnings` 再試行や `--nocapture` の常用も行わない。

### Observed forced-termination pattern (2026-08-18)

The interrupted run printed a very large warning transcript from
`cargo test -q ... -- --nocapture`, matched zero tests, and then remained in
`Waiting for background terminal` while several background terminals were
still active. A second Cargo command with a different `RUSTFLAGS` was also
queued. The host kernel log later confirmed `global_oom` and
`Out of memory: Killed process ... (codex)` while Cargo/rustc workers were
resident (Codex anonymous RSS was about 10.3 GiB). The forced termination was
therefore host-level OOM, not a Rust panic or an application-code test crash.

Prevent recurrence with this fixed sequence:

1. Run one focused library target with `CARGO_BUILD_JOBS=4 cargo test
   --profile quick --lib <filter>` and omit `--nocapture`.
2. Treat `0 passed`/`0 tests` as a filter error; rerun the complete test path
   with `--exact` only after confirming the name.
3. Never start a second Cargo command, warning-suppression retry, or
   `--release` build while another top-level Cargo process is active.
4. After interruption, inspect `cargo`/`rustc` processes and wait or stop
   redundant children before resuming. Only an empty process check permits
   the next Cargo invocation.

Use `--release` only for an active card's final evidence. Day-to-day
iteration uses `--profile quick`; its parallel code generation is the
repository's intended fast path.

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
- current scope and parked resume: read `active_lane_status` and the workstream

## Restart Notes

- handoff frontier: read `current_blocker_token` in `CURRENT_STATE.toml`
- when `work_mode = "design_stop"`, stop the goal-driven execution loop here and review the frontier card before selecting more work
- read `latest_card_path` before editing
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
