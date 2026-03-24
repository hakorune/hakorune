#!/usr/bin/env bash
# hakorune_emit_mir.sh — Emit MIR(JSON) using Hakorune Stage‑B + MirBuilder (Hako‑first)
#
# Usage: tools/hakorune_emit_mir.sh <input.hako> <out.json>
# Notes:
#  - Runs the Stage‑B compiler (Hako) to emit Program(JSON v0), then feeds it to MirBuilderBox.emit_from_program_json_v0.
#  - Defaults to the Hakorune VM path; no inline Ny compiler; Stage‑3 enabled.
#  - Selfhost safety defaults: strict+planner_required ON for this helper unless caller overrides
#    `HAKO_JOINIR_STRICT` / `HAKO_JOINIR_PLANNER_REQUIRED`.
#  - Keeps defaults conservative: no global flips; this is a helper for dev/CI scripts.

set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "Usage: $0 <input.hako> <out.json>" >&2
  exit 2
fi

IN="$1"
OUT="$2"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
fi

# Resolve nyash/hakorune binary via test_runner helper (ensures consistent env)
if [ ! -f "$IN" ]; then
  echo "[FAIL] input not found: $IN" >&2
  exit 1
fi

# Resolve nyash/hakorune binary (simple detection; test_runner will override later if sourced)
if [ -z "${NYASH_BIN:-}" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then
    export NYASH_BIN="$ROOT/target/release/hakorune"
  else
    export NYASH_BIN="$ROOT/target/release/nyash"
  fi
fi

CODE="$(cat "$IN")"
STAGEB_JOINIR_STRICT="${HAKO_JOINIR_STRICT:-1}"
STAGEB_JOINIR_PLANNER_REQUIRED="${HAKO_JOINIR_PLANNER_REQUIRED:-1}"

extract_using_imports_json_from_source() {
  local src_file="$1"
  python3 - "$src_file" <<'PY'
import json
import pathlib
import re
import sys

path = pathlib.Path(sys.argv[1])
try:
    text = path.read_text(encoding="utf-8")
except Exception:
    print("{}")
    raise SystemExit(0)

imports = {}
for raw in text.splitlines():
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

    if not target:
        continue

    if len(target) >= 2 and target[0] == target[-1] and target[0] in ("'", '"'):
        target = target[1:-1]
    if not target:
        continue

    if not alias:
        parts = [p for p in re.split(r"[./]", target) if p]
        if not parts:
            continue
        alias = parts[-1]

    if not alias:
        continue

    imports[alias] = target

print(json.dumps(imports, separators=(",", ":")))
PY
}

merge_import_maps_json() {
  local left_json="${1-}"
  local right_json="${2-}"
  if [ -z "$left_json" ]; then
    left_json='{}'
  fi
  if [ -z "$right_json" ]; then
    right_json='{}'
  fi
  python3 - "$left_json" "$right_json" <<'PY'
import json
import sys

def parse_obj(raw):
    if raw is None or str(raw).strip() == "":
        return {}
    try:
        v = json.loads(raw)
    except Exception:
        return {}
    return v if isinstance(v, dict) else {}

left = parse_obj(sys.argv[1] if len(sys.argv) > 1 else "{}")
right = parse_obj(sys.argv[2] if len(sys.argv) > 2 else "{}")
out = dict(left)
out.update(right)
print(json.dumps(out, separators=(",", ":")))
PY
}

merge_program_json_imports() {
  local prog_json="$1"
  local imports_json="${2-}"
  if [ -z "$imports_json" ]; then
    imports_json='{}'
  fi
  python3 - "$prog_json" "$imports_json" <<'PY'
import json
import sys

prog_raw = sys.argv[1]
imports_raw = sys.argv[2] if len(sys.argv) > 2 else "{}"

try:
    prog = json.loads(prog_raw)
except Exception:
    print(prog_raw)
    raise SystemExit(0)

if not isinstance(prog, dict):
    print(prog_raw)
    raise SystemExit(0)

try:
    imports = json.loads(imports_raw) if imports_raw.strip() else {}
except Exception:
    imports = {}
if not isinstance(imports, dict):
    imports = {}

current = prog.get("imports")
if not isinstance(current, dict):
    current = {}

for k, v in imports.items():
    if isinstance(k, str) and isinstance(v, str) and k and v and k not in current:
        current[k] = v

prog["imports"] = current
print(json.dumps(prog, separators=(",", ":")))
PY
}

extract_mir_payload_from_stdout_file() {
  local stdout_file="$1"
  awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag' "$stdout_file"
}

persist_mir_payload_from_stdout_file() {
  local stdout_file="$1" out_path="$2"
  local mir
  mir="$(extract_mir_payload_from_stdout_file "$stdout_file")"
  if [ -z "$mir" ]; then
    return 1
  fi
  printf '%s' "$mir" > "$out_path"
  return 0
}

SOURCE_USING_IMPORTS_JSON="$(extract_using_imports_json_from_source "$IN")"
BASE_MIRBUILDER_IMPORTS_JSON="${HAKO_MIRBUILDER_IMPORTS-}"
if [ -z "$BASE_MIRBUILDER_IMPORTS_JSON" ]; then
  BASE_MIRBUILDER_IMPORTS_JSON='{}'
fi
MERGED_MIRBUILDER_IMPORTS_JSON="$(merge_import_maps_json "$BASE_MIRBUILDER_IMPORTS_JSON" "$SOURCE_USING_IMPORTS_JSON")"
export HAKO_MIRBUILDER_IMPORTS="$MERGED_MIRBUILDER_IMPORTS_JSON"

try_direct_emit_mir_json() {
  local out_path="$1"
  rm -f "$out_path" || true
  if ! HAKO_STAGEB_FUNC_SCAN="${HAKO_STAGEB_FUNC_SCAN:-}" \
       HAKO_JOINIR_STRICT="$STAGEB_JOINIR_STRICT" HAKO_JOINIR_PLANNER_REQUIRED="$STAGEB_JOINIR_PLANNER_REQUIRED" \
       HAKO_MIR_BUILDER_FUNCS="${HAKO_MIR_BUILDER_FUNCS:-}" \
       HAKO_MIR_BUILDER_CALL_RESOLVE="${HAKO_MIR_BUILDER_CALL_RESOLVE:-}" \
       NYASH_JSON_SCHEMA_V1=${NYASH_JSON_SCHEMA_V1:-1} \
       NYASH_MIR_UNIFIED_CALL=${NYASH_MIR_UNIFIED_CALL:-1} \
       "$NYASH_BIN" --emit-mir-json "$out_path" "$IN" >/dev/null 2>&1; then
    return 1
  fi
  if [ ! -s "$out_path" ]; then
    echo "[FAIL] direct MIR emit returned success without payload: $out_path" >&2
    return 1
  fi
  if ! grep -q '"functions"' "$out_path"; then
    echo "[FAIL] direct MIR emit produced non-MIR payload: $out_path" >&2
    return 1
  fi
  return 0
}

coerce_stageb_program_json_v0_result_kind() {
  local result_kind="${1-}"
  case "$result_kind" in
    stageb-fail|stageb-invalid)
      printf '%s' "$result_kind"
      return 0
      ;;
    *)
      echo "[FAIL] unexpected Stage-B Program(JSON) fallback kind: ${result_kind:-<empty>}" >&2
      return 1
      ;;
  esac
}

stageb_program_json_v0_mainline_only_fail_message() {
  local result_kind="$1"
  case "$result_kind" in
    stageb-invalid)
      printf '%s' "[FAIL] Stage-B output invalid under mainline-only mode (compat fallback disabled)"
      ;;
    stageb-fail)
      printf '%s' "[FAIL] Stage-B failed under mainline-only mode (compat fallback disabled)"
      ;;
  esac
}

stageb_program_json_v0_direct_emit_success_label() {
  local result_kind="$1"
  case "$result_kind" in
    stageb-invalid)
      printf '%s' "direct-emit-fallback"
      ;;
    stageb-fail)
      printf '%s' "direct-emit"
      ;;
  esac
}

stageb_program_json_v0_direct_emit_fail_message() {
  local result_kind="$1"
  case "$result_kind" in
    stageb-invalid)
      printf '%s' "[FAIL] Stage‑B output invalid and direct emit failed"
      ;;
    stageb-fail)
      printf '%s' "[FAIL] Stage-B and direct MIR emit both failed"
      ;;
  esac
}

exit_after_forced_direct_emit() {
  local out_path="$1"
  if try_direct_emit_mir_json "$out_path"; then
    echo "[OK] MIR JSON written (direct-emit-forced): $out_path"
    exit 0
  fi
  echo "[FAIL] direct MIR emit forced but failed" >&2
  exit 1
}

exit_after_stageb_program_json_v0_fallback_policy() {
  local result_kind out_path mainline_only_fail success_label direct_emit_fail
  result_kind="$(coerce_stageb_program_json_v0_result_kind "${1-}")" || exit 1
  out_path="$2"
  mainline_only_fail="$(stageb_program_json_v0_mainline_only_fail_message "$result_kind")"
  success_label="$(stageb_program_json_v0_direct_emit_success_label "$result_kind")"
  direct_emit_fail="$(stageb_program_json_v0_direct_emit_fail_message "$result_kind")"

  if [ "${HAKO_EMIT_MIR_MAINLINE_ONLY:-0}" = "1" ]; then
    echo "$mainline_only_fail" >&2
    exit 1
  fi
  if try_direct_emit_mir_json "$out_path"; then
    echo "[OK] MIR JSON written ($success_label): $out_path"
    exit 0
  fi
  echo "$direct_emit_fail" >&2
  exit 1
}

extract_loop_force_limit_from_program_json() {
  local prog_json="$1"
  printf '%s' "$prog_json" | grep -o '"type":"Int","value":[0-9]*' | head -1 | grep -o '[0-9]*$' || echo "10"
}

write_loop_force_jsonfrag_mir_json() {
  local limit="$1" out_path="$2"
  cat > "$out_path" <<'MIRJSON'
{
  "functions": [{
    "name": "main",
    "params": [],
    "locals": [],
    "blocks": [
      {
        "id": 0,
        "instructions": [
          {"op": "const", "dst": 1, "value": {"type": "i64", "value": 0}},
          {"op": "const", "dst": 2, "value": {"type": "i64", "value": LIMIT_PLACEHOLDER}},
          {"op": "jump", "target": 1}
        ]
      },
      {
        "id": 1,
        "instructions": [
          {"op": "phi", "dst": 6, "incoming": [[2, 0], [6, 2]]},
          {"op": "phi", "dst": 3, "incoming": [[1, 0], [5, 2]]},
          {"op": "compare", "operation": "<", "lhs": 3, "rhs": 6, "dst": 4},
          {"op": "branch", "cond": 4, "then": 2, "else": 3}
        ]
      },
      {
        "id": 2,
        "instructions": [
          {"op": "const", "dst": 10, "value": {"type": "i64", "value": 1}},
          {"op": "binop", "operation": "+", "lhs": 3, "rhs": 10, "dst": 5},
          {"op": "jump", "target": 1}
        ]
      },
      {
        "id": 3,
        "instructions": [
          {"op": "ret", "value": 3}
        ]
      }
    ]
  }]
}
MIRJSON
  sed -i "s/LIMIT_PLACEHOLDER/$limit/g" "$out_path"
}

# Explicit direct-emit mode (parity baseline)
if [ "${HAKO_EMIT_MIR_FORCE_DIRECT:-0}" = "1" ]; then
  exit_after_forced_direct_emit "$OUT"
fi

# 1) Stage‑B: Hako parser emits Program(JSON v0) to stdout
# Extract Program JSON robustly using Python3 bracket balancing
extract_program_json() {
  python3 -c '
import sys

stdin = sys.stdin.read()
# Find the start of Program JSON (look for "kind":"Program")
start = stdin.find("\"kind\":\"Program\"")
if start < 0:
    sys.exit(1)

# Walk back to find the opening brace of the object containing "kind":"Program"
pos = start
depth = 0
while pos >= 0:
    if stdin[pos] == "{":
        depth += 1
        if depth == 1:
            break
    elif stdin[pos] == "}":
        depth -= 1
    pos -= 1

if pos < 0:
    sys.exit(1)

# Walk forward and find matching closing brace with string/escape handling.
obj_start = pos
depth = 0
in_string = False
escape = False
i = obj_start

while i < len(stdin):
    ch = stdin[i]
    if escape:
        escape = False
    elif in_string:
        if ch == "\\":
            escape = True
        elif ch == "\"":
            in_string = False
    else:
        if ch == "\"":
            in_string = True
        elif ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                print(stdin[obj_start:i + 1])
                sys.exit(0)
    i += 1

sys.exit(1)
'
}

emit_stageb_program_json_v0() {
  local raw_prog_json normalized_prog_json
  raw_prog_json="$(execute_stageb_program_json_v0_raw)" || return 1
  normalized_prog_json="$(coerce_stageb_program_json_v0_output "$raw_prog_json")" || return $?
  printf '%s' "$normalized_prog_json"
  return 0
}

execute_stageb_program_json_v0_raw() {
  local prog_json_out rc
  set +e
  prog_json_out=$((cd "$ROOT" && \
                  NYASH_JSON_ONLY=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
                  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
                  HAKO_JOINIR_STRICT="$STAGEB_JOINIR_STRICT" HAKO_JOINIR_PLANNER_REQUIRED="$STAGEB_JOINIR_PLANNER_REQUIRED" \
                  HAKO_STAGEB_FUNC_SCAN="${HAKO_STAGEB_FUNC_SCAN:-}" \
                  NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
                  NYASH_ENABLE_USING=${NYASH_ENABLE_USING:-1} HAKO_ENABLE_USING=${HAKO_ENABLE_USING:-1} \
                  "$NYASH_BIN" --backend vm "$ROOT/lang/src/compiler/entry/compiler_stageb.hako" -- --source "$CODE") 2>/dev/null | extract_program_json)
  rc=$?
  set -e

  if [ $rc -ne 0 ] || [ -z "$prog_json_out" ]; then
    return 1
  fi
  printf '%s' "$prog_json_out"
  return 0
}

coerce_stageb_program_json_v0_output() {
  local prog_json_out="$1"
  if ! printf '%s' "$prog_json_out" | grep -q '"kind"\s*:\s*"Program"'; then
    return 2
  fi

  # Stage-B Program(JSON) may omit alias-less using entries in `imports`.
  # Merge source-derived using aliases so Program→MIR delegate paths stay stable.
  prog_json_out="$(merge_program_json_imports "$prog_json_out" "$MERGED_MIRBUILDER_IMPORTS_JSON")"
  printf '%s' "$prog_json_out"
  return 0
}

load_stageb_program_json_v0() {
  local prog_json_out
  STAGEB_PROGRAM_JSON_V0_OUT=""
  STAGEB_PROGRAM_JSON_V0_RESULT_KIND=""

  if ! prog_json_out="$(execute_stageb_program_json_v0_raw)"; then
    STAGEB_PROGRAM_JSON_V0_RESULT_KIND="stageb-fail"
    return 1
  fi

  if ! prog_json_out="$(coerce_stageb_program_json_v0_output "$prog_json_out")"; then
    STAGEB_PROGRAM_JSON_V0_RESULT_KIND="stageb-invalid"
    return 1
  fi

  STAGEB_PROGRAM_JSON_V0_OUT="$prog_json_out"
  return 0
}
STAGEB_PROGRAM_JSON_V0_OUT=""
STAGEB_PROGRAM_JSON_V0_RESULT_KIND=""
if ! load_stageb_program_json_v0; then
  exit_after_stageb_program_json_v0_fallback_policy "$STAGEB_PROGRAM_JSON_V0_RESULT_KIND" "$OUT"
fi
PROG_JSON_OUT="$STAGEB_PROGRAM_JSON_V0_OUT"

# 2) Convert Program(JSON v0) → MIR(JSON)
#    Prefer selfhost builder first when explicitly requested; otherwise use delegate (Gate‑C) for stability.

try_selfhost_builder() {
  local prog_json="$1" out_path="$2"

  # FORCE=1 direct assembly shortcut (dev toggle, bypasses using resolution)
  if [ "${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-0}" = "1" ]; then
    local limit
    limit="$(extract_loop_force_limit_from_program_json "$prog_json")"
    write_loop_force_jsonfrag_mir_json "$limit" "$out_path"

    if [ "${HAKO_SELFHOST_TRACE:-0}" = "1" ]; then
      echo "[selfhost-direct:ok] Direct MIR assembly (FORCE=1), limit=$limit" >&2
    fi

    return 0
  fi

  # Builder box selection (default: hako.mir.builder)
  local builder_box="${HAKO_MIR_BUILDER_BOX:-hako.mir.builder}"

  local tmp_hako; tmp_hako=$(mktemp --suffix .hako)
  render_selfhost_builder_runner_hako "$tmp_hako" "$builder_box"
  local tmp_stdout; tmp_stdout=$(mktemp)

  # Trace mode: analyze Program(JSON) before passing to builder
  if [ "${HAKO_SELFHOST_TRACE:-0}" = "1" ]; then
    local prog_len=${#prog_json}
    local loop_count=$(printf '%s' "$prog_json" | grep -o '"type":"Loop"' 2>/dev/null | wc -l | tr -d ' \n')
    local cmp_count=$(printf '%s' "$prog_json" | grep -o '"type":"Compare"' 2>/dev/null | wc -l | tr -d ' \n')
    loop_count=${loop_count:-0}
    cmp_count=${cmp_count:-0}
    local cwd="$(pwd)"
    local toml_status="absent"
    if [ -f "$ROOT/nyash.toml" ]; then
      toml_status="present"
    fi
    echo "[builder/selfhost-first:trace] builder_box=$builder_box prog_json_len=$prog_len tokens=Loop:$loop_count,Compare:$cmp_count cwd=$cwd nyash.toml=$toml_status" >&2
  fi

  set +e
  # Run from repo root to ensure nyash.toml is available for using resolution.
  # Capture both stdout and stderr (2>&1) instead of discarding stderr.
  execute_selfhost_builder_runner "$tmp_hako" "$tmp_stdout" "$prog_json"
  local rc=$?
  set -e

  # Enhanced failure diagnostics
  if [ $rc -ne 0 ]; then
    if [ "${HAKO_SELFHOST_NO_DELEGATE:-0}" = "1" ]; then
      echo "[builder/selfhost-first:fail:child:rc=$rc]" >&2
      echo "[builder/selfhost-first:fail:detail] Last 80 lines of output:" >&2
      tail -n 80 "$tmp_stdout" >&2 || true
    fi
    cleanup_selfhost_builder_runner_temp "$tmp_hako" "$tmp_stdout"
    # Don't return immediately - check for fallback below
  fi

  if [ $rc -eq 0 ] && ! grep -q "\[builder/selfhost-first:ok\]" "$tmp_stdout"; then
    if [ "${HAKO_SELFHOST_NO_DELEGATE:-0}" = "1" ]; then
      echo "[builder/selfhost-first:fail:no-ok-marker]" >&2
      echo "[builder/selfhost-first:fail:detail] Last 80 lines of output:" >&2
      tail -n 80 "$tmp_stdout" >&2 || true
    fi
    cleanup_selfhost_builder_runner_temp "$tmp_hako" "$tmp_stdout"
    rc=1
  fi

  # Try min builder fallback if enabled and initial builder failed
  if [ "${HAKO_SELFHOST_TRY_MIN:-0}" = "1" ] && [ $rc -ne 0 ] && [ "$builder_box" != "hako.mir.builder.min" ]; then
    if [ "${HAKO_SELFHOST_NO_DELEGATE:-0}" = "1" ]; then
      echo "[builder/selfhost-first:trying-min-fallback]" >&2
    fi

    # Retry with min builder
    cleanup_selfhost_builder_runner_temp "$tmp_hako" "$tmp_stdout"
    HAKO_MIR_BUILDER_BOX="hako.mir.builder.min" try_selfhost_builder "$prog_json" "$out_path"
    return $?
  fi

  # Return original failure if no fallback or if fallback not triggered
  if [ $rc -ne 0 ]; then
    cleanup_selfhost_builder_runner_temp "$tmp_hako" "$tmp_stdout"
    return 1
  fi

  if ! capture_selfhost_builder_mir_payload "$tmp_stdout" "$out_path"; then
    cleanup_selfhost_builder_runner_temp "$tmp_hako" "$tmp_stdout"
    return 1
  fi
  cleanup_selfhost_builder_runner_temp "$tmp_hako" "$tmp_stdout"
  echo "[OK] MIR JSON written (selfhost-first): $out_path"
  return 0
}

render_selfhost_builder_runner_hako() {
  local tmp_hako="$1" builder_box="$2"
  if [ "$builder_box" = "hako.mir.builder.min" ]; then
    cat >"$tmp_hako" <<'HCODE'
using "hako.mir.builder.internal.runner_min" as BuilderRunnerMinBox
static box Main { method main(args) {
  local prog_json = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if prog_json == null { print("[builder/selfhost-first:fail:nojson]"); return 1 }
  local mir_out = BuilderRunnerMinBox.run(prog_json)
  if mir_out == null { print("[builder/selfhost-first:fail:emit]"); return 1 }
  print("[builder/selfhost-first:ok]")
  print("[MIR_OUT_BEGIN]")
  print("" + mir_out)
  print("[MIR_OUT_END]")
  return 0
} }
HCODE
  else
    cat >"$tmp_hako" <<'HCODE'
using "__BUILDER_BOX__" as MirBuilderBox
static box Main {
method _emit_mir_checked(prog_json) {
  local mir_out = MirBuilderBox.emit_from_program_json_v0(prog_json, null)
  if mir_out == null { print("[builder/selfhost-first:fail:emit]"); return null }
  return mir_out
}
method main(args) {
  local prog_json = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if prog_json == null { print("[builder/selfhost-first:fail:nojson]"); return 1 }
  local mir_out = me._emit_mir_checked(prog_json)
  if mir_out == null { return 1 }
  print("[builder/selfhost-first:ok]")
  print("[MIR_OUT_BEGIN]")
  print("" + mir_out)
  print("[MIR_OUT_END]")
  return 0
} }
HCODE
    sed -i "s|__BUILDER_BOX__|$builder_box|g" "$tmp_hako"
  fi
}

execute_selfhost_builder_runner() {
  local tmp_hako="$1" tmp_stdout="$2" prog_json="$3"
  (cd "$ROOT" && \
    HAKO_MIR_BUILDER_INTERNAL=1 HAKO_MIR_BUILDER_REGISTRY=1 \
    HAKO_MIR_BUILDER_TRACE="${HAKO_SELFHOST_TRACE:-}" \
    HAKO_MIR_BUILDER_LOOP_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_JSONFRAG:-}" \
    HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG="${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-}" \
    HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE="${HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE:-}" \
    HAKO_MIR_BUILDER_JSONFRAG_PURIFY="${HAKO_MIR_BUILDER_JSONFRAG_PURIFY:-}" \
    HAKO_MIR_BUILDER_METHODIZE="${HAKO_MIR_BUILDER_METHODIZE:-}" \
    HAKO_MIR_BUILDER_NORMALIZE_TAG="${HAKO_MIR_BUILDER_NORMALIZE_TAG:-}" \
    HAKO_MIR_BUILDER_DEBUG="${HAKO_MIR_BUILDER_DEBUG:-}" \
    NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-0}" NYASH_FILEBOX_MODE="core-ro" HAKO_PROVIDER_POLICY="safe-core-first" \
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
    HAKO_JOINIR_STRICT="$STAGEB_JOINIR_STRICT" HAKO_JOINIR_PLANNER_REQUIRED="$STAGEB_JOINIR_PLANNER_REQUIRED" \
    NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
    NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
    NYASH_USE_NY_COMPILER=0 HAKO_USE_NY_COMPILER=0 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
    NYASH_MACRO_DISABLE=1 HAKO_MACRO_DISABLE=1 \
    HAKO_BUILDER_PROGRAM_JSON="$prog_json" \
    "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 | tee "$tmp_stdout" >/dev/null)
}

capture_selfhost_builder_mir_payload() {
  local tmp_stdout="$1" out_path="$2"
  persist_mir_payload_from_stdout_file "$tmp_stdout" "$out_path"
}

cleanup_selfhost_builder_runner_temp() {
  local tmp_hako="$1" tmp_stdout="$2"
  rm -f "${tmp_hako:-}" "${tmp_stdout:-}" 2>/dev/null || true
}

# Provider-first delegate: call env.mirbuilder.emit(prog_json) and capture v1 JSON
try_provider_emit() {
  local prog_json="$1" out_path="$2"
  local tmp_hako; tmp_hako=$(mktemp --suffix .hako)
  render_provider_emit_runner_hako "$tmp_hako"
  local tmp_stdout; tmp_stdout=$(mktemp)
  set +e
  execute_provider_emit_runner "$tmp_hako" "$tmp_stdout" "$prog_json"
  local rc=$?
  set -e
  if [ $rc -ne 0 ] || ! grep -q "\[provider/emit:ok\]" "$tmp_stdout"; then
    cleanup_provider_emit_runner_temp "$tmp_hako" "$tmp_stdout"
    return 1
  fi
  if ! capture_provider_emit_mir_payload "$tmp_stdout" "$out_path"; then
    cleanup_provider_emit_runner_temp "$tmp_hako" "$tmp_stdout"
    return 1
  fi
  cleanup_provider_emit_runner_temp "$tmp_hako" "$tmp_stdout"
  echo "[OK] MIR JSON written (delegate:provider): $out_path"
  return 0
}

render_provider_emit_runner_hako() {
  local tmp_hako="$1"
  cat >"$tmp_hako" <<'HCODE'
static box Main { method main(args) {
  local p = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if p == null { print("[provider/emit:nojson]"); return 1 }
  local a = new ArrayBox(); a.push(p)
  local out = hostbridge.extern_invoke("env.mirbuilder", "emit", a)
  print("[provider/emit:ok]")
  print("[MIR_OUT_BEGIN]")
  print("" + out)
  print("[MIR_OUT_END]")
  return 0
} }
HCODE
}

execute_provider_emit_runner() {
  local tmp_hako="$1" tmp_stdout="$2" prog_json="$3"
  (cd "$ROOT" && \
    NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-0}" NYASH_FILEBOX_MODE="core-ro" \
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
    HAKO_JOINIR_STRICT="$STAGEB_JOINIR_STRICT" HAKO_JOINIR_PLANNER_REQUIRED="$STAGEB_JOINIR_PLANNER_REQUIRED" \
    NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1 NYASH_PARSER_ALLOW_SEMICOLON=1 \
    HAKO_BUILDER_PROGRAM_JSON="$prog_json" \
    "$NYASH_BIN" --backend vm "$tmp_hako" 2>&1 | tee "$tmp_stdout" >/dev/null)
}

capture_provider_emit_mir_payload() {
  local tmp_stdout="$1" out_path="$2"
  persist_mir_payload_from_stdout_file "$tmp_stdout" "$out_path"
}

cleanup_provider_emit_runner_temp() {
  local tmp_hako="$1" tmp_stdout="$2"
  rm -f "${tmp_hako:-}" "${tmp_stdout:-}" 2>/dev/null || true
}

try_legacy_program_json_delegate() {
  local prog_json="$1" out_path="$2"
  local tmp_prog
  tmp_prog=$(mktemp)
  printf '%s' "$prog_json" > "$tmp_prog"

  if HAKO_STAGEB_FUNC_SCAN="${HAKO_STAGEB_FUNC_SCAN:-}" \
     HAKO_JOINIR_STRICT="$STAGEB_JOINIR_STRICT" HAKO_JOINIR_PLANNER_REQUIRED="$STAGEB_JOINIR_PLANNER_REQUIRED" \
     HAKO_MIR_BUILDER_FUNCS="${HAKO_MIR_BUILDER_FUNCS:-}" \
     HAKO_MIR_BUILDER_CALL_RESOLVE="${HAKO_MIR_BUILDER_CALL_RESOLVE:-}" \
     NYASH_JSON_SCHEMA_V1=${NYASH_JSON_SCHEMA_V1:-1} \
     NYASH_MIR_UNIFIED_CALL=${NYASH_MIR_UNIFIED_CALL:-1} \
     "$NYASH_BIN" --program-json-to-mir "$out_path" --json-file "$tmp_prog" >/dev/null 2>&1; then
    rm -f "$tmp_prog" || true
    echo "[OK] MIR JSON written (delegate-legacy): $out_path"
    return 0
  fi

  rm -f "$tmp_prog" || true
  return 1
}

emit_mir_json_via_delegate_routes() {
  local prog_json="$1" out_path="$2"
  if try_provider_emit "$prog_json" "$out_path"; then
    return 0
  fi
  if try_legacy_program_json_delegate "$prog_json" "$out_path"; then
    return 0
  fi
  return 1
}

try_selfhost_builder_first_route() {
  local prog_json="$1" out_path="$2"
  if [ "${HAKO_SELFHOST_BUILDER_FIRST:-0}" != "1" ]; then
    return 1
  fi
  if try_selfhost_builder "$prog_json" "$out_path"; then
    return 0
  fi
  if [ "${HAKO_SELFHOST_NO_DELEGATE:-0}" = "1" ]; then
    echo "[FAIL] selfhost-first failed and delegate disabled" >&2
    return 2
  fi
  return 1
}

try_loop_force_jsonfrag_route() {
  local prog_json="$1" out_path="$2"
  if [ "${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-0}" != "1" ]; then
    return 1
  fi
  local limit
  limit="$(extract_loop_force_limit_from_program_json "$prog_json")"
  write_loop_force_jsonfrag_mir_json "$limit" "$out_path"
  echo "[OK] MIR JSON written (provider-force-jsonfrag): $out_path"
  return 0
}

emit_mir_json_via_non_direct_routes() {
  local prog_json="$1" out_path="$2"
  local selfhost_rc=0

  try_selfhost_builder_first_route "$prog_json" "$out_path" || selfhost_rc=$?
  case "$selfhost_rc" in
    0)
      return 0
      ;;
    2)
      return 1
      ;;
  esac

  if try_loop_force_jsonfrag_route "$prog_json" "$out_path"; then
    return 0
  fi
  if emit_mir_json_via_delegate_routes "$prog_json" "$out_path"; then
    return 0
  fi
  return 1
}

# When forcing JSONFrag loop, default-enable normalize+purify (dev-only, no default changes)
if [ "${HAKO_MIR_BUILDER_LOOP_FORCE_JSONFRAG:-0}" = "1" ]; then
  export HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE="${HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE:-1}"
  export HAKO_MIR_BUILDER_JSONFRAG_PURIFY="${HAKO_MIR_BUILDER_JSONFRAG_PURIFY:-1}"
fi

if emit_mir_json_via_non_direct_routes "$PROG_JSON_OUT" "$OUT"; then
  exit 0
fi
echo "[FAIL] Program→MIR delegate failed (provider+legacy)" >&2
exit 1
