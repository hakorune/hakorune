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
- exact next:
  1. `CURRENT_TASK.md`
  2. `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  3. `docs/development/current/main/design/rune-v1-metadata-unification-ssot.md`
  4. `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
  5. `docs/development/current/main/15-Workstream-Map.md`
  6. `docs/development/current/main/10-Now.md`
- immediate action:
  - verify `stage` axis and compressed `K-axis` reading in `CURRENT_TASK.md` first
  - verify `@rune` canonical surface and declaration-local `attrs.runes` carrier next
  - keep `K2-core = RawArray first truthful substrate pilot` as the next structural target
  - keep execution order as Rune primitive control plane first, then `K2-core` RawArray pilot, then Map parked
  - treat `Map` perf as regression/evidence until a new exact blocker says otherwise

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
