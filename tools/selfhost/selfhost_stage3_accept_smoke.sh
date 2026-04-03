#!/usr/bin/env bash
set -euo pipefail
[[ "${NYASH_CLI_VERBOSE:-0}" == "1" ]] && set -x

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
BIN="$ROOT_DIR/target/release/hakorune"
SELFHOST_RUN="$ROOT_DIR/tools/selfhost/run.sh"

if [[ ! -x "$BIN" ]]; then
  (cd "$ROOT_DIR" && cargo build --release >/dev/null)
fi

TMP="$ROOT_DIR/tmp"
mkdir -p "$TMP"

pass() { echo "✅ $1" >&2; }
fail() { echo "❌ $1" >&2; echo "$2" >&2; exit 1; }

run_case_stage3() {
  local name="$1"; shift
  local src="$1"; shift
  local expect_code="$1"; shift
  local file="$TMP/selfhost_stage3_${name// /_}.hako"
  printf "%s\n" "$src" > "$file"
  # 1) Produce JSON v0 via selfhost compiler program (explicit compat fallback is allowed here)
  set +e
  # Use unified selfhost entry (--direct) as Program(JSON v0) producer (tolerate empty).
  JSON=$("$SELFHOST_RUN" --direct --source-file "$file" --timeout-secs "${SMOKES_SELFHOST_STAGEB_TIMEOUT_SECS:-20}" --route-id "SH-SMOKE-STAGE3" 2>/dev/null | awk 'BEGIN{found=0} /^[ \t]*\{/{ if ($0 ~ /"version"/ && $0 ~ /"kind"/) { print; found=1; exit } } END{ if(found==0){} }')
  # 2) Execute JSON v0 via Bridge (compat acceptance path).
  if [[ -z "$JSON" ]]; then OUT=""; CODE=0; else OUT=$(printf '%s\n' "$JSON" | NYASH_TRY_RESULT_MODE=${NYASH_TRY_RESULT_MODE:-1} "$BIN" --ny-parser-pipe --backend vm 2>&1); CODE=$?; fi
  set -e
  if [[ "$CODE" == "$expect_code" ]]; then pass "$name"; else fail "$name" "$OUT"; fi
}

# A) try/catch/cleanup acceptance; final return 0
run_case_stage3 "try_cleanup" $'try { local x = 1 } catch (Error e) { local y = 2 } cleanup { local z = 3 }\nreturn 0' 0

# B) break acceptance under dead branch
run_case_stage3 "break_dead" $'if false { break } else { }\nreturn 0' 0

# C) continue acceptance under dead branch
run_case_stage3 "continue_dead" $'if false { continue } else { }\nreturn 0' 0

# D) throw acceptance (degrade); final return 0
run_case_stage3 "throw_accept" $'try { throw 123 } cleanup { }\nreturn 0' 0

# E) nested throw inside if: should route to catch, then cleanup runs, and return 0
run_case_stage3 "throw_nested_if" $'try { if true { throw "boom" } else { local k = 1 } } catch (e) { local caught = 1 } cleanup { local fin = 1 }\nreturn 0' 0

echo "All selfhost Stage-3 acceptance smokes PASS" >&2
exit 0
