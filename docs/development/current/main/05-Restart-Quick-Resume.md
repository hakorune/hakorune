---
Status: Active
Date: 2026-04-02
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
- Exact focus: `99W1 lock watch-1 caller groups`
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
  7. `docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md`
- cleanup bands:

  | Band | State |
  | --- | --- |
  | Now | `99W1 lock watch-1 caller groups` |
  | Next | `99W2 lock watch-1 replacement contract gap` |
  | Later | `99X1` / `99X2` |

- W4 / W5 / W6 are landed. Path truth, semantic proof/archive homes, and the Rust compat-codegen chokepoint are fixed.
- `phase2044` llvmlite trio is monitor-only keep and its dedicated suite manifest is the final live keep bucket.
- `phase2120` pure keep and historical replay now live in separate semantic homes.
- the generic `llvm_codegen::emit_object_from_mir_json(...)` export is gone; the remaining helper is explicit at `legacy_mir_front_door::compile_object_from_legacy_mir_json(...)`.
- remaining explicit helper caller inventory is two surfaces: `compat_codegen_receiver.rs` and the archive-later surrogate under `module_string_dispatch/compat/`.
- adopted watch shape is:
  - first, one Rust-side no-helper `MIR(JSON text) -> object path` primitive closes `watch-1`
  - then, `watch-2` becomes `json_path -> read_to_string -> same primitive`
- active micro task is `99W1 lock watch-1 caller groups`.
- next queued micro task is `99W2 lock watch-1 replacement contract gap`.
- caller reduction order inside `watch-1` is `loader-cold extern -> hostbridge dispatch -> plugin-loader env.codegen`.
- both watches are currently `watch-only`, not demotable now.
- detailed W4/W5/W6 landed history stays in `29x-99`, not in this restart mirror.
- immediate action:
  - verify `stage` axis / replacement axis / naming split in `CURRENT_TASK.md`
  - keep `phase-29x backend owner cutover prep` as the current front
  - keep `phase2044` on the new bucket runners; do not treat the whole directory as one llvmlite lane
  - keep `LLVMEmitBox` fixed as compat/proof keep, treat `CodegenBridgeBox` as archive-later producer only, and continue caller-by-caller sequencing with the remaining selfhost wrapper and llvmlite proof surfaces after the `phase251` quarantine
  - use `29x-99` for macro wave / micro-task planning before any file move
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
