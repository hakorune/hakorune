---
Status: Active
Date: 2026-03-02
Scope: 再起動直後に 2〜5 分で開発再開するための最短手順。
Related:
  - CURRENT_TASK.md
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
tools/checks/dev_gate.sh runtime-exec-zero
bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh
HAKO_EMIT_MIR_MAINLINE_ONLY=1 NYASH_LLVM_SKIP_BUILD=1 \
tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --reuse-if-fresh 1
```

- This rebuilds the Stage1 bootstrap `launcher-exe` artifact; Stage2 distribution packaging is future work.
- `build_stage1` が artifact 欠落で失敗した場合は、下の「保守レーン」を先に 1 回実行する。

## 今日の再開点（kernel-mainline lane）

- Active next: `.hako` kernel mainline 最適化（no-fallback 固定）
- 最初の計測（strict / route drift 検知）:

```bash
bash tools/perf/run_kilo_hk_bench.sh strict 1 3
```

- 診断用（速度だけ見る暫定。結果一致ガードを外す）:

```bash
bash tools/perf/run_kilo_hk_bench.sh diagnostic 1 3
```

## 保守レーン（必要時のみ）

```bash
cargo check --release --bin hakorune
cargo build --release --bin hakorune
(cd crates/nyash_kernel && cargo build --release)
```

## 参照順（迷ったら）

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
