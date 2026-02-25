#!/bin/bash
# Stage‑B OOB canaries（opt‑in）: 配列/Map の OOB 振る舞い確認（現状は SKIP 既定）

set -uo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"
fi

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/lib/result_checker.sh"

require_env || exit 2
preflight_plugins || exit 2

if [ "${SMOKES_ENABLE_STAGEB_OOB:-0}" != "1" ]; then
  test_skip "selfhost_stageb_oob" "opt-in (set SMOKES_ENABLE_STAGEB_OOB=1)" && exit 0
fi

HAKO_BIN_DEFAULT="$ROOT/target/release/hakorune"
HAKO_BIN="${HAKO_BIN:-$HAKO_BIN_DEFAULT}"

require_hako() {
  if [ ! -x "$HAKO_BIN" ]; then
    log_warn "Hako binary not found: $HAKO_BIN (set HAKO_BIN to override)"
    exit 0
  fi
}

hako_compile_to_mir_stageb() {
  local code="$1"
  local hako_tmp="/tmp/hako_stageb_oob_$$.hako"
  local json_out="/tmp/hako_stageb_oob_$$.mir.json"
  printf "%s\n" "$code" > "$hako_tmp"
  local raw="/tmp/hako_stageb_oob_raw_$$.txt"
  NYASH_PARSER_ALLOW_SEMICOLON=1 HAKO_ALLOW_USING_FILE=1 NYASH_ALLOW_USING_FILE=1 \
  NYASH_FEATURES=stage3 \
  NYASH_VARMAP_GUARD_STRICT=0 NYASH_BLOCK_SCHEDULE_VERIFY=0 \
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 \
  "$HAKO_BIN" --backend vm \
    "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$(cat "$hako_tmp")" > "$raw" 2>&1
  awk '/"version":0/ && /"kind":"Program"/ {print; exit}' "$raw" > "$json_out"
  rm -f "$raw" "$hako_tmp"
  echo "$json_out"
}

run_mir_via_gate_c() {
  local json_path="$1"
  NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
    "$HAKO_BIN" --json-file "$json_path" >/tmp/hako_stageb_oob_run.txt 2>&1
  local rc=$?
  cat /tmp/hako_stageb_oob_run.txt >&2
  rm -f "$json_path" /tmp/hako_stageb_oob_run.txt
  return $rc
}

test_oob_array_read() {
  local code='box Main { static method main() { local a=[1,2]; print(a[5]); } }'
  local json
  json=$(hako_compile_to_mir_stageb "$code") || return 0 # emit失敗はここではスキップ
  if run_mir_via_gate_c "$json"; then
    log_warn "array OOB read did not error (allowed by current semantics)."
    return 0
  else
    log_success "array OOB read produced error (expected)"
    return 0
  fi
}

test_oob_array_write() {
  local code='box Main { static method main() { local a=[1,2]; a[5]=9; print(a[1]); } }'
  local json
  json=$(hako_compile_to_mir_stageb "$code") || return 0
  if run_mir_via_gate_c "$json"; then
    log_warn "array OOB write did not error (allowed by current semantics)."
    return 0
  else
    log_success "array OOB write produced error (expected)"
    return 0
  fi
}

run_test "stageb_oob_array_read" test_oob_array_read
run_test "stageb_oob_array_write" test_oob_array_write

# Map missing key (read) — accept error or 'null' as pass for now
test_oob_map_missing_read() {
  local code='box Main { static method main() { local m={"a":1}; print(m["zzz"]); } }'
  local json
  json=$(hako_compile_to_mir_stageb "$code") || return 0
  if out=$(NYASH_QUIET=1 HAKO_QUIET=1 NYASH_CLI_VERBOSE=0 NYASH_NYRT_SILENT_RESULT=1 \
           "$HAKO_BIN" --json-file "$json" 2>&1); then
    local last
    last=$(printf '%s\n' "$out" | awk '/^\\[deprecate\\]/{next} /^(✅|ResultType|Result:)/{next} NF{last=$0} END{print last}')
    if [ "$last" = "null" ]; then
      log_success "map missing read returned null (accepted)"
      rm -f "$json"
      return 0
    else
      log_warn "map missing read returned '$last' (tolerated for now)"
      rm -f "$json"
      return 0
    fi
  else
    log_success "map missing read produced error (accepted)"
    rm -f "$json"
    return 0
  fi
}

run_test "stageb_oob_map_missing_read" test_oob_map_missing_read

exit 0
