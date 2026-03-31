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

- `phase-29bq` や code lane を触る日は、必要に応じて次も追加で回す:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
bash tools/selfhost/run_lane_a_daily.sh
./tools/selfhost/run.sh --gate --planner-required 1 --max-cases 5 --jobs 4
```

## 今日の再開点（active lane）

- Active next: `policy-refresh`
- active order: `stage / docs / naming` -> `K1 done-enough stop-line` -> `K2-core acceptance lock` -> `K2-wide deferred` -> `zero-rust default`
- `K-axis` is read as `K0 / K1 / K2` build/runtime stages
- `K2-core` / `K2-wide` are task packs inside `K2`
- exact next:
  1. `CURRENT_TASK.md`
  2. `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  3. `docs/development/current/main/design/rune-v1-metadata-unification-ssot.md`
  4. `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
  5. `docs/development/current/main/15-Workstream-Map.md`
  6. `docs/development/current/main/10-Now.md`
- immediate action:
  - verify `stage` axis / replacement axis / naming split in `CURRENT_TASK.md` first
  - fix `stage / docs / naming` order before reopening deeper implementation lanes
  - read the `K-axis` stage progression as `K0 -> K1 -> K2`; treat `K2-core` as the first task pack inside `K2`
  - treat `Rune` as landed keep (`@rune` canonical surface, legacy aliases compat keep), not as the current blocker lane
  - keep `K2-core acceptance lock` as the next structural step and read its smoke/evidence gate from the existing `nyash_kernel` RawArray contract tests
  - keep `K1 done-enough` fixed before promoting `K2-core`
  - keep `RawMap` deferred in `K2-wide`; treat map perf as regression/evidence until a new exact blocker says otherwise

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
5. `docs/development/current/main/phases/phase-29bq/README.md`
