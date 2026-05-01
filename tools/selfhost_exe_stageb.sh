#!/usr/bin/env bash
# selfhost_exe_stageb.sh — route-selectable emit → ny-llvmc (crate) → EXE
# Purpose: Build a native EXE from a Nyash .hako source for explicit
# bootstrap/compat work. This is not the selfhost_build.sh mainline facade.
# Usage: tools/selfhost_exe_stageb.sh <input.hako> [-o <out>] [--run]
#
# Emit route (env):
#   HAKORUNE_STAGE1_EMIT_ROUTE=direct  (default MIR-first route)
#     Direct --emit-mir-json route.
#   HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate
#     Explicit bridge compat capsule: Stage-B compiler emits Program(JSON v0),
#     then env.mirbuilder.emit bridge.
#
# Prerequisites (one-time setup):
#   cargo build --release -p nyash-llvm-compiler
#   (cd crates/nyash_kernel && cargo build --release)
#   cargo build --release
set -euo pipefail

OUT="a.out"
DO_RUN=0
if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <input.hako> [-o <out>] [--run]" >&2
  exit 2
fi

INPUT=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -o) OUT="$2"; shift 2 ;;
    --run) DO_RUN=1; shift ;;
    *) INPUT="$1"; shift ;;
  esac
done
if [[ ! -f "$INPUT" ]]; then
  echo "error: input not found: $INPUT" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
EMIT_ROUTE="${HAKORUNE_STAGE1_EMIT_ROUTE:-direct}"

# shellcheck source=/dev/null
source "$ROOT_DIR/tools/selfhost/lib/program_json_mir_bridge.sh"
# shellcheck source=/dev/null
source "$ROOT_DIR/tools/selfhost/lib/stageb_program_json_capture.sh"

TMP_JSON="$(mktemp --suffix .json)"
TMP_FILES=("$TMP_JSON")
cleanup_tmp() { rm -f "${TMP_FILES[@]}" 2>/dev/null || true; }
trap cleanup_tmp EXIT

resolve_nyash_bin() {
  if [[ -z "${NYASH_BIN:-}" ]]; then
    if [[ -x "$ROOT_DIR/target/release/hakorune" ]]; then
      NYASH_BIN="$ROOT_DIR/target/release/hakorune"
    else
      NYASH_BIN="$ROOT_DIR/target/release/nyash"
    fi
  fi
  if [[ ! -x "$NYASH_BIN" ]]; then
    echo "[emit] error: nyash/hakorune binary not found: $NYASH_BIN" >&2
    echo "       hint: run cargo build --release --bin hakorune" >&2
    exit 2
  fi
}

merge_source_imports_into_program_json() {
  local src_file="$1"
  local in_program_json="$2"
  local out_program_json="$3"
  python3 - "$src_file" "$in_program_json" "$out_program_json" <<'PY'
import json
import pathlib
import re
import sys

src_path = pathlib.Path(sys.argv[1])
in_prog_path = pathlib.Path(sys.argv[2])
out_prog_path = pathlib.Path(sys.argv[3])

try:
    source = src_path.read_text(encoding="utf-8")
except Exception:
    source = ""

program = json.loads(in_prog_path.read_text(encoding="utf-8"))
if not isinstance(program, dict):
    raise SystemExit("program json must be an object")

imports = program.get("imports")
if not isinstance(imports, dict):
    imports = {}

for raw in source.splitlines():
    line = raw.split("//", 1)[0].strip()
    if not line.startswith("using "):
        continue
    if line.endswith(";"):
        line = line[:-1].strip()
    body = line[len("using "):].strip()
    if not body:
        continue

    alias = None
    target = body
    if " as " in body:
        target, alias = body.rsplit(" as ", 1)
        target = target.strip()
        alias = alias.strip()

    if len(target) >= 2 and target[0] == target[-1] and target[0] in ("'", '"'):
        target = target[1:-1]

    if not alias:
        parts = [p for p in re.split(r"[./]", target) if p]
        if parts:
            alias = parts[-1]

    if alias and target and alias not in imports:
        imports[alias] = target

program["imports"] = imports
out_prog_path.write_text(json.dumps(program, separators=(",", ":")), encoding="utf-8")
PY
}

emit_mir_direct() {
  local tmp_log timeout_secs
  tmp_log="$(mktemp)"
  TMP_FILES+=("$tmp_log")
  timeout_secs="${HAKORUNE_STAGE1_DIRECT_TIMEOUT_SECS:-120}"

  set +e
  if [[ "$timeout_secs" =~ ^[0-9]+$ ]] && [[ "$timeout_secs" -gt 0 ]]; then
    timeout --preserve-status "${timeout_secs}s" env \
      HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
      HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
      HAKO_STAGEB_FUNC_SCAN="${HAKO_STAGEB_FUNC_SCAN:-}" \
      HAKO_MIR_BUILDER_FUNCS="${HAKO_MIR_BUILDER_FUNCS:-}" \
      HAKO_MIR_BUILDER_CALL_RESOLVE="${HAKO_MIR_BUILDER_CALL_RESOLVE:-}" \
      NYASH_JSON_SCHEMA_V1="${NYASH_JSON_SCHEMA_V1:-1}" \
      NYASH_MIR_UNIFIED_CALL="${NYASH_MIR_UNIFIED_CALL:-1}" \
      NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}" \
      HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}" \
      NYASH_MACRO_DISABLE="${NYASH_MACRO_DISABLE:-1}" \
      HAKO_MACRO_DISABLE="${HAKO_MACRO_DISABLE:-1}" \
      "$NYASH_BIN" --emit-mir-json "$TMP_JSON" "$INPUT" >"$tmp_log" 2>&1
  else
    HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
    HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
    HAKO_STAGEB_FUNC_SCAN="${HAKO_STAGEB_FUNC_SCAN:-}" \
    HAKO_MIR_BUILDER_FUNCS="${HAKO_MIR_BUILDER_FUNCS:-}" \
    HAKO_MIR_BUILDER_CALL_RESOLVE="${HAKO_MIR_BUILDER_CALL_RESOLVE:-}" \
    NYASH_JSON_SCHEMA_V1="${NYASH_JSON_SCHEMA_V1:-1}" \
    NYASH_MIR_UNIFIED_CALL="${NYASH_MIR_UNIFIED_CALL:-1}" \
    NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}" \
    HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}" \
    NYASH_MACRO_DISABLE="${NYASH_MACRO_DISABLE:-1}" \
    HAKO_MACRO_DISABLE="${HAKO_MACRO_DISABLE:-1}" \
      "$NYASH_BIN" --emit-mir-json "$TMP_JSON" "$INPUT" >"$tmp_log" 2>&1
  fi
  local emit_rc=$?
  set -e
  if [[ "$emit_rc" -eq 124 || "$emit_rc" -eq 137 || "$emit_rc" -eq 143 ]]; then
    echo "[emit] direct route timed out after ${timeout_secs}s: $INPUT" >&2
    echo "       hint: use HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate for the bridge compat capsule" >&2
    tail -n 80 "$tmp_log" >&2 || true
    return "$emit_rc"
  fi
  if [[ "$emit_rc" -ne 0 ]]; then
    echo "[emit] direct route failed (rc=$emit_rc): $INPUT" >&2
    if grep -Fq "[joinir/freeze]" "$tmp_log" || grep -Fq "[plan/freeze:contract]" "$tmp_log"; then
      echo "       hint: direct route currently compiles merged prelude and may hit JoinIR unsupported loops" >&2
      echo "       hint2: use HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate only as an explicit bridge compat capsule" >&2
    fi
    tail -n 80 "$tmp_log" >&2 || true
    return "$emit_rc"
  fi

  if [[ ! -s "$TMP_JSON" ]] || ! grep -q '"functions"' "$TMP_JSON"; then
    echo "[emit] direct route produced non-MIR payload: $TMP_JSON" >&2
    tail -n 80 "$tmp_log" >&2 || true
    return 1
  fi

  echo "[emit-route] direct MIR-first (--emit-mir-json)"
  return 0
}

emit_mir_stageb_delegate() {
  local tmp_log tmp_prog_raw tmp_prog_merged code prog_json_out
  tmp_log="$(mktemp)"
  tmp_prog_raw="$(mktemp --suffix .program.json)"
  tmp_prog_merged="$(mktemp --suffix .program.merged.json)"
  TMP_FILES+=("$tmp_log" "$tmp_prog_raw" "$tmp_prog_merged")
  code="$(cat "$INPUT")"

  set +e
  prog_json_out=$((cd "$ROOT_DIR" && \
                  NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
                  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
                  HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
                  HAKO_STAGEB_FUNC_SCAN="${HAKO_STAGEB_FUNC_SCAN:-}" \
                  NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
                  NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}" HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}" \
                  "$NYASH_BIN" --backend vm "$ROOT_DIR/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$code") 2>"$tmp_log" | stageb_program_json_extract_from_stdin)
  local stageb_rc=$?
  set -e
  if [[ "$stageb_rc" -ne 0 ]] || [[ -z "$prog_json_out" ]]; then
    echo "[emit] stageb route failed to produce Program(JSON v0) (rc=$stageb_rc): $INPUT" >&2
    tail -n 80 "$tmp_log" >&2 || true
    return 1
  fi

  printf '%s' "$prog_json_out" >"$tmp_prog_raw"
  merge_source_imports_into_program_json "$INPUT" "$tmp_prog_raw" "$tmp_prog_merged"

  : > "$tmp_log"
  set +e
  HAKO_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}" \
  HAKO_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}" \
  HAKO_MIR_BUILDER_FUNCS="${HAKO_MIR_BUILDER_FUNCS:-}" \
  HAKO_MIR_BUILDER_CALL_RESOLVE="${HAKO_MIR_BUILDER_CALL_RESOLVE:-}" \
  NYASH_JSON_SCHEMA_V1="${NYASH_JSON_SCHEMA_V1:-1}" \
  NYASH_MIR_UNIFIED_CALL="${NYASH_MIR_UNIFIED_CALL:-1}" \
    program_json_mir_bridge_emit "$NYASH_BIN" "$tmp_prog_merged" "$TMP_JSON" "[emit]" >"$tmp_log" 2>&1
  local to_mir_rc=$?
  set -e
  if [[ "$to_mir_rc" -ne 0 ]]; then
    echo "[emit] stageb route failed in env.mirbuilder.emit bridge (rc=$to_mir_rc): $INPUT" >&2
    tail -n 80 "$tmp_log" >&2 || true
    return "$to_mir_rc"
  fi

  if [[ ! -s "$TMP_JSON" ]] || ! grep -q '"functions"' "$TMP_JSON"; then
    echo "[emit] stageb route produced non-MIR payload: $TMP_JSON" >&2
    tail -n 80 "$tmp_log" >&2 || true
    return 1
  fi

  echo "[emit-route] stageb-delegate compat-capsule (--backend vm compiler_stageb.hako -> env.mirbuilder.emit)"
  return 0
}

resolve_nyash_bin

case "$EMIT_ROUTE" in
  stageb-delegate)
    emit_mir_stageb_delegate
    ;;
  direct)
    emit_mir_direct
    ;;
  *)
    echo "[emit] unknown HAKORUNE_STAGE1_EMIT_ROUTE: $EMIT_ROUTE" >&2
    echo "       allowed: stageb-delegate | direct" >&2
    exit 2
    ;;
esac

echo "[emit] MIR JSON: $TMP_JSON ($(wc -c < "$TMP_JSON") bytes)"

# 2) Build EXE via crate backend (ny-llvmc)
NYASH_LLVM_BACKEND=crate \
NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT_DIR/target/release/ny-llvmc}" \
NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT_DIR/target/release}" \
  bash "$ROOT_DIR/tools/ny_mir_builder.sh" --in "$TMP_JSON" --emit exe -o "$OUT" --quiet >/dev/null
echo "[link] EXE: $OUT"

if [[ "$DO_RUN" = "1" ]]; then
  set +e
  _silent="${NYASH_NYRT_SILENT_RESULT:-}"
  if [[ -n "$_silent" ]]; then
    "$OUT"
    rc=$?
  else
    NYASH_NYRT_SILENT_RESULT=1 "$OUT"
    rc=$?
  fi
  set -e
  echo "[run] exit=$rc"
fi

exit 0
