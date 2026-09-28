# Resolve and run a python command under the first interpreter that can
# import llvmlite. The src/llvm_py surface is an opt-in compat/oracle keep
# lane (LLVMLITE-ORACLE-COVERAGE-D0 parked; G2/AUTO0 targets zero daily
# llvmlite dependency), so callers must treat exit 127 as a typed keep-lane
# skip instead of a hard failure.
#
# Resolution order: ambient python3, the checkout .venv, then a uv ephemeral
# env pinned to the LLVM 18 profile (llvmlite 0.47.x).
#
# Usage:
#   if PYTHONPATH=src/llvm_py:src llvmlite_python -m unittest some.tests; then
#     ...
#   elif [[ $? -eq 127 ]]; then
#     echo "[tag] SKIP: no interpreter provides llvmlite; keep-lane pack deferred"
#   else
#     exit 1
#   fi

LLVMLITE_PYTHON_VIA=""

_llvmlite_lib_root() {
  cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd
}

llvmlite_python() {
  if python3 -c 'import llvmlite' >/dev/null 2>&1; then
    LLVMLITE_PYTHON_VIA="python3"
    python3 "$@"
    return $?
  fi
  local repo_root
  repo_root="$(_llvmlite_lib_root)"
  if [[ -x "$repo_root/.venv/bin/python" ]] \
    && "$repo_root/.venv/bin/python" -c 'import llvmlite' >/dev/null 2>&1; then
    LLVMLITE_PYTHON_VIA=".venv"
    "$repo_root/.venv/bin/python" "$@"
    return $?
  fi
  if command -v uv >/dev/null 2>&1; then
    LLVMLITE_PYTHON_VIA="uv:llvmlite==0.47.0"
    uv run --with 'llvmlite==0.47.0' --no-project python "$@"
    return $?
  fi
  return 127
}
