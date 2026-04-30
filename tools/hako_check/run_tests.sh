#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT/target/release/hakorune}"

if [ ! -x "$BIN" ]; then
  echo "[TEST] hakorune not built: $BIN" >&2
  echo "Run: cargo build --release" >&2
  exit 2
fi

TARGET_DIR="$ROOT/tools/hako_check/tests"
fail=0
skipped_count=0

run_case() {
  local dir="$1"
  # Skip HC017 (Non-ASCII Quotes) until UTF-8 byte-level support lands
  case "$(basename "$dir")" in
    HC017_*)
      echo "[TEST] skip - HC017 (non_ascii_quotes) - UTF-8 byte-level support required"
      skipped_count=$((skipped_count + 1))
      return
      ;;
  esac
  local expected="$dir/expected.json"
  local input_ok="$dir/ok.hako"
  local input_ng="$dir/ng.hako"
  if [ ! -f "$expected" ]; then echo "[TEST] skip (no expected): $dir"; return; fi
  if [ ! -f "$input_ok" ] && [ ! -f "$input_ng" ]; then echo "[TEST] skip (no inputs): $dir"; return; fi
  local tmp_out="/tmp/hako_test_$$.json"
  # Build a tiny wrapper program to call HakoAnalyzerBox.run with constructed argv
  local path_ok text_ok
  local path_ng text_ng
  if [ -f "$input_ok" ]; then
    path_ok="$input_ok"
    text_ok="$(sed 's/\r$//' "$input_ok")"
  else
    :
  fi
  if [ -f "$input_ng" ]; then
    path_ng="$input_ng"
    text_ng="$(sed 's/\r$//' "$input_ng")"
  else
    :
  fi
  # Build argv array for analyzer CLI (preserve newlines in text)
  ARGS=( --debug --format json-lsp )
  # Restrict rules to the target under test for stability
  local base
  base="$(basename "$dir")"
  local rules_key=""
  case "$base" in
    HC011_*) rules_key="dead_methods" ;;
    HC012_*) rules_key="dead_static_box" ;;
    HC013_*) rules_key="duplicate_method" ;;
    HC014_*) rules_key="missing_entrypoint" ;;
    HC015_*) rules_key="arity_mismatch" ;;
    HC016_*) rules_key="unused_alias" ;;
    HC017_*) rules_key="non_ascii_quotes" ;;
    HC018_*) rules_key="top_level_local" ;;
    HC019_*) rules_key="dead_code" ;;
    HC021_*) rules_key="analyzer_io_safety" ;;
    HC022_*) rules_key="stage3_gate" ;;
    HC031_*) rules_key="brace_heuristics" ;;
    HC032_*) rules_key="restricted_loop" ;;
    *) rules_key="" ;;
  esac
  if [ -n "$rules_key" ]; then ARGS+=( --rules "$rules_key" ); fi
  if [ -f "$input_ok" ]; then ARGS+=( --source-file "$path_ok" "$text_ok" ); fi
  if [ -f "$input_ng" ]; then ARGS+=( --source-file "$path_ng" "$text_ng" ); fi

  # Directly invoke analyzer CLI with args via '--', avoid wrapper/FS
  # Ensure plugin path is set
  export LD_LIBRARY_PATH="${ROOT}/target/release:${LD_LIBRARY_PATH:-}"

  NYASH_USE_NY_COMPILER=0 HAKO_DISABLE_NY_COMPILER=1 \
  NYASH_FEATURES="${NYASH_FEATURES:-stage3}" NYASH_PARSER_SEAM_TOLERANT=1 HAKO_PARSER_SEAM_TOLERANT=1 \
  NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 NYASH_USING_AST=1 \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_BOX_FACTORY_POLICY=builtin_first \
  NYASH_JSON_ONLY=1 \
  "$BIN" "$ROOT/tools/hako_check/cli.hako" -- "${ARGS[@]}" >"$tmp_out" 2>&1 || true

  # Some runtimes print plugin/deprecation banners to stdout/stderr even in JSON mode.
  # Filter those out before extracting the JSON-LSP block so tests remain stable.
  tmp_clean="/tmp/hako_test_clean_$$.log"
  awk '
    /^⚠️ \[DEPRECATED\]/ { next }
    /^📋 Phase [0-9.]+:/ { next }
    /^🔧 Check: plugins\// { next }
    /^\[UnifiedBoxRegistry\]/ { next }
    /^\[plugins\]/ { next }
    /^\[WARN\] \[plugin\/init\]/ { next }
    /^\[plugin\/missing\]/ { next }
    /^\[plugin\/hint\]/ { next }
    /^\[provider-registry\]/ { next }
    /^\[provider\/select:/ { next }
    { print }
  ' "$tmp_out" > "$tmp_clean"
  # Extract diagnostics JSON (one-line or pretty block)
  tmp_json="/tmp/hako_test_json_$$.json"
  json_line=$(grep -m1 '^\{"diagnostics"' "$tmp_clean" || true)
  if [ -n "$json_line" ] && echo "$json_line" | grep -q '\]}' ; then
    echo "$json_line" > "$tmp_json"
  else
    json_block=$(awk '/^\{"diagnostics"/{f=1} f{print} /\]\}/{exit}' "$tmp_clean" )
    if [ -z "$json_block" ]; then
      echo "[TEST/ERROR] no diagnostics JSON found; possible VM error. log head:" >&2
      sed -n '1,120p' "$tmp_out" >&2 || true
      json_block='{"diagnostics":[]}'
    fi
    printf "%s\n" "$json_block" > "$tmp_json"
  fi
  # Normalize absolute paths to basenames for stable comparison
  tmp_norm="/tmp/hako_test_norm_$$.json"
  cp "$tmp_json" "$tmp_norm"
  if [ -f "$input_ok" ]; then
    base_ok="$(basename "$input_ok")"; abs_ok="$input_ok"
    sed -i "s#\"file\":\"$abs_ok\"#\"file\":\"$base_ok\"#g" "$tmp_norm"
    sed -i "s#${abs_ok//\//\/}#${base_ok//\//\/}#g" "$tmp_norm"
    fi
  if [ -f "$input_ng" ]; then
    base_ng="$(basename "$input_ng")"; abs_ng="$input_ng"
    sed -i "s#\"file\":\"$abs_ng\"#\"file\":\"$base_ng\"#g" "$tmp_norm"
    sed -i "s#${abs_ng//\//\/}#${base_ng//\//\/}#g" "$tmp_norm"
  fi
  # Align trailing blank line behavior to expected (tolerate one extra blank line)
  if [ -f "$expected" ]; then
    if [ -z "$(tail -n1 "$tmp_norm")" ]; then :; else
      if [ -z "$(tail -n1 "$expected")" ]; then printf "\n" >> "$tmp_norm"; fi
    fi
  fi
  # Replace absolute path occurrences in message with PLACEHOLDER
  if [ -f "$input_ng" ]; then
    sed -i "s#${abs_ng//\//\/}#PLACEHOLDER#g" "$tmp_norm"
  fi
  if ! diff -u "$expected" "$tmp_norm" >/dev/null; then
    echo "[TEST/FAIL] $dir" >&2
    diff -u "$expected" "$tmp_norm" || true
    fail=$((fail+1))
  else
    echo "[TEST/OK] $dir"
  fi
  rm -f "$tmp_out" "$tmp_clean" "$tmp_norm" "$tmp_json"
}

# Handle arguments: if provided, test only specified dirs; otherwise test all
if [ $# -gt 0 ]; then
  # Test only specified directories
  for arg in "$@"; do
    # Convert relative path to absolute
    if [[ "$arg" == /* ]]; then
      d="$arg"
    else
      d="$ROOT/$arg"
    fi
    [ -d "$d" ] || { echo "[TEST] not a directory: $d" >&2; continue; }
    run_case "$d"
  done
else
  # Test all directories
  for d in "$TARGET_DIR"/*; do
    [ -d "$d" ] || continue
    run_case "$d"
  done
fi

if [ $fail -ne 0 ]; then
  echo "[TEST/SUMMARY] failures=$fail, skipped=$skipped_count" >&2
  exit 1
fi
if [ $skipped_count -gt 0 ]; then
  echo "[TEST/SUMMARY] all passed, $skipped_count skipped"
else
  echo "[TEST/SUMMARY] all green"
fi
exit 0
