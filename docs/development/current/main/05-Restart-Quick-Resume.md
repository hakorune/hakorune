---
Status: Active
Date: 2026-04-04
Scope: 再起動直後に 2〜5 分で開発再開するための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/build-lane-separation-ssot.md
  - docs/tools/README.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-68x .hako runner authority/compat/facade recut`
- landed micro task: `49xD1 proof / closeout`
- landed micro task: `52xA2 archive README / wrapper wording rewrite`
- landed micro task: `52xB1 archive pack orchestrator wording cleanup`
- landed micro task: `52xC1 proof / closeout`
- landed micro task: `53xA1 residual VM caller inventory lock`
- landed micro task: `53xA2 proof-only / compat keep classification`
- landed micro task: `53xB1 rust-vm delete-ready source peel`
- landed micro task: `53xB2 vm-hako reference keep freeze`
- landed micro task: `53xC1 archive-ready docs/examples / wrapper cleanup`
- landed micro task: `53xD1 proof / closeout`
- landed micro task: `54xA1 successor lane inventory lock`
- landed micro task: `54xA2 candidate lane ranking`
- landed micro task: `54xB1 successor lane decision`
- landed micro task: `54xB2 retirement corridor lock`
- landed micro task: `54xD1 proof / closeout`
- landed micro task: `55xA1 route-surface inventory lock`
- landed micro task: `55xA2 backend/default/help exposure freeze`
- landed micro task: `55xB1 cli/backend affordance cleanup`
- landed micro task: `55xB2 selfhost route-surface cleanup`
- landed micro task: `55xC1 dispatch/orchestrator explicit keep narrowing`
- landed micro task: `55xD1 proof / closeout`
- landed micro task: `56xA1 proof-only keep inventory lock`
- landed micro task: `56xA2 compat keep boundary freeze`
- landed micro task: `56xB1 stage-a compat route pruning prep`
- landed micro task: `56xB2 vm fallback/core.hako keep pruning`
- landed micro task: `56xC1 proof smoke keep pruning`
- landed micro task: `56xD1 proof / closeout`
- landed micro task: `57xA1 residual rust-vm delete-ready inventory lock`
- landed micro task: `57xA2 keep/delete/archive classification freeze`
- landed micro task: `57xB1 caller-zero audit`
- landed micro task: `57xB2 removal candidate prep`
- landed micro task: `57xC1 removal wave`
- landed micro task: `57xD1 proof / closeout`
- landed micro task: `58xA1 successor lane inventory lock`
- landed micro task: `58xA2 candidate lane ranking`
- landed micro task: `58xB1 successor lane decision`
- landed micro task: `58xD1 proof / closeout`
- landed micro task: `59xA1 route-surface inventory lock`
- landed micro task: `60xA1 proof/compat keep inventory lock`
- landed micro task: `60xA2 compat keep boundary freeze`
- landed micro task: `60xB1 stage-a compat seam pruning`
- landed micro task: `60xB2 vm_fallback/core.hako keep pruning continuation`
- landed micro task: `60xC1 proof smoke keep pruning continuation`
- landed micro task: `60xD1 proof / closeout`
- landed micro task: `61xA1 residual caller inventory rerun`
- landed micro task: `61xA2 keep/delete-ready classification freeze`
- landed micro task: `61xB1 caller-zero proof bundle`
- landed micro task: `61xB2 removal candidate shortlist`
- landed micro task: `61xD1 proof / closeout`
- landed micro task: `62xA1 delete-ready candidate confirmation`
- landed micro task: `62xA2 removal/no-op decision`
- landed micro task: `62xB1 delete-ready removal` (no-op)
- landed micro task: `62xD1 proof / closeout`
- landed micro task: `63xA1 retirement-decision evidence lock`
- landed micro task: `63xA2 retire-vs-residual decision`
- landed micro task: `63xB1 residual keep stop-line or retirement plan freeze`
- landed micro task: `63xD1 proof / closeout`
- landed micro task: `64xA1 successor lane inventory lock`
- landed micro task: `64xA2 candidate lane ranking`
- landed micro task: `64xB1 successor lane decision`
- landed micro task: `64xD1 proof / closeout`
- landed micro task: `65xA1 stage1/selfhost owner inventory lock`
- landed micro task: `65xA2 mainline contract / proof lock`
- landed micro task: `65xB1 runner authority owner cleanup`
- landed micro task: `65xB2 shell contract owner cleanup`
- landed micro task: `65xC1 mainline proof bundle refresh`
- landed micro task: `65xD1 proof / closeout`
- active micro task: `68xB1 facade/entry split`
- ranked successor corridor after `68x`:
  - `69x rust runner product/keep/reference recut`
  - `70x caller-zero archive sweep`
- fixed corridor after `60x`:
  - `61x residual rust-vm caller-zero audit rerun`
  - `62x rust-vm delete-ready removal wave`
  - `63x rust-vm final retirement decision`
- post-`44xE1`: `phase-44x proof / closeout` (landed)
- raw backend default flip stays deferred; vm residual cleanup stays below direct/core mainline

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-68x/README.md`
4. `docs/development/current/main/phases/phase-68x/68x-90-hako-runner-authority-compat-facade-recut-ssot.md`
5. `docs/development/current/main/phases/phase-68x/68x-91-task-board.md`
6. `cargo check --manifest-path Cargo.toml --bin hakorune`

## Optional Checks

- `phase-29x` や code lane を触る日だけ追加:

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
bash tools/selfhost/run_lane_a_daily.sh
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```

## 保守レーン（必要時のみ）

```bash
cargo check --release --bin hakorune
cargo build --release --bin hakorune
(cd crates/nyash_kernel && cargo build --release)
```
