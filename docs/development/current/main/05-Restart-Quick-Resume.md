---
Status: Active
Date: 2026-04-05
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

## Current

- lane: `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`
- current front: `MiniJsonLoader.read_quoted_from 最小抽出 fixture を VM/LLVM EXE parity で固定する`
- blocker: `none`
- recent landed:
  - `phase-100 Pinned Read-Only Captures`
  - `phase-99 Trim/escape 実コード寄り強化（VM+LLVM EXE）`
  - `phase-98 Plugin loader fail-fast + LLVM parityの持続化`
  - `phase-97 LLVM EXE parity for MiniJsonLoader fixtures`
  - `phase-95 json_loader escape loop E2E lock`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/phases/phase-100/README.md`

## Successor Corridor

1. `phase-102 real-app read_quoted loop regression (VM + LLVM EXE)`

## Parked After Optimization

- `vm-hako` small reference interpreter recut

## Current Proof Bundle

```bash
cargo check --manifest-path Cargo.toml --bin hakorune
bash tools/selfhost/mainline/stage1_mainline_smoke.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase95_json_loader_escape_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase96_json_loader_next_non_ws_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_json_loader_escape_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase100_pinned_local_receiver_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase100_mutable_accumulator_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase100_string_accumulator_llvm_exe.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase99_escape_trailing_backslash_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase99_escape_trailing_backslash_llvm_exe.sh
git diff --check
```

## Optional Checks

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
bash tools/selfhost/run_lane_a_daily.sh
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```
