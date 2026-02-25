---
Status: Active
Date: 2026-02-23
Scope: 再起動直後に 2〜5 分で開発再開するための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
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
PERF_GATE_BENCH_COMPARE_ENV_CHECK=1 \
PERF_GATE_AOT_SKIP_BUILD_CHECK=1 \
PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK=1 \
PERF_GATE_KILO_PARITY_LOCK_CHECK=1 \
bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh
```

## 今日の再開点（perf lane）

- Active next: `LLVM-HOT-20`（kilo/text workload の structural hotspot 仕分け）
- 最初の計測（基準線）:

```bash
bash tools/perf/bench_compare_c_py_vs_hako.sh kilo_kernel_small 1 3
```

## 参照順（迷ったら）

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
