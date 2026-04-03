#!/usr/bin/env bash
# selfhost_build.sh — Hybrid selfhost build (Stage‑B → Program(JSON v0) → optional MIR/run/exe)
#
# Goals (80/20):
# - Take a Hako source (.hako), compile with Hakorune Stage‑B to Program(JSON v0).
# - Optionally run via Gate‑C/Core Direct (in‑proc) to verify exit code.
# - (Future) Optionally convert to MIR and build an executable via ny-llvmc.
#
# Usage:
#   tools/selfhost/selfhost_build.sh --in source.hako [--mir out.json] [--run]
#   Options:
#     --in FILE     Input .hako source file (required)
#     --json FILE   Retired wrapper surface (compat-only; rejected with redirect)
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
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_stageb.sh" ]; then
  # Stage-B producer owner lives in its own helper file.
  # Keep this script focused on direct-run / exe-artifact / dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_stageb.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_direct.sh" ]; then
  # Direct MIR / core-direct owner lives in its own helper file.
  # Keep this script focused on exe-artifact / dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_direct.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh" ]; then
  # EXE artifact owner lives in its own helper file.
  # Keep this script focused on dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_dispatch.sh" ]; then
  # Final route dispatcher lives in its own helper file.
  # Keep this script focused on arg parsing and top-level tail orchestration.
  source "$ROOT/tools/selfhost/lib/selfhost_build_dispatch.sh"
fi
RAW_KEEP="${NYASH_SELFHOST_KEEP_RAW:-0}"
RAW_DIR="${NYASH_SELFHOST_RAW_DIR:-$ROOT/logs/selfhost}"
if [ "$RAW_KEEP" = "1" ]; then
  mkdir -p "$RAW_DIR" 2>/dev/null || RAW_KEEP=0
fi

IN=""
JSON_OUT=""
MIR_OUT=""
EXE_OUT=""
DO_RUN=0
KEEP_TMP=0

exit_program_json_wrapper_retired() {
  echo "[selfhost] --json is retired from selfhost_build.sh" >&2
  echo "           use --mir for public/bootstrap output" >&2
  echo "           use tools/dev/phase29ch_program_json_compat_route_probe.sh or raw --emit-program-json-v0 for explicit compat work" >&2
  exit 2
}

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

if [ -n "$JSON_OUT" ]; then
  exit_program_json_wrapper_retired
fi

tmp_json="${JSON_OUT:-/tmp/hako_stageb_$$.json}"

# Emit Program(JSON v0; prefer BuildBox for emit-only when HAKO_USE_BUILDBOX=1)
RAW="/tmp/hako_stageb_raw_$$.txt"
stageb_rc=0
SRC_CONTENT="$(cat "$IN")"
stageb_cmd_desc=""
apply_selfhost_env
emit_stageb_program_json_raw "$RAW" || stageb_rc=$?

extract_ok=0
if extract_program_json_v0_from_raw "$RAW" "$tmp_json"; then
  extract_ok=1
fi

raw_log="$(persist_stageb_raw_snapshot "$RAW" "$tmp_json" "$extract_ok")"

if [ "$extract_ok" != "1" ]; then
  exit_after_stageb_emit_failure "$RAW" "$raw_log"
fi
rm -f "$RAW" 2>/dev/null || true

dispatch_stageb_downstream_outputs "$tmp_json"
exit $?
