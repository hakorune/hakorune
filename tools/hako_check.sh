#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[ERROR] hakorune binary not found: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

if [ $# -lt 1 ]; then
  echo "Usage: $0 [--format text|dot|json-lsp] <file-or-dir|file> [more...]" >&2
  exit 2
fi

fail=0
FORMAT="text"
EXTRA_ARGS=""

# Parse optional flags (--format, --dead-code, --rules, etc.)
while [ $# -gt 0 ]; do
  case "${1:-}" in
    --format)
      FORMAT="$2"
      shift 2 || true
      ;;
    --dead-code|--rules|--no-ast|--debug)
      EXTRA_ARGS="$EXTRA_ARGS $1"
      shift
      ;;
    *)
      break
      ;;
  esac
done
list_targets() {
  local p="$1"
  if [ -d "$p" ]; then
    find "$p" -type f -name '*.hako' | sort
  else
    echo "$p"
  fi
}

run_one() {
  local f="$1"
  # Run analyzer main with inlined source text to avoid FileBox dependency
  local text
  text="$(sed 's/\r$//' "$f")"

  # Phase 156: Generate MIR JSON for CFG-based analysis and pass inline
  local mir_json_path="/tmp/hako_check_mir_$$.json"
  local mir_json_content=""
  if [ -x "$ROOT/tools/hakorune_emit_mir.sh" ]; then
    "$ROOT/tools/hakorune_emit_mir.sh" "$f" "$mir_json_path" >/dev/null 2>&1 || true
    if [ -f "$mir_json_path" ]; then
      mir_json_content="$(cat "$mir_json_path")"
    fi
  fi

  # Build args array with optional MIR JSON
  local args_arr=("--source-file" "$f" "$text")
  if [ -n "$mir_json_content" ]; then
    args_arr+=("--mir-json-content" "$mir_json_content")
  fi

  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_DISABLE_NY_COMPILER=1 \
  HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
  NYASH_PARSER_SEAM_TOLERANT=1 \
  HAKO_PARSER_SEAM_TOLERANT=1 \
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 \
  HAKO_ENABLE_USING=1 \
  NYASH_USING_AST=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-8000}" \
  HAKO_CHECK_DEBUG="${HAKO_CHECK_DEBUG:-0}" \
  HAKO_CHECK_VERBOSE="${HAKO_CHECK_VERBOSE:-0}" \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- "${args_arr[@]}" --format "$FORMAT" $EXTRA_ARGS \
    >"/tmp/hako_lint_out_$$.log" 2>&1 || true
  local out rc
  out="$(cat "/tmp/hako_lint_out_$$.log")"; rc=0

  # Phase 1: Filter out debug noise unless HAKO_CHECK_DEBUG=1
  if [ "${HAKO_CHECK_DEBUG:-0}" != "1" ]; then
    out="$(echo "$out" | grep -v '^\[DEBUG' | grep -v '^\[ControlForm::' | grep -v '^\[BUILDER\]' | grep -v '^\[rule/exec\]')"
  fi

  # Extract RC
  if echo "$out" | grep -q '^RC: '; then
    rc="$(echo "$out" | sed -n 's/^RC: //p' | tail -n1)"
  else rc=1; fi
  if [ "$rc" != "0" ]; then
    echo "$out" | sed -n '1,200p'
    fail=$((fail+1))
  fi
  rm -f "/tmp/hako_lint_out_$$.log" "$mir_json_path"
}

if [ "$FORMAT" = "dot" ]; then
  # Aggregate all targets and render DOT once
  TMP_LIST="/tmp/hako_targets_$$.txt"; : >"$TMP_LIST"
  for p in "$@"; do list_targets "$p" >>"$TMP_LIST"; done
  mapfile -t FILES <"$TMP_LIST"
  rm -f "$TMP_LIST"
  ARGS=()
  for f in "${FILES[@]}"; do
    text="$(sed 's/\r$//' "$f")"
    ARGS+=(--source-file "$f" "$text")
  done

  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_DISABLE_NY_COMPILER=1 \
  HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
  NYASH_PARSER_SEAM_TOLERANT=1 \
  HAKO_PARSER_SEAM_TOLERANT=1 \
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 \
  HAKO_ENABLE_USING=1 \
  NYASH_USING_AST=1 \
  NYASH_JSON_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-8000}" \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --format dot "${FILES[@]}" \
    >"/tmp/hako_lint_out_$$.log" 2>/tmp/hako_lint_err_$$.log || true
  rc=$?
  # Only print DOT graph body to STDOUT
  awk '/^digraph /, /^}/' "/tmp/hako_lint_out_$$.log"
  rm -f "/tmp/hako_lint_out_$$.log" "/tmp/hako_lint_err_$$.log"
  exit $([ "$rc" -eq 0 ] && echo 0 || echo 1)
elif [ "$FORMAT" = "json-lsp" ]; then
  # Aggregate and emit pure JSON (no summaries). Exit code = findings count.
  TMP_LIST="/tmp/hako_targets_$$.txt"; : >"$TMP_LIST"
  for p in "$@"; do list_targets "$p" >>"$TMP_LIST"; done
  mapfile -t FILES <"$TMP_LIST"
  rm -f "$TMP_LIST"

  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_DISABLE_NY_COMPILER=1 \
  HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
  NYASH_PARSER_SEAM_TOLERANT=1 \
  HAKO_PARSER_SEAM_TOLERANT=1 \
  NYASH_PARSER_ALLOW_SEMICOLON=1 \
  NYASH_ENABLE_USING=1 \
  HAKO_ENABLE_USING=1 \
  NYASH_USING_AST=1 \
  NYASH_JSON_ONLY=1 \
  NYASH_NY_COMPILER_TIMEOUT_MS="${NYASH_NY_COMPILER_TIMEOUT_MS:-8000}" \
  "$BIN" --backend vm "$ROOT/tools/hako_check/cli.hako" -- --format json-lsp "${ARGS[@]}"
  exit $?
else
  for p in "$@"; do
    while IFS= read -r f; do run_one "$f"; done < <(list_targets "$p")
  done
fi

if [ $fail -ne 0 ]; then
  echo "[lint/summary] failures: $fail" >&2
  exit 1
fi
echo "[lint/summary] all clear" >&2
exit 0
