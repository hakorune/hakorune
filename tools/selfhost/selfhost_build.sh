#!/usr/bin/env bash
# selfhost_build.sh — Hybrid selfhost build (Stage‑B → Program(JSON v0) → optional run via Core‑Direct)
#
# Goals (80/20):
# - Take a Hako source (.hako), compile with Hakorune Stage‑B to Program(JSON v0).
# - Optionally run via Gate‑C/Core Direct (in‑proc) to verify exit code.
# - (Future) Optionally convert to MIR and build an executable via ny-llvmc.
#
# Usage:
#   tools/selfhost/selfhost_build.sh --in source.hako [--json out.json] [--run]
#   Options:
#     --in FILE     Input .hako source file (required)
#     --json FILE   Output Program(JSON v0) to FILE
#     --run         Run via Core-Direct after compilation
#     --mir FILE    Also emit MIR(JSON) to FILE
#     --exe FILE    Build native EXE via ny-llvmc
#     --keep-tmp    Keep temporary files
#     --core        Deprecated (JoinIR Core は常時 ON のため無視・警告のみ)
#     --strict      Phase 81: Enable Strict mode (fail-fast, no fallback)
#   Env:
#     NYASH_BIN: path to hakorune/nyash binary (auto-detected if omitted)
#     NYASH_ROOT: repo root (auto-detected)
#     NYASH_JOINIR_CORE: Deprecated (常時 ON のため無視・警告のみ)
#     NYASH_JOINIR_STRICT: Set to 1 for Strict mode
#
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
BIN="${NYASH_BIN:-}"
if [ -z "${BIN}" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then BIN="$ROOT/target/release/hakorune";
  elif [ -x "$ROOT/target/release/nyash" ]; then BIN="$ROOT/target/release/nyash";
  else echo "[selfhost] error: NYASH_BIN not set and no binary found under target/release" >&2; exit 2; fi
fi
SMOKE_ENV_SKIP_EXPORTS=1
if [ -f "$ROOT/tools/smokes/v2/lib/env.sh" ]; then
  source "$ROOT/tools/smokes/v2/lib/env.sh"
fi
RAW_KEEP="${NYASH_SELFHOST_KEEP_RAW:-0}"
RAW_DIR="${NYASH_SELFHOST_RAW_DIR:-$ROOT/logs/selfhost}"
if [ "$RAW_KEEP" = "1" ]; then
  mkdir -p "$RAW_DIR" 2>/dev/null || RAW_KEEP=0
fi
timestamp_now() { date +%Y%m%d_%H%M%S; }

IN=""
JSON_OUT=""
MIR_OUT=""
EXE_OUT=""
DO_RUN=0
KEEP_TMP=0

apply_selfhost_env() {
  export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
  export NYASH_PARSER_ALLOW_SEMICOLON=1
  export NYASH_ALLOW_USING_FILE=0
  export HAKO_ALLOW_USING_FILE=0
  export NYASH_USING_AST=1
  export NYASH_VARMAP_GUARD_STRICT=0
  export NYASH_BLOCK_SCHEDULE_VERIFY=0
  export NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0
  # Ensure core plugins (Console/Array/Map/String/Integer) are discoverable
  export NYASH_PLUGIN_PATH="${NYASH_PLUGIN_PATH:-$ROOT/target/release}"
  export NYASH_PLUGIN_PATHS="${NYASH_PLUGIN_PATHS:-$NYASH_PLUGIN_PATH}"
  # Phase 80/81: JoinIR Core/Strict mode propagation
  # NYASH_JOINIR_CORE は常時 ON のため no-op（警告のみ）。STRICT は互換のため通す。
  if [ -n "${NYASH_JOINIR_STRICT:-}" ]; then
    export NYASH_JOINIR_STRICT
  fi
}

while [ $# -gt 0 ]; do
  case "$1" in
    --in) IN="$2"; shift 2;;
    --json) JSON_OUT="$2"; shift 2;;
    --run) DO_RUN=1; shift;;
    --mir) MIR_OUT="$2"; shift 2;;
    --keep-tmp) KEEP_TMP=1; shift;;
    --exe) EXE_OUT="$2"; shift 2;;
    --strict) export NYASH_JOINIR_STRICT=1; shift;; # Phase 81: Fail-Fast mode
    --core) echo "[selfhost] --core is deprecated (JoinIR is always on); ignoring" >&2; shift;;
    *) echo "[selfhost] unknown arg: $1" >&2; exit 2;;
  esac
done

if [ -z "$IN" ]; then echo "[selfhost] --in <file.hako> is required" >&2; exit 2; fi
if [ ! -f "$IN" ]; then echo "[selfhost] input not found: $IN" >&2; exit 2; fi

tmp_json="${JSON_OUT:-/tmp/hako_stageb_$$.json}"

# Emit Program(JSON v0; prefer BuildBox for emit-only when HAKO_USE_BUILDBOX=1)
RAW="/tmp/hako_stageb_raw_$$.txt"
stageb_rc=0
SRC_CONTENT="$(cat "$IN")"
stageb_cmd_desc=""
if [ "${HAKO_USE_BUILDBOX:-0}" = "1" ] && [ "$DO_RUN" = "0" ] && [ -z "$EXE_OUT" ]; then
  apply_selfhost_env
  stageb_cmd_desc="BuildBox.emit_program_json_v0 via compiler build_box"
  if [ -z "${HAKO_STAGEB_MODULE_ROOTS_LIST:-}" ]; then
    roots_list="$(collect_stageb_module_roots_list "$ROOT" || true)"
    if [ -n "${roots_list:-}" ]; then
      export HAKO_STAGEB_MODULE_ROOTS_LIST="$roots_list"
    fi
  fi
  WRAP="/tmp/hako_buildbox_wrap_$$.hako"
  cat > "$WRAP" <<'HAKO'
using lang.compiler.build.build_box as BuildBox
static box Main { method main(args) {
  local src = env.get("HAKO_SRC");
  local j = BuildBox.emit_program_json_v0(src, null);
  print(j);
  return 0;
} }
HAKO
  (
    export HAKO_SRC="$SRC_CONTENT"
    cd "$ROOT" && "$BIN" --backend vm "$WRAP"
  ) > "$RAW" 2>&1 || stageb_rc=$?
  rm -f "$WRAP" 2>/dev/null || true
else
  apply_selfhost_env
  stageb_cmd_desc="compiler.hako --stage-b --stage3"
  (
    export HAKO_SRC="$SRC_CONTENT"
    cd "$ROOT" && \
      "$BIN" --backend vm \
        "$ROOT/lang/src/compiler/entry/compiler.hako" -- \
        --stage-b --stage3
  ) > "$RAW" 2>&1 || stageb_rc=$?
fi

extract_ok=0
if awk '(/"version":0/ && /"kind":"Program"/){print;found=1;exit} END{exit(found?0:1)}' "$RAW" > "$tmp_json"; then
  extract_ok=1
fi

if [ "$RAW_KEEP" = "1" ]; then
  ts="$(timestamp_now)"
  raw_log="$RAW_DIR/stageb_${ts}_$$.log"
  {
    echo "[selfhost/raw] cmd: ${stageb_cmd_desc:-unknown}"
    echo "[selfhost/raw] rc_stageb=${stageb_rc} extract_ok=${extract_ok}"
    echo "[selfhost/raw] src=${IN}"
    echo "[selfhost/raw] --- stdout+stderr ---"
    cat "$RAW"
  } > "$raw_log" 2>/dev/null || true
  if [ "$extract_ok" = "1" ] && [ -s "$tmp_json" ]; then
    cp "$tmp_json" "$RAW_DIR/stageb_${ts}_$$.json" 2>/dev/null || true
  fi
fi

if [ "$extract_ok" != "1" ]; then
  echo "[selfhost] Stage‑B emit failed" >&2
  tail -n 120 "$RAW" >&2 || true
  if [ "$RAW_KEEP" = "1" ] && [ -n "${raw_log:-}" ]; then
    echo "[selfhost/debug] RAW log: $raw_log" >&2
  fi
  if [ "$KEEP_TMP" != "1" ]; then rm -f "$RAW" 2>/dev/null || true; fi
  exit 1
fi
rm -f "$RAW" 2>/dev/null || true

if [ -n "$JSON_OUT" ]; then
  echo "[selfhost] JSON v0 written: $tmp_json" >&2
fi

# Optional: emit MIR(JSON) from source (runner compiles .hako directly; Stage‑B JSON is for reference)
if [ -n "$MIR_OUT" ]; then
  echo "[selfhost] emitting MIR JSON → $MIR_OUT" >&2
  "$BIN" --backend mir --emit-mir-json "$MIR_OUT" "$IN" >/dev/null
fi

# Optional: build native EXE via ny-llvmc harness (fallback path; parses original source)
if [ -n "$EXE_OUT" ]; then
  # Requirements: ny-llvmc present and harness envs
  NYLL="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}"
  if [ ! -x "$NYLL" ] && [ ! -f "$NYLL" ]; then
    echo "[selfhost] ny-llvmc not found: $NYLL (Set NYASH_NY_LLVM_COMPILER or build ny-llvmc)" >&2
    exit 2
  fi
  NYRT_DIR="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}"
  export NYASH_LLVM_USE_HARNESS=1
  export NYASH_NY_LLVM_COMPILER="$NYLL"
  export NYASH_EMIT_EXE_NYRT="$NYRT_DIR"

  # Convert Program(JSON v0) → MIR(JSON) via runner
  MIR_TMP="${MIR_OUT:-/tmp/hako_stageb_mir_$$.json}"
  echo "[selfhost] converting Program(JSON v0) → MIR(JSON) → EXE" >&2
  "$BIN" --json-file "$tmp_json" --program-json-to-mir "$MIR_TMP"

  # Build EXE via ny-llvmc
  "$NYLL" --in "$MIR_TMP" --emit exe --nyrt "$NYRT_DIR" --out "$EXE_OUT"

  # Cleanup
  if [ "$KEEP_TMP" != "1" ]; then
    if [ -z "$JSON_OUT" ]; then rm -f "$tmp_json" 2>/dev/null || true; fi
    if [ -z "$MIR_OUT" ]; then rm -f "$MIR_TMP" 2>/dev/null || true; fi
  fi
  exit 0
fi

if [ "$DO_RUN" = "1" ]; then
  # Run via Core‑Direct (in‑proc), quiet
  set +e
  NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 HAKO_CORE_DIRECT=1 HAKO_CORE_DIRECT_INPROC=1 \
    NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$BIN" --json-file "$tmp_json" >/dev/null 2>&1
  rc=$?
  set -e
  if [ "$KEEP_TMP" != "1" ] && [ -z "$JSON_OUT" ]; then rm -f "$tmp_json" 2>/dev/null || true; fi
  exit $rc
else
  # Emit-only
  echo "$tmp_json"
fi
