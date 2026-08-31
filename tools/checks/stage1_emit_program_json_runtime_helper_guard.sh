#!/usr/bin/env bash
# Guard: public BuildBox.emit_program_json_v0(source, null) lowers to the phase-1 compatibility runtime helper.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NY_LLVM_C="$ROOT_DIR/target/release/ny-llvmc"
FIXTURE="$ROOT_DIR/apps/tests/mir_shape_guard/lowering_plan_stage1_emit_program_json_runtime_helper_same_module_min_v1.mir.json"
SELECTED_LAUNCH_SOURCE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_selected_launch_emit.inc"
PLANNED_DEFINITION_SOURCE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_function_definition_emit.inc"
DEFINITION_SEAM_SOURCE="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_same_module_function_emit.inc"
TAG="stage1-emit-program-json-runtime-helper-guard"

if [ ! -f "$FIXTURE" ]; then
  echo "[$TAG] fixture missing: $FIXTURE" >&2
  exit 1
fi

if [ ! -f "$SELECTED_LAUNCH_SOURCE" ]; then
  echo "[$TAG] selected launch owner missing: $SELECTED_LAUNCH_SOURCE" >&2
  exit 1
fi
if [ ! -f "$PLANNED_DEFINITION_SOURCE" ]; then
  echo "[$TAG] planned-definition owner missing: $PLANNED_DEFINITION_SOURCE" >&2
  exit 1
fi
if [ ! -f "$DEFINITION_SEAM_SOURCE" ]; then
  echo "[$TAG] planned-definition seam missing: $DEFINITION_SEAM_SOURCE" >&2
  exit 1
fi
FFI_BUILD="$ROOT_DIR/tools/build_hako_llvmc_ffi.sh"
if [ ! -f "$FFI_BUILD" ]; then
  echo "[$TAG] fresh FFI build owner missing: $FFI_BUILD" >&2
  exit 1
fi

python3 - "$SELECTED_LAUNCH_SOURCE" "$PLANNED_DEFINITION_SOURCE" "$DEFINITION_SEAM_SOURCE" <<'PY'
from pathlib import Path
import sys

def check_lifecycle(path, variable, failure, activate, reject, prefix):
    source = Path(path).read_text(encoding="utf-8")
    if f"struct SameModuleFunctionContextSnapshot {variable};" in source:
        raise SystemExit(f"{prefix} snapshot must not be stack-owned")
    required = (
        f"{variable} = calloc(1, sizeof(*{variable}));",
        failure,
        f"same_module_function_save_function_context({variable});",
        activate,
        "same_module_function_emit_function_pipeline(\n",
        f"same_module_function_restore_function_context({variable});",
        f"free({variable});",
    )
    missing = [token for token in required if token not in source]
    if missing:
        raise SystemExit(f"{prefix} snapshot lifecycle drifted: " + ", ".join(missing))
    positions = [source.index(token) for token in required[2:]]
    if positions != sorted(positions):
        raise SystemExit(f"{prefix} snapshot save/activate/emit/restore/free order drifted")
    allocation = source.index(required[0])
    allocation_failure = source.index(required[1])
    save = source.index(required[2])
    if not allocation < allocation_failure < save:
        raise SystemExit(f"{prefix} allocation failure must precede context save")
    if reject not in source[allocation_failure:save]:
        raise SystemExit(f"{prefix} allocation failure must reject before activation")

seam = Path(sys.argv[3]).read_text(encoding="utf-8")
include_token = '#include "hako_llvmc_ffi_same_module_function_definition_emit.inc"'
if include_token not in seam:
    raise SystemExit("planned-definition helpers must be included at the definition seam")
if "auto int same_module_function_definition_is_eligible(" in seam:
    raise SystemExit("planned-definition helpers must not remain inline in the seam")

check_lifecycle(
    sys.argv[1],
    "launch_snapshot",
    "launch_context_snapshot_allocation_failed",
    "same_module_function_activate_function_context(&launch_function);",
    "goto GEN_ABORT;",
    "selected launch",
)
check_lifecycle(
    sys.argv[2],
    "saved_context",
    "context_snapshot_allocation_failed",
    "same_module_function_activate_function_context(&local_function);",
    "return -1;",
    "planned-definition",
)
PY

build_log="$(mktemp /tmp/stage1_emit_program_json_runtime_helper_build.XXXXXX.log)"
log="$(mktemp /tmp/stage1_emit_program_json_runtime_helper.XXXXXX.log)"
obj="$(mktemp /tmp/stage1_emit_program_json_runtime_helper.XXXXXX.o)"
trap 'rm -f "$build_log" "$log" "$obj"' EXIT

set +e
(cd "$ROOT_DIR" && bash "$FFI_BUILD") >"$build_log" 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  echo "[$TAG] failed to build fresh selected-C FFI (rc=$rc)" >&2
  tail -n 120 "$build_log" >&2 || true
  exit "$rc"
fi

set +e
(cd "$ROOT_DIR" && cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc) >>"$build_log" 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  echo "[$TAG] failed to build current ny-llvmc (rc=$rc)" >&2
  tail -n 120 "$build_log" >&2 || true
  exit "$rc"
fi

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[$TAG] ny-llvmc missing after build: $NY_LLVM_C" >&2
  tail -n 120 "$build_log" >&2 || true
  exit 2
fi

set +e
NYASH_LLVM_ROUTE_TRACE=1 \
  "$NY_LLVM_C" --in "$FIXTURE" --emit obj --out "$obj" >"$log" 2>&1
rc=$?
set -e
if [ "$rc" -ne 0 ]; then
  echo "[$TAG] ny-llvmc failed to compile runtime-helper fixture (rc=$rc)" >&2
  tail -n 120 "$log" >&2 || true
  exit "$rc"
fi

if ! grep -Fq "consumer=mir_call_stage1_emit_program_json_emit" "$log"; then
  echo "[$TAG] runtime-helper route not consumed" >&2
  tail -n 80 "$log" >&2 || true
  exit 1
fi

if [ ! -s "$obj" ]; then
  echo "[$TAG] object output missing" >&2
  tail -n 80 "$log" >&2 || true
  exit 1
fi

echo "[$TAG] ok"
