#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="llvm-py-keep-lane-probe"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
source "$ROOT_DIR/tools/checks/lib/llvmlite_python.sh"

# src/llvm_py is an opt-in compat/oracle keep lane (parked at
# LLVMLITE-ORACLE-COVERAGE-D0; G2/AUTO0 targets zero daily llvmlite
# dependency). The keep coverage must still run wherever an interpreter
# provides llvmlite, but a daily gate must not hard-fail on hosts where
# none does.

TEST_TARGET="src/llvm_py/tests/test_strlen_fast.py"

cd "$ROOT_DIR"
rc=0
PYTHONPATH="src/llvm_py:." llvmlite_python -m unittest "$TEST_TARGET" || rc=$?
case "$rc" in
  0)
    echo "[$TAG] ok interpreter=$LLVMLITE_PYTHON_VIA"
    ;;
  127)
    echo "[$TAG] SKIP: no interpreter provides llvmlite (python3, .venv, uv); keep lane is opt-in"
    ;;
  *)
    exit "$rc"
    ;;
esac
