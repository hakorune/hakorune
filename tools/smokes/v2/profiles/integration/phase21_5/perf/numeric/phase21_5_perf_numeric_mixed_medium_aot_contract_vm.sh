#!/bin/bash
# phase21_5_perf_numeric_mixed_medium_aot_contract_vm.sh
#
# Contract pin: AOT 経路の健全性確認
# - numeric_mixed_medium を AOT で実行し、status=ok を検証
# - Phase 21.5 ladder の AOT sentinel として使用

set -euo pipefail

SMOKE_NAME="phase21_5_perf_numeric_mixed_medium_aot_contract_vm"

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
require_env || exit 2

# AOT sentinel 実行
# PERF_SKIP_VM_PREFLIGHT: AOT 契約固定のため VM preflight は明示スキップ
# PERF_VM_TIMEOUT: 重い VM ケースでも系列測定が切れないよう余裕を持たせる
# HAKO_VM_MAX_STEPS: 800k iterations 用の step budget（VM 系列測定用）
OUT=$(PERF_AOT=1 PERF_SKIP_VM_PREFLIGHT=1 PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
       NYASH_LLVM_BACKEND=crate NYASH_LLVM_USE_HARNESS=0 \
       NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
       HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-100000000}" \
  bash "$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh" numeric_mixed_medium 1 1 2>&1) || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: bench command failed"
  exit 1
}

# 検証条件1: VM ベンチ行がある
if ! printf '%s\n' "$OUT" | grep -q '\[bench\] name=numeric_mixed_medium '; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing [bench] name=numeric_mixed_medium line"
  exit 1
fi

# 検証条件2: AOT 行がある
if ! printf '%s\n' "$OUT" | grep -q '\[bench\] name=numeric_mixed_medium (aot)'; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: missing [bench] name=numeric_mixed_medium (aot) line"
  exit 1
fi

# 検証条件3: AOT 行で status=ok（1行に限定してノイズ回避）
if ! printf '%s\n' "$OUT" | grep -qE '\[bench\] name=numeric_mixed_medium \(aot\).*status=ok'; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: AOT status is not ok"
  exit 1
fi

test_pass "$SMOKE_NAME"
