---
Status: Active
Date: 2026-03-24
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
- まず緑確認をしてから、当日の active next に戻る。

## 2〜5分 再開手順（そのまま実行）

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh
```

- `phase-29cj` を触る日は、必要に応じて次も追加で回す:

```bash
bash tools/selfhost_identity_check.sh --mode smoke
```

## 今日の再開点（mainline lane）

- Active next: `phase-29cj` formal close sync
- exact next:
  1. `CURRENT_TASK.md`
  2. `docs/development/current/main/15-Workstream-Map.md`
  3. `docs/development/current/main/phases/phase-29cj/README.md`
- immediate action:
  - near-thin-floor reinventory across `MirBuilderBox.hako`, `stage1_cli_env.hako`, `stage1_cli.hako`, and `launcher.hako`

## 保守レーン（必要時のみ）

```bash
cargo check --release --bin hakorune
cargo build --release --bin hakorune
(cd crates/nyash_kernel && cargo build --release)
```

## 参照順（迷ったら）

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/phases/phase-29cj/README.md`
