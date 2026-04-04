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

- lane: `phase-51x compat-codegen archival sweep`
- landed micro task: `49xD1 proof / closeout`
- active micro task: `51xD1 proof / closeout`
- next micro task: `next source lane selection`
- after next: `none`
- post-`44xE1`: `phase-44x proof / closeout` (landed)
- raw backend default flip stays deferred; vm residual cleanup stays below direct/core mainline

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-51x/README.md`
4. `docs/development/current/main/phases/phase-51x/51x-90-compat-codegen-archival-sweep-ssot.md`
5. `docs/development/current/main/phases/phase-51x/51x-91-task-board.md`
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
