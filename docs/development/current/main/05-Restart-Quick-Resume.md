---
Status: Active
Date: 2026-03-31
Scope: 再起動直後に 2〜5 分で開発再開するための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/build-lane-separation-ssot.md
  - docs/tools/README.md
---

# Restart Quick Resume

## 目的

- 再起動後に「どこから再開するか」を迷わないための単一入口。
- まず current gate / blocker を確認してから、当日の active next に戻る。

## 2〜5分 再開手順（そのまま実行）

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
```

- `phase-29x` や code lane を触る日は、必要に応じて次も追加で回す:

```bash
bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
bash tools/selfhost/run_lane_a_daily.sh
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```

## 今日の再開点（active lane）

- Restart handoff: landed `K2-wide` / `zero-rust` rows stay accepted, `stage2plus` acceptance bundle is complete, and `phase-29x` is the active front.
- backend lane detail is canonical in the backend-lane docs:
  - `llvmlite` = compat/probe keep lane
  - `ny-llvm` / `ny-llvmc` = daily mainline AOT lane
  - `native` = explicit replay/canary lane

- Active next: `phase-29x backend owner cutover prep`
- Current blocker: `none`
- Exact focus: `29x-98 LLVMEmitBox keep/archive decision (proof/example callers remain archive-later)`
- boundary audit result: `RuntimeDataBox` remains facade-only; delete stays on `MapBox` / `RawMap` only
- active order: `stage / docs / naming` -> `K1 done-enough stop-line` -> `K2-core accepted stop-line` -> `K2-wide boundary-shrink lock-down (closed)` -> `zero-rust default operationalization (landed)` -> `stage2plus entry / first optimization wave (accepted)` -> `phase-29x backend owner cutover prep`
- `K-axis` is read as `K0 / K1 / K2` build/runtime stages
- `K2-core` / `K2-wide` are task packs inside `K2`
- exact next:
  1. `CURRENT_TASK.md`
  2. `docs/development/current/main/15-Workstream-Map.md`
  3. `docs/development/current/main/phases/phase-29x/README.md`
  4. `docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md`
  5. `docs/development/current/main/phases/phase-29x/29x-91-task-board.md`
  6. `docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md`
- immediate action:
  - verify `stage` axis / replacement axis / naming split in `CURRENT_TASK.md`
  - keep `phase-29x backend owner cutover prep` as the current front
  - keep compat callers explicit and record archive conditions for proof/example callers before touching `emit_object_from_mir_json(...)`
  - keep portability split explicit: `.hako` capability facade, native keep leaf glue
  - keep migration task notes in root/docs/phase owners; `target/**`, `artifacts/**`, `dist/**` stay binaries/bundles only

## 保守レーン（必要時のみ）

```bash
cargo check --release --bin hakorune
cargo build --release --bin hakorune
(cd crates/nyash_kernel && cargo build --release)
```

## 参照順（迷ったら）

1. `CURRENT_TASK.md`
2. `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
3. `docs/development/current/main/15-Workstream-Map.md`
4. `docs/development/current/main/10-Now.md`
5. `docs/development/current/main/phases/phase-29x/README.md`
